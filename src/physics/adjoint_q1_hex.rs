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

use super::adjoint::SimpElasticMaterial;
use super::linear::masked_dot;
use super::q1_hex_elasticity::{hex_cell_strain_energy, hex_solve_pcg_masked};
use super::time_orchestration::MechanicsInnerLoopConfig;

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

impl AdjointComplianceQ1Hex {
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
    ) -> (Tensor<B, 1>, f32)
    where
        B: AutodiffBackend<FloatElem = f32>,
        B::InnerBackend: Backend<FloatElem = f32>,
    {
        let nx1 = nx + 1;
        let ny1 = ny + 1;
        let n_nodes = nx1 * ny1 * (nz + 1);
        let n_cells = nx * ny * nz;

        debug_assert_eq!(
            rho_autodiff.dims(),
            [1, n_nodes, 1],
            "AdjointComplianceQ1Hex: rho shape must be [1, n_nodes, 1]"
        );

        let rho_inner = rho_autodiff.clone().inner();
        let rho_flat = rho_inner.clone().into_data().value;

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

        let f_flat = body_force.clone().into_data().value;
        let m_flat = boundary_mask.clone().into_data().value;
        debug_assert_eq!(f_flat.len(), n_nodes * 3);
        debug_assert_eq!(m_flat.len(), n_nodes * 3);

        let mut u = vec![0.0_f32; n_nodes * 3];
        let mut diag = vec![0.0_f32; n_nodes * 3];
        let mut scratch = vec![0.0_f32; n_nodes * 3];
        let max_it = cg.max_cg_iterations.max(1);

        hex_solve_pcg_masked(
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            material.nu,
            &e_cell,
            &f_flat,
            &m_flat,
            &mut u,
            &mut diag,
            &mut scratch,
            max_it,
            cg.use_preconditioner,
            cg.cg_tolerance,
        );

        let device = rho_autodiff.device();
        let u_tensor_inner = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::from_data(
            Data::new(u.clone(), Shape::new([1, n_nodes, 3])),
            &device,
        );

        let comp = masked_dot(&body_force, &u_tensor_inner, &boundary_mask);

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

        let rho_e_det_inner = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::from_data(
            Data::new(rho_e_law.clone(), Shape::new([1, n_cells, 1])),
            &device,
        );
        let rho_e_det_ad = Tensor::<B, 3>::from_inner(rho_e_det_inner);

        let ge_inner = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::from_data(
            Data::new(ge.clone(), Shape::new([1, n_cells, 1])),
            &device,
        );
        let ge_ad = Tensor::<B, 3>::from_inner(ge_inner);

        let lin_a = ge_ad.clone().mul(rho_e_ad).sum();
        let lin_b = ge_ad.mul(rho_e_det_ad).sum();
        let c_pad = Tensor::<B, 1>::from_inner(comp.clone());
        let surrogate = lin_a.sub(lin_b).add(c_pad).reshape([1]);
        let c_raw = comp.into_scalar();

        (surrogate, c_raw)
    }
}
