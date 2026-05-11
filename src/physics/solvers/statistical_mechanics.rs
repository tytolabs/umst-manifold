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
//! ## Johnson (1993) vs [`upscale_potentials`] tensor rows
//!
//! **Placeholder `K`:** **`[B, 2]`** rows carry only \((\varepsilon, \sigma)\); the JZG isothermal bulk
//! modulus needs **reduced** \((\rho^*, T^*)\) in addition, so that branch keeps the analytic
//! \(K \propto \varepsilon/\sigma^3\) map (scaling contract — `statmech_lj_bridge_contract`).
//!
//! **Johnson `K` in Burn:** **`[B, 4]`** rows \((\varepsilon,\sigma,\rho^*,T^*)\) set **`K`** from
//! [`physical_bulk_modulus_johnson1993`] per batch row (host **`f64`**, stored as **`f32`**). This is a
//! **matrix acceptance** step toward verification **#9** (EOS-grounded \(K\) at explicit state), not a
//! full virial/MD calibration. **`γ_gc`** remains the \(\varepsilon/\sigma^2\) placeholder from the
//! first two columns. **Autodiff:** the **`[B,4]`** **`K`** branch materialises constants — use **`[B,2]`**
//! when differentiating through **`K`**, or wait for a native Burn EOS map.
//!
//! Optional feature **`statistical-mechanics-johnson-reference`** still adds the scalar
//! [`bulk_modulus_from_lj_state_johnson1993`] re-export for reduced \(K^*\) parity tests; it does not change
//! the **`[B,2]`** / **`[B,4]`** contracts above.
//!
//! ## Path sketch: virial / Johnson EOS → [`upscale_potentials`] (target, not current Burn wiring)
//!
//! 1. **Virial / simulation:** pair forces → pressure \(P(\rho,T)\) (or \(P^*\) in reduced LJ units) →
//!    isothermal bulk modulus \(K_T=\rho(\partial P/\partial\rho)_T\) at the **same** cutoff / tail
//!    protocol as the label data.
//! 2. **Johnson (1993) reference lane (`f64`):** analytic reduced EOS gives \(P^*(\rho^*,T^*)\) and
//!    \(K^*=\rho^*(\partial P^*/\partial\rho^*)_{T^*}\); physical modulus
//!    \(K_T=(\varepsilon/\sigma^3)K^*\) via [`super::lj_johnson_1993_reference::bulk_modulus_from_reduced`].
//! 3. **Today's [`upscale_potentials`]:** **`[B,2]`** keeps the analytic placeholder; **`[B,4]`** rows
//!    \((\varepsilon,\sigma,\rho^*,T^*)\) set **`K`** from the Johnson (1993) **`f64`** surface (host
//!    row loop, **`f32`** storage) via [`physical_bulk_modulus_johnson1993`]. **`γ_gc`** remains the
//!    same \(\varepsilon/\sigma^2\) placeholder from the first two columns. The **`[B,4]`** branch is
//!    **not** AD-safe (constants materialised with [`Tensor::from_data`]); differentiable EOS in Burn
//!    is still future work.
//!
//! ### Tensor channel design \((\rho^*,T^*)\)
//!
//! Forward-looking options (Solver-Status **DEFERRAL — Statistical mechanics**):
//! - **Wide row:** `lennard_jones_params` shaped **`[B, 4]`** with columns
//!   \((\varepsilon,\sigma,\rho^*,T^*)\) (or SI \(\rho,T\) with documented normalization), then slice
//!   \(\varepsilon,\sigma\) for scaling and \(\rho^*,T^*\) for the EOS / virial surrogate inside Burn.
//! - **Twin tensors:** keep **`[B,2]`** for \((\varepsilon,\sigma)\) and add **`[B,2]`** for
//!   \((\rho^*,T^*)\) (or \((\rho,T)\)) to a new method on [`StatisticalBridge`] so types stay explicit.
//!
//! **Scalar bridge today (always-on `f64`):** [`physical_bulk_modulus_johnson1993`] and
//! [`relative_placeholder_bulk_modulus_gap_vs_johnson1993`] compose Johnson \(K^*\) with
//! \((\varepsilon,\sigma)\) and compare the analytic placeholder \(C_K\varepsilon/\sigma^3\) to that
//! physical \(K_T\). They do **not** call [`upscale_potentials`]; Burn `f32` rows can differ at
//! ~\(10^{-7}\) relative scale from the closed-form placeholder.
//!
//! ### Feature gate sketch
//!
//! - **Default:** [`upscale_potentials`] compiles both **`[B,2]`** (placeholder **`K`**) and **`[B,4]`**
//!   (Johnson **`K`** via host loop). No extra **`#[cfg]`** is required for the wide row.
//! - **`statistical-mechanics-johnson-reference`:** adds [`bulk_modulus_from_lj_state_johnson1993`] (`f64`,
//!   reduced \(K^*\)) re-export for scalar-only parity; optional for **`[B,4]`** tensor **`K`**, which uses
//!   [`physical_bulk_modulus_johnson1993`] (always on).
//!
//! ## Placeholder behaviour
//!
//! **`[B,2]`** [`upscale_potentials`] applies the analytic placeholder for **`K`** and **`γ_gc`**. **`[B,4]`**
//! uses Johnson for **`K`** only; **`γ_gc`** still uses the same \(\varepsilon/\sigma^2\) placeholder. This is
//! not a full Kirkwood–Buff / coexistence calibration; it preserves explicit state for **`K`** until
//! virial / coexistence bridges land.

