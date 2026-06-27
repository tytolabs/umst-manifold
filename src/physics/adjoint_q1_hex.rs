// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Discrete-adjoint **compliance** surrogate for SIMP-modulated **Q1 hex** continuum elasticity on a
//! Cartesian brick (`nx × ny × nz` cells).
//!
//! Forward equilibrium uses [`crate::physics::q1_hex_elasticity::hex_solve_pcg_masked`] on the **inner**
//! backend (no autodiff through PCG). Sensitivities reuse the same surrogate linearisation pattern as
//! [`crate::physics::adjoint::AdjointCompliance`]: element-wise factors `g_e ≈ ∂c/∂ρ_e` paired with
//! the differentiable relation between nodal `ρ` and element-averaged design density
//! `ρ_e = (1/8) Σ_{k∈corners} ρ_k`.

use burn::tensor::{
    backend::{AutodiffBackend, Backend},
    Data, Int, Shape, Tensor,
};

use super::adjoint::{
    AdjointComplianceDiagnostics, AdjointFiniteStageAudit, AdjointForwardPhaseTiming,
    HexPreconditionerKind, SimpElasticMaterial,
};
use super::linear::masked_dot;
use super::mechanics::{BarNetworkPcgReport, SelfWeightConfig};
use super::q1_hex_elasticity::{
    hex_cell_strain_energy, hex_equilibrium_rel_residual, hex_pcg_use_f64_lane,
    hex_solve_pcg_masked, HexPcgPrecondKind, HexStructuredOperatorCache,
};
use super::time_orchestration::MechanicsInnerLoopConfig;
use std::time::Instant;

/// Per-solve knobs for Q1-hex forward (warm-start / preconditioner selection).
#[derive(Clone, Debug, Default)]
pub struct Q1HexSolveOptions {
    /// When true and [`Self::pcg_seed_displacement`] matches DOF length, seed PCG with that `u`.
    pub pcg_warm_start: bool,
    pub pcg_seed_displacement: Option<Vec<f32>>,
    /// When `Some`, overrides Jacobi vs block-Jacobi for the matrix-free PCG call.
    pub precond_kind: Option<HexPreconditionerKind>,
    /// Reuse uniform-brick `ke_unit` for matrix-free `K·u` (metrics-match).
    pub use_operator_cache: bool,
}

fn map_hex_pcg_precond(kind: HexPreconditionerKind) -> HexPcgPrecondKind {
    match kind {
        HexPreconditionerKind::None => HexPcgPrecondKind::None,
        HexPreconditionerKind::JacobiDiagonal => HexPcgPrecondKind::JacobiDiagonal,
        HexPreconditionerKind::BlockJacobiNodal3x3 => HexPcgPrecondKind::BlockJacobiNodal3x3,
        HexPreconditionerKind::GeometricMultigridVCycle => HexPcgPrecondKind::GeometricMultigridVCycle,
    }
}

/// Discrete-adjoint compliance for extruded Q1-hex plates / bricks (batch **1**).
pub struct AdjointComplianceQ1Hex;

fn node_id(ix: usize, iy: usize, iz: usize, nx1: usize, ny1: usize) -> usize {
    ix + iy * nx1 + iz * nx1 * ny1
}

/// Corner-major indices for [`burn::tensor::Tensor::gather`] along the node axis (`[1, N, 1]` layout).
fn hex_cell_corner_gather_indices(nx: usize, ny: usize, nz: usize) -> Vec<i64> {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let mut v = Vec::with_capacity(nx * ny * nz * 8);
    for cz in 0..nz {
        for cy in 0..ny {
            for cx in 0..nx {
                for k in 0usize..8 {
                    let (ix, iy, iz) = match k {
                        0 => (cx, cy, cz),
                        1 => (cx + 1, cy, cz),
                        2 => (cx + 1, cy + 1, cz),
                        3 => (cx, cy + 1, cz),
                        4 => (cx, cy, cz + 1),
                        5 => (cx + 1, cy, cz + 1),
                        6 => (cx + 1, cy + 1, cz + 1),
                        7 => (cx, cy + 1, cz + 1),
                        _ => unreachable!(),
                    };
                    v.push(node_id(ix, iy, iz, nx1, ny1) as i64);
                }
            }
        }
    }
    v
}

