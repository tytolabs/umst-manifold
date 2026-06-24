// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

use burn::tensor::{backend::Backend, Tensor};

use super::dec_typestate::{B1Incidence, DecTypestateError, ScalarChannelSelector};

/// Homogeneous material composition carrier (0D/1D batching): phase fractions / recipe columns.
pub struct MaterialCompositionTensor<B: Backend> {
    pub fractions: Tensor<B, 2>, // [Batch, Features]
}

use std::marker::PhantomData;

/// An empty trait representing a formal proof witness.
pub trait Proof: Send + Sync + 'static {}
pub struct ClausiusDuhemProof;
impl Proof for ClausiusDuhemProof {}

/// The Ultimate Modular and Extensible Tensor State (UMST)
/// A Proof-Carrying, E(3)-Equivariant Topological Manifold.
#[derive(Clone)]
pub struct UnifiedMaterialStateTensor<B: Backend> {
    // --- 1. The Domain: 4D Spacetime Sparse Coordinates ---
    /// Shape: [N_active_voxels, 5] -> (Batch, Time_Global, X, Y, Z)
    pub coords: Tensor<B, 2, burn::tensor::Int>,

    // --- 2. The Topology: Cellular Sheaf (Discrete Exterior Calculus) ---
    /// B1 Boundary Matrix: Nodes to Edges (1-cells). Required for gradients (flow).
    pub edges_b1: Tensor<B, 2, burn::tensor::Int>,
    /// B2 Boundary Matrix: Edges to Faces (2-cells). Required for curl (vorticity/stress).
    /// **Layout:** shape `[2, K]` — row `0` = global edge index, row `1` = signed incidence `±1`
    /// per column; partition columns into faces via
    /// [`crate::physics::dec_primal::primal_d1_edge_flux_to_faces`]. Many 1-D call sites still use a
    /// placeholder (`[2, 1]`) until 2-cells exist.
    pub faces_b2: Tensor<B, 2, burn::tensor::Int>,

    // --- 3. The Features: E(3)-Equivariant Property Spaces ---
    /// 0th-Order Tensors (Scalars): Temperature, Porosity, Age, Dignity Score.
    pub scalar_features: Tensor<B, 2>, // [N, F_scalars]
    /// 1st-Order Tensors (Vectors): Heat Flux, Velocity, Deformation gradients.
    pub vector_features: Tensor<B, 3>, // [N, F_vectors, 3]
    /// 2nd-Order Tensors (Matrices): Cauchy Stress Tensor, Strain Tensor.
    pub matrix_features: Tensor<B, 4>, // [N, F_matrices, 3, 3]

    pub resolution_mm: [f32; 3],

    /// Optional embedding of each active node in **world-space SI metres** (`[N, 3]`).
    pub node_positions: Option<Tensor<B, 2>>,

    /// Per-node displacement BC: `1.0` = free, `0.0` = fixed. Common layouts: `[1, N, 3]`, `[N, 3, 1]`,
    /// or `[N, 1, 3]` (flattened to `[N, 3]` inside [`crate::physics::solvers::ThmcSolver::step`]).
    pub displacement_bc_mask: Tensor<B, 3>,

    /// `1.0` where mix/topology edits are allowed. Shape `[N, 1]`.
    pub policy_editable_mask: Tensor<B, 2>,
    /// Optional BLAKE3-style-capable digest slot for wiring a runtime material catalog/schema witness.
    /// Only consulted when **`formal-witness`** is enabled and both UMST + gateway sides supply `Some(..)`.
    #[cfg(feature = "formal-witness")]
    pub catalog_schema_digest: Option<[u8; 32]>,
}

impl<B: Backend> UnifiedMaterialStateTensor<B> {
    /// Validate [`Self::edges_b1`] as oriented primal **B₁** incidence (`[2, E]`).
    #[inline]
    pub fn try_b1_incidence(&self) -> Result<B1Incidence<B>, DecTypestateError> {
        B1Incidence::try_new(self.edges_b1.clone())
    }

    /// Cold DEC staging pre-check: assemble [`super::dec_typestate::VerifiedUMST`] from live fields.
    ///
    /// Distinct from proof-carrying [`VerifiedUMST<B, P>`] on the gateway hot path.
    #[inline]
    pub fn try_as_verified_dec_bundle(
        &self,
        channel: usize,
    ) -> Result<super::dec_typestate::VerifiedUMST<B>, DecTypestateError> {
        super::dec_typestate::VerifiedUMST::try_assemble(
            self.edges_b1.clone(),
            self.scalar_features.dims()[1],
            channel,
        )
    }

