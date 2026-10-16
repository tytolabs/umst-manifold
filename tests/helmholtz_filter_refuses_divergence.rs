// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! The Helmholtz density filter never returns an unconverged field as an answer
//! (cell HELMHOLTZ-FILTER-REFUSES-DIVERGENCE).
//!
//! The filter solves `(I + s·L_std) x = ρ` with `s = (r/dx)²` by Richardson iteration, which
//! converges only for a step `ω < 2/λmax`. Each field the filter returns is checked here
//! against its own residual `‖ρ − A x‖ / ‖ρ‖`, so a filter that returns a diverged or
//! stalled field fails; a run that cannot reach its tolerance must refuse.

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::physics::error::PhysicsError;
use umst_manifold::physics::laplacian::TopologicalLaplacian;
use umst_manifold::physics::topology_filter::HelmholtzFilter;

type B = NdArray<f32>;

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

/// `‖ρ − (x − s·L_ours x)‖ / ‖ρ‖` of a returned field.
fn relative_residual(rho: &Tensor<B, 3>, x: &Tensor<B, 3>, e: &Tensor<B, 2, Int>, s: f32) -> f32 {
    let lx = TopologicalLaplacian::scalar_laplacian(x.clone(), e.clone(), x.zeros_like());
    let r = rho.clone().sub(x.clone().sub(lx.mul_scalar(s)));
    let norm = |t: Tensor<B, 3>| t.powf_scalar(2.0).sum().into_data().value[0].sqrt();
    norm(r) / norm(rho.clone())
}

fn rho(n: usize) -> Vec<f32> {
    (0..n).map(|i| ((i * 7919) % 13) as f32 / 13.0).collect()
}

/// The filter's answer either meets its tolerance or is a typed refusal.
fn answer_or_refusal(n: usize, pairs: &[(i64, i64)], s: f32, tol: f32) {
    let e = edges(pairs);
    let r = field(&rho(n));
    let helm = HelmholtzFilter::new(s.sqrt(), 0, tol);
    match helm.apply(r.clone(), e.clone(), 1.0) {
        Ok(x) => {
            let rel = relative_residual(&r, &x, &e, s);
            assert!(
                rel.is_finite() && rel <= 2.0 * tol,
                "returned a field with residual {rel}"
            );
        }
        Err(PhysicsError::Diverged { eq_rel, .. }) => assert!(eq_rel > tol),
        Err(other) => panic!("unexpected refusal {other:?}"),
    }
}

#[test]
fn a_wide_filter_on_a_chain_meets_its_tolerance() {
    // s = 100: the old floor ω ≥ 0.008 exceeds 2/λmax ≈ 2/401 and the iteration diverges.
    let n = 50;
    let pairs: Vec<(i64, i64)> = (0..n as i64 - 1).map(|i| (i, i + 1)).collect();
    answer_or_refusal(n, &pairs, 100.0, 1e-4);
}

#[test]
fn a_star_graph_is_bounded_by_its_maximum_degree() {
    // Average degree ≈ 2 while the hub has degree n − 1, so λmax ≈ 1 + s·n: a step from the
    // average degree diverges.
    let n = 200;
    let pairs: Vec<(i64, i64)> = (1..n as i64).map(|i| (0, i)).collect();
    answer_or_refusal(n, &pairs, 1.0, 1e-4);
}

#[test]
fn a_tolerance_below_single_precision_is_refused_with_its_residual() {
    let n = 30;
    let pairs: Vec<(i64, i64)> = (0..n as i64 - 1).map(|i| (i, i + 1)).collect();
    let e = edges(&pairs);
    let r = field(&rho(n));
    let out = HelmholtzFilter::new(1.0, 0, 1e-12).apply(r, e, 1.0);
    match out {
        Err(PhysicsError::Diverged { eq_rel, .. }) => assert!(eq_rel > 1e-12 && eq_rel.is_finite()),
        other => panic!(
            "expected a refusal carrying the last residual, got {:?}",
            other.map(|_| ())
        ),
    }
}
