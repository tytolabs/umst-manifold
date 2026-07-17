// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! RW-FP-PRABHU PB-1 — gate routing microbench (`canonical_transition_outcome`).

use umst_manifold::gate::{canonical_transition_outcome, ThermodynamicStateSnapshot};

const N: usize = 10_000;

#[test]
fn prabhu_gate_route_timing() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.0, 293.15, 80.0);
    let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
    let dt = 28.0 * 24.0 * 3600.0;

    // Warm-up
    for _ in 0..100 {
        let _ = canonical_transition_outcome(&old, &new, dt);
    }

    let start = std::time::Instant::now();
    for _ in 0..N {
        let _ = canonical_transition_outcome(&old, &new, dt);
    }
    let elapsed_us = start.elapsed().as_secs_f64() * 1e6;
    let us_per_call = elapsed_us / N as f64;

    println!("gate_route_us_per_call={us_per_call:.3}");
    eprintln!("prabhu_pb1_ok n={N} total_us={elapsed_us:.1}");
}
