// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Statistical-mechanics → continuum bridge (Phase 9) — **research-phase placeholder**.
//!
//! This module pins the **tensor contract** for lifting atomistic or coarse-grained potentials
//! (here: Lennard-Jones-style parameters) to **macroscopic thermodynamic intensities** used by
//! DEC / mechanics cartridges: an isotropic **bulk modulus** \(K\) and a **grand-canonical surface
//! energy** \(\gamma_{\mathrm{gc}}\) (interface free energy per area at fixed chemical potential).
//!
//! ## Intended physics (deferred implementations)
//! - Map dispersive/repulsive scales \((\varepsilon, \sigma)\) — or an equivalent pair of LJ
//!   knobs — through a specified reference state (density, temperature, cutoff scheme) to \(K\)
//!   via virial / fluctuation formulas or a calibrated EOS bridge.
//! - Obtain \(\gamma_{\mathrm{gc}}\) from interface widening, Kirkwood–Buff-style excess
//!   quantities, or direct coexistence grand-potential differences; all deferred to later phases.
//!
//! ## Stable lane vs this module
//!
//! Feature `statistical-mechanics-vinet` (in `solver-stable`) runs **Vinet scalar EOS** regression
//! tests in `tests/verification/statmech_vinet_eos.rs` only. That harness does **not** call this
//! module; [`upscale_potentials`] uses a **dimensionally motivated placeholder** — scaling is
//! locked by `tests/verification/statmech_lj_bridge_contract.rs`, while **Johnson (1993) LJ EOS**
//! `f64` reference checks (`statmech_lj_johnson_eos_reference`) document that this placeholder is
//! **not** a calibrated virial / EOS bridge.
//!
//! **Reference (f64, not Burn):** Johnson–Zollweg–Gubbins (1993) reduced LJ pressure and bulk
//! modulus checks live in [`super::lj_johnson_1993_reference`] and
//! `tests/verification/statmech_lj_johnson_eos_reference.rs`.
//!
//! With feature **`statistical-mechanics-johnson-reference`**, [`bulk_modulus_from_lj_state_johnson1993`]
//! is also exposed here as an optional counterpart to [`upscale_potentials`] (still Burn `f32`
//! placeholder). Without the feature, use [`super::lj_johnson_1993_reference::bulk_modulus_from_lj_state_johnson1993`].
//!
//! ## Why Johnson is not compiled into [`upscale_potentials`]
//!
//! The Burn bridge contract is **`[B, 2]` → `(K, γ_gc)`** with columns \((\varepsilon, \sigma)\) only.
//! The JZG (1993) isothermal bulk modulus needs **reduced state** \((\rho^*, T^*)\) in addition to
//! \((\varepsilon, \sigma)\) to form \(K^*\), then \(K_T = (\varepsilon/\sigma^3)\,K^*\). There is no
//! \((\rho^*, T^*)\) channel in the tensor API and no agreed default reference state, so wiring the
//! reference EOS **inside** [`upscale_potentials`] would hide physics or break type clarity. With the
//! opt-in feature, compare scalars side-by-side (see unit tests below and
//! `tests/verification/statmech_lj_johnson_eos_reference.rs`). **`upscale_potentials` stays partial:**
//! analytic placeholder for \(K\) and \(\gamma_{\mathrm{gc}}\) until a stateful bridge lands.
//!
//! ## Placeholder behaviour
//!
//! [`upscale_potentials`] applies a **simple analytic placeholder** (dimensionless prefactors ×
//! powers of \(\varepsilon\) and \(\sigma\)) so outputs are **finite, non-trivial**, and remain
//! fully differentiable in Burn. This is not a calibrated EOS; it only preserves the intended
//! scaling dimensions until virial / coexistence bridges land.

use burn::tensor::{backend::Backend, Tensor};

/// Dimensionless scale for the analytic bulk-modulus placeholder
/// \(K = C_K \, \varepsilon / \sigma^3\) (same dimensions as ε/σ³ up to the cartridge’s unit system).
pub const ANALYTIC_BULK_MODULUS_SCALE: f32 = 1.0;

/// Dimensionless scale for the analytic grand-canonical surface-energy placeholder
/// \(\gamma_{\mathrm{gc}} = C_\gamma \, \varepsilon / \sigma^2\).
pub const ANALYTIC_SURFACE_ENERGY_SCALE: f32 = 1.0;

/// Carrier for Phase 9 statistical-mechanics bridging logic (currently a zero-sized type).
///
/// A unit struct keeps **stateless** mappings explicit: any calibration data (reference density,
/// virial schemes, cutoffs) should eventually arrive via config structs or cartridge traits rather
/// than hidden globals.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StatisticalBridge;

