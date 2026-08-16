// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Thermodynamic control barrier function (CBF) for PPO / [`ManifoldGateway`] admissibility.
//!
//! **Honest status:** scalar + tensor barrier bookkeeping is measured — not physics GREEN,
//! not `PRODUCTION_WIRED`, not `MASTER_RETICK`. Gate-layer aliases live in [`crate::gate::cbf`];
//! volumetric CD proxies bridge via [`crate::gate::cbf_bridge`].
//!
//! **Witness:** `cargo test -p umst-manifold cbf` · cell `W29-005-CBF`.

use burn::tensor::{backend::Backend, Tensor};
use num_traits::ToPrimitive;

pub use crate::core::error_boundary::CbfReject;

/// W29 deepen cell id — honest barrier slice only.
pub const CBF_CELL_ID: &str = "W29-005-CBF";

/// Honest posture tag — tests deepen only; no GREEN invent (`MASTER_RETICK=no`).
pub const CBF_POSTURE_TAG: &str = "honest-cbf-barrier-only";

/// PHY-001 morphism id @ AI manifold CBF band.
pub const CBF_BARRIER_MORPHISM_ID: &str = "thermodynamic_cbf_landauer_cd_barrier";

/// Conservative cushion on the Landauer branch inside [`ThermodynamicCBF::verify_tensor_update`].
pub const LANDAUER_MARGIN_FACTOR: f64 = 1.05;

/// Honest physics posture — barrier computes; continuum lift deferred.
pub const CBF_PHYSICS_GREEN: bool = false;

/// Production wiring at trait / cartridge seam — deferred beyond W29 slice.
pub const CBF_PRODUCTION_WIRED: bool = false;

/// Control Barrier Function (CBF) for Thermodynamic Admissibility.
/// This enforces the Clausius-Duhem inequality and Landauer erasure costs.
///
/// It acts as a Natural Transformation between the Topology State and the physical
/// limits defined by global thermodynamic bounds.
#[derive(Debug, Clone, PartialEq)]
pub struct ThermodynamicCBF {
    /// Local temperature of the physical system (Kelvin)
    pub temperature_k: f64,
    /// Available energy budget for the PPO agent, sourced from the external thermodynamic budget source.
    pub available_credit_joules: f64,
    /// Converts the **batch sum** of integrated dissipation `d_int` (cartridge-native `f32` units from
    /// [`PhysicalResult::dissipation`](crate::core::traits::PhysicalResult::dissipation) reductions)
    /// into additional joule-equivalent entropy production for the Clausius–Duhem check in
    /// [`Self::verify_tensor_update`]. Defaults to `1.0` (identity bridge); set per cartridge when
    /// dissipation is calibrated to SI. See `docs/DISSIPATION_CBF_AUDIT.md`.
    pub k_phys_dint_to_joules: f64,
}

/// Honest slice posture — barrier evaluators landed, physics GREEN refused.
#[must_use]
pub const fn cbf_posture_is_honest() -> bool {
    !CBF_PHYSICS_GREEN && !CBF_PRODUCTION_WIRED
}

/// W29 honest posture bundle — evaluators landed, physics GREEN refused.
#[must_use]
pub const fn cbf_w29_honest_posture_bundle() -> bool {
    cbf_posture_is_honest() && !CBF_PHYSICS_GREEN && !CBF_PRODUCTION_WIRED
}

/// Whether the CBF barrier morphism is pinned @ HEAD (identity clamp + Landauer margin semantics).
#[must_use]
pub fn cbf_barrier_morphism_pinned() -> bool {
    CBF_BARRIER_MORPHISM_ID == "thermodynamic_cbf_landauer_cd_barrier"
        && CBF_POSTURE_TAG == "honest-cbf-barrier-only"
        && CBF_CELL_ID == "W29-005-CBF"
        && (LANDAUER_MARGIN_FACTOR - 1.05).abs() < f64::EPSILON
}

