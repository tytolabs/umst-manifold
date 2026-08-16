// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Extruded Cartesian slab mechanics using **8-node trilinear (Q1) hex** continuum elasticity on the
//! brick lattice (`nx × ny × nz` cells).
//!
//! formal_anchor: Literature  
//! formal_citation: Bendsoe & Sigmund 2003, *Topology Optimization*; Bathe 2006, *Finite Element Procedures* (hex elements)  
//! formal_form: \(\int_{\Omega} \mathbf B^{\mathsf T}\mathbf D(E(\rho))\mathbf B\,\mathrm d\Omega\,\mathbf u=\mathbf f\) with
//! \(E(\rho)=E_{\min}+(E_0-E_{\min})\rho^p\) per cell (mean nodal \(\rho\) on the eight corners).
//!
//! # Stress output
//!
//! [`ExtrudedPlateMechanics::solve_equilibrium`] returns **displacement** from the Q1-hex solve.
//! The Voigt `[B,N,6]` second return is currently **zeros** — callers needing Cauchy stress should use
//! post-processing (e.g. strain recovery at Gauss points) in a follow-up; the bar-network era’s
//! rank-one nodal stress is not meaningful for Q1 solids.
//!
//! # Shear locking (verification)
//!
//! For thin slabs (`span/thickness` large), **equal-order Q1 hex** bending is dominated by
//! transverse-shear locking; centre deflection can sit orders of magnitude below Kirchhoff plate
//! tables even when the discrete equilibrium residual is tiny. Do not read thin-plate analytic
//! values off this solid element without reduced integration / plate theory extensions.
//!
//! For **uniform transverse pressure** `q` (force per top-surface area) compared to Kirchhoff
//! formulas that use total load `q L_x L_y`, assemble the top-face nodal `f_z` with
//! [`ExtrudedPlateMechanics::body_force_top_uniform_pressure`]. Applying a constant `-q dx dy` at
//! every top node over-counts load by a factor `(nx+1)(ny+1)/(nx ny)`.
//!
//! Enabled with **`topology-density-evolution`**, **`mechanics-voigt-cauchy`**, or bundles that
//! include either (e.g. **`solver-stable`** / **`solver-experimental`**).
//!
//! # Honest boundary (W29-053)
//!
//! Q1-hex PCG equilibrium + bilinear top-pressure lumping are **landed**. Voigt Cauchy recovery
//! remains a **zero placeholder**; thin-plate Kirchhoff centre-deflection gate stays open
//! (shear locking). Batch `>1` is refused. Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER`.

/// W29 deepen cell — extruded-plate Q1-hex honest fence bundle.
pub const W29_EXTRUDED_PLATE_DEEPEN_CELL: &str = "W29-053-EXTRUDED_PLATE";

/// Honest posture tag — Q1-hex extruded plate landed; fleet TO / Kirchhoff GREEN refused.
pub const EXTRUDED_PLATE_POSTURE_TAG: &str = "honest-q1-hex-extruded-plate-research-lane";

/// Honest physics posture — unit/linearity contracts pass; does not certify Kirchhoff R2.1 or fleet TO.
pub const EXTRUDED_PLATE_PHYSICS_GREEN: bool = false;

/// Production topology-optimisation wiring — not claimed by extruded-plate solve alone.
pub const EXTRUDED_PLATE_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const EXTRUDED_PLATE_MASTER: bool = false;

/// Whether matrix-free Q1-hex PCG equilibrium is landed on this surface.
pub const EXTRUDED_PLATE_Q1_HEX_EQUILIBRIUM_LANDED: bool = true;

/// Whether bilinear-consistent top-face pressure lumping is landed.
pub const EXTRUDED_PLATE_TOP_PRESSURE_LUMPING_LANDED: bool = true;