impl StatisticalBridge {
    /// Maps Lennard-Jones parameter rows to bulk modulus and grand-canonical surface energy.
    ///
    /// # Argument shapes
    /// - `lennard_jones_params`: `[B, 2]` — batch **`B`**, two channels per row. By convention
    ///   column `0` is \(\varepsilon\) (energy well depth) and column `1` is \(\sigma\)
    ///   (zero-crossing separation) in reduced or physical units consistent with the cartridge.
    ///
    /// # Returns
    /// - `bulk_modulus`: `[B, 1]` — isotropic \(K\) placeholder.
    /// - `surface_energy_gc`: `[B, 1]` — \(\gamma_{\mathrm{gc}}\) placeholder.
    ///
    /// # Placeholder mapping
    /// - \(K = \texttt{ANALYTIC\_BULK\_MODULUS\_SCALE} \cdot \varepsilon / \sigma^3\) → `[B, 1]`
    /// - \(\gamma_{\mathrm{gc}} = \texttt{ANALYTIC\_SURFACE\_ENERGY\_SCALE} \cdot \varepsilon / \sigma^2\) → `[B, 1]`
    ///
    /// **`lennard_jones_params` is not validated** beyond reading `dims()[0]`; callers must supply
    /// `[B, 2]` until explicit asserts are added. Division by \(\sigma\) follows tensor math (no extra
    /// clamp); avoid \(\sigma \to 0\) in training if gradients should stay well-behaved.
    pub fn upscale_potentials<B: Backend<FloatElem = f32>>(
        &self,
        lennard_jones_params: Tensor<B, 2>,
    ) -> (Tensor<B, 2>, Tensor<B, 2>) {
        let batch = lennard_jones_params.dims()[0];
        let eps = lennard_jones_params.clone().slice([0..batch, 0..1]);
        let sig = lennard_jones_params.slice([0..batch, 1..2]);
        let sig_sq = sig.clone().mul(sig.clone());
        let sig_cu = sig_sq.clone().mul(sig);
        let bulk_modulus = eps
            .clone()
            .div(sig_cu)
            .mul_scalar(ANALYTIC_BULK_MODULUS_SCALE);
        let surface_energy_gc = eps.div(sig_sq).mul_scalar(ANALYTIC_SURFACE_ENERGY_SCALE);
        (bulk_modulus, surface_energy_gc)
    }
}

/// Stateless entry point: same contract as [`StatisticalBridge::upscale_potentials`].
///
/// Prefer this free function when no `StatisticalBridge` instance is already in scope; it
/// delegates to [`StatisticalBridge::upscale_potentials`] on a [`StatisticalBridge`] value.
#[inline]
pub fn upscale_potentials<B: Backend<FloatElem = f32>>(
    lennard_jones_params: Tensor<B, 2>,
) -> (Tensor<B, 2>, Tensor<B, 2>) {
    StatisticalBridge.upscale_potentials(lennard_jones_params)
}

/// Johnson (1993) reduced bulk modulus `K*(ρ*, T*)` — **opt-in** re-export from the `f64` reference lane.
///
/// Does **not** replace [`upscale_potentials`]; compare numerically at a chosen `(ρ*, T*)` and map
/// to physical `K` with [`super::lj_johnson_1993_reference::bulk_modulus_from_reduced`].
#[cfg(feature = "statistical-mechanics-johnson-reference")]
#[inline]
#[must_use]
pub fn bulk_modulus_from_lj_state_johnson1993(rho_star: f64, t_star: f64) -> f64 {
    super::lj_johnson_1993_reference::bulk_modulus_from_lj_state_johnson1993(rho_star, t_star)
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let (k, gamma) = upscale_potentials(lj.clone());
        assert_eq!(k.dims(), [2, 1]);
        assert_eq!(gamma.dims(), [2, 1]);

        let (k2, g2) = StatisticalBridge.upscale_potentials(lj);
        assert_eq!(k2.dims(), [2, 1]);
        assert_eq!(g2.dims(), [2, 1]);
    }

    #[test]
    fn upscale_potentials_analytic_scales_match_epsilon_sigma_powers() {
        let dev = NdArrayDevice::Cpu;
        // Rows: (ε, σ) = (0.1, 0.2), (0.3, 0.4)
        let lj: Tensor<B, 2> = Tensor::from_data(
            Data::new(vec![0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32], Shape::new([2, 2])),
            &dev,
        );
        let (k, gamma) = upscale_potentials(lj);
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

    /// Johnson EOS physical \(K_T\) at a fixed \((\rho^*, T^*)\) vs Burn placeholder \(K \propto \varepsilon/\sigma^3\).
    ///
    /// Documents that [`upscale_potentials`] cannot match the reference without \((\rho^*, T^*)\) inputs.
    #[cfg(feature = "statistical-mechanics-johnson-reference")]
    #[test]
    fn upscale_placeholder_bulk_modulus_documented_gap_vs_johnson_scalar_path() {
        use super::super::lj_johnson_1993_reference::bulk_modulus_from_reduced;

        let t_star = 2.0_f64;
        let rho_star = 0.2_f64;
        let epsilon = 1.0_f64;
        let sigma = 0.8_f64;

        let k_star = bulk_modulus_from_lj_state_johnson1993(rho_star, t_star);
        let k_johnson = bulk_modulus_from_reduced(k_star, epsilon, sigma);

        let dev = NdArrayDevice::Cpu;
        let lj: Tensor<B, 2> = Tensor::from_data(
            Data::new(vec![epsilon as f32, sigma as f32], Shape::new([1, 2])),
            &dev,
        );
        let (k_tensor, _) = upscale_potentials(lj);
        let k_placeholder = f64::from(k_tensor.into_data().value[0]);

        let rel = ((k_placeholder - k_johnson) / k_johnson).abs();
        assert!(
            rel > 0.2,
            "expected placeholder K to disagree strongly with JZG-derived K_T at this state (rel_err={rel})"
        );
    }
}