fn count_nonfinite(v: &[f32]) -> usize {
    v.iter().filter(|x| !x.is_finite()).count()
}

fn audit_u_post_solve(u: &[f32], mask: &[f32]) -> (usize, usize, f32) {
    let mut u_nf = 0usize;
    let mut pinned_nf = 0usize;
    let mut pinned_abs_max = 0.0_f32;
    for (&ui, &m) in u.iter().zip(mask) {
        if !ui.is_finite() {
            u_nf += 1;
        }
        if m < 0.5 {
            if !ui.is_finite() {
                pinned_nf += 1;
            }
            pinned_abs_max = pinned_abs_max.max(ui.abs());
        }
    }
    (u_nf, pinned_nf, pinned_abs_max)
}

fn build_finite_audit(
    u: &[f32],
    mask: &[f32],
    ge: &[f32],
    nodal_sens: &[f32],
) -> AdjointFiniteStageAudit {
    let (u_nf, pinned_nf, pinned_max) = audit_u_post_solve(u, mask);
    let ge_nf = count_nonfinite(ge);
    let sens_nf = count_nonfinite(nodal_sens);
    let first_bad = if u_nf > 0 || pinned_nf > 0 {
        Some("u_post_solve")
    } else if ge_nf > 0 {
        Some("element_ge")
    } else if sens_nf > 0 {
        Some("nodal_scatter")
    } else {
        None
    };
    AdjointFiniteStageAudit {
        u_nonfinite: u_nf,
        u_pinned_nonfinite: pinned_nf,
        u_pinned_abs_max: pinned_max,
        ge_nonfinite: ge_nf,
        nodal_sens_nonfinite: sens_nf,
        first_bad_stage: first_bad,
    }
}

/// Bruyneel–Duysinx load derivative \(2\mathbf u^\top (\partial \mathbf f_{\mathrm{sw}}/\partial\rho)\) per node.
///
/// With \(\mathbf f_{\mathrm{sw}} = \rho^q V g \hat{\mathbf d}\), each nodal entry is
/// \(2 q \rho^{q-1} V g\, (\mathbf u\cdot\hat{\mathbf d})\).
#[must_use]
pub fn self_weight_load_nodal_sensitivity(
    u: &[f32],
    rho_flat: &[f32],
    n_nodes: usize,
    sw: &SelfWeightConfig,
) -> Vec<f32> {
    let [dx, dy, dz] = sw.direction;
    let mag = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-30);
    let ux = dx / mag;
    let uy = dy / mag;
    let uz = dz / mag;
    let pref = 2.0_f32 * sw.mass_penalty_q * sw.voxel_volume_m3 * sw.gravity_m_s2;
    let q = sw.mass_penalty_q;
    let mut sens = vec![0.0_f32; n_nodes];
    for i in 0..n_nodes.min(rho_flat.len()) {
        let rho = rho_flat[i].clamp(1e-30_f32, 1.0_f32);
        let rho_pow = if (q - 1.0).abs() < 1e-6 {
            1.0_f32
        } else {
            rho.powf(q - 1.0)
        };
        let u_dot_d = u[i * 3] * ux + u[i * 3 + 1] * uy + u[i * 3 + 2] * uz;
        sens[i] = pref * rho_pow * u_dot_d;
    }
    sens
}