/// Whether Voigt `[B,N,6]` Cauchy recovery is wired (honestly open — zeros placeholder).
pub const EXTRUDED_PLATE_VOIGT_CAUCHY_RECOVERY_WIRED: bool = false;

/// Whether thin-plate Kirchhoff centre-deflection gate is closed (honestly open — shear locking).
pub const EXTRUDED_PLATE_KIRCHHOFF_THIN_PLATE_GATE_WIRED: bool = false;

/// Whether batch>1 equilibrium is implemented (honestly open).
pub const EXTRUDED_PLATE_BATCH_GT1_WIRED: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const EXTRUDED_PLATE_HONEST_FENCE: &str = "q1_hex_equilibrium_landed=true top_pressure_lumping_landed=true voigt_cauchy_recovery_wired=false kirchhoff_thin_plate_gate_wired=false batch_gt1_wired=false production_wired=false master_composition_wired=false physics_green=false";

const _: () = assert!(!EXTRUDED_PLATE_PHYSICS_GREEN);
const _: () = assert!(!EXTRUDED_PLATE_PRODUCTION_WIRED);
const _: () = assert!(!EXTRUDED_PLATE_MASTER);
const _: () = assert!(!EXTRUDED_PLATE_VOIGT_CAUCHY_RECOVERY_WIRED);
const _: () = assert!(!EXTRUDED_PLATE_KIRCHHOFF_THIN_PLATE_GATE_WIRED);
const _: () = assert!(!EXTRUDED_PLATE_BATCH_GT1_WIRED);

/// Typed probe for extruded-plate posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtrudedPlatePostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub q1_hex_equilibrium_landed: bool,
    pub top_pressure_lumping_landed: bool,
    pub voigt_cauchy_recovery_wired: bool,
    pub kirchhoff_thin_plate_gate_wired: bool,
    pub batch_gt1_wired: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for extruded-plate mechanics.
#[must_use]
pub fn extruded_plate_honest_posture_bundle() -> ExtrudedPlatePostureProbe {
    ExtrudedPlatePostureProbe {
        physics_green: EXTRUDED_PLATE_PHYSICS_GREEN,
        production_wired: EXTRUDED_PLATE_PRODUCTION_WIRED,
        master: EXTRUDED_PLATE_MASTER,
        q1_hex_equilibrium_landed: EXTRUDED_PLATE_Q1_HEX_EQUILIBRIUM_LANDED,
        top_pressure_lumping_landed: EXTRUDED_PLATE_TOP_PRESSURE_LUMPING_LANDED,
        voigt_cauchy_recovery_wired: EXTRUDED_PLATE_VOIGT_CAUCHY_RECOVERY_WIRED,
        kirchhoff_thin_plate_gate_wired: EXTRUDED_PLATE_KIRCHHOFF_THIN_PLATE_GATE_WIRED,
        batch_gt1_wired: EXTRUDED_PLATE_BATCH_GT1_WIRED,
        honest_fence: EXTRUDED_PLATE_HONEST_FENCE,
        posture_tag: EXTRUDED_PLATE_POSTURE_TAG,
        deepen_cell: W29_EXTRUDED_PLATE_DEEPEN_CELL,
    }
}

