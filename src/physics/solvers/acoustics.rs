// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Acoustic / elastodynamic wave propagation (Phase 8).
//!
//! Newmark-β implicit integration for **M ü + C u̇ + K u = F** on nodal vectors `[B, N, 3]`.
//! - **Mass** `M` is a per-node **3×3 block diagonal** `[B, N, 3, 3]` built from volumetric density
//!   `ρ` and caller-supplied nodal control volume `V` (kg = ρ·V per node; isotropic blocks `m·I`).
//! - **Stiffness** `K` combines an optional **local** Hooke-style nodal operator (same tensor contract
//!   as the historical scaffold) with the **axial bar-network** graph matvec
//!   [`crate::physics::mechanics::VectorMechanicsSolver::bar_matvec`] — the same scatter-sum pattern
//!   as [`crate::physics::laplacian::TopologicalLaplacian::scalar_laplacian`].
//! - **Damping** `C` is a per-node block diagonal `[B, N, 3, 3]` (Rayleigh-style blocks can be folded
//!   in by the caller).
//!
//! When a bar graph is present, the effective operator
//! **S = M + γΔt C + βΔt² K** is applied matrix-free with host **[`super::krylov_host::gmres_f32_try`**].
//! When no graph is supplied and `S` is block-diagonal per node, a closed-form **batched 3×3**
//! solve is used (differentiable through Burn).

use burn::tensor::{backend::Backend, Int, Tensor};

#[cfg(feature = "acoustics-newmark")]
use burn::tensor::{Data, Shape};

#[cfg(feature = "acoustics-newmark")]
use crate::core::iterate_until::iterate_until;

/// Semi-discrete acoustic / elastic wave integrator (Newmark family).
///
/// [`Self::newmark_beta`] and [`Self::newmark_gamma`] select the implicitness / damping of the
/// average-acceleration scheme (e.g. average acceleration: β = ¼, γ = ½).
pub struct AcousticWaveSolver {
    /// Physical time step Δt (seconds).
    pub dt: f32,
    /// Newmark β parameter (scheme-dependent; typical ¼ for unconditional stability with γ = ½).
    pub newmark_beta: f32,
    /// Newmark γ parameter (typically ½ for second-order accuracy).
    pub newmark_gamma: f32,
}

/// Host GMRES controls for the implicit Newmark acceleration solve when a **bar graph** is active.
#[derive(Debug, Clone, Copy)]
pub struct AcousticGmresConfig {
    pub max_iter: usize,
    pub rel_tol: f32,
}

impl Default for AcousticGmresConfig {
    fn default() -> Self {
        Self {
            max_iter: 256,
            rel_tol: 1e-4_f32,
        }
    }
}

/// Cached axial bar-network tensors for [`AcousticWaveSolver::step_wave`] (same assembly recipe as
/// quasi-static mechanics on the DEC 1-skeleton).
#[derive(Debug, Clone)]
pub struct AcousticBarNetwork<B: Backend> {
    pub n_v: usize,
    pub k_axial: Tensor<B, 3>,
    pub edge_unit: Tensor<B, 3>,
    pub src_indices: Tensor<B, 3, Int>,
    pub tgt_indices: Tensor<B, 3, Int>,
    pub edge_len: Tensor<B, 3>,
}

impl<B: Backend<FloatElem = f32>> AcousticBarNetwork<B> {
    /// Assemble edge axial stiffness and unit tangents from nodal Young's modulus, damage, and geometry.
    ///
    /// * `coords_n3` — `[N, 3]` reference coordinates.
    /// * `edges_b1` — `[2, E]` endpoint indices (same layout as mechanics / Laplacian).
    /// * `youngs_bn1` — `[B, N, 1]` Pa.
    /// * `damage_bn1` — `[B, N, 1]` in `[0, 1]`.
    pub fn assemble_axial_bar_graph(
        coords_n3: Tensor<B, 2>,
        edges_b1: Tensor<B, 2, Int>,
        youngs_bn1: Tensor<B, 3>,
        damage_bn1: Tensor<B, 3>,
        cross_section_area: f32,
    ) -> Self {
        use crate::physics::dec_operators::DecEdgeOperators;
        use crate::physics::topology::EdgeTopology;

        const DAMAGE_REG: f32 = 1e-6;

        let n_v = coords_n3.dims()[0];
        let batch = youngs_bn1.dims()[0];
        let topo = EdgeTopology::new(edges_b1.clone());
        let n_edges = topo.n_edges();

        let coords_b = coords_n3
            .clone()
            .unsqueeze_dim::<3>(0)
            .expand([batch, n_v, 3]);

        let src_indices = topo.expand_src_gather_indices(batch, 3);
        let tgt_indices = topo.expand_tgt_gather_indices(batch, 3);

        let c_src = coords_b.clone().gather(1, src_indices.clone());
        let c_tgt = coords_b.gather(1, tgt_indices.clone());
        let delta_geom = c_tgt.sub(c_src);
        let edge_len = delta_geom
            .clone()
            .powf_scalar(2.0)
            .sum_dim(2)
            .sqrt()
            .clamp(1e-12, f32::MAX)
            .reshape([batch, n_edges, 1]);
        let edge_unit = delta_geom.div(edge_len.clone());

        let e_on_edges =
            DecEdgeOperators::arithmetic_mean_on_edges(youngs_bn1.clone(), edges_b1.clone());
        let d_on_edges =
            DecEdgeOperators::arithmetic_mean_on_edges(damage_bn1.clone(), edges_b1.clone());
        let dmg = Tensor::ones_like(&d_on_edges)
            .sub(d_on_edges)
            .powf_scalar(2.0)
            .add_scalar(DAMAGE_REG);

        let k_axial = e_on_edges
            .mul_scalar(cross_section_area)
            .div(edge_len.clone())
            .mul(dmg);

        Self {
            n_v,
            k_axial,
            edge_unit,
            src_indices,
            tgt_indices,
            edge_len,
        }
    }
}