use burn::tensor::{backend::Backend, Data, Shape, Tensor};

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
    /// Maps Lennard-Jones parameter rows to bulk modulus \(K\) and grand-canonical surface energy
    /// \(\gamma_{\mathrm{gc}}\).
    ///
    /// # Argument shapes
    /// - **`[B, 2]`:** columns \((\varepsilon,\sigma)\). **`K`** uses the analytic placeholder
    ///   \(K = \texttt{ANALYTIC\_BULK\_MODULUS\_SCALE}\,\varepsilon/\sigma^3\); **`γ_gc`** uses
    ///   \(\texttt{ANALYTIC\_SURFACE\_ENERGY\_SCALE}\,\varepsilon/\sigma^2\). Fully differentiable in Burn.
    /// - **`[B, 4]`:** columns \((\varepsilon,\sigma,\rho^*,T^*)\). **`K`** uses Johnson (1993)
    ///   [`physical_bulk_modulus_johnson1993`] per row (host **`f64`**, stored **`f32`**); **`γ_gc`**
    ///   still uses the same \(\varepsilon/\sigma^2\) placeholder from the first two columns. The **`[B,4]`**
    ///   **`K`** branch is **not** AD-safe (`Tensor::from_data`); use **`[B,2]`** when taping **`K`**.
    ///
    /// # Returns
    /// - `bulk_modulus`: `[B, 1]`
    /// - `surface_energy_gc`: `[B, 1]`
    ///
    /// Division by \(\sigma\) follows tensor math (no extra clamp); avoid \(\sigma \to 0\) in training
    /// if gradients should stay well-behaved on the **`[B,2]`** branch.
    pub fn upscale_potentials<B: Backend<FloatElem = f32>>(
        &self,
        lennard_jones_params: Tensor<B, 2>,
    ) -> (Tensor<B, 2>, Tensor<B, 2>) {
        let dims = lennard_jones_params.dims();
        let batch = dims[0];
        let cols = dims[1];
        let device = lennard_jones_params.device();

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
            (bulk_modulus, surface_energy_gc)
        } else if cols == 4 {
            let row_vals: Vec<f32> = lennard_jones_params.clone().into_data().value;
            assert_eq!(
                row_vals.len(),
                batch * 4,
                "upscale_potentials [B,4]: flat length mismatch"
            );
            let mut k_host = Vec::with_capacity(batch);
            for i in 0..batch {
                let o = i * 4;
                let epsilon = f64::from(row_vals[o]);
                let sigma = f64::from(row_vals[o + 1]);
                let rho_star = f64::from(row_vals[o + 2]);
                let t_star = f64::from(row_vals[o + 3]);
                let k_t = physical_bulk_modulus_johnson1993(rho_star, t_star, epsilon, sigma);
                k_host.push(k_t as f32);
            }
            let bulk_modulus =
                Tensor::from_data(Data::new(k_host, Shape::new([batch, 1])), &device);
            let eps = lennard_jones_params.clone().slice([0..batch, 0..1]);
            let sig = lennard_jones_params.clone().slice([0..batch, 1..2]);
            let sig_sq = sig.clone().mul(sig.clone());
            let surface_energy_gc = eps.div(sig_sq).mul_scalar(ANALYTIC_SURFACE_ENERGY_SCALE);
            (bulk_modulus, surface_energy_gc)
        } else {
            panic!(
                "StatisticalBridge::upscale_potentials: expected [B,2] or [B,4], got dims {dims:?}"
            );
        }
    }
}

