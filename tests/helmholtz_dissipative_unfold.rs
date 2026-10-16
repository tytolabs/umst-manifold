// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! The Helmholtz density filter as a metered unfold of dissipative steps
//! (cell UNFOLD-DISSIPATION-STEP, the filter as its caller).
//!
//! With an energy budget, `HelmholtzFilter::apply` runs each Richardson step through
//! `dissipative_unfold`, admitted by the core gate on the passive route `ψ = ½xᵀAx`,
//! `φ = ½|v|²`, and metered by the host package energy counter. The metered field is checked
//! against the unmetered Richardson loop (the differential oracle) and against its own
//! residual; a budget too small for the solve is a typed refusal carrying the last residual.
#![cfg(feature = "topology-density-evolution")]

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use ordered_float::NotNan;
use umst_manifold::physics::error::PhysicsError;
use umst_manifold::physics::topology_filter::HelmholtzFilter;
use umst_math::solve_combinator::{landauer_step_joules, EnergyBudget};

type B = NdArray<f32>;
const T_K: f64 = 293.15;

fn edges(pairs: &[(i64, i64)]) -> Tensor<B, 2, Int> {
    let mut flat: Vec<i64> = pairs.iter().map(|p| p.0).collect();
    flat.extend(pairs.iter().map(|p| p.1));
    Tensor::from_data(
        Data::new(flat, Shape::new([2, pairs.len()])).convert(),
        &NdArrayDevice::default(),
    )
}

fn field(values: &[f32]) -> Tensor<B, 3> {
    Tensor::from_data(
        Data::new(values.to_vec(), Shape::new([1, values.len(), 1])),
        &NdArrayDevice::default(),
    )
}

fn rho(n: usize) -> Vec<f32> {
    (0..n).map(|i| ((i * 7919) % 13) as f32 / 13.0).collect()
}

fn values(t: Tensor<B, 3>) -> Vec<f32> {
    t.into_data().value
}

/// Metered and unmetered fields of one problem.
fn both(n: usize, pairs: &[(i64, i64)], radius: f32, budget: EnergyBudget) -> (Vec<f32>, Vec<f32>) {
    let e = edges(pairs);
    let r = field(&rho(n));
    let plain = HelmholtzFilter::new(radius, 0, 1e-5);
    let metered = HelmholtzFilter::new(radius, 0, 1e-5).with_energy_budget(budget);
    let oracle = values(
        plain
            .apply(r.clone(), e.clone(), 1.0)
            .expect("unmetered loop"),
    );
    let unfolded = values(metered.apply(r, e, 1.0).expect("metered unfold"));
    (oracle, unfolded)
}

fn chain(n: usize) -> Vec<(i64, i64)> {
    (0..n as i64 - 1).map(|i| (i, i + 1)).collect()
}

#[test]
fn the_metered_unfold_agrees_with_the_richardson_loop() {
    let budget = EnergyBudget::from_joules_at(1.0, T_K).expect("budget");
    for (n, pairs, radius) in [
        (40, chain(40), 2.0_f32),
        (60, (1..60_i64).map(|i| (0, i)).collect::<Vec<_>>(), 1.0),
    ] {
        let (oracle, unfolded) = both(n, &pairs, radius, budget);
        let scale = oracle.iter().fold(0.0_f32, |m, v| m.max(v.abs()));
        let gap = oracle
            .iter()
            .zip(&unfolded)
            .fold(0.0_f32, |m, (a, b)| m.max((a - b).abs()));
        // Both stop at relative residual 1e-5 against ‖ρ‖; λmin(A) ≥ 1 bounds each error by
        // 1e-5·‖ρ‖, so the two fields differ by at most 2e-5·‖ρ‖ (plus binary32 rounding).
        let rho_norm = rho(n).iter().map(|v| v * v).sum::<f32>().sqrt();
        assert!(gap <= 2e-5 * rho_norm + 1e-6 * scale, "n={n} gap={gap}");
    }
}

#[test]
fn a_budget_below_the_solve_is_refused_with_its_residual() {
    let floor = landauer_step_joules(
        NotNan::new(T_K).expect("T"),
        NotNan::new(64.0 * 40.0).expect("bits"),
    )
    .expect("floor")
    .ucrs_ledger_joules()
    .expect("joules")
    .into_inner();
    // Three steps' worth of floor (absent spends are debited at the floor; a measured host
    // spends more): the solve needs more steps than that.
    let budget = EnergyBudget::from_joules_at(3.0 * floor, T_K).expect("budget");
    let out = HelmholtzFilter::new(2.0, 0, 1e-5)
        .with_energy_budget(budget)
        .apply(field(&rho(40)), edges(&chain(40)), 1.0);
    match out {
        Err(PhysicsError::Diverged {
            eq_rel,
            pcg_iterations,
        }) => {
            assert!(eq_rel > 1e-5 && eq_rel.is_finite());
            assert!(pcg_iterations >= 1);
        }
        other => panic!(
            "expected a refusal with the last residual, got {:?}",
            other.map(|_| ())
        ),
    }
}