impl AcousticWaveSolver {
    /// One implicit Newmark-β step for **M ü + C u̇ + K u = F**.
    ///
    /// # Tensor shapes
    ///
    /// | Tensor | Shape | Role |
    /// |--------|-------|------|
    /// | `displacement`, `velocity`, `acceleration` | `[B, N, 3]` | **u**, **u̇**, **ü** |
    /// | `nodal_density` | `[B, N, 1]` | Mass density ρ (kg/m³) |
    /// | `nodal_volume` | `[B, N, 1]` | Nodal control volume V (m³); nodal mass scalar m = ρ·V |
    /// | `body_force` | `[B, N, 3]` | **F** (N) |
    /// | `damping_bn33` | `[B, N, 3, 3]` | Block-diagonal **C** |
    /// | `stiffness_local_bn44` | `[B, N, 3, 3]` | Extra nodal stiffness (Hooke-style `(K_loc u)[b,n,:] = K[b,n,:,:]·u[b,n,:]`) |
    ///
    /// When `bar_network` is **`None`**, the effective Jacobian per node is **3×3** and is solved
    /// in batch tensor form. When **`Some`**, a matrix-free **GMRES** step on the packed `[3N]`
    /// vector is used (host bridge; autodiff does not cross the Krylov solve).
    ///
    /// ## Default builds (`acoustics-newmark` **off**)
    /// Returns the inputs unchanged.
    #[allow(clippy::too_many_arguments)]
    pub fn step_wave<B: Backend<FloatElem = f32>>(
        &self,
        displacement: Tensor<B, 3>,
        velocity: Tensor<B, 3>,
        acceleration: Tensor<B, 3>,
        nodal_density: Tensor<B, 3>,
        nodal_volume: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        damping_bn33: Tensor<B, 4>,
        stiffness_local_bn44: Tensor<B, 4>,
        bar_network: Option<AcousticBarNetwork<B>>,
        gmres_cfg: Option<AcousticGmresConfig>,
    ) -> (Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>) {
        #[cfg(not(feature = "acoustics-newmark"))]
        {
            let _ = (
                self.dt,
                self.newmark_beta,
                self.newmark_gamma,
                nodal_density,
                nodal_volume,
                body_force,
                damping_bn33,
                stiffness_local_bn44,
            );
            let _ = (bar_network, gmres_cfg);
            (displacement, velocity, acceleration)
        }

        #[cfg(feature = "acoustics-newmark")]
        {
            step_wave_experimental(
                self,
                displacement,
                velocity,
                acceleration,
                nodal_density,
                nodal_volume,
                body_force,
                damping_bn33,
                stiffness_local_bn44,
                bar_network,
                gmres_cfg.unwrap_or_default(),
            )
        }
    }

    /// Run **`num_steps`** Newmark steps using [`iterate_until`] (bounded driver; autodiff-friendly
    /// when each step stays on the tensor-only dense path).
    #[cfg(feature = "acoustics-newmark")]
    #[allow(clippy::too_many_arguments)]
    pub fn step_wave_iterate<B: Backend<FloatElem = f32>>(
        &self,
        num_steps: usize,
        displacement: Tensor<B, 3>,
        velocity: Tensor<B, 3>,
        acceleration: Tensor<B, 3>,
        nodal_density: Tensor<B, 3>,
        nodal_volume: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        damping_bn33: Tensor<B, 4>,
        stiffness_local_bn44: Tensor<B, 4>,
        bar_network: Option<AcousticBarNetwork<B>>,
        gmres_cfg: Option<AcousticGmresConfig>,
    ) -> (Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>, usize) {
        let mut st = (
            displacement,
            velocity,
            acceleration,
            nodal_density,
            nodal_volume,
            body_force,
            damping_bn33,
            stiffness_local_bn44,
            bar_network,
            gmres_cfg,
        );
        let k = iterate_until(num_steps, &mut st, |s| {
            let (
                ref mut u,
                ref mut v,
                ref mut a,
                ref rho,
                ref vol,
                ref f,
                ref damp,
                ref kloc,
                ref bar,
                ref gcfg,
            ) = *s;
            let (un, vn, an) = self.step_wave(
                u.clone(),
                v.clone(),
                a.clone(),
                rho.clone(),
                vol.clone(),
                f.clone(),
                damp.clone(),
                kloc.clone(),
                bar.clone(),
                gcfg.as_ref().copied(),
            );
            *u = un;
            *v = vn;
            *a = an;
            core::ops::ControlFlow::Continue(())
        });
        let (u, v, a, ..) = st;
        (u, v, a, k)
    }
}

#[cfg(feature = "acoustics-newmark")]
const DET_EPS: f32 = 1e-30_f32;

#[cfg(feature = "acoustics-newmark")]
fn elem_bn33<B: Backend<FloatElem = f32>>(m: &Tensor<B, 4>, i: usize, j: usize) -> Tensor<B, 3> {
    m.clone().narrow(2, i, 1).narrow(3, j, 1).squeeze::<3>(3)
}

/// `(K_loc u)[b,n,:] = Σ_j K[b,n,i,j] u[b,n,j]` with shapes `[B,N,3]`.
#[cfg(feature = "acoustics-newmark")]
fn contract_block33_displacement<B: Backend<FloatElem = f32>>(
    stiffness: Tensor<B, 4>,
    displacement: Tensor<B, 3>,
) -> Tensor<B, 3> {
    let u_col = displacement.unsqueeze_dim::<4>(3);
    stiffness.matmul(u_col).squeeze::<3>(3)
}

/// Per-node nodal mass matrix **m I₃** with `m = ρ · V` (kg), shape `[B, N, 3, 3]`.
#[cfg(feature = "acoustics-newmark")]
pub fn nodal_mass_matrix_bn33<B: Backend<FloatElem = f32>>(
    nodal_density_bn1: Tensor<B, 3>,
    nodal_volume_bn1: Tensor<B, 3>,
) -> Tensor<B, 4> {
    let device = nodal_density_bn1.device();
    let m_scalar = nodal_density_bn1.mul(nodal_volume_bn1);
    let zero = Tensor::<B, 3>::zeros(m_scalar.dims(), &device);
    let row0 = Tensor::cat(vec![m_scalar.clone(), zero.clone(), zero.clone()], 2);
    let row1 = Tensor::cat(vec![zero.clone(), m_scalar.clone(), zero.clone()], 2);
    let row2 = Tensor::cat(vec![zero.clone(), zero, m_scalar], 2);
    Tensor::cat(
        vec![
            row0.unsqueeze_dim::<4>(2),
            row1.unsqueeze_dim::<4>(2),
            row2.unsqueeze_dim::<4>(2),
        ],
        2,
    )
}

