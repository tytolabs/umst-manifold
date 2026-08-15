// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! R12-2 rank-0 commutation law witness — `BurnRank0Algebra` ≡ `ScalarAlgebra` exactly.

use umst_algebra_burn::{BurnRank0Algebra, BurnRank0Field};
use umst_cartridge_api::{ScalarAlgebra, TensorAlgebra};

/// Probe grid for rank-0 commutation (host f64).
const PROBE_GRID: &[f64] = &[
    0.0,
    1.0,
    -1.0,
    2.5,
    -3.5,
    0.1,
    4.0e-6,
    2.4e-3,
    1.2e-4,
    f64::MIN_POSITIVE,
    1.0e12,
];

#[test]
fn rank0_zero_commutes_exactly() {
    let scalar_z = ScalarAlgebra::zero();
    let burn_z = BurnRank0Algebra::zero().to_f64();
    assert_eq!(scalar_z, burn_z);
}

#[test]
fn rank0_add_commutes_on_probe_grid() {
    for &lhs in PROBE_GRID {
        for &rhs in PROBE_GRID {
            let s = lhs + rhs;
            let b = BurnRank0Algebra::add(BurnRank0Field(lhs), BurnRank0Field(rhs)).to_f64();
            assert_eq!(s, b, "add mismatch at ({lhs}, {rhs})");
        }
    }
}

#[test]
fn rank0_mul_commutes_on_probe_grid() {
    for &lhs in PROBE_GRID {
        for &rhs in PROBE_GRID {
            let s = lhs * rhs;
            let b = BurnRank0Algebra::mul(BurnRank0Field(lhs), BurnRank0Field(rhs)).to_f64();
            assert_eq!(s, b, "mul mismatch at ({lhs}, {rhs})");
        }
    }
}

#[test]
fn rank0_contract_commutes_on_probe_grid() {
    for &lhs in PROBE_GRID {
        for &rhs in PROBE_GRID {
            let s = lhs * rhs;
            let b = BurnRank0Algebra::contract(BurnRank0Field(lhs), BurnRank0Field(rhs)).to_f64();
            assert_eq!(s, b, "contract mismatch at ({lhs}, {rhs})");
        }
    }
}

#[test]
fn rank0_grad_commutes_on_probe_grid() {
    for &x in PROBE_GRID {
        let s = x;
        let b = BurnRank0Algebra::grad(BurnRank0Field(x)).to_f64();
        assert_eq!(s, b, "grad mismatch at {x}");
    }
}

#[test]
fn rank0_perturbation_lhs_diverges_under_add() {
    let base = 1.0;
    let perturbed = 1.0 + f64::EPSILON;
    let r_base = BurnRank0Algebra::add(BurnRank0Field(base), BurnRank0Field(0.0)).to_f64();
    let r_pert = BurnRank0Algebra::add(BurnRank0Field(perturbed), BurnRank0Field(0.0)).to_f64();
    assert_ne!(
        r_base, r_pert,
        "perturbation test must diverge — tautology refused"
    );
}

#[test]
fn rank0_perturbation_rhs_diverges_under_mul() {
    let lhs = 2.0;
    let rhs_base = 3.0;
    let rhs_pert = 3.5;
    let r_base = BurnRank0Algebra::mul(BurnRank0Field(lhs), BurnRank0Field(rhs_base)).to_f64();
    let r_pert = BurnRank0Algebra::mul(BurnRank0Field(lhs), BurnRank0Field(rhs_pert)).to_f64();
    assert_ne!(
        r_base, r_pert,
        "perturbation test must diverge — tautology refused"
    );
}

#[test]
fn rank0_scalar_bridge_roundtrip_is_exact() {
    for &value in PROBE_GRID {
        let field = BurnRank0Algebra::f64_to_field(value).expect("rank-0 bridge");
        let back = BurnRank0Algebra::field_to_f64(&field).expect("rank-0 bridge");
        assert_eq!(value, back);
    }
}