    /// Blend full `proposed_scalar_features` (`[N, F]`) toward the current [`Self::scalar_features`]
    /// using [`Self::policy_editable_mask`] broadcast over all scalar channels:
    /// `result = proposed * m + original * (1 - m)` (same rule as [`Self::project_scalar_channel`]).
    pub fn apply_policy_mask(&self, proposed_scalar_features: Tensor<B, 2>) -> Tensor<B, 2> {
        let dims = self.scalar_features.dims();
        let n = dims[0];
        let f = dims[1];
        debug_assert_eq!(
            proposed_scalar_features.dims(),
            dims,
            "apply_policy_mask: proposed shape must match scalar_features [N, F]"
        );
        let m = self
            .policy_editable_mask
            .clone()
            .reshape([n, 1])
            .expand([n, f]);
        let one = Tensor::<B, 2>::ones_like(&m);
        proposed_scalar_features
            .mul(m.clone())
            .add(self.scalar_features.clone().mul(one.sub(m)))
    }

    /// Bulk scalar projection over every channel at once (same tensor blend as [`Self::apply_policy_mask`]).
    pub fn project_all_scalars(&self, proposed_scalar_features: Tensor<B, 2>) -> Tensor<B, 2> {
        self.apply_policy_mask(proposed_scalar_features)
    }

    /// Blend `proposed` (`[N, 1]`) into scalar column `channel` using [`Self::policy_editable_mask`].
    pub fn project_scalar_channel<C: ScalarChannelSelector>(
        &self,
        channel: C,
        proposed: Tensor<B, 2>,
    ) -> Tensor<B, 2> {
        let channel = channel.scalar_channel_index();
        let n = self.scalar_features.dims()[0];
        let old = self
            .scalar_features
            .clone()
            .slice([0..n, channel..channel + 1]);
        let m = self.policy_editable_mask.clone().reshape([n, 1]);
        let one = Tensor::<B, 2>::ones_like(&m);
        proposed.mul(m.clone()).add(old.mul(one.sub(m)))
    }

    /// Pin [`Self::catalog_schema_digest`] to compiled `catalog.lock.json` (formal-witness lane).
    #[cfg(feature = "formal-witness")]
    #[must_use]
    pub fn with_lock_catalog_schema_digest(mut self) -> Self {
        self.catalog_schema_digest =
            Some(crate::runtime::catalog::lock_upstream_catalog_digest_bytes());
        self
    }

    /// Replace column `channel` of `scalar_features` with `col` (`[N, 1]`).
    pub fn write_scalar_channel<C: ScalarChannelSelector>(
        &mut self,
        channel: C,
        col: Tensor<B, 2>,
    ) {
        let channel = channel.scalar_channel_index();
        let n = self.scalar_features.dims()[0];
        let f = self.scalar_features.dims()[1];
        assert!(
            channel < f,
            "write_scalar_channel: channel {channel} out of range for F={f}"
        );
        let before = self.scalar_features.clone().slice([0..n, 0..channel]);
        self.scalar_features = if channel + 1 >= f {
            Tensor::cat(vec![before, col], 1)
        } else {
            let after = self.scalar_features.clone().slice([0..n, channel + 1..f]);
            Tensor::cat(vec![before, col, after], 1)
        };
    }
}

/// The Mathematically Secured Tensor State.
/// Structurally impossible to use in downstream systems unless the Physics Gate has validated it.
pub struct VerifiedUMST<B: Backend, P: Proof> {
    pub state: UnifiedMaterialStateTensor<B>,
    _witness: PhantomData<P>,
}

impl<B: Backend, P: Proof> VerifiedUMST<B, P> {
    /// Only the physics gateway (Kleisli Arrow) can construct this.
    pub(crate) fn new(state: UnifiedMaterialStateTensor<B>) -> Self {
        Self {
            state,
            _witness: PhantomData,
        }
    }

    /// Explicit morphism: DEC staging witness → proof-carrying gateway bundle.
    pub(crate) fn lift_after_dec_staging_witness(
        _staging: super::dec_typestate::VerifiedUMST<B>,
        state: UnifiedMaterialStateTensor<B>,
    ) -> Self {
        Self::new(state)
    }
}
