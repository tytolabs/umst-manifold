// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! R2: signed margin monotonicity + soft-tracks-hard integration witness.

use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::ai::constraint_loss::{
    clausius_duhem_margin, clausius_duhem_violation, explain_clausius_duhem_violation,
};
use umst_manifold::gate::transition_proposal::transition_outcome;
use umst_manifold::gate::ThermodynamicStateSnapshot;
use umst_manifold::gate::TRANSITION_TOLERANCE;
use umst_manifold::runtime::gate::{
    admissibility_from_margin, explain_cd_transition_host, AdmissibilityMargin, AdmissibilityToken,
};

type B = NdArray<f32>;

fn scalar_tensor(dev: &NdArrayDevice, values: &[f32]) -> Tensor<B, 1> {
    let b = values.len();
    Tensor::<B, 1>::from_data(Data::new(values.to_vec(), Shape::new([b])), dev)
}

#[test]
fn margin_host_matches_dissipation() {
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
    let host = transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
    let explanation = explain_cd_transition_host(&old, &new, dt, TRANSITION_TOLERANCE);
    assert!(
        (explanation.margin.value() - host.dissipation as f32).abs() < 1e-3,
        "margin {} != dissipation {}",
        explanation.margin.value(),
        host.dissipation
    );
    assert!(
        (explanation.violation - explanation.margin.violation()).abs() < 1e-6,
        "violation must equal relu(-margin)"
    );
}

#[test]
fn soft_tracks_hard_monotonicity_sweep() {
    let dev = NdArrayDevice::default();
    let old = ThermodynamicStateSnapshot {
        density: 2200.0,
        temperature: 300.0,
        free_energy: -2.0e5,
        entropy: 0.2,
        reaction_extent: 0.5,
        strength: 20.0,
    };
    let dt = 1.0_f64;

    let mut margins = Vec::new();
    let mut violations = Vec::new();
    let mut hard_reject = Vec::new();

    for spike in [0.0_f64, 5e4, 1.5e5, 3.0e5] {
        let new = ThermodynamicStateSnapshot {
            free_energy: old.free_energy + spike,
            ..old
        };
        let explanation = explain_cd_transition_host(&old, &new, dt, TRANSITION_TOLERANCE);
        margins.push(explanation.margin.value());
        violations.push(explanation.violation);
        hard_reject.push(explanation.admissibility == AdmissibilityToken::Inadmissible);

        let margin_t = clausius_duhem_margin(
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let violation_t = clausius_duhem_violation(
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let m: Vec<f32> = margin_t.into_data().value;
        let v: Vec<f32> = violation_t.into_data().value;
        assert!((v[0] - (-m[0]).max(0.0)).abs() < 1e-5);
    }

    for i in 1..margins.len() {
        assert!(
            margins[i] <= margins[i - 1] + 1e-3,
            "margin must decrease (more negative) as ψ spikes: {:?}",
            margins
        );
        assert!(
            violations[i] >= violations[i - 1] - 1e-3,
            "violation must be monotone non-decreasing: {:?}",
            violations
        );
        if violations[i] > 1e-4 {
            assert!(hard_reject[i], "positive violation must HARD reject");
        }
    }

    let max_margin = margins.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let min_margin = margins.iter().copied().fold(f32::INFINITY, f32::min);
    assert!(
        max_margin > min_margin + 1e-3,
        "sweep must span margin range: max={max_margin} min={min_margin} margins={margins:?}"
    );
    assert_eq!(
        admissibility_from_margin(AdmissibilityMargin(max_margin)),
        AdmissibilityToken::Admissible
    );
}

#[test]
fn explain_hot_path_carries_margin_token_parity() {
    let dev = NdArrayDevice::default();
    let old = ThermodynamicStateSnapshot {
        density: 2400.0,
        temperature: 293.15,
        free_energy: -1.35e5,
        entropy: 0.05,
        reaction_extent: 0.42,
        strength: 12.7,
    };
    let new = ThermodynamicStateSnapshot {
        free_energy: -1.0e4,
        ..old
    };
    let explanation = explain_clausius_duhem_violation(
        scalar_tensor(&dev, &[old.density as f32]),
        scalar_tensor(&dev, &[new.density as f32]),
        scalar_tensor(&dev, &[old.free_energy as f32]),
        scalar_tensor(&dev, &[new.free_energy as f32]),
        scalar_tensor(&dev, &[1.0_f32]),
    );
    assert!(explanation.margin.value() < 0.0);
    assert!(explanation.violation > 0.0);
    assert_eq!(explanation.admissibility, AdmissibilityToken::Inadmissible);
    assert_eq!(
        explanation.admissibility,
        admissibility_from_margin(explanation.margin)
    );
}
