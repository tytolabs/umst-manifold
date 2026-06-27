// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

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

use burn::tensor::{backend::Backend, Data, Int, Shape, Tensor};

use super::q1_hex_elasticity;
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
    pub fn coords_bn3<B: Backend<FloatElem = f32>>(&self, device: &B::Device) -> Tensor<B, 3> {
        let n3 = self.node_coords_n3(device);
        let [n, three] = n3.dims();
        assert_eq!(three, 3);
        n3.reshape([1, n, 3])
    }

    /// Q1-hex equilibrium solve and placeholder Voigt stress `[B,N,6]` (see module docs).
    pub fn solve_equilibrium<B: Backend<FloatElem = f32>>(
        &self,
        rho_projected: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        material: ElasticMaterial,
        cg_config: &CgConfig,
    ) -> (Tensor<B, 3>, Tensor<B, 3>) {
        let n = self.n_nodes();
        let [batch, n_rho, c] = rho_projected.dims();
        assert_eq!(batch, 1, "Q1 hex plate: batch>1 not implemented");
        assert_eq!(n_rho, n, "rho N must match extruded grid");
        assert_eq!(c, 1);
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
        debug_assert_eq!(f_flat.len(), n * 3);
        debug_assert_eq!(m_flat.len(), n * 3);

        let mut u = vec![0.0_f32; n * 3];
        let mut diag = vec![0.0_f32; n * 3];
        let mut scratch = vec![0.0_f32; n * 3];

        let max_it = cg_config.max_cg_iterations.max(1);

        let _pcg = q1_hex_elasticity::hex_solve_pcg_masked(
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
            q1_hex_elasticity::hex_precond_from_use_preconditioner(cg_config.use_preconditioner),
            cg_config.cg_tolerance,
        );
        let _ = _pcg;

        let u_tensor: Tensor<B, 3> =
            Tensor::from_data(Data::new(u, Shape::new([1, n, 3])), &device);
        let voigt = Tensor::<B, 3>::zeros([batch, n, 6], &device);
        (u_tensor, voigt)
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