/// Q1-hex extruded plate SSOT landed with production/master/GREEN composition honestly open.
#[must_use]
pub fn extruded_plate_posture_honest(probe: &ExtrudedPlatePostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.q1_hex_equilibrium_landed
        && probe.top_pressure_lumping_landed
        && !probe.voigt_cauchy_recovery_wired
        && !probe.kirchhoff_thin_plate_gate_wired
        && !probe.batch_gt1_wired
        && probe
            .honest_fence
            .contains("q1_hex_equilibrium_landed=true")
        && probe
            .honest_fence
            .contains("voigt_cauchy_recovery_wired=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// Validate extruded-plate posture honesty — fail closed on fake production/master/GREEN claims.
pub fn validate_extruded_plate_posture_honesty() -> Result<(), &'static str> {
    let probe = extruded_plate_honest_posture_bundle();
    if probe.physics_green {
        return Err(
            "EXTRUDED_PLATE_PHYSICS_GREEN must stay false — linearity ≠ Kirchhoff/fleet TO",
        );
    }
    if probe.production_wired {
        return Err(
            "EXTRUDED_PLATE_PRODUCTION_WIRED must stay false until embodied TO loop closes",
        );
    }
    if probe.master {
        return Err("EXTRUDED_PLATE_MASTER must stay false until master composition pin lands");
    }
    if probe.voigt_cauchy_recovery_wired {
        return Err(
            "EXTRUDED_PLATE_VOIGT_CAUCHY_RECOVERY_WIRED must stay false while return is zeros",
        );
    }
    if probe.kirchhoff_thin_plate_gate_wired {
        return Err(
            "EXTRUDED_PLATE_KIRCHHOFF_THIN_PLATE_GATE_WIRED must stay false under shear locking",
        );
    }
    if probe.batch_gt1_wired {
        return Err("EXTRUDED_PLATE_BATCH_GT1_WIRED must stay false until batch>1 is implemented");
    }
    if !extruded_plate_posture_honest(&probe) {
        return Err("extruded_plate_posture_honest failed");
    }
    Ok(())
}

use burn::tensor::{backend::Backend, Data, Int, Shape, Tensor};

use super::error::PhysicsError;
use super::hex_elasticity;
use super::time_orchestration::MechanicsInnerLoopConfig;

/// Isotropic elastic parameters + SIMP modulus law.
#[derive(Clone, Copy, Debug)]
pub struct ElasticMaterial {
    pub e0: f32,
    pub nu: f32,
    pub simp_p: f32,
    pub e_min: f32,
}

/// Conjugate-gradient controls (alias of [`MechanicsInnerLoopConfig`]).
pub type CgConfig = MechanicsInnerLoopConfig;

/// Uniform extruded grid: `nx × ny × nz` **cells** ⇒ \((nx+1)(ny+1)(nz+1)\) nodes.
#[derive(Clone, Debug)]
pub struct ExtrudedPlateMechanics {
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    pub dx: f32,
    pub dy: f32,
    pub dz: f32,
}

impl ExtrudedPlateMechanics {
    #[must_use]
    pub fn n_nodes(&self) -> usize {
        (self.nx + 1) * (self.ny + 1) * (self.nz + 1)
    }

    /// Graph edges `[2, E]` on the extruded hex skeleton (for Helmholtz / filters).
    pub fn edges_b1<B: Backend<FloatElem = f32>>(&self, device: &B::Device) -> Tensor<B, 2, Int> {
        self.hex_grid_edges_b1(device)
    }

    /// Node coordinates `[1, N, 3]` for batch-1 density networks.
    pub fn coords_bn3<B: Backend<FloatElem = f32>>(
        &self,
        device: &B::Device,
    ) -> Result<Tensor<B, 3>, PhysicsError> {
        let n3 = self.node_coords_n3(device);
        let [n, three] = n3.dims();
        if three != 3 {
            return Err(PhysicsError::ShapeMismatch {
                context: "ExtrudedPlateMechanics::coords_bn3",
                detail: "expected last dim 3",
            });
        }
        Ok(n3.reshape([1, n, 3]))
    }

    /// Q1-hex equilibrium solve and placeholder Voigt stress `[B,N,6]` (see module docs).
    pub fn solve_equilibrium<B: Backend<FloatElem = f32>>(
        &self,
        rho_projected: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        material: ElasticMaterial,
        cg_config: &CgConfig,
    ) -> Result<(Tensor<B, 3>, Tensor<B, 3>), PhysicsError> {
        let n = self.n_nodes();
        let [batch, n_rho, c] = rho_projected.dims();
        if batch != 1 {
            return Err(PhysicsError::ShapeMismatch {
                context: "ExtrudedPlateMechanics::solve_equilibrium",
                detail: "batch>1 not implemented",
            });
        }
        if n_rho != n {
            return Err(PhysicsError::ShapeMismatch {
                context: "ExtrudedPlateMechanics::solve_equilibrium",
                detail: "rho N must match extruded grid",
            });
        }
        if c != 1 {
            return Err(PhysicsError::ShapeMismatch {
                context: "ExtrudedPlateMechanics::solve_equilibrium",
                detail: "rho channel must be 1",
            });
        }
        let [b_f, n_f, c_f] = body_force.dims();
        if b_f != 1 || n_f != n || c_f != 3 {
            return Err(PhysicsError::ShapeMismatch {
                context: "ExtrudedPlateMechanics::solve_equilibrium",
                detail: "body_force must be [1, N, 3] matching extruded grid",
            });
        }
        let [b_m, n_m, c_m] = boundary_mask.dims();
        if b_m != 1 || n_m != n || c_m != 3 {
            return Err(PhysicsError::ShapeMismatch {
                context: "ExtrudedPlateMechanics::solve_equilibrium",
                detail: "boundary_mask must be [1, N, 3] matching extruded grid",
            });
        }
        let device = rho_projected.device();

        let nx1 = self.nx + 1;
        let ny1 = self.ny + 1;
        let n_cells = self.nx * self.ny * self.nz;

        let rho_flat = rho_projected.clone().into_data().value;
        let mut e_cell = vec![0.0_f32; n_cells];
        for cz in 0..self.nz {
            for cy in 0..self.ny {
                for cx in 0..self.nx {
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
                        let nid = ix + iy * nx1 + iz * nx1 * ny1;
                        sum += rho_flat[nid];
                    }
                    let rho_e = sum * (1.0 / 8.0_f32);
                    let e_e = rho_e.powf(material.simp_p) * (material.e0 - material.e_min)
                        + material.e_min;
                    let cidx = cx + cy * self.nx + cz * self.nx * self.ny;
                    e_cell[cidx] = e_e;
                }
            }
        }

        let f_flat = body_force.clone().into_data().value;
        let m_flat = boundary_mask.clone().into_data().value;

        let mut u = vec![0.0_f32; n * 3];
        let mut diag = vec![0.0_f32; n * 3];
        let mut scratch = vec![0.0_f32; n * 3];

        let max_it = cg_config.max_cg_iterations.max(1);
        let rel_tol = cg_config.pcg_tolerance.max(cg_config.cg_tolerance).max(0.0);

        let pcg = hex_elasticity::hex_solve_pcg_masked(
            self.nx,
            self.ny,
            self.nz,
            self.dx,
            self.dy,
            self.dz,
            material.nu,
            &e_cell,
            &f_flat,
            &m_flat,
            &mut u,
            &mut diag,
            &mut scratch,
            max_it,
            hex_elasticity::hex_precond_from_use_preconditioner(cg_config.use_preconditioner),
            cg_config.cg_tolerance,
            None,
        );
        if rel_tol > 0.0 && (!pcg.rel_residual.is_finite() || pcg.rel_residual > rel_tol) {
            return Err(PhysicsError::Diverged {
                eq_rel: pcg.rel_residual,
                pcg_iterations: pcg.iterations,
            });
        }

        let u_tensor: Tensor<B, 3> =
            Tensor::from_data(Data::new(u, Shape::new([1, n, 3])), &device);
        let voigt = Tensor::<B, 3>::zeros([batch, n, 6], &device);
        Ok((u_tensor, voigt))
    }

    /// Analytic total transverse load for uniform pressure `q`: `-q L_x L_y`.
    #[must_use]
    pub fn top_uniform_pressure_total_fz(&self, q: f32) -> f32 {
        -q * (self.nx as f32) * self.dx * (self.ny as f32) * self.dy
    }

    /// Nodal body-force vector `[N,3]` (flat `3N`) for **uniform pressure** `q` on the top face
    /// `z = nz dz`: bilinear-consistent lumping on the `(nx+1)(ny+1)` top nodes so
    /// `sum_i f_z(i) = -q L_x L_y` with `L_x = nx dx`, `L_y = ny dy`.
    #[must_use]
    pub fn body_force_top_uniform_pressure(&self, q: f32) -> Vec<f32> {
        let nx1 = self.nx + 1;
        let ny1 = self.ny + 1;
        let n = nx1 * ny1 * (self.nz + 1);
        let mut bf = vec![0.0_f32; n * 3];
        let iz = self.nz;
        let cell = q * self.dx * self.dy;
        for iy in 0..=self.ny {
            let ay = if iy == 0 || iy == self.ny {
                0.5_f32
            } else {
                1.0_f32
            };
            for ix in 0..=self.nx {
                let ax = if ix == 0 || ix == self.nx {
                    0.5_f32
                } else {
                    1.0_f32
                };
                let nid = ix + iy * nx1 + iz * nx1 * ny1;
                bf[nid * 3 + 2] = -cell * ax * ay;
            }
        }
        bf
    }

    /// Sum of nodal `f_z` entries in a flat `[N,3]` body-force vector.
    #[must_use]
    pub fn sum_fz_flat(body_force_flat: &[f32]) -> f32 {
        body_force_flat.chunks_exact(3).map(|uvw| uvw[2]).sum()
    }

    fn node_coords_n3<B: Backend<FloatElem = f32>>(&self, device: &B::Device) -> Tensor<B, 2> {
        let n = self.n_nodes();
        let nx1 = self.nx + 1;
        let ny1 = self.ny + 1;
        let mut data = vec![0.0f32; n * 3];
        for iz in 0..=self.nz {
            for iy in 0..=self.ny {
                for ix in 0..=self.nx {
                    let id = ix + iy * nx1 + iz * nx1 * ny1;
                    data[id * 3] = ix as f32 * self.dx;
                    data[id * 3 + 1] = iy as f32 * self.dy;
                    data[id * 3 + 2] = iz as f32 * self.dz;
                }
            }
        }
        Tensor::from_data(Data::new(data, Shape::new([n, 3])), device)
    }

    fn hex_grid_edges_b1<B: Backend<FloatElem = f32>>(
        &self,
        device: &B::Device,
    ) -> Tensor<B, 2, Int> {
        let nx1 = self.nx + 1;
        let ny1 = self.ny + 1;
        let idx =
            |ix: usize, iy: usize, iz: usize| -> i64 { (ix + iy * nx1 + iz * nx1 * ny1) as i64 };
        let mut pairs: Vec<(i64, i64)> = Vec::new();
        for iz in 0..=self.nz {
            for iy in 0..=self.ny {
                for ix in 0..self.nx {
                    pairs.push((idx(ix, iy, iz), idx(ix + 1, iy, iz)));
                }
            }
        }
        for iz in 0..=self.nz {
            for iy in 0..self.ny {
                for ix in 0..=self.nx {
                    pairs.push((idx(ix, iy, iz), idx(ix, iy + 1, iz)));
                }
            }
        }
        for iz in 0..self.nz {
            for iy in 0..=self.ny {
                for ix in 0..=self.nx {
                    pairs.push((idx(ix, iy, iz), idx(ix, iy, iz + 1)));
                }
            }
        }
        let ne = pairs.len();
        let flat_f: Vec<f32> = {
            let mut v = Vec::with_capacity(ne * 2);
            for (a, _) in &pairs {
                v.push(*a as f32);
            }
            for (_, b) in &pairs {
                v.push(*b as f32);
            }
            v
        };
        Tensor::<B, 1>::from_data(Data::new(flat_f, Shape::new([ne * 2])), device)
            .reshape([2, ne])
            .int()
    }
}

