// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Acoustic / elastodynamic wave propagation (Phase 8) — **scaffold**.
//!
//! Newmark-β time integration on the semi-discrete wave equation is the intended path; this module
//! pins the solver surface (time step, Newmark parameters, and tensor contracts) for DEC assembly
//! and coupling with [`crate::physics::laplacian`] / mechanics operators in later work.

use burn::tensor::{backend::Backend, Tensor};

/// Semi-discrete acoustic / elastic wave integrator (Newmark family).
///
/// [`Self::newmark_beta`] and [`Self::newmark_gamma`] select the implicitness / damping of the
/// average-acceleration scheme (e.g. average acceleration: β = ¼, γ = ½; linear acceleration:
/// β = ⅙, γ = ½).
pub struct AcousticWaveSolver {
    /// Physical time step Δt (seconds).
    pub dt: f32,
    /// Newmark β parameter (scheme-dependent; typical ¼ for unconditional stability with γ = ½).
    pub newmark_beta: f32,
    /// Newmark γ parameter (typically ½ for second-order accuracy).
    pub newmark_gamma: f32,
}

impl AcousticWaveSolver {
    /// One explicit documentation pass for the semi-discrete wave step (Phase 8 stub).
    ///
    /// # Tensor contracts (documentation)
    ///
    /// | Argument | Intended shape | Role |
    /// |----------|------------------|------|
    /// | `displacement` | `[B, N, 3]` | Nodal displacement **u** (m). |
    /// | `velocity` | `[B, N, 3]` | Nodal velocity **u̇** (m/s). |
    /// | `acceleration` | `[B, N, 3]` | Nodal acceleration **ü** (m/s²). |
    /// | `nodal_density` | `[B, N, 1]` | Mass density ρ (kg/m³) per node. |
    /// | `elasticity` | `[B, N, 3, 3]` | Nodal stiffness matrix **C** (Pa) mapping strain ↔ stress in the linearized elastic/acoustic law; anisotropy is encoded per node. |
    ///
    /// Returns updated `(displacement, velocity, acceleration)` with identical shapes.
    ///
    /// ## Default builds (`solver-experimental` **off**)
    /// Returns the inputs unchanged — documented no-op so default `cargo test` stays green.
    ///
    /// ## `--features solver-experimental`
    /// Implicit Newmark-β tensor step for **M ü + K u = 0** (homogeneous, no damping). Assumptions
    /// (lumped `a·ρ` mass diagonal, local `C·u` stiffness contraction, per-node 3×3 solve) are spelled
    /// out on the internal `step_wave_experimental` helper.
    #[allow(unused_variables)]
    pub fn step_wave<B: Backend<FloatElem = f32>>(
        &self,
        displacement: Tensor<B, 3>,
        velocity: Tensor<B, 3>,
        acceleration: Tensor<B, 3>,
        nodal_density: Tensor<B, 3>,
        elasticity: Tensor<B, 4>,
    ) -> (Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>) {
        #[cfg(not(feature = "acoustics-newmark"))]
        {
            let _ = (
                self.dt,
                self.newmark_beta,
                self.newmark_gamma,
                nodal_density.clone(),
                elasticity.clone(),
            );
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
                elasticity,
            )
        }
    }
}

#[cfg(feature = "acoustics-newmark")]
const LUMPED_MASS_SCALE: f32 = 1.0;

#[cfg(feature = "acoustics-newmark")]
const DET_EPS: f32 = 1e-30_f32;

#[cfg(feature = "acoustics-newmark")]
fn elem_bn33<B: Backend<FloatElem = f32>>(m: &Tensor<B, 4>, i: usize, j: usize) -> Tensor<B, 3> {
    m.clone().narrow(2, i, 1).narrow(3, j, 1).squeeze::<3>(3)
}

/// `(K u)[b,n,:] = Σ_j C[b,n,i,j] u[b,n,j]` with shapes `[B,N,3]`.
#[cfg(feature = "acoustics-newmark")]
fn contract_elasticity_displacement<B: Backend<FloatElem = f32>>(
    elasticity: Tensor<B, 4>,
    displacement: Tensor<B, 3>,
) -> Tensor<B, 3> {
    let u_col = displacement.unsqueeze_dim::<4>(3);
    elasticity.matmul(u_col).squeeze::<3>(3)
}