/// Kinetic energy **½ u̇ᵀ M u̇** as a rank-1 tensor `[B, 1]` (sum over nodes and spatial components).
#[cfg(feature = "acoustics-newmark")]
pub fn nodal_kinetic_energy_bn1<B: Backend<FloatElem = f32>>(
    velocity_bn3: Tensor<B, 3>,
    mass_bn44: Tensor<B, 4>,
) -> Tensor<B, 2> {
    let batch_size = velocity_bn3.dims()[0];
    let v_col = velocity_bn3.clone().unsqueeze_dim::<4>(3);
    let mv = mass_bn44.matmul(v_col).squeeze::<3>(3);
    velocity_bn3
        .mul(mv)
        .sum_dim(2)
        .sum_dim(1)
        .mul_scalar(0.5_f32)
        .reshape([batch_size, 1])
}

#[cfg(feature = "acoustics-newmark")]
fn apply_mass_block33<B: Backend<FloatElem = f32>>(
    mass_bn44: &Tensor<B, 4>,
    x_bn3: Tensor<B, 3>,
) -> Tensor<B, 3> {
    let xc = x_bn3.unsqueeze_dim::<4>(3);
    mass_bn44.clone().matmul(xc).squeeze::<3>(3)
}

#[cfg(feature = "acoustics-newmark")]
fn apply_effective_operator_acceleration<B: Backend<FloatElem = f32>>(
    trial_a_bn3: Tensor<B, 3>,
    mass_bn44: &Tensor<B, 4>,
    damping_bn44: &Tensor<B, 4>,
    stiffness_local_bn44: &Tensor<B, 4>,
    gamma_dt: f32,
    beta_dt2: f32,
    bar: Option<&AcousticBarNetwork<B>>,
) -> Tensor<B, 3> {
    let ma = apply_mass_block33(mass_bn44, trial_a_bn3.clone());
    let ca = contract_block33_displacement(damping_bn44.clone(), trial_a_bn3.clone())
        .mul_scalar(gamma_dt);
    let k_loc_a = contract_block33_displacement(stiffness_local_bn44.clone(), trial_a_bn3.clone())
        .mul_scalar(beta_dt2);
    let mut out = ma.add(ca).add(k_loc_a);
    if let Some(bn) = bar {
        let batch = trial_a_bn3.dims()[0];
        let n_v = bn.n_v;
        debug_assert_eq!(trial_a_bn3.dims(), [batch, n_v, 3]);
        let k_bar = crate::physics::mechanics::VectorMechanicsSolver::bar_matvec(
            trial_a_bn3,
            &bn.k_axial,
            &bn.edge_unit,
            &bn.src_indices,
            &bn.tgt_indices,
            n_v,
            None,
            &bn.edge_len,
        )
        .mul_scalar(beta_dt2);
        out = out.add(k_bar);
    }
    out
}

/// Returns `S^{-1} rhs` for batched 3×3 systems `S · x = rhs`, shapes `[B,N,3,3]` and `[B,N,3]`.
#[cfg(feature = "acoustics-newmark")]
fn solve_batched_3x3<B: Backend<FloatElem = f32>>(
    s: Tensor<B, 4>,
    rhs_bn3: Tensor<B, 3>,
) -> Tensor<B, 3> {
    let s00 = elem_bn33(&s, 0, 0);
    let s01 = elem_bn33(&s, 0, 1);
    let s02 = elem_bn33(&s, 0, 2);
    let s10 = elem_bn33(&s, 1, 0);
    let s11 = elem_bn33(&s, 1, 1);
    let s12 = elem_bn33(&s, 1, 2);
    let s20 = elem_bn33(&s, 2, 0);
    let s21 = elem_bn33(&s, 2, 1);
    let s22 = elem_bn33(&s, 2, 2);

    let det = s00
        .clone()
        .mul(
            s11.clone()
                .mul(s22.clone())
                .sub(s12.clone().mul(s21.clone())),
        )
        .sub(
            s01.clone().mul(
                s10.clone()
                    .mul(s22.clone())
                    .sub(s12.clone().mul(s20.clone())),
            ),
        )
        .add(
            s02.clone().mul(
                s10.clone()
                    .mul(s21.clone())
                    .sub(s11.clone().mul(s20.clone())),
            ),
        );
    let det_safe = det.clone().clamp_min(DET_EPS);

    let cof00 = s11
        .clone()
        .mul(s22.clone())
        .sub(s12.clone().mul(s21.clone()));
    let cof01 = s02
        .clone()
        .mul(s21.clone())
        .sub(s01.clone().mul(s22.clone()));
    let cof02 = s01
        .clone()
        .mul(s12.clone())
        .sub(s02.clone().mul(s11.clone()));
    let cof10 = s12
        .clone()
        .mul(s20.clone())
        .sub(s10.clone().mul(s22.clone()));
    let cof11 = s00
        .clone()
        .mul(s22.clone())
        .sub(s02.clone().mul(s20.clone()));
    let cof12 = s10
        .clone()
        .mul(s02.clone())
        .sub(s00.clone().mul(s12.clone()));
    let cof20 = s10
        .clone()
        .mul(s21.clone())
        .sub(s20.clone().mul(s11.clone()));
    let cof21 = s01
        .clone()
        .mul(s20.clone())
        .sub(s00.clone().mul(s21.clone()));
    let cof22 = s00.mul(s11).sub(s01.mul(s10));

    let inv00 = cof00.div(det_safe.clone());
    let inv01 = cof01.div(det_safe.clone());
    let inv02 = cof02.div(det_safe.clone());
    let inv10 = cof10.div(det_safe.clone());
    let inv11 = cof11.div(det_safe.clone());
    let inv12 = cof12.div(det_safe.clone());
    let inv20 = cof20.div(det_safe.clone());
    let inv21 = cof21.div(det_safe.clone());
    let inv22 = cof22.div(det_safe);

    let row0 = Tensor::cat(vec![inv00, inv01, inv02], 2).unsqueeze_dim::<4>(2);
    let row1 = Tensor::cat(vec![inv10, inv11, inv12], 2).unsqueeze_dim::<4>(2);
    let row2 = Tensor::cat(vec![inv20, inv21, inv22], 2).unsqueeze_dim::<4>(2);
    let inv_s = Tensor::cat(vec![row0, row1, row2], 2);

    let rhs_col = rhs_bn3.unsqueeze_dim::<4>(3);
    inv_s.matmul(rhs_col).squeeze::<3>(3)
}