fn nodal_sensitivity_from_cell_ge(
    ge: &[f32],
    nx: usize,
    ny: usize,
    nz: usize,
    n_nodes: usize,
) -> Vec<f32> {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let mut sens = vec![0.0_f32; n_nodes];
    for cz in 0..nz {
        for cy in 0..ny {
            for cx in 0..nx {
                let cidx = cx + cy * nx + cz * nx * ny;
                let g = ge[cidx];
                for (ix, iy, iz) in [
                    (cx, cy, cz),
                    (cx + 1, cy, cz),
                    (cx + 1, cy + 1, cz),
                    (cx, cy + 1, cz),
                    (cx, cy, cz + 1),
                    (cx + 1, cy, cz + 1),
                    (cx + 1, cy + 1, cz + 1),
                    (cx, cy + 1, cz + 1),
                ] {
                    let nid = node_id(ix, iy, iz, nx1, ny1);
                    sens[nid] += g * (1.0 / 8.0_f32);
                }
            }
        }
    }
    sens
}

pub(crate) struct HexForwardState {
    rho_e_law: Vec<f32>,
    u: Vec<f32>,
    ge: Vec<f32>,
    cell_strain_energy: Vec<f32>,
    pcg: BarNetworkPcgReport,
    eq_rel: f32,
    nodal_sensitivity: Vec<f32>,
    finite_audit: AdjointFiniteStageAudit,
    phase_timing: AdjointForwardPhaseTiming,
    precond_kind: HexPreconditionerKind,
}

/// Post-processing audit for B6 **c1** gate diagnosis (no autodiff).
#[derive(Clone, Debug)]
pub struct Q1HexComplianceAudit {
    pub compliance: f32,
    pub strain_energy_total: f32,
    pub cell_strain_energy: Vec<f32>,
    pub equilibrium_rel_residual: f32,
}

/// Spatial fractions for hypothesis **H-A** (top-face load over void columns).
#[derive(Clone, Debug)]
pub struct Q1HexTopVoidColumnFractions {
    /// \(\sum f_i u_i\) on free DOFs at top-face nodes in void columns / total compliance.
    pub compliance_fraction: f32,
    /// Sum of hex cell strain energy in the top layer over void columns / total strain energy.
    pub strain_energy_fraction: f32,
    /// Fraction of \((n_x{+}1)(n_y{+}1)\) columns with top-face \(\rho <\) threshold.
    pub void_column_fraction_xy: f32,
    pub void_rho_threshold: f32,
}

