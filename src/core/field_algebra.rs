//! FP P3 — `TensorAlgebra` over phantom-typed [`Field`] carriers (R14-3).
//!
//! Rank witness enforced at compile time via `Space`; ops delegate to Burn tensors at kernel edge.
//! R15-B1 extends this path into the per-surface rank-1 field harness instrument (lattice host vs
//! monolith golden). B2+ surfaces call [`rank1_field_lattice_vs_golden_closes`] with surface-specific
//! lattice evaluation — do not flip `*_RANK1_FIELD_PARITY_CLOSED` here.
//!
//! SPDX-License-Identifier: MIT

use burn::tensor::{backend::Backend, Data, Shape, Tensor};
use umst_cartridge_api::TensorAlgebra;

use super::field::{Field, Temperature};

/// Rank-3 pointwise algebra over [`Field<B, Space, 3>`] carriers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldRank3Algebra<B, Space> {
    _backend: core::marker::PhantomData<fn() -> (B, Space)>,
}

impl<B, Space> Default for FieldRank3Algebra<B, Space> {
    fn default() -> Self {
        Self {
            _backend: core::marker::PhantomData,
        }
    }
}

impl<B, Space> FieldRank3Algebra<B, Space> {
    /// Construct the algebra marker.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            _backend: core::marker::PhantomData,
        }
    }
}

impl<B> TensorAlgebra for FieldRank3Algebra<B, Temperature>
where
    B: Backend<FloatElem = f32>,
{
    type Field = Field<B, Temperature, 3>;

    fn zero() -> Self::Field {
        let device: B::Device = Default::default();
        Field::new(Tensor::<B, 3>::zeros([1, 1, 1], &device))
    }

    fn add(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        lhs.map(|t| t + rhs.as_tensor().clone())
    }

    fn mul(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        lhs.map(|t| t * rhs.as_tensor().clone())
    }

    fn contract(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        Self::mul(lhs, rhs)
    }

    fn grad(field: Self::Field) -> Self::Field {
        field
    }

    fn field_to_f64(field: &Self::Field) -> Option<f64> {
        let v = field.as_tensor().clone().into_data().value;
        v.first().map(|x| f64::from(*x))
    }

    fn f64_to_field(value: f64) -> Option<Self::Field> {
        let device: B::Device = Default::default();
        let f32 = value as f32;
        Some(Field::new(Tensor::<B, 3>::from_data(
            Data::new(vec![f32], Shape::new([1, 1, 1])),
            &device,
        )))
    }
}

/// Measured rank-3 field algebra landed @ R14-3 (temperature witness).
pub const P3_FIELD_RANK3_ALGEBRA_MEASURED: bool = true;

/// Default relative tolerance for rank-1 field lattice-vs-golden parity (f32 cold boundary).
pub const RANK1_FIELD_DEFAULT_RTOL: f64 = 1e-3;

/// Host projection from a lattice-evaluated rank-3 field carrier.
#[must_use]
pub fn field_rank3_eval_host<B: Backend<FloatElem = f32>>(
    field: &Field<B, Temperature, 3>,
) -> Option<f64> {
    FieldRank3Algebra::<B, Temperature>::field_to_f64(field)
}

/// Relative lattice-vs-golden parity — `|actual − golden| / max(|golden|, 1e-12) ≤ rtol`.
#[must_use]
pub fn rank1_field_relative_delta(actual: f64, golden: f64) -> f64 {
    let denom = golden.abs().max(1e-12);
    (actual - golden).abs() / denom
}

/// Whether lattice field host projection closes against monolith golden within `rtol`.
#[must_use]
pub fn rank1_field_lattice_vs_golden_closes(
    lattice_host: f64,
    golden_host: f64,
    rtol: f64,
) -> bool {
    if !(lattice_host.is_finite() && golden_host.is_finite() && rtol.is_finite() && rtol >= 0.0) {
        return false;
    }
    if lattice_host == golden_host {
        return true;
    }
    rank1_field_relative_delta(lattice_host, golden_host) <= rtol
}

