// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Rank-0 `TensorAlgebra` — exact `f64` commutation with [`ScalarAlgebra`](umst_cartridge_api::ScalarAlgebra).

use umst_cartridge_api::TensorAlgebra;

/// Slice identifier for rank-0 exact path.
pub const RANK0_SLICE_ID: &str = "rank-0-exact";

/// Exact rank-0 field carrier — `#[repr(transparent)]` over host `f64`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BurnRank0Field(pub f64);

impl BurnRank0Field {
    /// Lift host scalar.
    #[must_use]
    pub const fn from_f64(value: f64) -> Self {
        Self(value)
    }

    /// Project to host scalar.
    #[must_use]
    pub const fn to_f64(self) -> f64 {
        self.0
    }
}

/// Rank-0 Burn algebra — exact `f64` semantics matching `ScalarAlgebra`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BurnRank0Algebra;

impl TensorAlgebra for BurnRank0Algebra {
    type Field = BurnRank0Field;

    fn zero() -> Self::Field {
        BurnRank0Field(0.0)
    }

    fn add(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        BurnRank0Field(lhs.0 + rhs.0)
    }

    fn mul(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        BurnRank0Field(lhs.0 * rhs.0)
    }

    fn contract(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        BurnRank0Field(lhs.0 * rhs.0)
    }

    fn grad(field: Self::Field) -> Self::Field {
        field
    }

    fn field_to_f64(field: &Self::Field) -> Option<f64> {
        Some(field.0)
    }

    fn f64_to_field(value: f64) -> Option<Self::Field> {
        Some(BurnRank0Field(value))
    }
}

#[cfg(test)]
mod tests {
    use umst_cartridge_api::ScalarAlgebra;

    use super::*;

    #[test]
    fn rank0_field_is_f64_sized() {
        assert_eq!(
            core::mem::size_of::<BurnRank0Field>(),
            core::mem::size_of::<f64>()
        );
    }

    #[test]
    fn rank0_ops_match_scalar_algebra_at_probes() {
        const PROBES: &[(f64, f64)] = &[(0.0, 0.0), (1.0, 2.5), (-3.5, 0.1), (4.0e-6, 2.4e-3)];

        for &(lhs, rhs) in PROBES {
            let s_sum = ScalarAlgebra::add(lhs, rhs);
            let b_sum = BurnRank0Algebra::add(BurnRank0Field(lhs), BurnRank0Field(rhs)).to_f64();
            assert_eq!(s_sum, b_sum);

            let s_mul = ScalarAlgebra::mul(lhs, rhs);
            let b_mul = BurnRank0Algebra::mul(BurnRank0Field(lhs), BurnRank0Field(rhs)).to_f64();
            assert_eq!(s_mul, b_mul);

            let s_contract = ScalarAlgebra::contract(lhs, rhs);
            let b_contract =
                BurnRank0Algebra::contract(BurnRank0Field(lhs), BurnRank0Field(rhs)).to_f64();
            assert_eq!(s_contract, b_contract);

            let s_grad = ScalarAlgebra::grad(lhs);
            let b_grad = BurnRank0Algebra::grad(BurnRank0Field(lhs)).to_f64();
            assert_eq!(s_grad, b_grad);
        }

        assert_eq!(ScalarAlgebra::zero(), BurnRank0Algebra::zero().to_f64());
    }
}