#[cfg(feature = "acoustics-newmark")]
fn total_stiffness_displacement<B: Backend<FloatElem = f32>>(
    u_bn3: Tensor<B, 3>,
    stiffness_local_bn44: &Tensor<B, 4>,
    bar: Option<&AcousticBarNetwork<B>>,
) -> Tensor<B, 3> {
    let mut ku = contract_block33_displacement(stiffness_local_bn44.clone(), u_bn3.clone());
    if let Some(bn) = bar {
        let batch = u_bn3.dims()[0];
        let n_v = bn.n_v;
        ku = ku.add(
            crate::physics::mechanics::VectorMechanicsSolver::bar_matvec(
                u_bn3,
                &bn.k_axial,
                &bn.edge_unit,
                &bn.src_indices,
                &bn.tgt_indices,
                n_v,
                None,
                &bn.edge_len,
            ),
        );
        let _ = batch;
    }
    ku
}

#[cfg(feature = "acoustics-newmark")]
fn pack_bn3_to_flat<B: Backend<FloatElem = f32>>(
    x: Tensor<B, 3>,
    batch_row: usize,
    n_v: usize,
) -> Vec<f32> {
    let flat: Tensor<B, 1> = x
        .slice([batch_row..batch_row + 1, 0..n_v, 0..3])
        .reshape([n_v * 3]);
    flat.into_data().value
}

/// Host **GMRES** acceleration solve for one batch row when a **bar graph** is present.
///
/// # Autodiff (Burn `Autodiff`)
///
/// Krylov iterates are **`f32` host vectors**; each matvec rebuilds a batch tensor, applies
/// [`apply_effective_operator_acceleration`], then **packs back to host** ([`pack_bn3_to_flat`]).
/// That host↔tensor boundary means **reverse-mode AD does not differentiate through the implicit
/// linear solve** the way a closed-form inverse would. For **backward parity** and tape-preserving
/// multi-step drivers ([`AcousticWaveSolver::step_wave_iterate`]), keep **`bar_network: None`**
/// so [`step_wave_experimental`] uses the batched **3×3** dense solve ([`solve_batched_3x3`]) on
/// device. Graph + GMRES remains the forward / residual-accuracy path only until an AD-aware
/// alternative (e.g. implicit diff or tensor Krylov) is scoped.
#[cfg(feature = "acoustics-newmark")]
#[allow(clippy::too_many_arguments)]
fn solve_acceleration_gmres_batch_row<B: Backend<FloatElem = f32>>(
    device: &B::Device,
    template: &Tensor<B, 3>,
    batch_row: usize,
    n_v: usize,
    rhs_flat: &[f32],
    mass_bn44: Tensor<B, 4>,
    damping_bn44: Tensor<B, 4>,
    stiffness_local_bn44: Tensor<B, 4>,
    gamma_dt: f32,
    beta_dt2: f32,
    bar: Option<&AcousticBarNetwork<B>>,
    gmres: AcousticGmresConfig,
) -> Result<Tensor<B, 3>, String> {
    use super::krylov_host::gmres_f32_try;

    let n = n_v * 3;
    let mass_c = mass_bn44.clone();
    let damp_c = damping_bn44.clone();
    let kloc_c = stiffness_local_bn44.clone();
    let bar_owned = bar.cloned();

    let mut matvec = move |v: &[f32]| -> Result<Vec<f32>, crate::physics::PhysicsError> {
        let row: Tensor<B, 3> =
            Tensor::from_data(Data::new(Vec::from(v), Shape::new([1, n_v, 3])), device);
        let u_full = crate::physics::mechanics::VectorMechanicsSolver::embed_batch_row(
            template, batch_row, n_v, row,
        );
        let y = apply_effective_operator_acceleration(
            u_full,
            &mass_c,
            &damp_c,
            &kloc_c,
            gamma_dt,
            beta_dt2,
            bar_owned.as_ref(),
        );
        Ok(pack_bn3_to_flat(y, batch_row, n_v))
    };

    let x_flat = gmres_f32_try(
        &mut matvec,
        rhs_flat,
        n,
        gmres.max_iter.min(n),
        gmres.rel_tol,
    )?;

    let row: Tensor<B, 3> = Tensor::from_data(Data::new(x_flat, Shape::new([1, n_v, 3])), device);
    let u_full = crate::physics::mechanics::VectorMechanicsSolver::embed_batch_row(
        template, batch_row, n_v, row,
    );
    Ok(u_full.slice([batch_row..batch_row + 1, 0..n_v, 0..3]))
}

#[cfg(feature = "acoustics-newmark")]
#[allow(clippy::too_many_arguments)]
fn step_wave_experimental<B: Backend<FloatElem = f32>>(
    solver: &AcousticWaveSolver,
    displacement: Tensor<B, 3>,
    velocity: Tensor<B, 3>,
    acceleration: Tensor<B, 3>,
    nodal_density: Tensor<B, 3>,
    nodal_volume: Tensor<B, 3>,
    body_force: Tensor<B, 3>,
    damping_bn33: Tensor<B, 4>,
    stiffness_local_bn44: Tensor<B, 4>,
    bar_network: Option<AcousticBarNetwork<B>>,
    gmres_cfg: AcousticGmresConfig,
) -> (Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>) {
    let dt = solver.dt;
    let beta_nm = solver.newmark_beta;
    let gamma = solver.newmark_gamma;
    let dt2 = dt * dt;
    let acc_n = acceleration.clone();

    let device = displacement.device();
    let m_mat = nodal_mass_matrix_bn33(nodal_density, nodal_volume);

    let v_p = velocity
        .clone()
        .add(acc_n.clone().mul_scalar(dt * (1.0_f32 - gamma)));
    let u_p = displacement
        .clone()
        .add(velocity.clone().mul_scalar(dt))
        .add(acc_n.clone().mul_scalar(dt2 * (0.5_f32 - beta_nm)));

    let ku_p = total_stiffness_displacement(u_p, &stiffness_local_bn44, bar_network.as_ref());
    let cv_p = contract_block33_displacement(damping_bn33.clone(), v_p);
    let rhs_acc = body_force.sub(cv_p).sub(ku_p);

    let gamma_dt = gamma * dt;
    let beta_dt2 = beta_nm * dt2;

    let acc_next = if let Some(ref bar) = bar_network {
        let batch = displacement.dims()[0];
        let n_v = bar.n_v;
        let template = Tensor::<B, 3>::zeros([batch, n_v, 3], &device);
        let mut acc_acc = Tensor::<B, 3>::zeros([batch, n_v, 3], &device);
        let mut gmres_ok = true;
        for b in 0..batch {
            let rhs_flat = pack_bn3_to_flat(rhs_acc.clone(), b, n_v);
            match solve_acceleration_gmres_batch_row(
                &device,
                &template,
                b,
                n_v,
                &rhs_flat,
                m_mat.clone(),
                damping_bn33.clone(),
                stiffness_local_bn44.clone(),
                gamma_dt,
                beta_dt2,
                bar_network.as_ref(),
                gmres_cfg,
            ) {
                Ok(row_acc) => {
                    acc_acc = acc_acc.slice_assign([b..b + 1, 0..n_v, 0..3], row_acc);
                }
                Err(_) => {
                    gmres_ok = false;
                    break;
                }
            }
        }
        if gmres_ok {
            acc_acc
        } else {
            let s_mat = m_mat
                .clone()
                .add(damping_bn33.clone().mul_scalar(gamma_dt))
                .add(stiffness_local_bn44.clone().mul_scalar(beta_dt2));
            solve_batched_3x3(s_mat, rhs_acc)
        }
    } else {
        let s_mat = m_mat
            .clone()
            .add(damping_bn33.clone().mul_scalar(gamma_dt))
            .add(stiffness_local_bn44.clone().mul_scalar(beta_dt2));
        solve_batched_3x3(s_mat, rhs_acc)
    };

    let u_next = displacement.add(velocity.clone().mul_scalar(dt)).add(
        acc_n
            .clone()
            .mul_scalar(dt2 * (0.5_f32 - beta_nm))
            .add(acc_next.clone().mul_scalar(dt2 * beta_nm)),
    );

    let v_next = velocity.add(
        acc_n
            .mul_scalar(dt * (1.0_f32 - gamma))
            .add(acc_next.clone().mul_scalar(dt * gamma)),
    );

    (u_next, v_next, acc_next)
}