impl AdjointComplianceQ1Hex {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn forward_state_for_compliance(
        rho_flat: &[f32],
        nx: usize,
        ny: usize,
        nz: usize,
        dx: f32,
        dy: f32,
        dz: f32,
        f_flat: &[f32],
        m_flat: &[f32],
        material: SimpElasticMaterial,
        cg: &MechanicsInnerLoopConfig,
        self_weight: Option<SelfWeightConfig>,
        solve_options: &Q1HexSolveOptions,
    ) -> HexForwardState {
        let nx1 = nx + 1;
        let ny1 = ny + 1;
        let n_nodes = nx1 * ny1 * (nz + 1);
        let n_cells = nx * ny * nz;

        let t_assemble = Instant::now();
        let mut e_cell = vec![0.0_f32; n_cells];
        let mut rho_e_law = vec![0.0_f32; n_cells];

        for cz in 0..nz {
            for cy in 0..ny {
                for cx in 0..nx {
                    let mut sum = 0.0_f32;
                    for (ix, iy, iz) in [
                        (cx, cy, cz),
                        (cx + 1, cy, cz),
                        (cx + 1, cy + 1, cz),
                        (cx, cy + 1, cz),
                        (cx, cy, cz + 1),
                        (cx + 1, cy, cz + 1),
                        (cx + 1, cy + 1, cz + 1),
                        (cx, cy + 1, cz + 1),
                    ] {
                        let nid = node_id(ix, iy, iz, nx1, ny1);
                        sum += rho_flat[nid];
                    }
                    let rho_e = sum * (1.0 / 8.0_f32);
                    let rho_clamped = rho_e.clamp(0.0_f32, 1.0_f32);
                    let e_e = rho_clamped.powf(material.p) * (material.e0 - material.e_min)
                        + material.e_min;
                    let cidx = cx + cy * nx + cz * nx * ny;
                    e_cell[cidx] = e_e;
                    rho_e_law[cidx] = rho_clamped;
                }
            }
        }

        let assemble_ms = t_assemble.elapsed().as_secs_f64() * 1000.0;

        let mut u = if solve_options.pcg_warm_start {
            if let Some(seed) = &solve_options.pcg_seed_displacement {
                if seed.len() == n_nodes * 3 {
                    seed.clone()
                } else {
                    vec![0.0_f32; n_nodes * 3]
                }
            } else {
                vec![0.0_f32; n_nodes * 3]
            }
        } else {
            vec![0.0_f32; n_nodes * 3]
        };
        let mut diag = vec![0.0_f32; n_nodes * 3];
        let mut scratch = vec![0.0_f32; n_nodes * 3];
        let max_it = cg.max_cg_iterations.max(1);
        let rel_tol = cg.pcg_tolerance.max(cg.cg_tolerance);
        let precond_kind = solve_options.precond_kind.unwrap_or_else(|| {
            HexPreconditionerKind::from_use_preconditioner(cg.use_preconditioner)
        });

        let op_cache_holder = if solve_options.use_operator_cache {
            Some(HexStructuredOperatorCache::new(
                nx, ny, nz, dx, dy, dz, material.nu,
            ))
        } else {
            None
        };
        let op_cache_ref = op_cache_holder.as_ref();

        let t_pcg = Instant::now();
        let hex_pcg = hex_solve_pcg_masked(
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            material.nu,
            &e_cell,
            f_flat,
            m_flat,
            &mut u,
            &mut diag,
            &mut scratch,
            max_it,
            map_hex_pcg_precond(precond_kind),
            rel_tol,
            op_cache_ref,
        );
        let pcg_ms = t_pcg.elapsed().as_secs_f64() * 1000.0;
        let t_adjoint = Instant::now();

        // f64 lane: bind on solver `rel_residual` (f64 `u64` before f32 round-trip).
        let eq_rel = if hex_pcg_use_f64_lane(nx, ny, nz) {
            hex_pcg.rel_residual
        } else {
            hex_equilibrium_rel_residual(
                nx,
                ny,
                nz,
                dx,
                dy,
                dz,
                material.nu,
                &e_cell,
                f_flat,
                m_flat,
                &u,
            )
        };

        let mut u_cell_energy = vec![0.0_f32; n_cells];
        hex_cell_strain_energy(
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            material.nu,
            &e_cell,
            &u,
            &mut u_cell_energy,
        );

        let mut ge = vec![0.0_f32; n_cells];
        for c in 0..n_cells {
            let e_e = e_cell[c].max(1e-30_f32);
            let rho_c = rho_e_law[c];
            let dk_drho =
                material.p * (material.e0 - material.e_min) * rho_c.powf(material.p - 1.0_f32);
            let psi = 2.0_f32 * u_cell_energy[c] / e_e.max(1e-30_f32);
            ge[c] = -dk_drho * psi;
        }

        let mut nodal_sensitivity = nodal_sensitivity_from_cell_ge(&ge, nx, ny, nz, n_nodes);
        if let Some(sw) = self_weight {
            let load_sens = self_weight_load_nodal_sensitivity(&u, rho_flat, n_nodes, &sw);
            for (s, l) in nodal_sensitivity.iter_mut().zip(load_sens) {
                *s += l;
            }
        }
        let finite_audit = build_finite_audit(&u, m_flat, &ge, &nodal_sensitivity);
        let pcg = BarNetworkPcgReport {
            iterations: hex_pcg.iterations,
            rel_residual: hex_pcg.rel_residual,
            stiffness_scale: 1.0,
            e_ref: material.e0,
            dx_char: dx.min(dy).min(dz),
        };

        let adjoint_ms = t_adjoint.elapsed().as_secs_f64() * 1000.0;
        let phase_timing = AdjointForwardPhaseTiming {
            assemble_ms,
            pcg_ms,
            adjoint_ms,
        };

        HexForwardState {
            rho_e_law,
            u,
            ge,
            cell_strain_energy: u_cell_energy,
            pcg,
            eq_rel,
            nodal_sensitivity,
            finite_audit,
            phase_timing,
            precond_kind,
        }
    }