#[cfg(test)]
mod extruded_plate_honest_fence_tests {
    use super::{
        extruded_plate_honest_posture_bundle, extruded_plate_posture_honest,
        validate_extruded_plate_posture_honesty, ExtrudedPlateMechanics,
        EXTRUDED_PLATE_BATCH_GT1_WIRED, EXTRUDED_PLATE_HONEST_FENCE,
        EXTRUDED_PLATE_KIRCHHOFF_THIN_PLATE_GATE_WIRED, EXTRUDED_PLATE_MASTER,
        EXTRUDED_PLATE_PHYSICS_GREEN, EXTRUDED_PLATE_POSTURE_TAG, EXTRUDED_PLATE_PRODUCTION_WIRED,
        EXTRUDED_PLATE_VOIGT_CAUCHY_RECOVERY_WIRED, W29_EXTRUDED_PLATE_DEEPEN_CELL,
    };

    #[test]
    fn extruded_plate_honest_fence_consts_refuse_green_production_master() {
        assert!(!EXTRUDED_PLATE_PHYSICS_GREEN);
        assert!(!EXTRUDED_PLATE_PRODUCTION_WIRED);
        assert!(!EXTRUDED_PLATE_MASTER);
        assert!(!EXTRUDED_PLATE_VOIGT_CAUCHY_RECOVERY_WIRED);
        assert!(!EXTRUDED_PLATE_KIRCHHOFF_THIN_PLATE_GATE_WIRED);
        assert!(!EXTRUDED_PLATE_BATCH_GT1_WIRED);
        assert!(EXTRUDED_PLATE_POSTURE_TAG.contains("honest"));
        assert!(EXTRUDED_PLATE_HONEST_FENCE.contains("production_wired=false"));
        assert!(EXTRUDED_PLATE_HONEST_FENCE.contains("physics_green=false"));
        assert!(EXTRUDED_PLATE_HONEST_FENCE.contains("voigt_cauchy_recovery_wired=false"));
    }

