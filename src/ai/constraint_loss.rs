// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Differentiable Clausius–Duhem slack for Burn training (hot path only).
//!
//! Host commit semantics: [`crate::gate::route::canonical_core_gate_outcome`] (Phase 0d).
//! Hot tensors evaluate CD slack only; cold host alignment uses the canonical Core gate.

use burn::tensor::activation::relu;
use burn::tensor::{backend::Backend, Tensor};

use crate::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;
pub use crate::runtime::gate::evidence::{
    admissibility_from_violation, AdmissibilityToken, ConstraintExplanation,
};
pub use crate::runtime::gate::{AdmissibilityMargin, ADMISSIBILITY_MARGIN_EPS};

/// Host-side Core gate net dissipation — routes through canonical surface (Phase 0d).
#[must_use]
pub fn canonical_core_net_dissipation_host(
    old_density: f64,
    new_density: f64,
    old_free_energy: f64,
    new_free_energy: f64,
    dt_s: f64,
    power_input: f64,
) -> f64 {
    crate::gate::route::canonical_core_gate_outcome(
        old_density,
        new_density,
        old_free_energy,
        new_free_energy,
        dt_s,
        power_input,
    )
    .net_dissipation
}

/// Numerical floor on `dt` matching [`crate::gate::transition_proposal::transition_outcome`].
const DT_EPS: f32 = 1e-10;

/// Boltzmann constant (J/K) — matches [`crate::constants::landauer_bit_energy_joules`] fallback path.
const K_BOLTZMANN_F32: f32 = 1.380_649e-23;

/// ln(2) bit factor for Landauer erasure floor.
const LN2_F32: f32 = std::f32::consts::LN_2;

/// Per-batch signed Clausius–Duhem margin `D_int = −ρ ψ̇`.
///
/// Hot-path CD slack only. For Mass + CD host alignment see
/// [`crate::gate::route::canonical_core_gate_outcome`].
pub fn clausius_duhem_margin<B: Backend<FloatElem = f32>>(
    old_density: Tensor<B, 1>,
    new_density: Tensor<B, 1>,
    old_free_energy: Tensor<B, 1>,
    new_free_energy: Tensor<B, 1>,
    dt_s: Tensor<B, 1>,
) -> Tensor<B, 1> {
    let rho = old_density.add(new_density).div_scalar(2.0);
    let psi_dot = new_free_energy
        .sub(old_free_energy)
        .div(dt_s.add_scalar(DT_EPS));
    psi_dot.mul(rho).neg()
}

/// Per-batch ReLU slack for Clausius–Duhem dissipation violation (`relu(−margin)`).
///
/// Mirrors the host gate surrogate `D_int = −ρ ψ̇` with
/// `ρ = (ρ_old + ρ_new) / 2`, `ψ̇ = (ψ_new − ψ_old) / (Δt + ε)`, and returns
/// `relu(−D_int)` so admissible transitions (non-negative dissipation) yield zero loss.
///
/// # Tensor contract
///
/// All inputs are shaped `[B]` with identical batch length.
pub fn clausius_duhem_violation<B: Backend<FloatElem = f32>>(
    old_density: Tensor<B, 1>,
    new_density: Tensor<B, 1>,
    old_free_energy: Tensor<B, 1>,
    new_free_energy: Tensor<B, 1>,
    dt_s: Tensor<B, 1>,
) -> Tensor<B, 1> {
    relu(
        clausius_duhem_margin(
            old_density,
            new_density,
            old_free_energy,
            new_free_energy,
            dt_s,
        )
        .neg(),
    )
}

/// Weighted violation slack from host gate evidence (`λ_cd · violation` per witness).
pub fn scaled_constraint_violation_penalty<B: Backend<FloatElem = f32>>(
    lambda_cd: f32,
    violations: Tensor<B, 1>,
) -> Tensor<B, 1> {
    let batch = violations.dims()[0];
    let device = violations.device();
    if lambda_cd == 0.0_f32 {
        return Tensor::zeros([batch], &device);
    }
    violations.mul_scalar(lambda_cd)
}