/// Stateless entry point: same contract as [`StatisticalBridge::upscale_potentials`].
///
/// Prefer this free function when no `StatisticalBridge` instance is already in scope; it
/// delegates to [`StatisticalBridge::upscale_potentials`] on a [`StatisticalBridge`] value.
///
/// **Contract:** argument is **`[B, 2]`** (placeholder **`K`**) or **`[B, 4]`**
/// \((\varepsilon,\sigma,\rho^*,T^*)\) (Johnson **`K`**, same **`γ_gc`** placeholder) — see
/// [`StatisticalBridge::upscale_potentials`].
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

/// Johnson reduced \(K^*(\rho^*,T^*)\) composed with \((\varepsilon,\sigma)\) → physical isothermal \(K_T\).
///
/// Delegates to [`super::lj_johnson_1993_reference`]; same scalar a future extended **`[B,4]`** (or
/// twin-tensor) Burn row would evaluate once \((\rho^*,T^*)\) sits beside \((\varepsilon,\sigma)\).
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

/// Relative error \(\lvert K_{\mathrm{ph}} - K_{\mathrm{J}}\rvert / \lvert K_{\mathrm{J}}\rvert\) between
/// the **analytic** placeholder \(K_{\mathrm{ph}} = C_K\,\varepsilon/\sigma^3\) ([`ANALYTIC_BULK_MODULUS_SCALE`])
/// and Johnson \(K_{\mathrm{J}}\) = [`physical_bulk_modulus_johnson1993`].
///
/// [`upscale_potentials`] uses the same formula in `f32`; expect \(\mathcal O(10^{-7})\) relative drift
/// vs this `f64` value at typical parameters.
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

    /// **`[B,4]`** `upscale_potentials`: each row's **`K`** matches Johnson \(K_T\) (`physical_bulk_modulus_johnson1993`).
    #[test]
    fn upscale_potentials_b4_batch_matches_physical_johnson_per_row() {
        let dev = NdArrayDevice::Cpu;
        let rows: Vec<f32> = vec![
            1.0, 0.9, 0.15, 2.5, //
            0.5, 1.1, 0.22, 2.0,
        ];
        let lj: Tensor<B, 2> = Tensor::from_data(Data::new(rows.clone(), Shape::new([2, 4])), &dev);
        let (k, gamma) = upscale_potentials(lj.clone());
        assert_eq!(k.dims(), [2, 1]);
        assert_eq!(gamma.dims(), [2, 1]);
        let kv = k.into_data().value;
        for (row, &kval) in rows.chunks_exact(4).zip(kv.iter()) {
            let want = super::physical_bulk_modulus_johnson1993(
                f64::from(row[2]),
                f64::from(row[3]),
                f64::from(row[0]),
                f64::from(row[1]),
            );
            assert_abs_diff_eq!(f64::from(kval), want, epsilon = 1.0e-5_f64);
        }
        let gv = gamma.into_data().value;
        let c_g = ANALYTIC_SURFACE_ENERGY_SCALE;
        for (row, &gval) in rows.chunks_exact(4).zip(gv.iter()) {
            let e = f64::from(row[0]);
            let s = f64::from(row[1]);
            assert_abs_diff_eq!(
                f64::from(gval),
                f64::from(c_g) * e / s.powi(2),
                epsilon = 1.0e-5
            );
        }
    }

    /// Documents that **`[B,2]`** [`upscale_potentials`] placeholder \(K\) disagrees with Johnson \(K_T\)
    /// at a reference \((\rho^*,T^*)\); relative gap matches the tensor-derived placeholder row.
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
        let (k_tensor, _) = upscale_potentials(lj);
        let k_placeholder = f64::from(k_tensor.into_data().value[0]);
        let k_johnson = super::physical_bulk_modulus_johnson1993(rho_star, t_star, epsilon, sigma);
        let rel_tensor = ((k_placeholder - k_johnson) / k_johnson).abs();

        assert!(
            rel > 0.2,
            "expected placeholder K to disagree strongly with JZG-derived K_T at this state (rel_err={rel})"
        );
        assert_abs_diff_eq!(rel, rel_tensor, epsilon = 5.0e-4_f64);
    }
}
