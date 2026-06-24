// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Differentiable Clausius–Duhem slack for Burn training (hot path only).
//!
//! Host commit semantics: [`crate::gate::transition_proposal::transition_outcome`].

use burn::tensor::activation::relu;
use burn::tensor::{backend::Backend, Tensor};

use crate::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;

/// Numerical floor on `dt` matching [`crate::gate::transition_proposal::transition_outcome`].
const DT_EPS: f32 = 1e-10;

/// Host-side admissibility witness for constraint telemetry (cold edge only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdmissibilityToken {
    Admissible,
    Inadmissible,
}

/// Pure-data explanation sidecar for a single constraint channel sample.
///
/// Built from detached host scalars after the Burn step — never inside the autodiff graph.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstraintExplanation {
    pub violation: f32,
    pub channel_id: &'static str,
    pub admissibility: AdmissibilityToken,
}

fn admissibility_from_violation(violation: f32) -> AdmissibilityToken {
    if violation <= 1e-4 {
        AdmissibilityToken::Admissible
    } else {
        AdmissibilityToken::Inadmissible
    }
}

/// Per-batch ReLU slack for Clausius–Duhem dissipation violation.
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
    let rho = old_density.add(new_density).div_scalar(2.0);
    let psi_dot = new_free_energy
        .sub(old_free_energy)
        .div(dt_s.add_scalar(DT_EPS));
    let d_int = psi_dot.mul(rho).neg();
    relu(d_int.neg())
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
    let violation = clausius_duhem_violation(
        old_density,
        new_density,
        old_free_energy,
        new_free_energy,
        dt_s,
    );
    let v = violation
        .into_data()
        .value
        .into_iter()
        .fold(0.0_f32, f32::max);
    ConstraintExplanation {
        violation: v,
        channel_id: CD_TRANSITION_CATALOG_ID,
        admissibility: admissibility_from_violation(v),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::{Data, Shape};
    use burn_ndarray::{NdArray, NdArrayDevice};
    use crate::gate::transition_proposal::transition_outcome;
    use crate::gate::ThermodynamicStateSnapshot;

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
        assert!(v[0].abs() < 1e-4, "admissible host path → zero slack, got {}", v[0]);
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
}
