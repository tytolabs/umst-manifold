// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Oracle harvest for `golden_learner_mutualinfoestimator_v0` — pinned inputs only.
//!
//! Trajectories cite existing tests:
//! - `epistemic_mi::tests::correlated_samples_yield_positive_mi`
//! - `epistemic_ppo::histogram_mi_tensor_respects_landauer_ln2_cap`

#![cfg(feature = "epistemic-ppo")]

use umst_manifold::ai::epistemic_mi::MutualInfoEstimator;
use umst_manifold::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;

/// Vector 1 — `epistemic_mi.rs` unit test trajectory (2×2, 300 ramp updates).
#[must_use]
pub fn harvest_correlated_2x2_300() -> f64 {
    let mut est = MutualInfoEstimator::new(2, 2);
    for i in 0..300 {
        let x = i as f64 / 300.0;
        est.update(&[x, x], &[x, x]);
    }
    est.estimate()
}

/// Vector 2 — `epistemic_ppo.rs` material-proxy trajectory (400 ramp updates).
#[must_use]
pub fn harvest_material_proxy_400() -> f64 {
    let mut est = MutualInfoEstimator::for_material_proxy();
    for i in 0..400 {
        let x = i as f64 / 400.0;
        est.update(
            &[x; UMST_SCALAR_CHANNEL_COUNT],
            &[x; UMST_SCALAR_CHANNEL_COUNT],
        );
    }
    est.estimate()
}

#[test]
fn mi_oracle_harvest_pins_from_existing_tests() {
    let v1 = harvest_correlated_2x2_300();
    let v2 = harvest_material_proxy_400();
    eprintln!("HARVEST correlated_2x2_300 mi_bits={v1:.17e}");
    eprintln!(
        "HARVEST material_proxy_400 mi_bits={v2:.17e} scalar_channels={UMST_SCALAR_CHANNEL_COUNT}"
    );
    assert!((v1 - 2.4689563066156435).abs() < 1e-12);
    assert!((v2 - 4.94547982979536).abs() < 1e-12);
    assert!(v1 >= 0.0);
    assert!(v2 >= 0.0);
}
