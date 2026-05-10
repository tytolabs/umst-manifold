// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::single_range_in_vec_init)]

//! AT2 variational phase-field fracture (Phase 2) — **minimal working path** behind
//! `solver-experimental`.
//!
//! - **Tensile strain energy (spectral)**: eigenvalues \(\lambda_i\) of the symmetric small
//!   strain tensor \(\varepsilon\). Positive spectral part
//!   \(\langle\varepsilon\rangle_+ = \sum_i \langle\lambda_i\rangle_+ \, \mathbf{n}_i\otimes\mathbf{n}_i\)
//!   (Macaulay \(\langle x\rangle_+ = \max(0,x)\)). We use the scalar surrogate
//!   \(\psi^+ = \tfrac{1}{2}\,\|\langle\varepsilon\rangle_+\|_F^2
//!   = \tfrac{1}{2}\sum_i \langle\lambda_i\rangle_+^2\), i.e. half the squared Frobenius norm of
//!   the tensile spectral projection (identity stiffness in the principal frame — document any
//!   rescaling if you later couple \(\lambda,\mu\) from the mechanical kernel).
//! - **Eigenvalues on tensor**: Burn 0.13 has no public `acos` / symmetric eigendecomposition on
//!   `Tensor`. We use **fixed-step cyclic Jacobi** diagonalization (only `sqrt`, `add`, `mul`,
//!   `sign`, `mask_where`, …) on each \(3\times3\) block, so the diagonals converge to the
//!   eigenvalue multiset. Same \(\psi^+\) as from sorted \(\lambda_i\) because it is symmetric in
//!   the three eigenvalues.
//! - Degradation (for documentation / future tight coupling with mechanics):
//!   \(g(d) = (1-d)^2 + \eta\).
//! - AT2-style nodal field: `Gc/l · d − Gc · l · Δ d ≈ 2(1-d) ψ⁺` with `Δ` from
//!   [`crate::physics::laplacian::TopologicalLaplacian::scalar_laplacian`] on `edges_b1`.
//! - Irreversibility `max(d_old, d_{trial})`, then clamp to `[0, 1]`.
//!
//! Default builds (no feature): [`PhaseFieldFractureSolver::update_damage`] is a **documented
//! no-op** — returns `damage` unchanged so `cargo test` stays green.

use burn::tensor::{backend::Backend, Int, Tensor};

#[cfg(feature = "fracture-at2")]
use crate::physics::laplacian::TopologicalLaplacian;

#[cfg(feature = "fracture-at2")]
/// Pseudo-time relaxation steps for the discrete AT2 balance (**fixed count**: no scalar tolerance inside the loop).
const DAMAGE_RELAXATION_ITERS: usize = 12;

#[cfg(feature = "fracture-at2")]
/// Under-relaxation factor for residual descent.
const RELAXATION_OMEGA: f32 = 0.12;

#[cfg(feature = "fracture-at2")]
/// Cyclic Jacobi sweeps \((0,1)\to(0,2)\to(1,2)\) per sweep; enough for `f32` diagonal drift \(\ll 10^{-4}\|\varepsilon\|\) in typical strain ranges.
const JACOBI_SWEEPS: usize = 18;

#[cfg(feature = "fracture-at2")]
/// Upper-triangle packing of symmetric strain per node (`[B,N,1]` each).
type SymStrainPackBn1<B> = (
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
);

/// Phase-field length scale \(l\) (AT2 gradient regularization strength).
pub struct PhaseFieldFractureSolver {
    pub length_scale: f32,
}