// --- 1-D periodic bar (dispersion / energy benchmarks) -------------------------------------------

#[cfg(feature = "acoustics-newmark")]
#[derive(Debug, Clone)]
/// Implicit Newmark-β integrator for the **1-D periodic** continuum model `ρ ∂²u/∂t² = E ∂²u/∂x²`
/// on a uniform grid (central finite-difference stiffness, lumped nodal mass `m = ρ Δx`).
///
/// **Implementation note (host `Vec<f32>` / `f64` Cholesky):** [`Self::step`], the periodic stiffness
/// helper `apply_k_periodic_1d`, and the workspace factorization intentionally use **scalar host loops**
/// and dense **f64** Cholesky
/// (see `tests/verification/acoustics_plane_wave.rs`) for fast dispersion / return-map checks — **not**
/// a Burn-tensor matvec. This is orthogonal to the tensor [`AcousticWaveSolver`] lane; tensorizing a
/// minimal periodic slice for tape-based AD remains **open** (same physics as the FD stencil;
/// would duplicate the circulant operator already validated here).
///
/// This path captures **spatial coupling** along the bar; it complements [`AcousticWaveSolver::step_wave`],
/// which targets general **3-D nodal** semi-discrete systems on the mechanics / DEC graph.
///
/// **Discretisation:** `(K u)_i = (E/Δx²) (2 u_i − u_{i−1} − u_{i+1})` with periodic indices.
///
/// **Stability (documentation):** the average-acceleration Newmark pair `(β, γ) = (¼, ½)` is
/// unconditionally stable for the undamped second-order system. **Explicit** central-difference
/// companions require `Δt ≤ CFL · Δx / c` with `c = √(E/ρ)` and `CFL` typically in `(0, 1]`.
///
/// **Phase 3 / R2.4 note:** return-map checks advance by \(T = 2\pi/\Omega\) where \(\Omega^2=\lambda_K/m\)
/// for lumped mass \(m=\rho\Delta x\) on the periodic stencil. Using only the FD dispersion
/// \(\omega_{\mathrm{disp}}=(2c/\Delta x)|\sin(k\Delta x/2)|\) (without the \(1/\Delta x\) factor from the
/// mass lump) mis-times \(T\) on a fixed bar length and produces order-one relative \(L^2\) slip vs `u₀`
/// at large \(n\) — see `tests/verification/acoustics_plane_wave.rs` (`semi_discrete_omega` helper).
///
/// formal_anchor: Literature  
/// formal_citation: Newmark 1959; Hughes 2000, §9.1 (implicit Newmark for structural dynamics)
pub struct AcousticNewmarkBar1dPeriodic {
    /// Number of grid points (cells = `n` on `[0, L)` with periodic wrap).
    pub n: usize,
    /// Domain length `L` (m).
    pub length: f32,
    /// Young's modulus `E` (Pa).
    pub youngs_modulus: f32,
    /// Mass density `ρ` (kg/m³).
    pub density: f32,
    pub newmark_beta: f32,
    pub newmark_gamma: f32,
}

#[cfg(feature = "acoustics-newmark")]
#[derive(Debug)]
/// Workspace reused across [`AcousticNewmarkBar1dPeriodic::step`] calls (Cholesky factor for fixed `Δt`).
pub struct AcousticNewmarkBar1dWork {
    /// Lower-triangular Cholesky factor of `S = M + β Δt² K` in **f64** (row-major, lower triangle).
    chol: Vec<f64>,
    ku: Vec<f32>,
    u_tilde: Vec<f32>,
    rhs: Vec<f32>,
    sol_y: Vec<f64>,
    sol_x: Vec<f64>,
    last_dt: f32,
}

#[cfg(feature = "acoustics-newmark")]
impl AcousticNewmarkBar1dPeriodic {
    pub fn dx(&self) -> f32 {
        self.length / self.n as f32
    }

    pub fn wave_speed(&self) -> f32 {
        (self.youngs_modulus / self.density).sqrt()
    }

    pub fn workspace(&self) -> AcousticNewmarkBar1dWork {
        let n = self.n;
        AcousticNewmarkBar1dWork {
            chol: vec![0.0_f64; n * n],
            ku: vec![0.0_f32; n],
            u_tilde: vec![0.0_f32; n],
            rhs: vec![0.0_f32; n],
            sol_y: vec![0.0_f64; n],
            sol_x: vec![0.0_f64; n],
            last_dt: -1.0_f32,
        }
    }

    /// Total mechanical energy `½ Σ m u̇² + ½ uᵀ K u` for the discrete periodic bar.
    pub fn mechanical_energy(&self, u: &[f32], v: &[f32]) -> f32 {
        debug_assert_eq!(u.len(), self.n);
        debug_assert_eq!(v.len(), self.n);
        let dx = self.dx();
        let m = self.density * dx;
        let mut ke = 0.0_f32;
        for &vi in v {
            ke += 0.5_f32 * m * vi * vi;
        }
        self.elastic_energy(u) + ke
    }

