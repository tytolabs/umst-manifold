// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Statistical-mechanics → continuum bridge (Phase 9).
//!
//! ## Shipped **`[B, 4]`** bridge (Burn `f32`, AD-safe)
//!
//! Rows \((\varepsilon,\sigma,\rho^*,T^*)\) drive a **third-order truncated virial** reduced pressure
//! \[
//! P^* = \rho^* T^* + T^* B_2^*(T^*)\,{\rho^*}^2 + T^* B_3^*(T^*)\,{\rho^*}^3
//! \]
//! with \(B_2^*\) a **Mayer second-virial surrogate** (polynomial in \(1/T^*\) fitted to the
//! numerical Mayer integral on \(T^*\in[1,5]\)) and \(B_3^*\) a **positive Padé surrogate**
//! \(1.2/(T^*+0.15)\) clamped to \([0.08,1.5]\) — a **differentiable** third-density correction
//! (not the full triangle Mayer integral; see **Open roadmap items** below).
//!
//! Isothermal bulk stiffness uses
//! \[
//! K^* = \rho^* \left(\frac{\partial P^*}{\partial \rho^*}\right)_{T^*}
//!     = \rho^* T^* + 2 T^* B_2^* {\rho^*}^2 + 3 T^* B_3^* {\rho^*}^3,
//! \qquad
//! K_T = (\varepsilon/\sigma^3)\, K^*.
//! \]
//! The forward map applies this **closed form** (pure Burn ops). It matches reverse-mode
//! \(\partial(\sum_i P^*_i)/\partial \rho^*_i\) on independent rows — verified in
//! `tests/verification/statmech_mechanics_fracture_bridge.rs`.
//!
//! ## Grand-canonical surface energy (KB-style scalar proxy)
//!
//! \[
//! \gamma_{\mathrm{gc}}
//!   = C_\gamma \,\frac{\varepsilon}{\sigma^2}\,
//!     \underbrace{\rho^*(1-\rho^*)}_{\text{mixture / excess window}}\,
//!     \sqrt{\frac{T^*}{T^* + \tfrac12}}
//! \]
//! (clamped window, differentiable). This is a **rank-0** Kirkwood–Buff / coexistence **tensor
//! approximation**: the \(\rho^*(1-\rho^*)\) factor mimics interface adsorption excess near a
//! two-phase mixture, modulated by a thermal weight — **not** a full KB integral or Widom route.
//!
//! ## **`[B, 2]`** lane
//!
//! Placeholder \(K \propto \varepsilon/\sigma^3\), \(\gamma \propto \varepsilon/\sigma^2\) (fully
//! differentiable; unchanged).
//!
//! ## Johnson (1993) **`f64`** reference (host, not the default **`[B,4]`** path)
//!
//! [`physical_bulk_modulus_johnson1993`] and [`relative_placeholder_bulk_modulus_gap_vs_johnson1993`]
//! remain for **scalar** comparisons vs JZG / teqp. The **`[B,4]`** host **`f64`** row loop that
//! materialised Johnson **`K_T`** was **removed** from [`upscale_potentials`] (Milestone 2.1).
//! Optional feature **`statistical-mechanics-johnson-reference`** still exposes
//! [`bulk_modulus_from_lj_state_johnson1993`] for reduced-\(K^*\) parity tests.
//!
//! ## Sub-grid → macro hooks
//!
//! Reference normalisation for mechanics / fracture scaling:
//! **`VIADU_K_REF_F32`**, **`GAMMA_GC_REF_VIADU_F32`** at \((\varepsilon,\sigma,\rho^*,T^*)=(1,1,0.2,2)\).
//!
//! ## Open roadmap items
//!
//! - Full **Mayer \(B_3^*\)** triangle integral and MD / Johnson dense-fluid agreement at
//!   \(\rho^* \gtrsim 0.15\): truncated virial is a **surrogate**; tighten vs Johnson or MD on a
//!   calibration grid when closing matrix **#9**.
//! - **3D topology optimisation** through mechanics equilibrium: out of scope; hooks are
//!   tensor-local scaling only.