/// Weighted Clausius–Duhem slack for gateway / PPO penalty hooks.
///
/// Returns zeros when `lambda_cd == 0` without building the violation graph.
pub fn scaled_clausius_duhem_violation<B: Backend<FloatElem = f32>>(
    lambda_cd: f32,
    old_density: Tensor<B, 1>,
    new_density: Tensor<B, 1>,
    old_free_energy: Tensor<B, 1>,
    new_free_energy: Tensor<B, 1>,
    dt_s: Tensor<B, 1>,
) -> Tensor<B, 1> {
    let batch = old_density.dims()[0];
    let device = old_density.device();
    if lambda_cd == 0.0_f32 {
        return Tensor::zeros([batch], &device);
    }
    clausius_duhem_violation(
        old_density,
        new_density,
        old_free_energy,
        new_free_energy,
        dt_s,
    )
    .mul_scalar(lambda_cd)
}

/// Per-batch ReLU slack when resolved bits exceed the available Landauer credit (joules).
///
/// Mirrors [`crate::ai::cbf::ThermodynamicCBF::calculate_landauer_cost`] at tensor granularity:
/// `erasure_cost = k_B · T · ln(2) · bits`, returns `relu(erasure_cost − credit_j)`.
pub fn landauer_slack_violation<B: Backend<FloatElem = f32>>(
    info_gain_bits: Tensor<B, 1>,
    temperature_k: f32,
    available_credit_joules: f32,
) -> Tensor<B, 1> {
    let bit_energy = temperature_k * LN2_F32 * K_BOLTZMANN_F32;
    let erasure_cost = info_gain_bits.mul_scalar(bit_energy);
    relu(erasure_cost.sub_scalar(available_credit_joules))
}

/// Weighted Landauer slack for gateway / PPO penalty hooks.
pub fn scaled_landauer_slack_violation<B: Backend<FloatElem = f32>>(
    lambda_landauer: f32,
    info_gain_bits: Tensor<B, 1>,
    temperature_k: f32,
    available_credit_joules: f32,
) -> Tensor<B, 1> {
    let batch = info_gain_bits.dims()[0];
    let device = info_gain_bits.device();
    if lambda_landauer == 0.0_f32 {
        return Tensor::zeros([batch], &device);
    }
    landauer_slack_violation(info_gain_bits, temperature_k, available_credit_joules)
        .mul_scalar(lambda_landauer)
}

