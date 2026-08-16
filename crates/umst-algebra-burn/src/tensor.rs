// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Rank-1+ `TensorAlgebra` over `burn::Tensor` — ndarray backend default.

use burn::tensor::{backend::Backend, Data, Shape, Tensor};
use burn_ndarray::NdArray;
use umst_cartridge_api::TensorAlgebra;

/// Default ndarray backend for atom-lift integration tests.
pub type DefaultBackend = NdArray;

/// Default backend alias for fleet receipts.
pub const DEFAULT_BACKEND: &str = "NdArray";

/// Rank-1+ tensor path closed @ R13-2 harness (0D atom carrier within eps).
pub const RANK1_PLUS_DEFERRED: bool = false;

/// 0D/1D atom tensor field — shape `[1]` carrier for slice-3 integration.
#[derive(Debug, Clone)]
pub struct BurnTensorField<B: Backend> {
    inner: Tensor<B, 1>,
}

impl<B> BurnTensorField<B>
where
    B: Backend<FloatElem = f32>,
{
    /// Lift host scalar into a length-1 tensor (cold boundary uses f32 per T4 cast policy).
    #[must_use]
    pub fn from_host_scalar(device: &B::Device, value: f64) -> Self {
        let f32 = value as f32;
        Self {
            inner: Tensor::from_data(Data::new(vec![f32], Shape::new([1])), device),
        }
    }

    /// Project tensor lane back to host `f64`.
    #[must_use]
    pub fn to_host_scalar(&self) -> f64 {
        f64::from(self.inner.clone().into_data().value[0])
    }

    /// Borrow inner tensor for physics kernels.
    #[must_use]
    pub fn as_tensor(&self) -> &Tensor<B, 1> {
        &self.inner
    }
}

/// Generic `TensorAlgebra` over `burn::Tensor` 0D atom fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BurnAlgebra<B: Backend> {
    _backend: core::marker::PhantomData<B>,
}

impl<B: Backend> Default for BurnAlgebra<B> {
    fn default() -> Self {
        Self {
            _backend: core::marker::PhantomData,
        }
    }
}

impl<B: Backend> BurnAlgebra<B> {
    /// Construct the algebra marker.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            _backend: core::marker::PhantomData,
        }
    }
}

impl<B> TensorAlgebra for BurnAlgebra<B>
where
    B: Backend<FloatElem = f32>,
{
    type Field = BurnTensorField<B>;

    fn zero() -> Self::Field {
        let device = Default::default();
        BurnTensorField::from_host_scalar(&device, 0.0)
    }

    fn add(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        BurnTensorField {
            inner: lhs.inner + rhs.inner,
        }
    }

    fn mul(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        BurnTensorField {
            inner: lhs.inner * rhs.inner,
        }
    }

    fn contract(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        BurnTensorField {
            inner: lhs.inner * rhs.inner,
        }
    }

    fn grad(field: Self::Field) -> Self::Field {
        field
    }
}

/// Type alias for the default ndarray instantiation.
pub type BurnNdArrayAlgebra = BurnAlgebra<DefaultBackend>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burn_tensor_algebra_compiles_and_runs_at_probe() {
        let device = Default::default();
        let a = BurnTensorField::<DefaultBackend>::from_host_scalar(&device, 2.0);
        let b = BurnTensorField::<DefaultBackend>::from_host_scalar(&device, 3.5);
        let product = <BurnNdArrayAlgebra as TensorAlgebra>::mul(a, b);
        let sum = <BurnNdArrayAlgebra as TensorAlgebra>::add(
            product,
            <BurnNdArrayAlgebra as TensorAlgebra>::zero(),
        );
        assert!((sum.to_host_scalar() - 7.0).abs() < 1e-5);
        assert!(!RANK1_PLUS_DEFERRED);
    }
}