use burn::tensor::{backend::Backend, Tensor};
use std::fmt;

use crate::physics::error::PhysicsError;

/// Fail-closed guard for LJ virial column tensors (`[B, 1]`).
fn guard_lj_column_tensor_shape(
    dims: [usize; 2],
    context: &'static str,
) -> Result<(), PhysicsError> {
    let [batch, cols] = dims;
    if batch == 0 {
        return Err(PhysicsError::ShapeMismatch {
            context,
            detail: "batch dimension must be > 0",
        });
    }
    if cols != 1 {
        return Err(PhysicsError::ShapeMismatch {
            context,
            detail: "expected [B, 1] column tensor",
        });
    }
    Ok(())
}

/// Reference row \((\varepsilon,\sigma,\rho^*,T^*) = (1,1,0.2,2)\) for **`[B,4]`** virial **`K^*`** (reduced).
///
/// Used by [`crate::physics::mechanics::scale_stiffness_young_first_channel_with_statmech_ratio`]
/// to normalise sub-grid stiffening.
pub const VIADU_LJ_STATE_EPS1_SIG1_RHO02_T2: (f32, f32, f32, f32) = (1.0, 1.0, 0.2, 2.0);

/// Isothermal **`K^*`** from the shipped virial closure at [`VIADU_LJ_STATE_EPS1_SIG1_RHO02_T2`].
pub const VIADU_K_REF_F32: f32 = 0.124_902_4;

/// **`γ_gc`** from [`surface_energy_gc_kb_proxy`] at [`VIADU_LJ_STATE_EPS1_SIG1_RHO02_T2`] with unit scales.
pub const GAMMA_GC_REF_VIADU_F32: f32 = 0.143_108_42;

/// Wrong `lennard_jones_params` shape for [`StatisticalBridge::upscale_potentials`] / [`upscale_potentials`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpscalePotentialsShapeError {
    /// First dimension (batch).
    pub batch: usize,
    /// Second dimension (must be **2** or **4**).
    pub cols: usize,
}

impl fmt::Display for UpscalePotentialsShapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StatisticalBridge::upscale_potentials: expected [B,2] or [B,4], got dims [{}, {}]",
            self.batch, self.cols
        )
    }
}

impl std::error::Error for UpscalePotentialsShapeError {}

/// Dimensionless scale for the analytic bulk-modulus placeholder on **`[B,2]`**
/// \(K = C_K \, \varepsilon / \sigma^3\).
pub const ANALYTIC_BULK_MODULUS_SCALE: f32 = 1.0;

/// Dimensionless scale on **`[B,2]`** for \(\gamma_{\mathrm{gc}} = C_\gamma \, \varepsilon / \sigma^2\).
pub const ANALYTIC_SURFACE_ENERGY_SCALE: f32 = 1.0;

/// Mayer \(B_2^*(T^*)\) surrogate: `a0 + a1/T* + a2/T*^2 + a3/T*^3` (fitted to the numerical Mayer
/// integral for LJ 12–6 on \(T^*\in\{1.0,1.2,\ldots,5.0\}\), max abs error \(\approx 3\times 10^{-3}\)).
#[inline]
fn lj_mayer_b2_star_coeffs() -> (f32, f32, f32, f32) {
    (0.908_480_1, -4.360_006, -2.483_973, 0.045_712_66)
}

/// Third-virial **surrogate** \(B_3^*(T^*)\): `clamp(0.08, 1.2/(T*+0.15), 1.5)` — positive, monotone
/// decreasing in \(T^*\); **not** the full Mayer \(B_3^*\) cluster integral (see module open roadmap items).
#[inline]
pub fn lj_virial_b3_star_surrogate_scalar(t_star: f32) -> f32 {
    (1.2 / (t_star + 0.15)).clamp(0.08, 1.5)
}