/// Structured explanation for Clausius–Duhem slack at the same batch contract as
/// [`clausius_duhem_violation`].
///
/// Aggregates batch elements by maximum violation (worst offender) for telemetry.
pub fn explain_clausius_duhem_violation<B: Backend<FloatElem = f32>>(
    old_density: Tensor<B, 1>,
    new_density: Tensor<B, 1>,
    old_free_energy: Tensor<B, 1>,
    new_free_energy: Tensor<B, 1>,
    dt_s: Tensor<B, 1>,
) -> ConstraintExplanation {
    let margin_tensor = clausius_duhem_margin(
        old_density.clone(),
        new_density.clone(),
        old_free_energy.clone(),
        new_free_energy.clone(),
        dt_s.clone(),
    );
    let violation = clausius_duhem_violation(
        old_density,
        new_density,
        old_free_energy,
        new_free_energy,
        dt_s,
    );
    let m = margin_tensor
        .clone()
        .into_data()
        .value
        .into_iter()
        .fold(0.0_f32, |a, b| if b < a { b } else { a });
    let v = violation
        .into_data()
        .value
        .into_iter()
        .fold(0.0_f32, f32::max);
    ConstraintExplanation {
        margin: AdmissibilityMargin(m),
        violation: v,
        channel_id: CD_TRANSITION_CATALOG_ID,
        admissibility: admissibility_from_violation(v),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::transition_proposal::transition_outcome;
    use crate::gate::ThermodynamicStateSnapshot;
    use burn::tensor::{Data, Shape};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn scalar_tensor(dev: &NdArrayDevice, values: &[f32]) -> Tensor<B, 1> {
        let b = values.len();
        Tensor::<B, 1>::from_data(Data::new(values.to_vec(), Shape::new([b])), dev)
    }

    #[test]
    fn clausius_duhem_violation_zero_when_host_admits() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2400.0,
            temperature: 293.15,
            free_energy: -1.35e5,
            entropy: 0.05,
            reaction_extent: 0.42,
            strength: 12.7,
        };
        let new = old;
        let dt = 1.0_f64;
        let host = transition_outcome(&old, &new, dt, 1e-6);
        assert!(host.energy_positive, "sanity: identity transition admits");

        let violation = clausius_duhem_violation(
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let v: Vec<f32> = violation.into_data().value;
        assert!(
            v[0].abs() < 1e-4,
            "admissible host path → zero slack, got {}",
            v[0]
        );
    }

    #[test]
    fn clausius_duhem_violation_matches_host_negative_dissipation() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let dt = 1.0_f64;
        let host = transition_outcome(&old, &new, dt, 1e-6);
        assert!(!host.energy_positive, "sanity: ψ spike rejects on host");

        let violation = clausius_duhem_violation(
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let v: Vec<f32> = violation.into_data().value;
        let expected = (-host.dissipation).max(0.0) as f32;
        assert!(
            v[0] > 0.0,
            "inadmissible transition must incur positive slack, got {}",
            v[0]
        );
        assert!(
            (v[0] - expected).abs() < 1.0,
            "slack {v0} should track host relu(-D_int) ≈ {expected}",
            v0 = v[0]
        );
    }

    #[test]
    fn explain_clausius_duhem_violation_admissible_token() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2400.0,
            temperature: 293.15,
            free_energy: -1.35e5,
            entropy: 0.05,
            reaction_extent: 0.42,
            strength: 12.7,
        };
        let new = old;
        let dt = 1.0_f64;

        let explanation = explain_clausius_duhem_violation(
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        assert!(
            explanation.violation.abs() < 1e-4,
            "admissible → zero violation, got {}",
            explanation.violation
        );
        assert_eq!(explanation.admissibility, AdmissibilityToken::Admissible);
        assert_eq!(explanation.channel_id, CD_TRANSITION_CATALOG_ID);
    }

    #[test]
    fn explain_clausius_duhem_violation_inadmissible_token() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let dt = 1.0_f64;

        let explanation = explain_clausius_duhem_violation(
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        assert!(
            explanation.violation > 0.0,
            "inadmissible → positive violation, got {}",
            explanation.violation
        );
        assert_eq!(explanation.admissibility, AdmissibilityToken::Inadmissible);
        assert_eq!(explanation.channel_id, CD_TRANSITION_CATALOG_ID);
    }

    #[test]
    fn scaled_clausius_duhem_violation_zero_when_lambda_disabled() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let dt = 1.0_f64;

        let penalty = scaled_clausius_duhem_violation(
            0.0_f32,
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let v: Vec<f32> = penalty.into_data().value;
        assert_eq!(v[0], 0.0_f32, "λ_cd = 0 must short-circuit to zero penalty");
    }

    #[test]
    fn scaled_clausius_duhem_violation_scales_slack() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let dt = 1.0_f64;
        let lambda = 2.5_f32;

        let slack = clausius_duhem_violation(
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let penalty = scaled_clausius_duhem_violation(
            lambda,
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let s: Vec<f32> = slack.into_data().value;
        let p: Vec<f32> = penalty.into_data().value;
        assert!(
            s[0] > 0.0,
            "inadmissible transition must incur positive slack"
        );
        assert!(
            (p[0] - lambda * s[0]).abs() < 1e-3,
            "penalty {p0} should equal λ·slack ≈ {expected}",
            p0 = p[0],
            expected = lambda * s[0]
        );
    }

    #[test]
    fn landauer_slack_violation_zero_when_credit_sufficient() {
        let dev = NdArrayDevice::default();
        let bits = scalar_tensor(&dev, &[0.01_f32]);
        let slack = landauer_slack_violation(bits, 300.0_f32, 1.0e6_f32);
        let v: Vec<f32> = slack.into_data().value;
        assert!(
            v[0].abs() < 1e-12,
            "ample credit → zero Landauer slack, got {}",
            v[0]
        );
    }

    #[test]
    fn landauer_slack_violation_positive_when_credit_exhausted() {
        let dev = NdArrayDevice::default();
        let bits = scalar_tensor(&dev, &[1.0_f32]);
        let slack = landauer_slack_violation(bits, 300.0_f32, 0.0_f32);
        let v: Vec<f32> = slack.into_data().value;
        assert!(v[0] > 0.0, "zero credit → positive Landauer slack");
        let expected = 300.0_f32 * LN2_F32 * K_BOLTZMANN_F32;
        assert!(
            (v[0] - expected).abs() < 1e-20,
            "slack {v0} should track k_B T ln2 bits ≈ {expected}",
            v0 = v[0]
        );
    }

    #[test]
    fn scaled_landauer_slack_violation_zero_when_lambda_disabled() {
        let dev = NdArrayDevice::default();
        let bits = scalar_tensor(&dev, &[1.0_f32]);
        let penalty = scaled_landauer_slack_violation(0.0_f32, bits, 300.0_f32, 0.0_f32);
        let v: Vec<f32> = penalty.into_data().value;
        assert_eq!(v[0], 0.0_f32);
    }
}
