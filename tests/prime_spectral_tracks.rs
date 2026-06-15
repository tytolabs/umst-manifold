// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![cfg(feature = "topology-density-evolution")]

use std::fs;
use std::path::PathBuf;

use umst_manifold::physics::prime_spectral_ntt::run_ntt_drift_study;
use umst_manifold::physics::prime_spectral_qmc::run_qmc_study;

fn workspace_outputs(sub: &str) -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .expect("manifest parent")
        .join("outputs")
        .join("prime-spectral-research")
        .join(sub)
}

#[test]
fn track1_ntt_drift_emit() {
    let seeds = [42_u64, 137];
    let mut records = Vec::new();
    for &seed in &seeds {
        records.push(run_ntt_drift_study(64, seed));
    }
    let hits = records.iter().filter(|r| r.pattern_hit).count();
    let verdict = if hits >= 2 {
        "meets_criterion"
    } else if records.iter().all(|r| r.ntt_zero_conservation_drift == 0.0) {
        "partial_hit_conservation_only"
    } else {
        "null"
    };
    let payload = serde_json::json!({
        "schema": "prime_spectral_track1_emit_v1",
        "seeds": seeds,
        "records": records,
        "verdict": verdict,
    });
    let dir = workspace_outputs("track1");
    fs::create_dir_all(&dir).expect("track1 dir");
    let path = dir.join("ntt-drift-study.json");
    fs::write(&path, serde_json::to_string_pretty(&payload).expect("json")).expect("write");
    assert!(path.is_file());
    assert!(
        records
            .iter()
            .all(|r| r.ntt_zero_conservation_drift == 0.0),
        "NTT zero-input conservation must hold"
    );
}

#[test]
fn track2_halton_qmc_emit() {
    let seeds = [42_u64, 137];
    let mut records = Vec::new();
    for &seed in &seeds {
        records.push(run_qmc_study(seed));
    }
    let hits = records.iter().filter(|r| r.pattern_hit).count();
    let verdict = if hits >= 2 {
        "meets_criterion"
    } else if records.iter().any(|r| r.halton_samples_to_tol < r.prng_samples_to_tol) {
        "partial_hit_single_seed"
    } else {
        "null"
    };
    let payload = serde_json::json!({
        "schema": "prime_spectral_track2_emit_v1",
        "seeds": seeds,
        "records": records,
        "verdict": verdict,
    });
    let dir = workspace_outputs("track2");
    fs::create_dir_all(&dir).expect("track2 dir");
    let path = dir.join("halton-qmc-study.json");
    fs::write(&path, serde_json::to_string_pretty(&payload).expect("json")).expect("write");
    assert!(path.is_file());
}