    /// Elastic energy `½ uᵀ K u` with the periodic stiffness above.
    pub fn elastic_energy(&self, u: &[f32]) -> f32 {
        debug_assert_eq!(u.len(), self.n);
        let mut ku = vec![0.0_f32; self.n];
        apply_k_periodic_1d(u, self.youngs_modulus, self.dx(), self.n, &mut ku);
        0.5_f32 * dot(u, &ku)
    }

    /// Advance one implicit Newmark step (`M ü + K u = 0`, undamped).
    pub fn step(
        &self,
        ws: &mut AcousticNewmarkBar1dWork,
        dt: f32,
        u: &mut [f32],
        v: &mut [f32],
        a: &mut [f32],
    ) {
        debug_assert_eq!(u.len(), self.n);
        debug_assert_eq!(v.len(), self.n);
        debug_assert_eq!(a.len(), self.n);

        if !self.prepare(ws, dt) {
            return;
        }

        let beta = self.newmark_beta;
        let gamma = self.newmark_gamma;
        let dt2 = dt * dt;
        let half_minus_beta = 0.5_f32 - beta;

        let acc_n: Vec<f32> = a.to_vec();
        for i in 0..self.n {
            ws.u_tilde[i] = u[i] + dt * v[i] + dt2 * half_minus_beta * acc_n[i];
        }
        apply_k_periodic_1d(
            &ws.u_tilde,
            self.youngs_modulus,
            self.dx(),
            self.n,
            &mut ws.ku,
        );
        for i in 0..self.n {
            ws.rhs[i] = -ws.ku[i];
        }
        cholesky_solve_lower64(&ws.chol, self.n, &ws.rhs, &mut ws.sol_y, &mut ws.sol_x, a);

        for i in 0..self.n {
            u[i] += dt * v[i] + dt2 * (half_minus_beta * acc_n[i] + beta * a[i]);
        }
        for i in 0..self.n {
            v[i] += dt * ((1.0_f32 - gamma) * acc_n[i] + gamma * a[i]);
        }
    }

    fn prepare(&self, ws: &mut AcousticNewmarkBar1dWork, dt: f32) -> bool {
        if (ws.last_dt - dt).abs() <= 1e-12_f32.max(dt * 1e-7_f32) && ws.last_dt >= 0.0_f32 {
            return true;
        }
        let dx = self.dx() as f64;
        let m_node = self.density as f64 * dx;
        let alpha =
            self.newmark_beta as f64 * (dt as f64).powi(2) * self.youngs_modulus as f64 / (dx * dx);
        fill_system_matrix_s64(&mut ws.chol, self.n, m_node, alpha);
        if cholesky_decompose_lower64(&mut ws.chol, self.n).is_err() {
            ws.last_dt = -1.0_f32;
            return false;
        }
        ws.last_dt = dt;
        true
    }
}

#[cfg(feature = "acoustics-newmark")]
fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum()
}

#[cfg(feature = "acoustics-newmark")]
// TRACKING (Phase 8 follow-up): optional Burn-tensor circulant matvec for this stencil would enable
// AD through the 1-D periodic bar without rewriting the f64 Cholesky reference path above.
fn apply_k_periodic_1d(u: &[f32], e: f32, dx: f32, n: usize, out: &mut [f32]) {
    let c = e / (dx * dx);
    for i in 0..n {
        let im = if i == 0 { n - 1 } else { i - 1 };
        let ip = if i + 1 == n { 0 } else { i + 1 };
        out[i] = c * (2.0_f32 * u[i] - u[im] - u[ip]);
    }
}

#[cfg(feature = "acoustics-newmark")]
fn fill_system_matrix_s64(mat: &mut [f64], n: usize, m: f64, alpha: f64) {
    debug_assert_eq!(mat.len(), n * n);
    mat.fill(0.0_f64);
    for i in 0..n {
        mat[i * n + i] = m + 2.0_f64 * alpha;
        let ip = (i + 1) % n;
        let im = (i + n - 1) % n;
        mat[i * n + ip] = -alpha;
        mat[i * n + im] = -alpha;
    }
}

#[cfg(feature = "acoustics-newmark")]
fn cholesky_decompose_lower64(a: &mut [f64], n: usize) -> Result<(), ()> {
    for i in 0..n {
        for j in 0..=i {
            let mut sum = a[i * n + j];
            for k in 0..j {
                sum -= a[i * n + k] * a[j * n + k];
            }
            if i == j {
                if sum <= 1e-30_f64 {
                    return Err(());
                }
                a[i * n + j] = sum.sqrt();
            } else {
                let d = a[j * n + j];
                if d.abs() < 1e-30_f64 {
                    return Err(());
                }
                a[i * n + j] = sum / d;
            }
        }
        for j in (i + 1)..n {
            a[i * n + j] = 0.0_f64;
        }
    }
    Ok(())
}

#[cfg(feature = "acoustics-newmark")]
fn cholesky_solve_lower64(
    l: &[f64],
    n: usize,
    b: &[f32],
    y: &mut [f64],
    x: &mut [f64],
    a_out: &mut [f32],
) {
    for i in 0..n {
        let mut s = b[i] as f64;
        for k in 0..i {
            s -= l[i * n + k] * y[k];
        }
        let d = l[i * n + i];
        y[i] = s / d;
    }
    for i in (0..n).rev() {
        let mut s = y[i];
        for k in (i + 1)..n {
            s -= l[k * n + i] * x[k];
        }
        let d = l[i * n + i];
        x[i] = s / d;
    }
    for i in 0..n {
        a_out[i] = x[i] as f32;
    }
}