impl ThermodynamicCBF {
    pub fn new(temperature_k: f64, initial_credit_joules: f64) -> Self {
        Self {
            temperature_k,
            available_credit_joules: initial_credit_joules,
            k_phys_dint_to_joules: 1.0,
        }
    }

    /// Calculates the Landauer erasure cost required to resolve `bits_of_uncertainty`.
    /// Cost = k_B * T * ln(2) * N_bits
    pub fn calculate_landauer_cost(&self, bits_of_uncertainty: f64) -> f64 {
        crate::constants::landauer_bit_energy_joules(self.temperature_k) * bits_of_uncertainty
    }

    /// Evaluates if a proposed topology update from the RL agent is thermodynamically permissible.
    ///
    /// # Arguments
    /// * `entropy_production_joules` - The macroscopic entropy produced by the material state transition.
    /// * `bits_resolved` - The mutual information gained by the PPO agent choosing this topology mutation.
    ///
    /// # Returns
    /// * `Ok(cost_in_joules)` if the agent has enough energy credit and the transition satisfies Clausius-Duhem.
    /// * `Err` if the agent attempts an unphysical or unfunded topological change.
    pub fn verify_and_deduct_update(
        &mut self,
        entropy_production_joules: f64,
        bits_resolved: f64,
    ) -> Result<f64, CbfReject> {
        // 1. Calculate the minimum thermodynamic cost of this computation
        let erasure_cost = self.calculate_landauer_cost(bits_resolved);

        // 2. Check Global Thermodynamic Limits (Economic/Computational Bound)

        if erasure_cost > self.available_credit_joules {
            return Err(CbfReject::InsufficientGlobalEnergyCredit {
                required_j: erasure_cost,
                available_j: self.available_credit_joules,
            });
        }

        // 3. Clausius-Duhem Inequality (Physical Bound)
        // The physical entropy produced must be strictly non-negative when accounting for Landauer erasure.
        let generalized_entropy = entropy_production_joules - erasure_cost;
        if generalized_entropy < 0.0 {
            return Err(CbfReject::ClausiusDuhemViolation {
                generalized_entropy,
            });
        }

        // 4. Deduct the exact erasure cost from the agent's energy pool
        self.available_credit_joules -= erasure_cost;

        Ok(erasure_cost)
    }