impl PhaseFieldFractureSolver {
    /// Update continuum damage from strain energy and fracture toughness.
    ///
    /// # Shapes (contract)
    /// - `strain`: `[B, N, 3, 3]` symmetric strain tensor.
    /// - `damage`, `fracture_energy_gc`: `[B, N, 1]`.
    /// - `edges_b1`: `[2, E]`.
    /// - Returns updated `damage` `[B, N, 1]`.
    ///
    /// ## Default builds (`solver-experimental` **off**)
    /// Returns `damage` unchanged (documented no-op / Phase 2 stub for downstream wiring tests).
    ///
    /// ## `--features solver-experimental`
    /// Runs the minimal AT2 relaxation documented in this module (spectral tensile \(\psi^+\)).
    #[allow(unused_variables)]
    pub fn update_damage<B: Backend<FloatElem = f32>>(
        &self,
        strain: Tensor<B, 4>,
        damage: Tensor<B, 3>,
        fracture_energy_gc: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
    ) -> Tensor<B, 3> {
        #[cfg(not(feature = "fracture-at2"))]
        {
            damage
        }

        #[cfg(feature = "fracture-at2")]
        {
            update_damage_experimental(self, strain, damage, fracture_energy_gc, edges_b1)
        }
    }
}

#[cfg(feature = "fracture-at2")]
fn update_damage_experimental<B: Backend<FloatElem = f32>>(
    solver: &PhaseFieldFractureSolver,
    strain: Tensor<B, 4>,
    damage_old: Tensor<B, 3>,
    fracture_energy_gc: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
) -> Tensor<B, 3> {
    let l = solver.length_scale.max(1e-12);
    let gc = fracture_energy_gc.clone().clamp_min(1e-30_f32);

    let psi_plus = tensile_strain_energy_density_spectral_jacobi(strain);

    let mut d = damage_old.clone();
    for _ in 0..DAMAGE_RELAXATION_ITERS {
        let lap_d = TopologicalLaplacian::scalar_laplacian(d.clone(), edges_b1.clone(), d.clone());
        let one_minus_d = Tensor::<B, 3>::ones_like(&d).sub(d.clone());
        let drive = one_minus_d.mul(psi_plus.clone()).mul_scalar(2.0);
        let lin = gc.clone().div_scalar(l).mul(d.clone());
        let grad_term = gc.clone().mul_scalar(l).mul(lap_d);
        let residual = lin.sub(drive).sub(grad_term);
        d = d.sub(residual.mul_scalar(RELAXATION_OMEGA));
        d = d.clamp(0.0_f32, 1.0_f32);
    }

    let out = d.max_pair(damage_old.clone()).clamp(0.0_f32, 1.0_f32);
    let s = out.clone().sum();
    // Stay on-device: finite iff `s == s` (NaN fails) and `|s| < ∞`.
    let sum_fin = s
        .clone()
        .equal(s.clone())
        .float()
        .mul(s.abs().lower_elem(f32::INFINITY).float())
        .greater_elem(0.5_f32)
        .expand(out.dims());
    damage_old.mask_where(sum_fin, out)
}

/// Extract upper-triangle entries of symmetric strain, each `[B, N, 1]`.
#[cfg(feature = "fracture-at2")]
fn strain_sym_components_bn1<B: Backend<FloatElem = f32>>(
    strain: Tensor<B, 4>,
) -> SymStrainPackBn1<B> {
    let [b, n, _, _] = strain.dims();
    let reshape_bn1 = [b, n, 1];
    let e00 = strain
        .clone()
        .narrow(2, 0, 1)
        .narrow(3, 0, 1)
        .reshape(reshape_bn1);
    let e01 = strain
        .clone()
        .narrow(2, 0, 1)
        .narrow(3, 1, 1)
        .reshape(reshape_bn1);
    let e02 = strain
        .clone()
        .narrow(2, 0, 1)
        .narrow(3, 2, 1)
        .reshape(reshape_bn1);
    let e11 = strain
        .clone()
        .narrow(2, 1, 1)
        .narrow(3, 1, 1)
        .reshape(reshape_bn1);
    let e12 = strain
        .clone()
        .narrow(2, 1, 1)
        .narrow(3, 2, 1)
        .reshape(reshape_bn1);
    let e22 = strain.narrow(2, 2, 1).narrow(3, 2, 1).reshape(reshape_bn1);
    (e00, e01, e02, e11, e12, e22)
}