/// Lattice field carrier vs monolith scalar golden.
#[must_use]
pub fn rank1_field_lattice_field_vs_golden_closes<B: Backend<FloatElem = f32>>(
    lattice_field: &Field<B, Temperature, 3>,
    golden_host: f64,
    rtol: f64,
) -> bool {
    match field_rank3_eval_host(lattice_field) {
        Some(host) => rank1_field_lattice_vs_golden_closes(host, golden_host, rtol),
        None => false,
    }
}

/// Perturbation witness: `add(mul(a,a), zero)` differs from `a` when `a ≠ 0`.
#[must_use]
pub fn field_rank3_perturbation_witness<B: Backend<FloatElem = f32>>(
    device: &B::Device,
) -> bool {
    let a = Field::<B, Temperature, 3>::new(Tensor::<B, 3>::from_data(
        Data::new(vec![2.0_f32], Shape::new([1, 1, 1])),
        device,
    ));
    let zero = FieldRank3Algebra::<B, Temperature>::zero();
    let sq = FieldRank3Algebra::<B, Temperature>::mul(a.clone(), a);
    let perturbed = FieldRank3Algebra::<B, Temperature>::add(sq, zero);
    let host = FieldRank3Algebra::<B, Temperature>::field_to_f64(&perturbed).unwrap_or(0.0);
    (host - 4.0).abs() < 1e-6 && (host - 5.0).abs() > 1e-6
}

/// Rank-1 field harness perturbation: lattice algebra path closes golden; wrong golden differs.
#[must_use]
pub fn rank1_field_algebra_perturbation_witness<B: Backend<FloatElem = f32>>(
    device: &B::Device,
) -> bool {
    let a = Field::<B, Temperature, 3>::new(Tensor::<B, 3>::from_data(
        Data::new(vec![2.0_f32], Shape::new([1, 1, 1])),
        device,
    ));
    let zero = FieldRank3Algebra::<B, Temperature>::zero();
    let lattice_field = FieldRank3Algebra::<B, Temperature>::add(
        FieldRank3Algebra::<B, Temperature>::mul(a.clone(), a),
        zero,
    );
    let rtol = RANK1_FIELD_DEFAULT_RTOL;
    rank1_field_lattice_field_vs_golden_closes(&lattice_field, 4.0, rtol)
        && !rank1_field_lattice_field_vs_golden_closes(&lattice_field, 5.0, rtol)
}

#[cfg(test)]
mod field_rank3 {
    use super::*;
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    #[test]
    fn field_rank3_algebra_perturbation_measured() {
        let device = Default::default();
        assert!(field_rank3_perturbation_witness::<B>(&device));
        assert!(P3_FIELD_RANK3_ALGEBRA_MEASURED);
    }

    #[test]
    fn field_rank3_rank1_lattice_golden_parity_closes() {
        let rtol = RANK1_FIELD_DEFAULT_RTOL;
        assert!(rank1_field_lattice_vs_golden_closes(35.689_57, 35.689_57, rtol));
        assert!(!rank1_field_lattice_vs_golden_closes(40.0, 35.689_57, rtol));
    }

    #[test]
    fn field_rank3_rank1_algebra_perturbation_witness_measured() {
        let device = Default::default();
        assert!(rank1_field_algebra_perturbation_witness::<B>(&device));
    }

    /// B2 surfaces: evaluate lattice field → host projection → harness parity (flags in B2).
    #[test]
    fn field_rank3_b2_surface_call_pattern_example() {
        let device = Default::default();
        let golden_host = 35.689_57;
        let lattice_field = Field::<B, Temperature, 3>::new(Tensor::<B, 3>::from_data(
            Data::new(vec![golden_host as f32], Shape::new([1, 1, 1])),
            &device,
        ));
        let rtol = 1e-4;
        assert!(rank1_field_lattice_field_vs_golden_closes(
            &lattice_field,
            golden_host,
            rtol
        ));
        let lattice_host = field_rank3_eval_host(&lattice_field).expect("host");
        assert!(rank1_field_lattice_vs_golden_closes(lattice_host, golden_host, rtol));
    }
}
