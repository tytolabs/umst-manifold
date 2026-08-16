// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Thin gate-layer alias around [`crate::ai::cbf::ThermodynamicCBF`] (no duplicated physics).

pub use crate::ai::cbf::ThermodynamicCBF as ThermodynamicCBFInner;

/// Newtype forwarding to [`ThermodynamicCBFInner`] via [`std::ops::Deref`].
pub struct GateThermodynamicCBF(pub ThermodynamicCBFInner);

impl GateThermodynamicCBF {
    pub fn new(temperature_k: f64, initial_credit_joules: f64) -> Self {
        Self(ThermodynamicCBFInner::new(
            temperature_k,
            initial_credit_joules,
        ))
    }

    pub fn into_inner(self) -> ThermodynamicCBFInner {
        self.0
    }
}

impl std::ops::Deref for GateThermodynamicCBF {
    type Target = ThermodynamicCBFInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for GateThermodynamicCBF {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::cbf::CbfReject;
    use approx::assert_relative_eq;
    use burn::tensor::Tensor;
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    const TEMP_K: f64 = 300.0_f64;
    const CREDIT_J: f64 = 1.0e-9_f64;

    #[test]
    fn gate_cbf_new_matches_inner_field_parity() {
        let gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let inner = ThermodynamicCBFInner::new(TEMP_K, CREDIT_J);
        assert_eq!(gate.temperature_k, inner.temperature_k);
        assert_eq!(gate.available_credit_joules, inner.available_credit_joules);
        assert_eq!(gate.k_phys_dint_to_joules, inner.k_phys_dint_to_joules);
    }

    #[test]
    fn gate_cbf_deref_forwards_landauer_cost() {
        let gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let inner = ThermodynamicCBFInner::new(TEMP_K, CREDIT_J);
        for &bits in &[0.0_f64, 0.25, 1.0, 8.0] {
            assert_relative_eq!(
                gate.calculate_landauer_cost(bits),
                inner.calculate_landauer_cost(bits),
                epsilon = 1.0e-30,
                max_relative = 1.0e-12
            );
        }
    }

    #[test]
    fn gate_cbf_into_inner_preserves_mutated_credit() {
        let mut gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        gate.available_credit_joules -= 1.0e-12;
        gate.k_phys_dint_to_joules = 2.5;
        let inner = gate.into_inner();
        assert_relative_eq!(
            inner.available_credit_joules,
            CREDIT_J - 1.0e-12,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
        assert_relative_eq!(inner.k_phys_dint_to_joules, 2.5, epsilon = 1.0e-30);
    }

    #[test]
    fn gate_cbf_verify_and_deduct_debits_credit_on_admissible_step() {
        let mut gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let bits = 2.0_f64;
        let erasure = gate.calculate_landauer_cost(bits);
        let cost = gate
            .verify_and_deduct_update(erasure, bits)
            .expect("admissible scalar step must debit credit");
        assert_relative_eq!(cost, erasure, epsilon = 1.0e-30, max_relative = 1.0e-9);
        assert_relative_eq!(
            gate.available_credit_joules,
            CREDIT_J - erasure,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
    }

    #[test]
    fn gate_cbf_rejects_insufficient_global_energy_credit() {
        let mut gate = GateThermodynamicCBF::new(TEMP_K, 0.0);
        let bits = 1.0_f64;
        let err = gate
            .verify_and_deduct_update(0.0, bits)
            .expect_err("zero credit must reject positive bit resolution");
        assert!(matches!(
            err,
            CbfReject::InsufficientGlobalEnergyCredit { .. }
        ));
    }

    #[test]
    fn gate_cbf_rejects_clausius_duhem_violation() {
        let mut gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let bits = 1.0_f64;
        let erasure = gate.calculate_landauer_cost(bits);
        let err = gate
            .verify_and_deduct_update(erasure * 0.5, bits)
            .expect_err("entropy below Landauer floor must violate CD");
        assert!(matches!(err, CbfReject::ClausiusDuhemViolation { .. }));
    }

    #[test]
    fn gate_cbf_verify_tensor_update_zero_bits_admits() {
        let dev = NdArrayDevice::default();
        let mut gate = GateThermodynamicCBF::new(TEMP_K, 1.0e-12_f64);
        gate.k_phys_dint_to_joules = 1.0;
        let d_int = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
        let info_gain = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
        gate.verify_tensor_update(d_int, info_gain)
            .expect("zero info gain and zero dissipation must admit");
    }

    #[test]
    fn gate_cbf_verify_tensor_update_clamps_negative_d_int() {
        let dev = NdArrayDevice::default();
        let mut gate = GateThermodynamicCBF::new(TEMP_K, 1.0e-12_f64);
        gate.k_phys_dint_to_joules = 1.0;
        let d_int = Tensor::<B, 1>::from_floats([-1.0e6_f32], &dev);
        let info_gain = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
        gate.verify_tensor_update(d_int, info_gain)
            .expect("negative d_int must clamp before CD check");
    }

    #[test]
    fn gate_cbf_tensor_credit_deduction_independent_of_d_int() {
        let dev = NdArrayDevice::default();
        let credit0 = 1.0e-9_f64;
        let mut gate_a = GateThermodynamicCBF::new(TEMP_K, credit0);
        gate_a.k_phys_dint_to_joules = 0.0;
        let mut gate_b = GateThermodynamicCBF::new(TEMP_K, credit0);
        gate_b.k_phys_dint_to_joules = 100.0;
        let bits = Tensor::<B, 1>::from_floats([4.0_f32], &dev);
        let d_zero = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
        let d_big = Tensor::<B, 1>::from_floats([1.0e6_f32], &dev);
        let cost_a = gate_a
            .verify_tensor_update(d_zero, bits.clone())
            .expect("finite info gain with zero d_int");
        let credit_after_a = gate_a.available_credit_joules;
        let cost_b = gate_b
            .verify_tensor_update(d_big, bits)
            .expect("finite info gain with large d_int");
        assert_relative_eq!(cost_a, cost_b, epsilon = 1e-30, max_relative = 1e-9);
        assert_relative_eq!(
            credit_after_a,
            gate_b.available_credit_joules,
            epsilon = 1e-30,
            max_relative = 1e-9
        );
    }

    #[test]
    fn gate_cbf_deref_mut_alias_updates_inner_credit() {
        let mut gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        {
            let inner: &mut ThermodynamicCBFInner = &mut gate;
            inner.available_credit_joules -= 5.0e-13;
        }
        assert_relative_eq!(
            gate.available_credit_joules,
            CREDIT_J - 5.0e-13,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
    }

    #[test]
    fn gate_cbf_new_defaults_k_phys_unity_bridge() {
        let gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        assert_relative_eq!(gate.k_phys_dint_to_joules, 1.0, epsilon = 1.0e-30);
    }

    #[test]
    fn gate_cbf_landauer_cost_zero_bits_is_zero() {
        let gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        assert_relative_eq!(gate.calculate_landauer_cost(0.0), 0.0, epsilon = 1.0e-30);
    }

    #[test]
    fn gate_cbf_scalar_cd_boundary_is_admissible() {
        let mut gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let bits = 1.0_f64;
        let erasure = gate.calculate_landauer_cost(bits);
        let cost = gate
            .verify_and_deduct_update(erasure, bits)
            .expect("entropy == erasure must sit on CD boundary");
        assert_relative_eq!(cost, erasure, epsilon = 1.0e-30, max_relative = 1.0e-9);
        assert_relative_eq!(
            gate.available_credit_joules,
            CREDIT_J - erasure,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
    }

    #[test]
    fn gate_cbf_sequential_scalar_debits_accumulate() {
        let mut gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let bits_each = 0.5_f64;
        let erasure_each = gate.calculate_landauer_cost(bits_each);
        gate.verify_and_deduct_update(erasure_each, bits_each)
            .expect("first admissible step");
        gate.verify_and_deduct_update(erasure_each, bits_each)
            .expect("second admissible step");
        assert_relative_eq!(
            gate.available_credit_joules,
            CREDIT_J - 2.0 * erasure_each,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
    }

    #[test]
    fn gate_cbf_deref_readonly_forwards_temperature_k() {
        let gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let via_deref: &ThermodynamicCBFInner = &gate;
        assert_relative_eq!(via_deref.temperature_k, TEMP_K, epsilon = 1.0e-30);
        assert_relative_eq!(gate.temperature_k, TEMP_K, epsilon = 1.0e-30);
    }

    #[test]
    fn gate_cbf_into_inner_preserves_unmutated_fields() {
        let gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let inner = gate.into_inner();
        assert_relative_eq!(inner.temperature_k, TEMP_K, epsilon = 1.0e-30);
        assert_relative_eq!(inner.available_credit_joules, CREDIT_J, epsilon = 1.0e-30);
        assert_relative_eq!(inner.k_phys_dint_to_joules, 1.0, epsilon = 1.0e-30);
    }

    #[test]
    fn gate_cbf_tensor_batch_sums_info_gain_elements() {
        let dev = NdArrayDevice::default();
        let mut gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let d_int = Tensor::<B, 1>::from_floats([0.0_f32, 0.0_f32], &dev);
        let info_gain = Tensor::<B, 1>::from_floats([1.0_f32, 3.0_f32], &dev);
        let total_bits = 4.0_f64;
        let expected_debit = gate.calculate_landauer_cost(total_bits);
        let cost = gate
            .verify_tensor_update(d_int, info_gain)
            .expect("batch sum of info_gain must admit");
        assert_relative_eq!(
            cost,
            expected_debit,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
        assert_relative_eq!(
            gate.available_credit_joules,
            CREDIT_J - expected_debit,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
    }

    #[test]
    fn gate_cbf_tensor_rejects_insufficient_global_credit() {
        let dev = NdArrayDevice::default();
        let mut gate = GateThermodynamicCBF::new(TEMP_K, 0.0);
        let d_int = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
        let info_gain = Tensor::<B, 1>::from_floats([2.0_f32], &dev);
        let err = gate
            .verify_tensor_update(d_int, info_gain)
            .expect_err("positive batch info_gain with zero credit must reject");
        assert!(matches!(
            err,
            CbfReject::InsufficientGlobalEnergyCredit { .. }
        ));
    }

    #[test]
    fn gate_cbf_tensor_high_dissipation_debits_only_landauer_cost() {
        let dev = NdArrayDevice::default();
        let credit = 1.0e-9_f64;
        let mut gate = GateThermodynamicCBF::new(TEMP_K, credit);
        gate.k_phys_dint_to_joules = 100.0;
        let bits = Tensor::<B, 1>::from_floats([2.0_f32], &dev);
        let d_large = Tensor::<B, 1>::from_floats([1.0e6_f32], &dev);
        let expected_debit = gate.calculate_landauer_cost(2.0);
        let cost = gate
            .verify_tensor_update(d_large, bits)
            .expect("large d_int must not block when credit funds Landauer debit");
        assert_relative_eq!(
            cost,
            expected_debit,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
        assert_relative_eq!(
            gate.available_credit_joules,
            credit - expected_debit,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
    }

    #[test]
    fn gate_cbf_tensor_negative_info_gain_rejects_clausius_duhem() {
        let dev = NdArrayDevice::default();
        let mut gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        let d_int = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
        let info_gain = Tensor::<B, 1>::from_floats([-1.0_f32], &dev);
        let err = gate
            .verify_tensor_update(d_int, info_gain)
            .expect_err("negative batch info_gain must violate CD through tensor shim");
        assert!(matches!(err, CbfReject::ClausiusDuhemViolation { .. }));
    }

    #[test]
    fn gate_cbf_reject_insufficient_credit_carries_required_and_available() {
        let mut gate = GateThermodynamicCBF::new(TEMP_K, 1.0e-20_f64);
        let bits = 8.0_f64;
        let required = gate.calculate_landauer_cost(bits);
        let err = gate
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

    #[test]
    fn w8e14_gate_cbf_deref_mut_forwards_credit_mutation() {
        let mut gate = GateThermodynamicCBF::new(TEMP_K, CREDIT_J);
        gate.available_credit_joules *= 0.5;
        assert_relative_eq!(
            gate.available_credit_joules,
            CREDIT_J * 0.5,
            epsilon = 1.0e-30
        );
    }
}