/// Reduced Mayer \(B_2^*(T^*)\) surrogate as pure Burn ops; `t_star` shape **`[B, 1]`**.
///
/// Returns [`PhysicsError::ShapeMismatch`] when `t_star` is not a non-empty `[B, 1]` column.
pub fn lj_mayer_b2_star_tensor<B: Backend<FloatElem = f32>>(
    t_star: Tensor<B, 2>,
) -> Result<Tensor<B, 2>, PhysicsError> {
    guard_lj_column_tensor_shape(t_star.dims(), "lj_mayer_b2_star_tensor")?;
    Ok(lj_mayer_b2_star_tensor_inner(t_star))
}

fn lj_mayer_b2_star_tensor_inner<B: Backend<FloatElem = f32>>(t_star: Tensor<B, 2>) -> Tensor<B, 2> {
    let (a0, a1, a2, a3) = lj_mayer_b2_star_coeffs();
    let inv = t_star.clone().recip();
    let inv2 = inv.clone().mul(inv.clone());
    let inv3 = inv2.clone().mul(inv.clone());
    Tensor::zeros_like(&t_star)
        .add_scalar(a0)
        .add(inv.mul_scalar(a1))
        .add(inv2.mul_scalar(a2))
        .add(inv3.mul_scalar(a3))
}

/// \(B_3^*(T^*)\) surrogate tensor; `t_star` shape **`[B, 1]`**.
///
/// Returns [`PhysicsError::ShapeMismatch`] when `t_star` is not a non-empty `[B, 1]` column.
pub fn lj_virial_b3_star_tensor<B: Backend<FloatElem = f32>>(
    t_star: Tensor<B, 2>,
) -> Result<Tensor<B, 2>, PhysicsError> {
    guard_lj_column_tensor_shape(t_star.dims(), "lj_virial_b3_star_tensor")?;
    Ok(lj_virial_b3_star_tensor_inner(t_star))
}

fn lj_virial_b3_star_tensor_inner<B: Backend<FloatElem = f32>>(t_star: Tensor<B, 2>) -> Tensor<B, 2> {
    t_star
        .clone()
        .add_scalar(0.15)
        .recip()
        .mul_scalar(1.2)
        .clamp(0.08, 1.5)
}

/// Third-order virial reduced pressure \(P^*(\rho^*,T^*)\); inputs **`[B,1]`** each.
pub fn reduced_pressure_lj_virial_third_order<B: Backend<FloatElem = f32>>(
    rho_star: Tensor<B, 2>,
    t_star: Tensor<B, 2>,
) -> Tensor<B, 2> {
    let b2 = lj_mayer_b2_star_tensor_inner(t_star.clone());
    let b3 = lj_virial_b3_star_tensor_inner(t_star.clone());
    let r = rho_star.clone();
    let r2 = r.clone().mul(r.clone());
    let r3 = r2.clone().mul(r.clone());
    let term1 = r.mul(t_star.clone());
    let term2 = t_star.clone().mul(b2).mul(r2);
    let term3 = t_star.mul(b3).mul(r3);
    term1.add(term2).add(term3)
}

/// Reduced isothermal \(K^* = \rho^* (\partial P^*/\partial \rho^*)_{T^*}\) for the virial closure (closed form).
pub fn reduced_isothermal_kt_star_virial_closed_form<B: Backend<FloatElem = f32>>(
    rho_star: Tensor<B, 2>,
    t_star: Tensor<B, 2>,
) -> Tensor<B, 2> {
    let b2 = lj_mayer_b2_star_tensor_inner(t_star.clone());
    let b3 = lj_virial_b3_star_tensor_inner(t_star.clone());
    let r = rho_star.clone();
    let r2 = r.clone().mul(r.clone());
    let r3 = r2.clone().mul(r.clone());
    let t = t_star;
    r.mul(t.clone())
        .add(t.clone().mul(b2).mul(r2).mul_scalar(2.0))
        .add(t.mul(b3).mul(r3).mul_scalar(3.0))
}