/// Jacobi tangent \(t=\tan\theta\) for annihilating \((p,q)\) off-diagonal (Golub–Van Loan stable form).
#[cfg(feature = "fracture-at2")]
fn jacobi_t_bn1<B: Backend<FloatElem = f32>>(
    app: Tensor<B, 3>,
    aqq: Tensor<B, 3>,
    apq: Tensor<B, 3>,
) -> Tensor<B, 3> {
    let apq_active = apq.clone().abs().greater_elem(1e-20_f32);
    let apq_tiny = apq.clone().abs().lower_elem(1e-20_f32);
    // When |apq|≈0 the Golub–Van Loan ratio is undefined; use a harmless denom so `rho` stays finite;
    // the returned tangent is zeroed wherever `apq_active` is false (see last line).
    let denom = apq
        .clone()
        .mul_scalar(2.0)
        .mask_where(apq_tiny, Tensor::<B, 3>::ones_like(&apq));
    let rho = app.clone().sub(aqq.clone()).div(denom);
    let sqrt_one_rho2 = rho.clone().mul(rho.clone()).add_scalar(1.0_f32).sqrt();
    let t_unequal = rho.clone().sign().div(rho.abs().add(sqrt_one_rho2));
    let t_equal_diag = apq.clone().sign();
    let t_branch = t_unequal.mask_where(app.sub(aqq).abs().lower_elem(1e-12_f32), t_equal_diag);
    Tensor::<B, 3>::zeros_like(&apq).mask_where(apq_active, t_branch)
}

#[cfg(feature = "fracture-at2")]
fn jacobi_cs_from_t_bn1<B: Backend<FloatElem = f32>>(
    t: Tensor<B, 3>,
) -> (Tensor<B, 3>, Tensor<B, 3>) {
    let c = t
        .clone()
        .powf_scalar(2.0)
        .add_scalar(1.0_f32)
        .sqrt()
        .recip();
    let s = t.mul(c.clone());
    (c, s)
}

#[cfg(feature = "fracture-at2")]
fn jacobi_sweep_01<B: Backend<FloatElem = f32>>(
    e00: Tensor<B, 3>,
    e01: Tensor<B, 3>,
    e02: Tensor<B, 3>,
    e11: Tensor<B, 3>,
    e12: Tensor<B, 3>,
    e22: Tensor<B, 3>,
) -> SymStrainPackBn1<B> {
    let t = jacobi_t_bn1(e00.clone(), e11.clone(), e01.clone());
    let (c, s) = jacobi_cs_from_t_bn1(t);
    let c2 = c.clone().mul(c.clone());
    let s2 = s.clone().mul(s.clone());
    let cs = c.clone().mul(s.clone());
    let e00_new = c2
        .clone()
        .mul(e00.clone())
        .sub(cs.clone().mul_scalar(2.0).mul(e01.clone()))
        .add(s2.clone().mul(e11.clone()));
    let e11_new = s2
        .clone()
        .mul(e00.clone())
        .add(cs.clone().mul_scalar(2.0).mul(e01.clone()))
        .add(c2.clone().mul(e11.clone()));
    let e01_new = c2.sub(s2.clone()).mul(e01).add(cs.mul(e00.sub(e11)));
    let e02_new = c.clone().mul(e02.clone()).sub(s.clone().mul(e12.clone()));
    let e12_new = s.mul(e02).add(c.mul(e12));
    (e00_new, e01_new, e02_new, e11_new, e12_new, e22)
}