#[cfg(all(test, feature = "acoustics-newmark"))]
mod acoustics_newmark_tests {
    use super::*;
    use burn::tensor::{Data, Shape};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    #[test]
    fn nodal_mass_kinetic_energy_matches_hand_scalar() {
        let dev = NdArrayDevice::Cpu;
        let rho = Tensor::<B, 3>::from_data(Data::new(vec![2.0_f32], Shape::new([1, 1, 1])), &dev);
        let vol = Tensor::<B, 3>::from_data(Data::new(vec![0.5_f32], Shape::new([1, 1, 1])), &dev);
        let m = nodal_mass_matrix_bn33(rho, vol);
        let v = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0_f32, 0.0, 0.0], Shape::new([1, 1, 3])),
            &dev,
        );
        let ke = nodal_kinetic_energy_bn1(v, m);
        let ke_host = ke.into_data().value[0];
        let m_scalar = 2.0_f32 * 0.5_f32;
        let expect = 0.5_f32 * m_scalar * 1.0_f32 * 1.0_f32;
        assert!(
            (ke_host - expect).abs() < 1e-5_f32,
            "ke tensor {ke_host} vs hand {expect}"
        );
    }

    #[test]
    fn newmark_dense_path_matches_homogeneous_chain_two_nodes() {
        let dev = NdArrayDevice::Cpu;
        let n = 2usize;
        let dt = 0.01_f32;
        let solver = AcousticWaveSolver {
            dt,
            newmark_beta: 0.25_f32,
            newmark_gamma: 0.5_f32,
        };
        let u = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let vel = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let acc = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let rho = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let vol = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0_f32, 1.0_f32], Shape::new([1, n, 1])),
            &dev,
        );
        let f = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let damp = Tensor::<B, 4>::zeros([1, n, 3, 3], &dev);
        let k_edge = 10.0_f32;
        let kloc = Tensor::<B, 2>::eye(3, &dev)
            .reshape([1, 1, 3, 3])
            .expand([1, n, 3, 3])
            .mul_scalar(k_edge);
        let (u1, _v1, a1) = solver.step_wave(u, vel, acc, rho, vol, f, damp, kloc, None, None);
        let _ = u1;
        let a1_flat = a1.into_data().value;
        assert_eq!(a1_flat.len(), n * 3);
        assert!(a1_flat.iter().all(|x| x.is_finite()));
    }
}

#[cfg(all(test, feature = "acoustics-newmark"))]
mod acoustics_graph_gmres_tests {
    use super::*;
    use burn::tensor::{Data, Int, Shape};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    #[test]
    fn graph_newmark_gmres_acceleration_residual_small() {
        let dev = NdArrayDevice::Cpu;
        let n = 2usize;
        let e_y = 100.0_f32;
        let a_sec = 1.0_f32;
        let coords: Tensor<B, 2> = Tensor::from_data(
            Data::new(vec![0.0_f32, 0.0, 0.0, 1.0, 0.0, 0.0], Shape::new([n, 3])),
            &dev,
        );
        let edges: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0_i64, 1_i64], Shape::new([2, 1])), &dev);
        let youngs =
            Tensor::<B, 3>::from_data(Data::new(vec![e_y, e_y], Shape::new([1, n, 1])), &dev);
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let bar =
            AcousticBarNetwork::assemble_axial_bar_graph(coords, edges, youngs, damage, a_sec);

        let dt = 0.02_f32;
        let beta = 0.25_f32;
        let gamma = 0.5_f32;
        let solver = AcousticWaveSolver {
            dt,
            newmark_beta: beta,
            newmark_gamma: gamma,
        };

        let u = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let vel = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let acc = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let rho = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let vol = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let f = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let damp = Tensor::<B, 4>::zeros([1, n, 3, 3], &dev);
        let k_zero = Tensor::<B, 4>::zeros([1, n, 3, 3], &dev);

        let (_u1, _v1, a1) = solver.step_wave(
            u,
            vel,
            acc,
            rho,
            vol,
            f,
            damp,
            k_zero,
            Some(bar),
            Some(AcousticGmresConfig {
                max_iter: 48,
                rel_tol: 1e-7_f32,
            }),
        );

        let a_flat = a1.into_data().value;
        assert!(
            a_flat.iter().all(|x| x.is_finite()),
            "GMRES acceleration must be finite"
        );
        let an = a_flat.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!(an < 1e6_f32, "acceleration norm unexpectedly large: {an}");
    }
}

#[cfg(all(test, feature = "acoustics-newmark"))]
mod acoustics_ad_iterate_tests {
    use super::*;
    use burn::backend::Autodiff;
    use burn::tensor::Tensor;
    use burn_ndarray::{NdArray, NdArrayDevice};

    type Inner = NdArray<f32>;
    type B = Autodiff<Inner>;

    /// Dense batched 3×3 Newmark path only (`bar_network: None`): full backward through
    /// [`AcousticWaveSolver::step_wave_iterate`] is supported. With `Some(bar)`, GMRES breaks the tape;
    /// see rustdoc on `solve_acceleration_gmres_batch_row` in this module.
    #[test]
    fn iterate_until_step_wave_backward_runs_nodal_dense() {
        let dev = NdArrayDevice::Cpu;
        let n = 2usize;
        let dt = 0.05_f32;
        let solver = AcousticWaveSolver {
            dt,
            newmark_beta: 0.25_f32,
            newmark_gamma: 0.5_f32,
        };
        let rho = Tensor::<B, 3>::full([1, n, 1], 1.5_f32, &dev).require_grad();
        let vol = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let damp = Tensor::<B, 4>::zeros([1, n, 3, 3], &dev);
        let kloc = Tensor::<B, 2>::eye(3, &dev)
            .reshape([1, 1, 3, 3])
            .expand([1, n, 3, 3])
            .mul_scalar(0.3_f32);
        let f = Tensor::<B, 3>::full([1, n, 3], 0.02_f32, &dev);

        let u0 = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let v0 = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let a0 = Tensor::<B, 3>::zeros([1, n, 3], &dev);

        let (u_end, _v, _a, _k) =
            solver.step_wave_iterate(32, u0, v0, a0, rho.clone(), vol, f, damp, kloc, None, None);

        let loss = u_end.clone().sum();
        let grads = loss.backward();
        let g_rho = rho.grad(&grads).expect("grad w.r.t rho");
        assert_eq!(g_rho.dims(), [1, n, 1]);
        let gn = g_rho.into_data().value.iter().map(|x| x.abs()).sum::<f32>();
        assert!(gn > 1e-12_f32, "expected non-zero grad norm, got {gn}");
    }
}

#[cfg(all(test, feature = "acoustics-newmark"))]
mod cholesky_residual_tests {
    use super::*;

    fn matvec64(s: &[f64], n: usize, x: &[f64], y: &mut [f64]) {
        for i in 0..n {
            let mut sum = 0.0_f64;
            for j in 0..n {
                sum += s[i * n + j] * x[j];
            }
            y[i] = sum;
        }
    }

    fn residual_norm64(s: &[f64], n: usize, x: &[f64], b: &[f64]) -> f64 {
        let mut ax = vec![0.0_f64; n];
        matvec64(s, n, x, &mut ax);
        let mut r = 0.0_f64;
        for i in 0..n {
            let d = ax[i] - b[i];
            r += d * d;
        }
        r.sqrt()
    }