    /// Forward equilibrium + compliance scalar for reference layouts (post-processing only).
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_compliance(
        rho_flat: &[f32],
        nx: usize,
        ny: usize,
        nz: usize,
        dx: f32,
        dy: f32,
        dz: f32,
        f_flat: &[f32],
        m_flat: &[f32],
        material: SimpElasticMaterial,
        cg: &MechanicsInnerLoopConfig,
        self_weight: Option<SelfWeightConfig>,
    ) -> (Q1HexComplianceAudit, Vec<f32>) {
        let state = Self::forward_state_for_compliance(
            rho_flat,
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            f_flat,
            m_flat,
            material,
            cg,
            self_weight,
            &Q1HexSolveOptions::default(),
        );
        let mut compliance = 0.0_f32;
        for i in 0..f_flat.len().min(state.u.len()).min(m_flat.len()) {
            if m_flat[i] > 0.5 {
                compliance += f_flat[i] * state.u[i];
            }
        }
        let strain_energy_total = state.cell_strain_energy.iter().sum();
        let audit = Q1HexComplianceAudit {
            compliance,
            strain_energy_total,
            cell_strain_energy: state.cell_strain_energy,
            equilibrium_rel_residual: state.eq_rel,
        };
        (audit, state.u)
    }

    /// Top-layer void-column fractions for **H-A** (roof load on non-design skin).
    #[allow(clippy::too_many_arguments)]
    pub fn top_void_column_fractions(
        audit: &Q1HexComplianceAudit,
        u: &[f32],
        rho_flat: &[f32],
        nx: usize,
        ny: usize,
        nz: usize,
        f_flat: &[f32],
        m_flat: &[f32],
        void_rho_threshold: f32,
    ) -> Q1HexTopVoidColumnFractions {
        let nx1 = nx + 1;
        let ny1 = ny + 1;
        let iz_top = nz;
        let mut void_cols = vec![false; nx1 * ny1];
        let mut n_void_cols = 0usize;
        for iy in 0..ny1 {
            for ix in 0..nx1 {
                let nid = ix + iy * nx1 + iz_top * nx1 * ny1;
                let rho_top = rho_flat.get(nid).copied().unwrap_or(1.0);
                let is_void = rho_top < void_rho_threshold;
                void_cols[ix + iy * nx1] = is_void;
                if is_void {
                    n_void_cols += 1;
                }
            }
        }
        let void_column_fraction_xy = n_void_cols as f32 / (nx1 * ny1).max(1) as f32;

        let mut comp_void_top = 0.0_f32;
        let mut comp_total = 0.0_f32;
        for iy in 0..ny1 {
            for ix in 0..nx1 {
                let nid = ix + iy * nx1 + iz_top * nx1 * ny1;
                if !void_cols[ix + iy * nx1] {
                    continue;
                }
                for d in 0..3 {
                    let idx = nid * 3 + d;
                    if idx < f_flat.len()
                        && idx < u.len()
                        && idx < m_flat.len()
                        && m_flat[idx] > 0.5
                    {
                        comp_void_top += f_flat[idx] * u[idx];
                    }
                }
            }
        }
        for i in 0..f_flat.len().min(u.len()).min(m_flat.len()) {
            if m_flat[i] > 0.5 {
                comp_total += f_flat[i] * u[i];
            }
        }
        let compliance_fraction = if comp_total.abs() > 1e-30 {
            comp_void_top / comp_total
        } else {
            f32::NAN
        };

        let mut se_void_top = 0.0_f32;
        for cz in nz.saturating_sub(1)..nz {
            for cy in 0..ny {
                for cx in 0..nx {
                    let col_ix = cx + 1;
                    let col_iy = cy + 1;
                    let void = void_cols
                        .get(col_ix + col_iy * nx1)
                        .copied()
                        .unwrap_or(false)
                        || void_cols.get(cx + col_iy * nx1).copied().unwrap_or(false)
                        || void_cols.get(col_ix + cy * nx1).copied().unwrap_or(false)
                        || void_cols.get(cx + cy * nx1).copied().unwrap_or(false);
                    if void {
                        let cidx = cx + cy * nx + cz * nx * ny;
                        se_void_top += audit.cell_strain_energy.get(cidx).copied().unwrap_or(0.0);
                    }
                }
            }
        }
        let strain_energy_fraction = if audit.strain_energy_total.abs() > 1e-30 {
            se_void_top / audit.strain_energy_total
        } else {
            f32::NAN
        };

        Q1HexTopVoidColumnFractions {
            compliance_fraction,
            strain_energy_fraction,
            void_column_fraction_xy,
            void_rho_threshold,
        }
    }