    /// Processes a full batch gradient step to ensure gradient descent directions
    /// do not push the topology into a physically invalid state.
    ///
    /// **Scalar / host barrier:** sums `info_gain` and **`d_int`** then performs deliberate
    /// `.into_scalar()` syncs for the [`crate::ai::ppo::ManifoldGateway`] stack — total resolved bits
    /// and batch-summed dissipation become `f64` for Landauer cost, material entropy bookkeeping, and
    /// credit deduction. Prefer keeping other solver paths free of per-iteration `.into_scalar()` unless
    /// a kernel truly needs a host scalar for control flow.
    pub fn verify_tensor_update<B: Backend>(
        &mut self,
        d_int: Tensor<B, 1>,
        info_gain: Tensor<B, 1>,
    ) -> Result<f64, CbfReject> {
        // Compute precise bits resolved via tensor sum reduction
        // info_gain contains the mutual information per batch element
        let sum_bits_tensor = info_gain.sum();
        let total_bits_resolved = sum_bits_tensor.into_scalar().to_f64().unwrap_or(0.0);

        // Batch-summed integrated dissipation (gateway convention: one CBF scalar per topology step).
        let d_sum = d_int.sum().into_scalar().to_f64().unwrap_or(0.0);
        let d_sum_nonneg = d_sum.max(0.0);
        let dissipation_entropy_joules = self.k_phys_dint_to_joules * d_sum_nonneg;

        // Landauer floor (informational branch) plus irreversible material entropy from `d_int`.
        // The margin remains a small conservative cushion on the Landauer term alone.
        let landauer_floor =
            self.calculate_landauer_cost(total_bits_resolved) * LANDAUER_MARGIN_FACTOR;
        let entropy_production = landauer_floor + dissipation_entropy_joules;

        self.verify_and_deduct_update(entropy_production, total_bits_resolved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use burn::tensor::Tensor;
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    const TEMP_K: f64 = 300.0_f64;
    const CREDIT_J: f64 = 1.0e-9_f64;

    #[test]
    fn cbf_posture_is_honest_witness() {
        assert!(cbf_posture_is_honest());
        assert!(!CBF_PHYSICS_GREEN);
        assert!(!CBF_PRODUCTION_WIRED);
    }

    #[test]
    fn cbf_w29_honest_posture_bundle_holds() {
        assert!(cbf_w29_honest_posture_bundle());
        assert_eq!(CBF_CELL_ID, "W29-005-CBF");
    }

    #[test]
    fn cbf_barrier_morphism_pinned_at_head() {
        assert!(cbf_barrier_morphism_pinned());
        assert_eq!(
            CBF_BARRIER_MORPHISM_ID,
            "thermodynamic_cbf_landauer_cd_barrier"
        );
    }

    #[test]
    fn cbf_posture_tag_honest_not_green() {
        assert!(CBF_POSTURE_TAG.contains("honest"));
        assert!(!CBF_POSTURE_TAG.to_ascii_lowercase().contains("green"));
        assert!(!CBF_POSTURE_TAG.to_ascii_lowercase().contains("production"));
    }

    #[test]
    fn cbf_landauer_margin_factor_matches_tensor_shim() {
        assert_relative_eq!(LANDAUER_MARGIN_FACTOR, 1.05, epsilon = 1.0e-30);
    }

    #[test]
    fn cbf_new_defaults_k_phys_unity_bridge() {
        let cbf = ThermodynamicCBF::new(TEMP_K, CREDIT_J);
        assert_relative_eq!(cbf.k_phys_dint_to_joules, 1.0, epsilon = 1.0e-30);
        assert_relative_eq!(cbf.temperature_k, TEMP_K, epsilon = 1.0e-30);
        assert_relative_eq!(cbf.available_credit_joules, CREDIT_J, epsilon = 1.0e-30);
    }

    #[test]
    fn cbf_verify_and_deduct_debits_credit_on_admissible_step() {
        let mut cbf = ThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let bits = 2.0_f64;
        let erasure = cbf.calculate_landauer_cost(bits);
        let cost = cbf
            .verify_and_deduct_update(erasure, bits)
            .expect("admissible scalar step must debit credit");
        assert_relative_eq!(cost, erasure, epsilon = 1.0e-30, max_relative = 1.0e-9);
        assert_relative_eq!(
            cbf.available_credit_joules,
            CREDIT_J - erasure,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
    }

    #[test]
    fn cbf_rejects_insufficient_global_energy_credit() {
        let mut cbf = ThermodynamicCBF::new(TEMP_K, 0.0);
        let bits = 1.0_f64;
        let err = cbf
            .verify_and_deduct_update(0.0, bits)
            .expect_err("zero credit must reject positive bit resolution");
        assert!(matches!(
            err,
            CbfReject::InsufficientGlobalEnergyCredit { .. }
        ));
    }

    #[test]
    fn cbf_rejects_clausius_duhem_violation() {
        let mut cbf = ThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let bits = 1.0_f64;
        let erasure = cbf.calculate_landauer_cost(bits);
        let err = cbf
            .verify_and_deduct_update(erasure * 0.5, bits)
            .expect_err("entropy below Landauer floor must violate CD");
        assert!(matches!(err, CbfReject::ClausiusDuhemViolation { .. }));
    }

    #[test]
    fn cbf_scalar_cd_boundary_is_admissible() {
        let mut cbf = ThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let bits = 1.0_f64;
        let erasure = cbf.calculate_landauer_cost(bits);
        cbf.verify_and_deduct_update(erasure, bits)
            .expect("entropy == erasure must sit on CD boundary");
    }

    #[test]
    fn cbf_verify_tensor_update_zero_bits_admits() {
        let dev = NdArrayDevice::default();
        let mut cbf = ThermodynamicCBF::new(TEMP_K, 1.0e-12_f64);
        cbf.k_phys_dint_to_joules = 1.0;
        let d_int = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
        let info_gain = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
        cbf.verify_tensor_update(d_int, info_gain)
            .expect("zero info gain and zero dissipation must admit");
    }

    #[test]
    fn cbf_verify_tensor_update_clamps_negative_d_int() {
        let dev = NdArrayDevice::default();
        let mut cbf = ThermodynamicCBF::new(TEMP_K, 1.0e-12_f64);
        cbf.k_phys_dint_to_joules = 1.0;
        let d_int = Tensor::<B, 1>::from_floats([-1.0e6_f32], &dev);
        let info_gain = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
        cbf.verify_tensor_update(d_int, info_gain)
            .expect("negative d_int must clamp before CD check");
    }

    #[test]
    fn cbf_tensor_batch_sums_info_gain_elements() {
        let dev = NdArrayDevice::default();
        let mut cbf = ThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let d_int = Tensor::<B, 1>::from_floats([0.0_f32, 0.0_f32], &dev);
        let info_gain = Tensor::<B, 1>::from_floats([1.0_f32, 3.0_f32], &dev);
        let total_bits = 4.0_f64;
        let expected_debit = cbf.calculate_landauer_cost(total_bits);
        let cost = cbf
            .verify_tensor_update(d_int, info_gain)
            .expect("batch sum of info_gain must admit");
        assert_relative_eq!(
            cost,
            expected_debit,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
    }

    #[test]
    fn cbf_tensor_rejects_insufficient_global_credit() {
        let dev = NdArrayDevice::default();
        let mut cbf = ThermodynamicCBF::new(TEMP_K, 0.0);
        let d_int = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
        let info_gain = Tensor::<B, 1>::from_floats([2.0_f32], &dev);
        let err = cbf
            .verify_tensor_update(d_int, info_gain)
            .expect_err("positive batch info_gain with zero credit must reject");
        assert!(matches!(
            err,
            CbfReject::InsufficientGlobalEnergyCredit { .. }
        ));
    }

    #[test]
    fn cbf_tensor_negative_info_gain_rejects_clausius_duhem() {
        let dev = NdArrayDevice::default();
        let mut cbf = ThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let d_int = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
        let info_gain = Tensor::<B, 1>::from_floats([-1.0_f32], &dev);
        let err = cbf
            .verify_tensor_update(d_int, info_gain)
            .expect_err("negative batch info_gain must violate CD through tensor shim");
        assert!(matches!(err, CbfReject::ClausiusDuhemViolation { .. }));
    }

    #[test]
    fn cbf_reject_insufficient_credit_carries_required_and_available() {
        let mut cbf = ThermodynamicCBF::new(TEMP_K, 1.0e-20_f64);
        let bits = 8.0_f64;
        let required = cbf.calculate_landauer_cost(bits);
        let err = cbf
            .verify_and_deduct_update(0.0, bits)
            .expect_err("credit below Landauer floor must report joules");
        match err {
            CbfReject::InsufficientGlobalEnergyCredit {
                required_j,
                available_j,
            } => {
                assert_relative_eq!(
                    required_j,
                    required,
                    epsilon = 1.0e-30,
                    max_relative = 1.0e-9
                );
                assert_relative_eq!(
                    available_j,
                    1.0e-20_f64,
                    epsilon = 1.0e-30,
                    max_relative = 1.0e-9
                );
            }
            other => panic!("expected InsufficientGlobalEnergyCredit, got {other:?}"),
        }
    }
}