/// Kirkwood–Buff / coexistence **scalar proxy** for \(\gamma_{\mathrm{gc}}\) (see module docs).
pub fn surface_energy_gc_kb_proxy<B: Backend<FloatElem = f32>>(
    epsilon: Tensor<B, 2>,
    sigma: Tensor<B, 2>,
    rho_star: Tensor<B, 2>,
    t_star: Tensor<B, 2>,
) -> Tensor<B, 2> {
    let sig_sq = sigma.clone().mul(sigma.clone());
    let one = Tensor::zeros_like(&sigma).add_scalar(1.0);
    let excess = rho_star
        .clone()
        .mul(one.sub(rho_star.clone()))
        .clamp(0.0, 0.3);
    let thermal = t_star
        .clone()
        .div(t_star.add_scalar(0.5))
        .sqrt()
        .clamp_min(1e-6_f32);
    epsilon
        .div(sig_sq)
        .mul(excess)
        .mul(thermal)
        .mul_scalar(ANALYTIC_SURFACE_ENERGY_SCALE)
}

/// Carrier for Phase 9 statistical-mechanics bridging logic (stateless).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StatisticalBridge;

impl StatisticalBridge {
    /// Maps Lennard-Jones parameter rows to bulk modulus \(K_T\) and \(\gamma_{\mathrm{gc}}\).
    ///
    /// # Shapes
    /// - **`[B, 2]`:** \((\varepsilon,\sigma)\). Placeholder \(K\), analytic \(\gamma\).
    /// - **`[B, 4]`:** \((\varepsilon,\sigma,\rho^*,T^*)\). **`K_T`** from virial \(K^*\) (Burn `f32`);
    ///   **`γ_gc`** from [`surface_energy_gc_kb_proxy`].
    pub fn upscale_potentials<B: Backend<FloatElem = f32>>(
        &self,
        lennard_jones_params: Tensor<B, 2>,
    ) -> Result<(Tensor<B, 2>, Tensor<B, 2>), UpscalePotentialsShapeError> {
        let [batch, cols] = lennard_jones_params.dims();

        if cols == 2 {
            let eps = lennard_jones_params.clone().slice([0..batch, 0..1]);
            let sig = lennard_jones_params.slice([0..batch, 1..2]);
            let sig_sq = sig.clone().mul(sig.clone());
            let sig_cu = sig_sq.clone().mul(sig);
            let bulk_modulus = eps
                .clone()
                .div(sig_cu)
                .mul_scalar(ANALYTIC_BULK_MODULUS_SCALE);
            let surface_energy_gc = eps.div(sig_sq).mul_scalar(ANALYTIC_SURFACE_ENERGY_SCALE);
            Ok((bulk_modulus, surface_energy_gc))
        } else if cols == 4 {
            let eps = lennard_jones_params.clone().slice([0..batch, 0..1]);
            let sig = lennard_jones_params.clone().slice([0..batch, 1..2]);
            let rho = lennard_jones_params.clone().slice([0..batch, 2..3]);
            let tstar = lennard_jones_params.clone().slice([0..batch, 3..4]);
            let k_star = reduced_isothermal_kt_star_virial_closed_form(rho.clone(), tstar.clone());
            let sig_cu = sig.clone().mul(sig.clone()).mul(sig.clone());
            let bulk_modulus = k_star.mul(eps.clone().div(sig_cu));
            let surface_energy_gc = surface_energy_gc_kb_proxy(eps, sig, rho, tstar);
            Ok((bulk_modulus, surface_energy_gc))
        } else {
            Err(UpscalePotentialsShapeError { batch, cols })
        }
    }
}