    /// Returns `(surrogate_loss, raw_compliance)` where `surrogate_loss` backpropagates like
    /// `∂c/∂ρ` for mean nodal SIMP per hex (`ρ_e` from eight corners), batch size **1**.
    #[allow(clippy::too_many_arguments)]
    pub fn forward_and_loss<B>(
        rho_autodiff: Tensor<B, 3>,
        nx: usize,
        ny: usize,
        nz: usize,
        dx: f32,
        dy: f32,
        dz: f32,
        body_force: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        boundary_mask: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        material: SimpElasticMaterial,
        cg: &MechanicsInnerLoopConfig,
        self_weight: Option<SelfWeightConfig>,
    ) -> (Tensor<B, 1>, f32)
    where
        B: AutodiffBackend<FloatElem = f32>,
        B::InnerBackend: Backend<FloatElem = f32>,
    {
        let (surrogate, c_raw, _) = Self::forward_loss_with_diagnostics(
            rho_autodiff,
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            body_force,
            boundary_mask,
            material,
            cg,
            self_weight,
            &Q1HexSolveOptions::default(),
        );
        (surrogate, c_raw)
    }

    /// Same as [`Self::forward_and_loss`] plus PCG / equilibrium / nodal sensitivity telemetry (B6 H4).
    #[allow(clippy::too_many_arguments)]
    pub fn forward_loss_with_diagnostics<B>(
        rho_autodiff: Tensor<B, 3>,
        nx: usize,
        ny: usize,
        nz: usize,
        dx: f32,
        dy: f32,
        dz: f32,
        body_force: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        boundary_mask: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        material: SimpElasticMaterial,
        cg: &MechanicsInnerLoopConfig,
        self_weight: Option<SelfWeightConfig>,
        solve_options: &Q1HexSolveOptions,
    ) -> (Tensor<B, 1>, f32, AdjointComplianceDiagnostics)
    where
        B: AutodiffBackend<FloatElem = f32>,
        B::InnerBackend: Backend<FloatElem = f32>,
    {
        let nx1 = nx + 1;
        let ny1 = ny + 1;
        let n_nodes = nx1 * ny1 * (nz + 1);
        debug_assert_eq!(
            rho_autodiff.dims(),
            [1, n_nodes, 1],
            "AdjointComplianceQ1Hex: rho shape must be [1, n_nodes, 1]"
        );

        let rho_inner = rho_autodiff.clone().inner();
        let rho_flat = rho_inner.clone().into_data().value;
        let f_flat = body_force.clone().into_data().value;
        let m_flat = boundary_mask.clone().into_data().value;
        debug_assert_eq!(f_flat.len(), n_nodes * 3);
        debug_assert_eq!(m_flat.len(), n_nodes * 3);

        let state = Self::forward_state_for_compliance(
            &rho_flat,
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            &f_flat,
            &m_flat,
            material,
            cg,
            self_weight,
            solve_options,
        );

        let device = rho_autodiff.device();
        let u_tensor_inner = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::from_data(
            Data::new(state.u.clone(), Shape::new([1, n_nodes, 3])),
            &device,
        );

        let comp = masked_dot(&body_force, &u_tensor_inner, &boundary_mask);

        let sens_ad = Tensor::<B, 3>::from_inner(
            Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::from_data(
                Data::new(state.nodal_sensitivity.clone(), Shape::new([1, n_nodes, 1])),
                &device,
            ),
        );
        let rho_det_ad = Tensor::<B, 3>::from_inner(Tensor::<
            <B as AutodiffBackend>::InnerBackend,
            3,
        >::from_data(
            Data::new(rho_flat.clone(), Shape::new([1, n_nodes, 1])),
            &device,
        ));
        let lin_a = rho_autodiff.clone().mul(sens_ad.clone()).sum();
        let lin_b = rho_det_ad.mul(sens_ad).sum();
        let c_pad = Tensor::<B, 1>::from_inner(comp.clone());
        let surrogate = lin_a.sub(lin_b).add(c_pad).reshape([1]);
        let c_raw = comp.into_scalar();

        let diag = AdjointComplianceDiagnostics {
            pcg: state.pcg,
            pcg_iters: state.pcg.iterations,
            equilibrium_rel_residual: state.eq_rel,
            nodal_sensitivity: state.nodal_sensitivity,
            finite_audit: Some(state.finite_audit),
            phase_timing: state.phase_timing,
            precond_kind: state.precond_kind,
            equilibrium_displacement: state.u,
        };

        (surrogate, c_raw, diag)
    }