    #[test]
    fn cholesky_solve_matches_system_matrix() {
        let n = 32_usize;
        let dx = 1.0_f64 / n as f64;
        let m_node = 1.0_f64 * dx;
        let dt = 0.001_f64;
        let beta = 0.25_f64;
        let e = 1.0_f64;
        let alpha = beta * dt * dt * e / (dx * dx);
        let mut s = vec![0.0_f64; n * n];
        fill_system_matrix_s64(&mut s, n, m_node, alpha);
        let s_orig = s.clone();
        cholesky_decompose_lower64(&mut s, n).expect("cholesky");
        let mut b = vec![0.0_f32; n];
        b[0] = 1.0_f32;
        let mut y = vec![0.0_f64; n];
        let mut x = vec![0.0_f64; n];
        let mut x_out = vec![0.0_f32; n];
        cholesky_solve_lower64(&s, n, &b, &mut y, &mut x, &mut x_out);
        let mut b64 = vec![0.0_f64; n];
        b64[0] = 1.0_f64;
        let mut x64 = vec![0.0_f64; n];
        for i in 0..n {
            x64[i] = x_out[i] as f64;
        }
        let res = residual_norm64(&s_orig, n, &x64, &b64);
        assert!(
            res < 1e-8_f64,
            "Cholesky residual ||S x - b|| too large: {res}"
        );
    }

    #[test]
    fn cholesky_solve_matches_system_matrix_n128() {
        let n = 128_usize;
        let dx = 1.0_f64 / n as f64;
        let m_node = 1.0_f64 * dx;
        let dt = (1.0_f64 / n as f64) / 1000.0_f64;
        let beta = 0.25_f64;
        let e = 1.0_f64;
        let alpha = beta * dt * dt * e / (dx * dx);
        let mut s = vec![0.0_f64; n * n];
        fill_system_matrix_s64(&mut s, n, m_node, alpha);
        let s_orig = s.clone();
        cholesky_decompose_lower64(&mut s, n).expect("cholesky");
        let mut b = vec![0.0_f32; n];
        b[0] = 1.0_f32;
        let mut y = vec![0.0_f64; n];
        let mut x = vec![0.0_f64; n];
        let mut x_out = vec![0.0_f32; n];
        cholesky_solve_lower64(&s, n, &b, &mut y, &mut x, &mut x_out);
        let mut b64 = vec![0.0_f64; n];
        b64[0] = 1.0_f64;
        let mut x64 = vec![0.0_f64; n];
        for i in 0..n {
            x64[i] = x_out[i] as f64;
        }
        let res = residual_norm64(&s_orig, n, &x64, &b64);
        assert!(
            res < 1e-6_f64,
            "Cholesky residual n=128 ||S x - b|| too large: {res}"
        );
    }
}

#[cfg(all(test, feature = "acoustics-newmark"))]
mod acoustics_idempotency_tests {
    use super::*;
    use burn::tensor::Tensor;
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn max_abs_drift(a: &[f32], b: &[f32]) -> f32 {
        a.iter()
            .zip(b.iter())
            .map(|(&x, &y)| (x - y).abs())
            .fold(0.0_f32, f32::max)
    }

    /// FP Manifesto §6: `step` on zero displacement/velocity/acceleration (undamped equilibrium)
    /// must be a fixed point — re-application must not drift.
    #[test]
    fn acoustic_newmark_bar_step_idempotent_on_zero_equilibrium() {
        let bar = AcousticNewmarkBar1dPeriodic {
            n: 32,
            length: 1.0_f32,
            youngs_modulus: 1.0_f32,
            density: 1.0_f32,
            newmark_beta: 0.25_f32,
            newmark_gamma: 0.5_f32,
        };
        let mut ws = bar.workspace();
        let dt = 0.01_f32;
        let mut u = vec![0.0_f32; bar.n];
        let mut v = vec![0.0_f32; bar.n];
        let mut a = vec![0.0_f32; bar.n];

        bar.step(&mut ws, dt, &mut u, &mut v, &mut a);
        let u1 = u.clone();
        let v1 = v.clone();
        let a1 = a.clone();

        bar.step(&mut ws, dt, &mut u, &mut v, &mut a);
        let tol = 1e-6_f32;
        assert!(
            max_abs_drift(&u, &u1) < tol && max_abs_drift(&v, &v1) < tol && max_abs_drift(&a, &a1) < tol,
            "re-step on equilibrated bar state must not drift (u/v/a)"
        );
        assert!(
            u.iter().chain(v.iter()).chain(a.iter()).all(|x| x.abs() < tol),
            "zero equilibrium must remain at rest after first step"
        );
    }

    /// FP Manifesto §6: dense nodal [`AcousticWaveSolver::step_wave`] on quiescent state with zero load.
    #[test]
    fn acoustic_wave_solver_step_idempotent_on_zero_equilibrium() {
        let dev = NdArrayDevice::Cpu;
        let n = 4_usize;
        let solver = AcousticWaveSolver {
            dt: 0.02_f32,
            newmark_beta: 0.25_f32,
            newmark_gamma: 0.5_f32,
        };
        let u = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let vel = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let acc = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let rho = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let vol = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let f = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let damp = Tensor::<B, 4>::zeros([1, n, 3, 3], &dev);
        let kloc = Tensor::<B, 2>::eye(3, &dev)
            .reshape([1, 1, 3, 3])
            .expand([1, n, 3, 3])
            .mul_scalar(0.5_f32);

        let (u1, v1, a1) = solver
            .step_wave(
                u.clone(),
                vel.clone(),
                acc.clone(),
                rho.clone(),
                vol.clone(),
                f.clone(),
                damp.clone(),
                kloc.clone(),
                None,
                None,
            )
            .expect("step_wave");
        let (u2, v2, a2) = solver
            .step_wave(
                u1.clone(),
                v1.clone(),
                a1.clone(),
                rho,
                vol,
                f,
                damp,
                kloc,
                None,
                None,
            )
            .expect("step_wave");

        let tol = 1e-6_f32;
        for (label, t0, t1) in [("u", u2, u1), ("v", v2, v1), ("a", a2, a1)] {
            let max_d = t0
                .sub(t1)
                .abs()
                .into_data()
                .value
                .iter()
                .map(|x| x.abs())
                .fold(0.0_f32, f32::max);
            assert!(max_d < tol, "{label} drift after re-step: {max_d}");
        }
    }
}
