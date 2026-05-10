// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Statistical-mechanics → continuum bridge (Phase 9) — **MVP scaffold**.
//!
//! This module pins the **tensor contract** for lifting atomistic or coarse-grained potentials
//! (here: Lennard-Jones-style parameters) to **macroscopic thermodynamic intensities** used by
//! DEC / mechanics cartridges: an isotropic **bulk modulus** \(K\) and a **grand-canonical surface
//! energy** \(\gamma_{\mathrm{gc}}\) (interface free energy per area at fixed chemical potential).
//!
//! ## Intended physics (not implemented in MVP)
//! - Map dispersive/repulsive scales \((\varepsilon, \sigma)\) — or an equivalent pair of LJ
//!   knobs — through a specified reference state (density, temperature, cutoff scheme) to \(K\)
//!   via virial / fluctuation formulas or a calibrated EOS bridge.
//! - Obtain \(\gamma_{\mathrm{gc}}\) from interface widening, Kirkwood–Buff-style excess
//!   quantities, or direct coexistence grand-potential differences; all deferred to later phases.
//!
//! ## MVP behavior
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
}