    /// Host diagnostics bundle at fixed nodal ρ (no autodiff).
    #[allow(clippy::too_many_arguments)]
    pub fn compliance_diagnostics_at_rho(
        rho_flat: &[f32],
        nx: usize,
        ny: usize,
        nz: usize,
        dx: f32,
        dy: f32,
        dz: f32,
        f_flat: &[f32],
        m_flat: &[f32],
        material: SimpElasticMaterial,
        cg: &MechanicsInnerLoopConfig,
        self_weight: Option<SelfWeightConfig>,
    ) -> AdjointComplianceDiagnostics {
        let state = Self::forward_state_for_compliance(
            rho_flat,
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            f_flat,
            m_flat,
            material,
            cg,
            self_weight,
            &Q1HexSolveOptions::default(),
        );
        AdjointComplianceDiagnostics {
            pcg: state.pcg,
            pcg_iters: state.pcg.iterations,
            equilibrium_rel_residual: state.eq_rel,
            nodal_sensitivity: state.nodal_sensitivity,
            finite_audit: Some(state.finite_audit),
            phase_timing: state.phase_timing,
            precond_kind: state.precond_kind,
            equilibrium_displacement: state.u,
        }
    }

    /// Inner-only compliance `f^T u` at fixed nodal ρ (finite-difference baseline; no autodiff).
    #[allow(clippy::too_many_arguments)]
    pub fn raw_compliance_at_rho(
        rho_flat: &[f32],
        nx: usize,
        ny: usize,
        nz: usize,
        dx: f32,
        dy: f32,
        dz: f32,
        f_flat: &[f32],
        m_flat: &[f32],
        material: SimpElasticMaterial,
        cg: &MechanicsInnerLoopConfig,
        self_weight: Option<SelfWeightConfig>,
    ) -> f32 {
        let state = Self::forward_state_for_compliance(
            rho_flat,
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            f_flat,
            m_flat,
            material,
            cg,
            self_weight,
            &Q1HexSolveOptions::default(),
        );
        let n_nodes = (nx + 1) * (ny + 1) * (nz + 1);
        debug_assert_eq!(f_flat.len(), n_nodes * 3);
        debug_assert_eq!(m_flat.len(), n_nodes * 3);
        let mut comp = 0.0_f32;
        for i in 0..n_nodes {
            for d in 0..3 {
                let k = i * 3 + d;
                if m_flat[k] > 0.5 {
                    comp += f_flat[k] * state.u[k];
                }
            }
        }
        comp
    }

