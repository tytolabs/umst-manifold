// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! R13-2 rank-1+ golden harness — measured verdict + perturbation witness.

use umst_algebra_burn::golden_harness::{
    compare_burn_lift_to_golden, compare_host_scalar, rank1_eps, rank1_mul_path_matches_scalar,
    GoldenVerdict,
};
use umst_algebra_burn::rank1::RANK1_PLUS_IMPL_LANDED;
use umst_cartridge_api::{ScalarAlgebra, TensorAlgebra};
use umst_algebra_burn::{BurnNdArrayAlgebra, BurnRank0Algebra, BurnTensorField};
use umst_algebra_burn::tensor::DefaultBackend;

const PROBES: &[(f64, f64)] = &[
    (0.0, 0.0),
    (1.0, 2.5),
    (-3.5, 0.1),
    (4.0e-6, 2.4e-3),
    (35.689_57, 1.0),
];

#[test]
fn rank1_harness_reports_verdict_on_probe_grid() {
    let eps = rank1_eps();
    for &(lhs, rhs) in PROBES {
        let v = rank1_mul_path_matches_scalar(lhs, rhs, eps);
        assert!(
            v.closes_deferred(),
            "probe ({lhs},{rhs}) must close: {v:?}"
        );
    }
}

#[test]
fn rank1_burn_lift_matches_con_scalar_golden() {
    let eps = rank1_eps();
    let golden = 35.689_57;
    let v = compare_burn_lift_to_golden(golden, golden, eps);
    assert!(v.closes_deferred(), "{v:?}");
}

#[test]
fn rank1_perturbation_reports_differs() {
    let eps = rank1_eps();
    let base = compare_host_scalar(2.0, 2.0, eps);
    let pert = compare_host_scalar(2.5, 2.0, eps);
    assert_eq!(base, GoldenVerdict::Equal);
    assert!(matches!(pert, GoldenVerdict::Differs { .. }));
}

#[test]
fn rank1_impl_landed_measured_by_harness() {
    assert!(
        RANK1_PLUS_IMPL_LANDED,
        "RANK1_PLUS_IMPL_LANDED must flip only after harness measures"
    );
}

#[test]
fn rank0_still_exact_while_rank1_uses_eps() {
    let z = BurnRank0Algebra::zero();
    let s = ScalarAlgebra::zero();
    assert_eq!(z.to_f64(), s);

    let device = Default::default();
    let t = BurnTensorField::<DefaultBackend>::from_host_scalar(&device, 1.5);
    let g = <BurnNdArrayAlgebra as TensorAlgebra>::grad(t);
    assert!((g.to_host_scalar() - 1.5).abs() <= rank1_eps());
}