#[cfg(feature = "acoustics-newmark")]
fn diagonal_mass_matrix_bn33<B: Backend<FloatElem = f32>>(
    m_scalar_bn1: Tensor<B, 3>,
    device: &B::Device,
) -> Tensor<B, 4> {
    let [b, n, one] = m_scalar_bn1.dims();
    debug_assert_eq!(one, 1);
    let zero = Tensor::<B, 3>::zeros([b, n, 1], device);
    let row0 = Tensor::cat(vec![m_scalar_bn1.clone(), zero.clone(), zero.clone()], 2);
    let row1 = Tensor::cat(vec![zero.clone(), m_scalar_bn1.clone(), zero.clone()], 2);
    let row2 = Tensor::cat(vec![zero.clone(), zero, m_scalar_bn1], 2);
    Tensor::cat(
        vec![
            row0.unsqueeze_dim::<4>(2),
            row1.unsqueeze_dim::<4>(2),
            row2.unsqueeze_dim::<4>(2),
        ],
        2,
    )
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
    // Positive definite `S` ⇒ det > 0; clamp avoids division by zero if inputs degenerate.
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
/// Experimental-only integrator: implicit Newmark-β step for **M ü + K u = 0**.
///
/// **Modelling simplifications**
///
/// 1. **Lumped mass:** `M` is diagonal and isotropic per node,
///    `m_{[b,n]} = a · ρ_{[b,n,0]}` with [`LUMPED_MASS_SCALE`] as `a`. Nodal volume / quadrature
///    lumping is **not** assembled from DEC mesh data; callers fold that into `a` or into effective ρ.
/// 2. **Stiffness–displacement product:** `(K u)[b,n,:] = C[b,n,:,:] · u[b,n,:]` (batched matvec).
///    This stands in for a mesh-assembled sparse **K** from gradients / DEC; only the local 3×3
///    nodal operator is used.
/// 3. **Implicit acceleration solve:** effective Jacobian `S = m I + β Δt² C` is inverted **per node**
///    via closed-form 3×3 inverse; `det(S)` is clamped below by [`DET_EPS`] for numerical stability.
/// 4. **Homogeneous:** no Rayleigh damping, no external force term **f**.
fn step_wave_experimental<B: Backend<FloatElem = f32>>(
    solver: &AcousticWaveSolver,
    displacement: Tensor<B, 3>,
    velocity: Tensor<B, 3>,
    acceleration: Tensor<B, 3>,
    nodal_density: Tensor<B, 3>,
    elasticity: Tensor<B, 4>,
) -> (Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>) {
    let dt = solver.dt;
    let beta = solver.newmark_beta;
    let gamma = solver.newmark_gamma;
    let dt2 = dt * dt;
    let half_minus_beta = 0.5_f32 - beta;
    let acc_n = acceleration.clone();

    let device = displacement.device();
    let m_scalar = nodal_density.mul_scalar(LUMPED_MASS_SCALE);
    let m_mat = diagonal_mass_matrix_bn33(m_scalar, &device);

    // û = u_n + Δt v_n + Δt² (½−β) a_n  — appears inside K·û on the RHS of the acceleration equation.
    let u_tilde = displacement
        .clone()
        .add(velocity.clone().mul_scalar(dt))
        .add(acc_n.clone().mul_scalar(dt2 * half_minus_beta));
    let ku_tilde = contract_elasticity_displacement(elasticity.clone(), u_tilde);

    // (M + β Δt² K) a_{n+1} = −K û  (homogeneous; implicit Newmark on M ü + K u = 0).
    let s_mat = m_mat.add(elasticity.mul_scalar(beta * dt2));
    let rhs_acc = ku_tilde.mul_scalar(-1.0_f32);
    let acc_next = solve_batched_3x3(s_mat, rhs_acc);

    // u_{n+1} = u_n + Δt v_n + Δt² [(½−β) a_n + β a_{n+1}]
    let u_next = displacement.add(velocity.clone().mul_scalar(dt)).add(
        acc_n
            .clone()
            .mul_scalar(dt2 * half_minus_beta)
            .add(acc_next.clone().mul_scalar(dt2 * beta)),
    );

    // v_{n+1} = v_n + Δt [(1−γ) a_n + γ a_{n+1}]
    let v_next = velocity.add(
        acc_n
            .mul_scalar(dt * (1.0_f32 - gamma))
            .add(acc_next.clone().mul_scalar(dt * gamma)),
    );

    (u_next, v_next, acc_next)
}
