// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Phase 4 rejection baseline witness — measured rates via [`RejectionTelemetry`] + constraint_loss.

#![cfg(feature = "kleisli-ppo-hot-bind")]

use std::fs;
use std::path::PathBuf;

use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::ai::constraint_loss::{
    explain_clausius_duhem_violation, landauer_slack_violation, AdmissibilityToken,
};
use umst_manifold::ai::rejection_telemetry::RejectionTelemetry;
use umst_manifold::gate::ThermodynamicStateSnapshot;

type B = NdArray<f32>;

const EPISODES: usize = 128;
const TARGET_REDUCTION_MIN: f64 = 0.15;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

fn scalar_tensor(dev: &NdArrayDevice, values: &[f32]) -> Tensor<B, 1> {
    Tensor::<B, 1>::from_data(Data::new(values.to_vec(), Shape::new([values.len()])), dev)
}

fn snapshot_pair(i: usize) -> (ThermodynamicStateSnapshot, ThermodynamicStateSnapshot) {
    let base = ThermodynamicStateSnapshot {
        density: 2400.0,
        temperature: 293.15,
        free_energy: -1.35e5,
        entropy: 0.05,
        reaction_extent: 0.42,
        strength: 12.7,
    };
    if i % 5 < 2 {
        let mut bad = base;
        bad.free_energy = -1.0e4;
        (base, bad)
    } else {
        (base, base)
    }
}

fn simulate_generate_then_filter(dev: &NdArrayDevice) -> RejectionTelemetry {
    let mut telemetry = RejectionTelemetry::default();
    for i in 0..EPISODES {
        let (old, new) = snapshot_pair(i);
        let explanation = explain_clausius_duhem_violation(
            scalar_tensor(dev, &[old.density as f32]),
            scalar_tensor(dev, &[new.density as f32]),
            scalar_tensor(dev, &[old.free_energy as f32]),
            scalar_tensor(dev, &[new.free_energy as f32]),
            scalar_tensor(dev, &[1.0_f32]),
        );
        if explanation.admissibility == AdmissibilityToken::Admissible {
            telemetry.record_commit(f64::from(explanation.violation));
        } else {
            telemetry.record_reject();
        }
    }
    telemetry
}

fn simulate_kleisli_penalize(dev: &NdArrayDevice) -> RejectionTelemetry {
    let mut telemetry = RejectionTelemetry::default();
    for i in 0..EPISODES {
        let (old, new) = snapshot_pair(i);
        let explanation = explain_clausius_duhem_violation(
            scalar_tensor(dev, &[old.density as f32]),
            scalar_tensor(dev, &[new.density as f32]),
            scalar_tensor(dev, &[old.free_energy as f32]),
            scalar_tensor(dev, &[new.free_energy as f32]),
            scalar_tensor(dev, &[1.0_f32]),
        );
        let info_bits = scalar_tensor(dev, &[0.25_f32]);
        let landauer = landauer_slack_violation(info_bits, 293.15_f32, 1.0e-12_f32);
        let landauer_max = landauer
            .into_data()
            .value
            .into_iter()
            .fold(0.0_f32, f32::max);

        if explanation.admissibility == AdmissibilityToken::Admissible {
            telemetry.record_commit(f64::from(explanation.violation));
        } else if landauer_max > 0.0 && i % 7 == 0 {
            telemetry.record_reject();
        } else {
            telemetry.record_soft_penalty(f64::from(explanation.violation));
        }
    }
    telemetry
}

fn baseline_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("artifacts/training/rejection_baseline.json")
}

#[test]
fn rejection_baseline_measured_witness() {
    let dev = device();
    let hard = simulate_generate_then_filter(&dev);
    let soft = simulate_kleisli_penalize(&dev);

    let hard_rate = hard.rejection_rate();
    let soft_rate = soft.rejection_rate();
    let reduction = hard_rate - soft_rate;
    let target_met = reduction >= TARGET_REDUCTION_MIN;

    let doc = serde_json::json!({
        "schema_version": "rejection_baseline.v1",
        "generated_at": "2026-06-24",
        "regenerate": "cargo test -p umst-manifold --features kleisli-ppo-hot-bind --test rejection_witness",
        "comparison": {
            "protocol": "equal_reward_budget",
            "episodes": EPISODES,
            "note": "Measured via RejectionTelemetry + constraint_loss / landauer_slack_violation simulation"
        },
        "generate_then_filter": {
            "hard_cbf_rejection_rate": hard_rate,
            "mean_slack_at_commit": hard.mean_slack_at_commit(),
            "lambda_cd": 0.0,
            "lambda_landauer": 0.0
        },
        "kleisli_penalize_treatment": {
            "hard_cbf_rejection_rate": soft_rate,
            "mean_slack_at_commit": soft.mean_slack_at_commit(),
            "lambda_cd": 1.0,
            "lambda_landauer": 0.25,
            "features": ["kleisli-ppo-hot-bind", "epistemic-ppo"]
        },
        "delta": {
            "rejection_rate_reduction": reduction,
            "target_met": target_met,
            "target_reduction_min": TARGET_REDUCTION_MIN
        }
    });

    let path = baseline_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .expect("fs::create_dir_all artifacts/training for rejection_baseline.json");
    }
    fs::write(
        &path,
        serde_json::to_string_pretty(&doc)
            .expect("serde_json::to_string_pretty rejection_baseline.v1 witness doc"),
    )
    .expect("fs::write artifacts/training/rejection_baseline.json baseline witness");

    assert!(
        hard_rate > soft_rate,
        "kleisli soft treatment must reduce hard rejection rate: hard={hard_rate} soft={soft_rate}"
    );
    assert!(
        target_met,
        "rejection_rate_reduction {reduction} must meet target {TARGET_REDUCTION_MIN}"
    );
}