#[cfg(feature = "fracture-at2")]
fn jacobi_sweep_02<B: Backend<FloatElem = f32>>(
    e00: Tensor<B, 3>,
    e01: Tensor<B, 3>,
    e02: Tensor<B, 3>,
    e11: Tensor<B, 3>,
    e12: Tensor<B, 3>,
    e22: Tensor<B, 3>,
) -> SymStrainPackBn1<B> {
    let t = jacobi_t_bn1(e00.clone(), e22.clone(), e02.clone());
    let (c, s) = jacobi_cs_from_t_bn1(t);
    let c2 = c.clone().mul(c.clone());
    let s2 = s.clone().mul(s.clone());
    let cs = c.clone().mul(s.clone());
    let e00_new = c2
        .clone()
        .mul(e00.clone())
        .sub(cs.clone().mul_scalar(2.0).mul(e02.clone()))
        .add(s2.clone().mul(e22.clone()));
    let e22_new = s2
        .clone()
        .mul(e00.clone())
        .add(cs.clone().mul_scalar(2.0).mul(e02.clone()))
        .add(c2.clone().mul(e22.clone()));
    let e02_new = c2
        .sub(s2.clone())
        .mul(e02)
        .add(cs.mul(e00.clone().sub(e22)));
    let e01_new = c.clone().mul(e01.clone()).sub(s.clone().mul(e12.clone()));
    let e12_new = s.mul(e01).add(c.mul(e12));
    (e00_new, e01_new, e02_new, e11, e12_new, e22_new)
}

#[cfg(feature = "fracture-at2")]
fn jacobi_sweep_12<B: Backend<FloatElem = f32>>(
    e00: Tensor<B, 3>,
    e01: Tensor<B, 3>,
    e02: Tensor<B, 3>,
    e11: Tensor<B, 3>,
    e12: Tensor<B, 3>,
    e22: Tensor<B, 3>,
) -> SymStrainPackBn1<B> {
    let t = jacobi_t_bn1(e11.clone(), e22.clone(), e12.clone());
    let (c, s) = jacobi_cs_from_t_bn1(t);
    let c2 = c.clone().mul(c.clone());
    let s2 = s.clone().mul(s.clone());
    let cs = c.clone().mul(s.clone());
    let e11_new = c2
        .clone()
        .mul(e11.clone())
        .sub(cs.clone().mul_scalar(2.0).mul(e12.clone()))
        .add(s2.clone().mul(e22.clone()));
    let e22_new = s2
        .clone()
        .mul(e11.clone())
        .add(cs.clone().mul_scalar(2.0).mul(e12.clone()))
        .add(c2.clone().mul(e22.clone()));
    let e12_new = c2
        .sub(s2.clone())
        .mul(e12)
        .add(cs.mul(e11.clone().sub(e22)));
    let e01_new = c.clone().mul(e01.clone()).sub(s.clone().mul(e02.clone()));
    let e02_new = s.mul(e01).add(c.mul(e02));
    (e00, e01_new, e02_new, e11_new, e12_new, e22_new)
}

/// Approximate eigenvalues by cyclic Jacobi diagonalization, then
/// \(\psi^+ = \tfrac{1}{2}\sum_i \langle\lambda_i\rangle_+^2\).
#[cfg(feature = "fracture-at2")]
fn tensile_strain_energy_density_spectral_jacobi<B: Backend<FloatElem = f32>>(
    strain: Tensor<B, 4>,
) -> Tensor<B, 3> {
    let (mut e00, mut e01, mut e02, mut e11, mut e12, mut e22) = strain_sym_components_bn1(strain);
    for _ in 0..JACOBI_SWEEPS {
        (e00, e01, e02, e11, e12, e22) = jacobi_sweep_01(e00, e01, e02, e11, e12, e22);
        (e00, e01, e02, e11, e12, e22) = jacobi_sweep_02(e00, e01, e02, e11, e12, e22);
        (e00, e01, e02, e11, e12, e22) = jacobi_sweep_12(e00, e01, e02, e11, e12, e22);
    }
    let l0 = e00.clamp_min(0.0_f32);
    let l1 = e11.clamp_min(0.0_f32);
    let l2 = e22.clamp_min(0.0_f32);
    l0.powf_scalar(2.0)
        .add(l1.powf_scalar(2.0))
        .add(l2.powf_scalar(2.0))
        .mul_scalar(0.5_f32)
}