/// Stateless entry: delegates to [`StatisticalBridge::upscale_potentials`].
#[inline]
pub fn upscale_potentials<B: Backend<FloatElem = f32>>(
    lennard_jones_params: Tensor<B, 2>,
) -> Result<(Tensor<B, 2>, Tensor<B, 2>), UpscalePotentialsShapeError> {
    StatisticalBridge.upscale_potentials(lennard_jones_params)
}

/// Host **`f64`** Johnson **\(K_T\)** for each **`[B,4]`** row via [`physical_bulk_modulus_johnson1993`].
///
/// **Johnson route** (reference / audits): tape-unsafe, host-only — use beside the default
/// **`[B,4]`** [`upscale_potentials`] path (third-order **virial surrogate** **`K_T`** in Burn `f32`).
pub fn upscale_potentials_b4_johnson_reference_bulk_modulus_host<B: Backend<FloatElem = f32>>(
    lennard_jones_params_b4: Tensor<B, 2>,
) -> Result<Vec<f64>, UpscalePotentialsShapeError> {
    let [batch, cols] = lennard_jones_params_b4.dims();
    if cols != 4 {
        return Err(UpscalePotentialsShapeError { batch, cols });
    }
    let flat = lennard_jones_params_b4.into_data().value;
    let mut out = Vec::with_capacity(batch);
    for r in 0..batch {
        let b = r * 4;
        let eps = f64::from(flat[b]);
        let sig = f64::from(flat[b + 1]);
        let rho = f64::from(flat[b + 2]);
        let t = f64::from(flat[b + 3]);
        out.push(physical_bulk_modulus_johnson1993(rho, t, eps, sig));
    }
    Ok(out)
}

/// Johnson reduced \(K^*\) — **opt-in** re-export (`f64` reference lane).
#[cfg(feature = "statistical-mechanics-johnson-reference")]
#[inline]
#[must_use]
pub fn bulk_modulus_from_lj_state_johnson1993(rho_star: f64, t_star: f64) -> f64 {
    super::lj_johnson_1993_reference::bulk_modulus_from_lj_state_johnson1993(rho_star, t_star)
}

/// Johnson physical \(K_T\) (`f64`); **not** used by default [`upscale_potentials`] **`[B,4]`** path.
#[inline]
#[must_use]
pub fn physical_bulk_modulus_johnson1993(
    rho_star: f64,
    t_star: f64,
    epsilon: f64,
    sigma: f64,
) -> f64 {
    let k_star =
        super::lj_johnson_1993_reference::bulk_modulus_from_lj_state_johnson1993(rho_star, t_star);
    super::lj_johnson_1993_reference::bulk_modulus_from_reduced(k_star, epsilon, sigma)
}