    #[test]
    fn extruded_plate_posture_probe_honest() {
        let probe = extruded_plate_honest_posture_bundle();
        assert_eq!(probe.deepen_cell, W29_EXTRUDED_PLATE_DEEPEN_CELL);
        assert!(extruded_plate_posture_honest(&probe));
        validate_extruded_plate_posture_honesty().expect("validate_extruded_plate_posture_honesty");
    }

    #[test]
    fn extruded_plate_top_pressure_lumping_conserves_total_fz() {
        let plate = ExtrudedPlateMechanics {
            nx: 4,
            ny: 3,
            nz: 2,
            dx: 0.25,
            dy: 1.0 / 3.0,
            dz: 0.05,
        };
        let q = 1200.0_f32;
        let bf = plate.body_force_top_uniform_pressure(q);
        let sum_fz = ExtrudedPlateMechanics::sum_fz_flat(&bf);
        let expect = plate.top_uniform_pressure_total_fz(q);
        assert!(
            (sum_fz - expect).abs() < 1e-4 * expect.abs().max(1.0),
            "sum_fz={sum_fz} expect={expect}"
        );
        // Naive per-node -q dx dy over-counts by (nx+1)(ny+1)/(nx ny).
        let naive = -q * plate.dx * plate.dy * ((plate.nx + 1) * (plate.ny + 1)) as f32;
        assert!(
            (naive / expect).abs() > 1.2,
            "naive over-count ratio should exceed 1.2, got {}",
            (naive / expect).abs()
        );
    }

    #[test]
    fn extruded_plate_n_nodes_matches_brick_lattice() {
        let plate = ExtrudedPlateMechanics {
            nx: 5,
            ny: 4,
            nz: 2,
            dx: 0.2,
            dy: 0.25,
            dz: 0.05,
        };
        assert_eq!(plate.n_nodes(), 6 * 5 * 3);
    }
}