    /// Retired gather surrogate for regression tests (H5 stage d).
    #[doc(hidden)]
    #[allow(clippy::too_many_arguments)]
    pub fn forward_gather_surrogate_for_test<B>(
        rho_autodiff: Tensor<B, 3>,
        nx: usize,
        ny: usize,
        nz: usize,
        dx: f32,
        dy: f32,
        dz: f32,
        body_force: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        boundary_mask: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        material: SimpElasticMaterial,
        cg: &MechanicsInnerLoopConfig,
        self_weight: Option<SelfWeightConfig>,
    ) -> Tensor<B, 1>
    where
        B: AutodiffBackend<FloatElem = f32>,
        B::InnerBackend: Backend<FloatElem = f32>,
    {
        let nx1 = nx + 1;
        let ny1 = ny + 1;
        let n_nodes = nx1 * ny1 * (nz + 1);
        let n_cells = nx * ny * nz;
        let rho_inner = rho_autodiff.clone().inner();
        let rho_flat = rho_inner.into_data().value;
        let f_flat = body_force.clone().into_data().value;
        let m_flat = boundary_mask.clone().into_data().value;
        let state = Self::forward_state_for_compliance(
            &rho_flat,
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            &f_flat,
            &m_flat,
            material,
            cg,
            self_weight,
            &Q1HexSolveOptions::default(),
        );
        let device = rho_autodiff.device();
        let u_tensor_inner = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::from_data(
            Data::new(state.u.clone(), Shape::new([1, n_nodes, 3])),
            &device,
        );
        let comp = masked_dot(&body_force, &u_tensor_inner, &boundary_mask);
        let idx_flat = hex_cell_corner_gather_indices(nx, ny, nz);
        let ids_i32: Vec<i32> = idx_flat.iter().map(|&x| x as i32).collect();
        let idx_inner = Tensor::<<B as AutodiffBackend>::InnerBackend, 1, Int>::from_ints(
            ids_i32.as_slice(),
            &device,
        )
        .reshape([1, n_cells * 8, 1]);
        let idx_tensor = Tensor::<B, 3, Int>::from_inner(idx_inner);
        let rho_e_ad = rho_autodiff
            .gather(1, idx_tensor)
            .reshape([1, n_cells, 8])
            .sum_dim(2)
            .div_scalar(8.0_f32)
            .reshape([1, n_cells, 1]);
        let rho_e_det_ad = Tensor::<B, 3>::from_inner(Tensor::<
            <B as AutodiffBackend>::InnerBackend,
            3,
        >::from_data(
            Data::new(state.rho_e_law.clone(), Shape::new([1, n_cells, 1])),
            &device,
        ));
        let ge_ad = Tensor::<B, 3>::from_inner(
            Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::from_data(
                Data::new(state.ge.clone(), Shape::new([1, n_cells, 1])),
                &device,
            ),
        );
        let lin_a = ge_ad.clone().mul(rho_e_ad).sum();
        let lin_b = ge_ad.mul(rho_e_det_ad).sum();
        let c_pad = Tensor::<B, 1>::from_inner(comp);
        lin_a.sub(lin_b).add(c_pad).reshape([1])
    }
}