/// Relative error between analytic **`[B,2]`** placeholder \(K\) and Johnson \(K_T\).
#[inline]
#[must_use]
pub fn relative_placeholder_bulk_modulus_gap_vs_johnson1993(
    rho_star: f64,
    t_star: f64,
    epsilon: f64,
    sigma: f64,
) -> f64 {
    let k_j = physical_bulk_modulus_johnson1993(rho_star, t_star, epsilon, sigma);
    let k_ph = f64::from(ANALYTIC_BULK_MODULUS_SCALE) * epsilon / sigma.powi(3);
    ((k_ph - k_j) / k_j).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::error::PhysicsError;
    use approx::assert_abs_diff_eq;
    use burn::tensor::{Data, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    #[test]
    fn upscale_potentials_output_shapes_match_batch() {
        let dev = NdArrayDevice::Cpu;
        let lj: Tensor<B, 2> = Tensor::from_data(
            Data::new(vec![0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32], Shape::new([2, 2])),
            &dev,
        );
        let (k, gamma) = upscale_potentials(lj.clone()).expect(
            "statistical_mechanics::upscale_potentials on [B,2] LJ state returns [B,1] bulk modulus and surface energy tensors (FP §6 Track G statmech residual)",
        );
        assert_eq!(k.dims(), [2, 1]);
        assert_eq!(gamma.dims(), [2, 1]);

        let (k2, g2) = StatisticalBridge
            .upscale_potentials(lj)
            .expect(
                "StatisticalBridge::upscale_potentials trait dispatch on [B,2] LJ state matches free fn output shapes (FP §6 Track G statmech residual)",
            );
        assert_eq!(k2.dims(), [2, 1]);
        assert_eq!(g2.dims(), [2, 1]);
    }

    #[test]
    fn upscale_potentials_analytic_scales_match_epsilon_sigma_powers() {
        let dev = NdArrayDevice::Cpu;
        let lj: Tensor<B, 2> = Tensor::from_data(
            Data::new(vec![0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32], Shape::new([2, 2])),
            &dev,
        );
        let (k, gamma) = upscale_potentials(lj).expect(
            "statistical_mechanics::upscale_potentials analytic ε/σ³ and ε/σ² scale law on [B,2] batch (FP §6 Track G statmech residual)",
        );
        let k_v = k.into_data().value;
        let g_v = gamma.into_data().value;
        assert_eq!(k_v.len(), 2);
        assert_eq!(g_v.len(), 2);

        let c_k = ANALYTIC_BULK_MODULUS_SCALE;
        let c_g = ANALYTIC_SURFACE_ENERGY_SCALE;
        assert_abs_diff_eq!(k_v[0], c_k * 0.1_f32 / 0.2_f32.powi(3), epsilon = 1.0e-5);
        assert_abs_diff_eq!(k_v[1], c_k * 0.3_f32 / 0.4_f32.powi(3), epsilon = 1.0e-5);
        assert_abs_diff_eq!(g_v[0], c_g * 0.1_f32 / 0.2_f32.powi(2), epsilon = 1.0e-5);
        assert_abs_diff_eq!(g_v[1], c_g * 0.3_f32 / 0.4_f32.powi(2), epsilon = 1.0e-5);

        assert!(k_v.iter().all(|x| x.is_finite() && *x > 0.0));
        assert!(g_v.iter().all(|x| x.is_finite() && *x > 0.0));
    }

    #[test]
    fn physical_bulk_modulus_johnson1993_matches_reduced_composition() {
        let rho = 0.2_f64;
        let t = 2.0_f64;
        let e = 1.0_f64;
        let s = 0.8_f64;
        let k_star =
            super::super::lj_johnson_1993_reference::bulk_modulus_from_lj_state_johnson1993(rho, t);
        let via_reduced =
            super::super::lj_johnson_1993_reference::bulk_modulus_from_reduced(k_star, e, s);
        assert_abs_diff_eq!(
            super::physical_bulk_modulus_johnson1993(rho, t, e, s),
            via_reduced,
            epsilon = 1.0e-15
        );
    }

    #[test]
    fn relative_placeholder_gap_matches_manual_supercritical_state() {
        let rho_star = 0.2_f64;
        let t_star = 2.0_f64;
        let epsilon = 1.0_f64;
        let sigma = 0.8_f64;
        let k_j = super::physical_bulk_modulus_johnson1993(rho_star, t_star, epsilon, sigma);
        let k_ph = f64::from(ANALYTIC_BULK_MODULUS_SCALE) * epsilon / sigma.powi(3);
        let manual = ((k_ph - k_j) / k_j).abs();
        assert_abs_diff_eq!(
            super::relative_placeholder_bulk_modulus_gap_vs_johnson1993(
                rho_star, t_star, epsilon, sigma,
            ),
            manual,
            epsilon = 1.0e-15
        );
    }

    #[cfg(feature = "statistical-mechanics-johnson-reference")]
    #[test]
    fn bulk_modulus_johnson1993_statmech_reexport_matches_lj_reference() {
        let rho = 0.2_f64;
        let t = 2.0_f64;
        assert_abs_diff_eq!(
            bulk_modulus_from_lj_state_johnson1993(rho, t),
            super::super::lj_johnson_1993_reference::bulk_modulus_from_lj_state_johnson1993(rho, t),
            epsilon = 1.0e-12
        );
    }

    /// **`[B,4]`** `K` matches closed-form virial \(K^*\) scaled by \(\varepsilon/\sigma^3\).
    #[test]
    fn upscale_potentials_b4_batch_matches_virial_kt_per_row() {
        let dev = NdArrayDevice::Cpu;
        let rows: Vec<f32> = vec![
            1.0, 0.9, 0.15, 2.5, //
            0.5, 1.1, 0.22, 2.0,
        ];
        let batch = 2usize;
        let lj: Tensor<B, 2> =
            Tensor::from_data(Data::new(rows.clone(), Shape::new([batch, 4])), &dev);
        let (k, _gamma) = upscale_potentials(lj.clone()).expect(
            "statistical_mechanics::upscale_potentials on [B,4] virial K_T per-row bulk modulus vs closed-form virial (FP §6 Track G statmech residual)",
        );
        assert_eq!(k.dims(), [batch, 1]);
        let kv = k.into_data().value;
        for (row, &kval) in rows.chunks_exact(4).zip(kv.iter()) {
            let eps = row[0];
            let sig = row[1];
            let rho = row[2];
            let t = row[3];
            let rho_t: Tensor<B, 2> =
                Tensor::from_data(Data::new(vec![rho], Shape::new([1, 1])), &dev);
            let t_t: Tensor<B, 2> = Tensor::from_data(Data::new(vec![t], Shape::new([1, 1])), &dev);
            let k_star = reduced_isothermal_kt_star_virial_closed_form(rho_t, t_t).into_scalar();
            let want = k_star * eps / sig.powi(3);
            assert_abs_diff_eq!(kval, want, epsilon = 2.0e-5_f32);
        }
    }

    #[test]
    fn upscale_potentials_rejects_bad_column_count() {
        let dev = NdArrayDevice::Cpu;
        let lj: Tensor<B, 2> = Tensor::from_data(
            Data::new(vec![1.0_f32, 2.0_f32, 3.0_f32], Shape::new([1, 3])),
            &dev,
        );
        let err = upscale_potentials(lj).unwrap_err();
        assert_eq!(err, UpscalePotentialsShapeError { batch: 1, cols: 3 });
    }

    #[test]
    fn lj_mayer_b2_star_tensor_rejects_non_column_shape() {
        let dev = NdArrayDevice::Cpu;
        let t_bad: Tensor<B, 2> =
            Tensor::from_data(Data::new(vec![1.0_f32, 2.0_f32], Shape::new([1, 2])), &dev);
        let err = lj_mayer_b2_star_tensor(t_bad).unwrap_err();
        assert!(matches!(
            err,
            PhysicsError::ShapeMismatch {
                context: "lj_mayer_b2_star_tensor",
                ..
            }
        ));
    }

    #[test]
    fn lj_virial_b3_star_tensor_rejects_empty_batch() {
        let dev = NdArrayDevice::Cpu;
        let t_empty: Tensor<B, 2> = Tensor::from_data(Data::new(Vec::<f32>::new(), Shape::new([0, 1])), &dev);
        let err = lj_virial_b3_star_tensor(t_empty).unwrap_err();
        assert!(matches!(
            err,
            PhysicsError::ShapeMismatch {
                context: "lj_virial_b3_star_tensor",
                ..
            }
        ));
    }

    #[test]
    fn upscale_potentials_b4_johnson_matches_cat_of_column_slices() {
        let dev = NdArrayDevice::Cpu;
        let batch = 2usize;
        let lj: Tensor<B, 2> = Tensor::from_data(
            Data::new(
                vec![
                    1.0_f32, 0.9_f32, 0.15_f32, 2.5_f32, 0.5_f32, 1.1_f32, 0.22_f32, 2.0_f32,
                ],
                Shape::new([batch, 4]),
            ),
            &dev,
        );
        let (k_full, g_full) = upscale_potentials(lj.clone())
            .expect("upscale_potentials [B,4] johnson cat parity (full tensor)");
        let eps = lj.clone().slice([0..batch, 0..1]);
        let sig = lj.clone().slice([0..batch, 1..2]);
        let rho = lj.clone().slice([0..batch, 2..3]);
        let tstar = lj.clone().slice([0..batch, 3..4]);
        let lj_cat = Tensor::cat(vec![eps, sig, rho, tstar], 1);
        let (k_cat, g_cat) = upscale_potentials(lj_cat)
            .expect("upscale_potentials [B,4] johnson cat parity (column slices)");
        assert!(k_full.clone().equal(k_cat.clone()).all().into_scalar());
        assert!(g_full.clone().equal(g_cat.clone()).all().into_scalar());
    }

    #[test]
    fn viadu_k_ref_constant_matches_tensor_row() {
        let dev = NdArrayDevice::Cpu;
        let (e, s, r, t) = VIADU_LJ_STATE_EPS1_SIG1_RHO02_T2;
        let lj = Tensor::<B, 2>::from_data(Data::new(vec![e, s, r, t], Shape::new([1, 4])), &dev);
        let (k, _) = upscale_potentials(lj).expect(
            "statistical_mechanics::upscale_potentials VIADU reference LJ row bulk modulus vs VIADU_K_REF_F32 (FP §6 Track G statmech residual)",
        );
        assert_abs_diff_eq!(k.into_scalar(), VIADU_K_REF_F32, epsilon = 1.0e-4_f32);
    }

    #[test]
    fn viadu_gamma_ref_constant_matches_tensor_row() {
        let dev = NdArrayDevice::Cpu;
        let (e, s, r, t) = VIADU_LJ_STATE_EPS1_SIG1_RHO02_T2;
        let lj = Tensor::<B, 2>::from_data(Data::new(vec![e, s, r, t], Shape::new([1, 4])), &dev);
        let (_, g) = upscale_potentials(lj).expect(
            "statistical_mechanics::upscale_potentials VIADU reference LJ row gamma_gc vs GAMMA_GC_REF_VIADU_F32 (FP §6 Track G statmech residual)",
        );
        assert_abs_diff_eq!(
            g.into_scalar(),
            GAMMA_GC_REF_VIADU_F32,
            epsilon = 1.0e-4_f32
        );
    }

    #[cfg(feature = "statistical-mechanics-johnson-reference")]
    #[test]
    fn upscale_placeholder_bulk_modulus_documented_gap_vs_johnson_scalar_path() {
        let t_star = 2.0_f64;
        let rho_star = 0.2_f64;
        let epsilon = 1.0_f64;
        let sigma = 0.8_f64;

        let rel = super::relative_placeholder_bulk_modulus_gap_vs_johnson1993(
            rho_star, t_star, epsilon, sigma,
        );

        let dev = NdArrayDevice::Cpu;
        let lj: Tensor<B, 2> = Tensor::from_data(
            Data::new(vec![epsilon as f32, sigma as f32], Shape::new([1, 2])),
            &dev,
        );
        let (k_tensor, _) = upscale_potentials(lj)
            .expect("upscale_potentials [B,2] placeholder bulk modulus row");
        let k_placeholder = f64::from(k_tensor.into_scalar());
        let k_johnson = super::physical_bulk_modulus_johnson1993(rho_star, t_star, epsilon, sigma);
        let rel_tensor = ((k_placeholder - k_johnson) / k_johnson).abs();

        assert!(
            rel > 0.2,
            "expected placeholder K to disagree strongly with JZG-derived K_T at this state (rel_err={rel})"
        );
        assert_abs_diff_eq!(rel, rel_tensor, epsilon = 5.0e-4_f64);
    }
}
