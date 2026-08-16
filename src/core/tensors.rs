// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! UMST tensor carrier — layout contract, policy masks, and gateway proof bundle.
//!
//! ## Honest fences (W29-030)
//!
//! - [`umst_tensors_production_wired`] and [`umst_tensors_master_retick_eligible`] stay **false**
//!   until full 2-cell DEC + gateway hardening lands.
//! - [`umst_tensors_b2_cells_landed`] stays **false** — `[2, 1]` `faces_b2` is staging placeholder.
//! - [`VerifiedUMST`] is **gateway-internal** — not a physics GREEN / MASTER retick witness.
//! - [`UnifiedMaterialStateTensor::faces_b2`] placeholders (`[2, 1]`) are staging-only;
//!   photonics / curl legs remain partial.

use std::marker::PhantomData;

use burn::tensor::{backend::Backend, Tensor};

use super::dec_typestate::{
    B1Incidence, DecTypestateError, ScalarChannelIdx, ScalarChannelSelector,
};
use super::field::{DamageField, Field, HumidityField, TemperatureField};
use super::umst_schema::{
    SCALAR_DAMAGE, SCALAR_HUMIDITY, SCALAR_TEMPERATURE, UMST_SCALAR_CHANNEL_COUNT,
};

/// W29 deepen cell id — carrier / gateway-proof slice only.
pub const UMST_TENSORS_CELL_ID: &str = "W29-030-TENSORS";

/// Operator-visible honesty string — does **not** authorize production flip or MASTER retick.
pub const UMST_TENSORS_HONEST_FENCE: &str =
    "umst_tensors_scaffold=true b2_cells_landed=false production_wired=false master_retick_eligible=false";

/// Staging `faces_b2` column count used by 1-D / two-node harnesses (`[2, 1]`).
pub const FACES_B2_STAGING_PLACEHOLDER_COLS: usize = 1;

/// Staging taxonomy for [`UnifiedMaterialStateTensor::faces_b2`] — never invents B₂ landed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FacesB2StagingKind {
    /// Canonical two-node placeholder `[2, 1]`.
    PlaceholderCols1,
    /// Multi-column `[2, K]` with `K > 1` still staging — **not** a B₂ landing claim.
    MultiColumnUnlanded { cols: usize },
    /// Row count ≠ 2 — layout-invalid for the carrier contract.
    InvalidRowCount { rows: usize },
}

/// Production gate flip — honest **false** until 2-cell DEC + gateway hardening measure.
#[must_use]
pub const fn umst_tensors_production_wired() -> bool {
    false
}

/// MASTER retick eligibility — honest **false**; gateway proof bundle ≠ physics GREEN.
#[must_use]
pub const fn umst_tensors_master_retick_eligible() -> bool {
    false
}

/// B₂ / 2-cell incidence on the live carrier — honest **false** until non-placeholder `faces_b2`.
#[must_use]
pub const fn umst_tensors_b2_cells_landed() -> bool {
    false
}

const _: () = assert!(!umst_tensors_production_wired());
const _: () = assert!(!umst_tensors_master_retick_eligible());
const _: () = assert!(!umst_tensors_b2_cells_landed());

/// Layout disagreements across nodal tensor banks on a live UMST bundle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UmstTensorLayoutError {
    /// `coords` row count must match `scalar_features` node axis.
    CoordsNodeMismatch { coords_n: usize, scalar_n: usize },
    /// `policy_editable_mask` node axis must match `scalar_features`.
    PolicyMaskMismatch { mask_n: usize, scalar_n: usize },
    /// `vector_features` node axis must match `scalar_features`.
    VectorNodeMismatch { vector_n: usize, scalar_n: usize },
    /// `matrix_features` node axis must match `scalar_features`.
    MatrixNodeMismatch { matrix_n: usize, scalar_n: usize },
    /// Optional `node_positions` row count must match active node count.
    NodePositionsMismatch { pos_n: usize, scalar_n: usize },
    /// `faces_b2` must remain `[2, K]` (placeholder columns allowed).
    FacesB2WrongRowCount { rows: usize },
}

/// W29-030 deepen census — prep wired; production / MASTER / B₂ claims blocked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UmstTensorsDeepenProbe {
    pub cell_id: &'static str,
    pub honest_fence: &'static str,
    pub b1_typestate_wired: bool,
    pub layout_validate_wired: bool,
    pub typed_scalar_views_wired: bool,
    pub policy_mask_projection_wired: bool,
    pub project_all_scalars_wired: bool,
    pub faces_b2_placeholder_detect_wired: bool,
    pub faces_b2_multi_col_unlanded_wired: bool,
    pub write_scalar_channel_wired: bool,
    pub verified_umst_gateway_only: bool,
    pub b2_cells_landed: bool,
    pub production_wired: bool,
    pub master_retick_eligible: bool,
}

/// Honest deepen probe — surfaces wired prep without inventing GREEN.
#[must_use]
pub fn umst_tensors_deepen_probe() -> UmstTensorsDeepenProbe {
    UmstTensorsDeepenProbe {
        cell_id: UMST_TENSORS_CELL_ID,
        honest_fence: UMST_TENSORS_HONEST_FENCE,
        b1_typestate_wired: true,
        layout_validate_wired: true,
        typed_scalar_views_wired: true,
        policy_mask_projection_wired: true,
        project_all_scalars_wired: true,
        faces_b2_placeholder_detect_wired: true,
        faces_b2_multi_col_unlanded_wired: true,
        write_scalar_channel_wired: true,
        verified_umst_gateway_only: true,
        b2_cells_landed: umst_tensors_b2_cells_landed(),
        production_wired: umst_tensors_production_wired(),
        master_retick_eligible: umst_tensors_master_retick_eligible(),
    }
}

/// Honesty gate for operator receipts — prep only, no production / MASTER / B₂ flip.
#[must_use]
pub fn umst_tensors_deepen_honest(probe: &UmstTensorsDeepenProbe) -> bool {
    probe.cell_id == UMST_TENSORS_CELL_ID
        && probe.honest_fence == UMST_TENSORS_HONEST_FENCE
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("master_retick_eligible=false")
        && probe.honest_fence.contains("b2_cells_landed=false")
        && probe.b1_typestate_wired
        && probe.layout_validate_wired
        && probe.typed_scalar_views_wired
        && probe.policy_mask_projection_wired
        && probe.project_all_scalars_wired
        && probe.faces_b2_placeholder_detect_wired
        && probe.faces_b2_multi_col_unlanded_wired
        && probe.write_scalar_channel_wired
        && probe.verified_umst_gateway_only
        && !probe.b2_cells_landed
        && !probe.production_wired
        && !probe.master_retick_eligible
}

/// Fail-closed honesty check — rejects any production / MASTER / B₂ invention.
#[must_use]
pub fn validate_umst_tensors_deepen_honesty() -> Result<(), &'static str> {
    if umst_tensors_production_wired() {
        return Err(
            "umst_tensors_production_wired must stay false until 2-cell DEC + gateway harden",
        );
    }
    if umst_tensors_master_retick_eligible() {
        return Err(
            "umst_tensors_master_retick_eligible must stay false (gateway ≠ physics GREEN)",
        );
    }
    if umst_tensors_b2_cells_landed() {
        return Err("umst_tensors_b2_cells_landed must stay false while faces_b2 is placeholder");
    }
    let probe = umst_tensors_deepen_probe();
    if !umst_tensors_deepen_honest(&probe) {
        return Err("umst_tensors_deepen_probe failed honesty gate");
    }
    Ok(())
}

/// Homogeneous material composition carrier (0D/1D batching): phase fractions / recipe columns.
pub struct MaterialCompositionTensor<B: Backend> {
    pub fractions: Tensor<B, 2>, // [Batch, Features]
}

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

/// Read-only typed scalar channel bundle lifted from [`UnifiedMaterialStateTensor::scalar_features`].
///
/// Each field is a **copy** in rank-3 plan layout `[1, N, 1]` (inverse of
/// [`crate::physics::thmc_umst_sync::sync_thmc_to_umst`]).
#[derive(Clone, Debug)]
pub struct UmstTypedScalarViews<B: Backend> {
    pub temperature: TemperatureField<B>,
    pub humidity: HumidityField<B>,
    pub damage: DamageField<B>,
}

impl<B: Backend> UnifiedMaterialStateTensor<B> {
    /// Active nodal count `N` from [`Self::scalar_features`] (`[N, F]`).
    #[inline]
    #[must_use]
    pub fn active_node_count(&self) -> usize {
        self.scalar_features.dims()[0]
    }

    /// Scalar channel width `F` from [`Self::scalar_features`].
    #[inline]
    #[must_use]
    pub fn n_scalar_channels(&self) -> usize {
        self.scalar_features.dims()[1]
    }

    /// Column count `K` of [`Self::faces_b2`] (`[2, K]`).
    #[inline]
    #[must_use]
    pub fn faces_b2_column_count(&self) -> usize {
        self.faces_b2.dims()[1]
    }

    /// True when `faces_b2` is the staging placeholder `[2, 1]` — **not** a B₂ landing claim.
    ///
    /// Does **not** flip [`umst_tensors_b2_cells_landed`]; multi-column placeholders still leave
    /// B₂ unlanded until a measured 2-cell DEC witness exists.
    #[inline]
    #[must_use]
    pub fn faces_b2_is_staging_placeholder(&self) -> bool {
        matches!(
            self.faces_b2_staging_kind(),
            FacesB2StagingKind::PlaceholderCols1
        )
    }

    /// Classify `faces_b2` staging shape — multi-column still leaves B₂ unlanded.
    #[inline]
    #[must_use]
    pub fn faces_b2_staging_kind(&self) -> FacesB2StagingKind {
        let rows = self.faces_b2.dims()[0];
        let cols = self.faces_b2_column_count();
        if rows != 2 {
            return FacesB2StagingKind::InvalidRowCount { rows };
        }
        if cols == FACES_B2_STAGING_PLACEHOLDER_COLS {
            FacesB2StagingKind::PlaceholderCols1
        } else {
            FacesB2StagingKind::MultiColumnUnlanded { cols }
        }
    }

    /// Cross-bank nodal layout check — does **not** assert physics GREEN.
    pub fn try_validate_layout(&self) -> Result<(), UmstTensorLayoutError> {
        let n = self.active_node_count();
        let coords_n = self.coords.dims()[0];
        if coords_n != n {
            return Err(UmstTensorLayoutError::CoordsNodeMismatch {
                coords_n,
                scalar_n: n,
            });
        }
        let mask_n = self.policy_editable_mask.dims()[0];
        if mask_n != n {
            return Err(UmstTensorLayoutError::PolicyMaskMismatch {
                mask_n,
                scalar_n: n,
            });
        }
        let vector_n = self.vector_features.dims()[0];
        if vector_n != n {
            return Err(UmstTensorLayoutError::VectorNodeMismatch {
                vector_n,
                scalar_n: n,
            });
        }
        let matrix_n = self.matrix_features.dims()[0];
        if matrix_n != n {
            return Err(UmstTensorLayoutError::MatrixNodeMismatch {
                matrix_n,
                scalar_n: n,
            });
        }
        if let Some(ref pos) = self.node_positions {
            let pos_n = pos.dims()[0];
            if pos_n != n {
                return Err(UmstTensorLayoutError::NodePositionsMismatch { pos_n, scalar_n: n });
            }
        }
        let b2_rows = self.faces_b2.dims()[0];
        if b2_rows != 2 {
            return Err(UmstTensorLayoutError::FacesB2WrongRowCount { rows: b2_rows });
        }
        Ok(())
    }

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

    /// Lift a nodal scalar column into a rank-3 plan-field copy `[1, N, 1]`.
    ///
    /// Burn's ownership model does not allow borrowing sub-columns; views are **copies**
    /// (see FP P3.6 migration plan).
    fn scalar_channel_plan_field<Space>(
        &self,
        channel: usize,
    ) -> Result<Field<B, Space, 3>, DecTypestateError> {
        let _ = ScalarChannelIdx::try_new(channel)?;
        let n = self.scalar_features.dims()[0];
        let f = self.scalar_features.dims()[1];
        if channel >= f {
            return Err(DecTypestateError::ScalarChannelOutOfRange {
                index: channel,
                channel_count: f,
            });
        }
        let col = self
            .scalar_features
            .clone()
            .slice([0..n, channel..channel + 1]);
        Ok(Field::new(col.reshape([1, n, 1])))
    }

    /// Read-only typed scalar channel views from [`Self::scalar_features`].
    ///
    /// Returns temperature, humidity, and damage plan fields at batch `B = 1` with shape
    /// `[1, N, 1]`, matching [`crate::physics::solvers::ThmcState`] layout contracts.
    pub fn typed_views(&self) -> Result<UmstTypedScalarViews<B>, DecTypestateError> {
        Ok(UmstTypedScalarViews {
            temperature: self.temperature_scalar_channel()?,
            humidity: self.humidity_scalar_channel()?,
            damage: self.damage_scalar_channel()?,
        })
    }

    /// Convenience shim: damage column as [`DamageField`].
    pub fn damage_scalar_channel(&self) -> Result<DamageField<B>, DecTypestateError> {
        self.scalar_channel_plan_field::<super::field::Damage>(SCALAR_DAMAGE)
    }

    /// Convenience shim: humidity column as [`HumidityField`].
    pub fn humidity_scalar_channel(&self) -> Result<HumidityField<B>, DecTypestateError> {
        self.scalar_channel_plan_field::<super::field::Humidity>(SCALAR_HUMIDITY)
    }

    /// Convenience shim: temperature column as [`TemperatureField`].
    pub fn temperature_scalar_channel(&self) -> Result<TemperatureField<B>, DecTypestateError> {
        self.scalar_channel_plan_field::<super::field::Temperature>(SCALAR_TEMPERATURE)
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

#[cfg(test)]
mod tensors_tests {
    use burn::tensor::{Data, Int, Shape};
    use burn_ndarray::{NdArray, NdArrayDevice};

    use super::*;

    type B = NdArray<f32>;

    fn device() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    /// Two-node staging UMST — same topology recipe as gate / apply_physics harnesses.
    fn staging_two_node_umst(
        scalars: Tensor<B, 2>,
        policy_mask: Tensor<B, 2>,
        node_positions: Option<Tensor<B, 2>>,
    ) -> UnifiedMaterialStateTensor<B> {
        let dev = device();
        let n = scalars.dims()[0];
        assert_eq!(scalars.dims()[1], UMST_SCALAR_CHANNEL_COUNT);
        let coords: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &dev);
        let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
            Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
            &dev,
        );
        let faces_b2: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &dev);
        UnifiedMaterialStateTensor {
            coords,
            edges_b1,
            faces_b2,
            scalar_features: scalars,
            vector_features: Tensor::<B, 3>::zeros([n, 1, 3], &dev),
            matrix_features: Tensor::<B, 4>::zeros([n, 1, 3, 3], &dev),
            resolution_mm: [1.0, 1.0, 1.0],
            node_positions,
            displacement_bc_mask: Tensor::<B, 3>::ones([1, n, 3], &dev),
            policy_editable_mask: policy_mask,
            #[cfg(feature = "formal-witness")]
            catalog_schema_digest: None,
        }
    }

    #[test]
    fn umst_tensors_honest_fence_blocks_production_and_master() {
        assert!(!umst_tensors_production_wired());
        assert!(!umst_tensors_master_retick_eligible());
        assert!(!umst_tensors_b2_cells_landed());
        assert!(UMST_TENSORS_HONEST_FENCE.contains("production_wired=false"));
        assert!(UMST_TENSORS_HONEST_FENCE.contains("master_retick_eligible=false"));
        assert!(UMST_TENSORS_HONEST_FENCE.contains("b2_cells_landed=false"));
        validate_umst_tensors_deepen_honesty().expect("tensors deepen honesty");
    }

    #[test]
    fn umst_tensors_deepen_probe_honest_not_green() {
        let probe = umst_tensors_deepen_probe();
        assert!(umst_tensors_deepen_honest(&probe));
        assert_eq!(probe.cell_id, UMST_TENSORS_CELL_ID);
        assert!(!probe.production_wired);
        assert!(!probe.master_retick_eligible);
        assert!(!probe.b2_cells_landed);
        assert!(probe.faces_b2_placeholder_detect_wired);
        assert!(probe.faces_b2_multi_col_unlanded_wired);
        assert!(probe.project_all_scalars_wired);
        assert!(probe.write_scalar_channel_wired);
    }

    #[test]
    fn umst_tensors_validate_layout_accepts_two_node_fixture() {
        let dev = device();
        let n = 2usize;
        let scalars = Tensor::<B, 2>::zeros([n, UMST_SCALAR_CHANNEL_COUNT], &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let umst = staging_two_node_umst(scalars, mask, None);
        umst.try_validate_layout().expect("two-node fixture layout");
        assert_eq!(umst.active_node_count(), n);
        assert_eq!(umst.n_scalar_channels(), UMST_SCALAR_CHANNEL_COUNT);
        assert!(umst.faces_b2_is_staging_placeholder());
        assert_eq!(
            umst.faces_b2_column_count(),
            FACES_B2_STAGING_PLACEHOLDER_COLS
        );
        // Placeholder detect ≠ B₂ landed.
        assert!(!umst_tensors_b2_cells_landed());
    }

    #[test]
    fn umst_tensors_validate_layout_rejects_coords_scalar_mismatch() {
        let dev = device();
        let n = 2usize;
        let scalars = Tensor::<B, 2>::zeros([n, UMST_SCALAR_CHANNEL_COUNT], &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let mut umst = staging_two_node_umst(scalars, mask, None);
        umst.coords = Tensor::<B, 2, Int>::zeros([n + 1, 5], &dev);
        let err = umst.try_validate_layout().unwrap_err();
        assert_eq!(
            err,
            UmstTensorLayoutError::CoordsNodeMismatch {
                coords_n: n + 1,
                scalar_n: n,
            }
        );
    }

    #[test]
    fn umst_tensors_validate_layout_rejects_policy_mask_mismatch() {
        let dev = device();
        let n = 2usize;
        let scalars = Tensor::<B, 2>::zeros([n, UMST_SCALAR_CHANNEL_COUNT], &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let mut umst = staging_two_node_umst(scalars, mask, None);
        umst.policy_editable_mask = Tensor::<B, 2>::ones([n + 1, 1], &dev);
        let err = umst.try_validate_layout().unwrap_err();
        assert_eq!(
            err,
            UmstTensorLayoutError::PolicyMaskMismatch {
                mask_n: n + 1,
                scalar_n: n,
            }
        );
    }

    #[test]
    fn umst_tensors_validate_layout_rejects_vector_matrix_mismatch() {
        let dev = device();
        let n = 2usize;
        let scalars = Tensor::<B, 2>::zeros([n, UMST_SCALAR_CHANNEL_COUNT], &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let mut umst = staging_two_node_umst(scalars, mask, None);
        umst.vector_features = Tensor::<B, 3>::zeros([n + 1, 1, 3], &dev);
        assert_eq!(
            umst.try_validate_layout().unwrap_err(),
            UmstTensorLayoutError::VectorNodeMismatch {
                vector_n: n + 1,
                scalar_n: n,
            }
        );
        umst.vector_features = Tensor::<B, 3>::zeros([n, 1, 3], &dev);
        umst.matrix_features = Tensor::<B, 4>::zeros([n + 1, 1, 3, 3], &dev);
        assert_eq!(
            umst.try_validate_layout().unwrap_err(),
            UmstTensorLayoutError::MatrixNodeMismatch {
                matrix_n: n + 1,
                scalar_n: n,
            }
        );
    }

    #[test]
    fn umst_tensors_validate_layout_rejects_faces_b2_wrong_rows() {
        let dev = device();
        let n = 2usize;
        let scalars = Tensor::<B, 2>::zeros([n, UMST_SCALAR_CHANNEL_COUNT], &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let mut umst = staging_two_node_umst(scalars, mask, None);
        umst.faces_b2 = Tensor::<B, 2, Int>::zeros([3, 1], &dev);
        let err = umst.try_validate_layout().unwrap_err();
        assert_eq!(err, UmstTensorLayoutError::FacesB2WrongRowCount { rows: 3 });
        assert!(!umst.faces_b2_is_staging_placeholder());
    }

    #[test]
    fn umst_tensors_b1_incidence_rejects_wrong_row_count() {
        let dev = device();
        let n = 2usize;
        let scalars = Tensor::<B, 2>::zeros([n, UMST_SCALAR_CHANNEL_COUNT], &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let mut umst = staging_two_node_umst(scalars, mask, None);
        umst.edges_b1 = Tensor::<B, 2, Int>::zeros([3, 2], &dev);
        let err = umst.try_b1_incidence().unwrap_err();
        assert_eq!(err, DecTypestateError::B1WrongRowCount { rows: 3 });
    }

    #[test]
    fn umst_tensors_try_as_verified_dec_bundle_accepts_damage_channel() {
        let dev = device();
        let n = 2usize;
        let scalars = Tensor::<B, 2>::zeros([n, UMST_SCALAR_CHANNEL_COUNT], &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let umst = staging_two_node_umst(scalars, mask, None);
        let staging = umst
            .try_as_verified_dec_bundle(SCALAR_DAMAGE)
            .expect("DEC staging bundle");
        assert_eq!(
            staging.channel().index(),
            SCALAR_DAMAGE,
            "staging channel must match requested damage index"
        );
        // Staging assemble ≠ production / MASTER / B₂ flip.
        assert!(!umst_tensors_production_wired());
        assert!(!umst_tensors_b2_cells_landed());
    }

    #[test]
    fn umst_tensors_policy_mask_blend_preserves_masked_nodes() {
        let dev = device();
        let n = 2usize;
        let f = UMST_SCALAR_CHANNEL_COUNT;
        let mut flat = vec![0.0_f32; n * f];
        flat[SCALAR_DAMAGE] = 0.1;
        flat[f + SCALAR_DAMAGE] = 0.2;
        let scalars = Tensor::from_data(Data::new(flat, Shape::new([n, f])), &dev);
        let mask = Tensor::from_data(Data::new(vec![1.0_f32, 0.0_f32], Shape::new([n, 1])), &dev);
        let umst = staging_two_node_umst(scalars.clone(), mask, None);

        let mut proposed = scalars.clone();
        proposed = proposed.add_scalar(0.5);
        let merged = umst.apply_policy_mask(proposed);
        let data: Vec<f32> = merged.into_data().value;
        assert!((data[SCALAR_DAMAGE] - (0.1 + 0.5)).abs() < 1e-5);
        assert!((data[f + SCALAR_DAMAGE] - 0.2).abs() < 1e-5);

        let ch_proposed =
            Tensor::from_data(Data::new(vec![0.9_f32, 0.8_f32], Shape::new([n, 1])), &dev);
        let ch_merged = umst.project_scalar_channel(SCALAR_DAMAGE, ch_proposed);
        let ch_data: Vec<f32> = ch_merged.into_data().value;
        assert!((ch_data[0] - 0.9).abs() < 1e-5);
        assert!((ch_data[1] - 0.2).abs() < 1e-5);
    }

    #[test]
    fn umst_tensors_write_scalar_channel_roundtrip_damage() {
        let dev = device();
        let n = 2usize;
        let f = UMST_SCALAR_CHANNEL_COUNT;
        let scalars = Tensor::<B, 2>::zeros([n, f], &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let mut umst = staging_two_node_umst(scalars, mask, None);
        let col = Tensor::from_data(Data::new(vec![0.3_f32, 0.7_f32], Shape::new([n, 1])), &dev);
        umst.write_scalar_channel(SCALAR_DAMAGE, col);
        let views = umst.typed_views().expect("typed views after write");
        let d: Vec<f32> = views.damage.as_tensor().clone().into_data().value;
        assert!((d[0] - 0.3).abs() < 1e-5);
        assert!((d[1] - 0.7).abs() < 1e-5);
        assert_eq!(umst.n_scalar_channels(), f);
    }

    #[test]
    fn umst_tensors_typed_views_lift_scalar_channels() {
        let dev = device();
        let n = 3usize;
        let f = UMST_SCALAR_CHANNEL_COUNT;
        let mut flat = vec![0.0_f32; n * f];
        for (i, row) in flat.chunks_mut(f).enumerate() {
            row[SCALAR_TEMPERATURE] = 300.0 + i as f32;
            row[SCALAR_HUMIDITY] = 0.5 + 0.1 * i as f32;
            row[SCALAR_DAMAGE] = 0.01 * i as f32;
        }
        let scalars = Tensor::from_data(Data::new(flat, Shape::new([n, f])), &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let umst = staging_two_node_umst(scalars, mask, None);
        let views = umst.typed_views().expect("typed scalar views");
        assert_eq!(views.temperature.as_tensor().dims(), [1, n, 1]);
        assert_eq!(views.humidity.as_tensor().dims(), [1, n, 1]);
        assert_eq!(views.damage.as_tensor().dims(), [1, n, 1]);
        let t0: Vec<f32> = views
            .temperature
            .as_tensor()
            .clone()
            .slice([0..1, 0..1, 0..1])
            .into_data()
            .value;
        assert!((t0[0] - 300.0).abs() < 1e-5);
    }

    #[test]
    fn umst_tensors_verified_umst_gateway_constructible() {
        let dev = device();
        let n = 2usize;
        let scalars = Tensor::<B, 2>::zeros([n, UMST_SCALAR_CHANNEL_COUNT], &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let umst = staging_two_node_umst(scalars, mask, None);
        let verified = VerifiedUMST::<B, ClausiusDuhemProof>::new(umst);
        assert_eq!(verified.state.active_node_count(), n);
        assert!(verified.state.try_validate_layout().is_ok());
        // Gateway constructibility ≠ MASTER / production eligibility.
        assert!(!umst_tensors_master_retick_eligible());
        assert!(!umst_tensors_production_wired());
    }

    #[test]
    fn umst_tensors_validate_layout_rejects_node_positions_mismatch() {
        let dev = device();
        let n = 2usize;
        let scalars = Tensor::<B, 2>::zeros([n, UMST_SCALAR_CHANNEL_COUNT], &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let bad_pos = Tensor::<B, 2>::zeros([n + 1, 3], &dev);
        let umst = staging_two_node_umst(scalars, mask, Some(bad_pos));
        let err = umst.try_validate_layout().unwrap_err();
        assert_eq!(
            err,
            UmstTensorLayoutError::NodePositionsMismatch {
                pos_n: n + 1,
                scalar_n: n,
            }
        );
    }

    #[test]
    fn umst_tensors_project_all_scalars_matches_policy_mask() {
        let dev = device();
        let n = 2usize;
        let f = UMST_SCALAR_CHANNEL_COUNT;
        let mut flat = vec![0.0_f32; n * f];
        flat[SCALAR_TEMPERATURE] = 290.0;
        flat[f + SCALAR_TEMPERATURE] = 310.0;
        let scalars = Tensor::from_data(Data::new(flat, Shape::new([n, f])), &dev);
        let mask = Tensor::from_data(Data::new(vec![0.0_f32, 1.0_f32], Shape::new([n, 1])), &dev);
        let umst = staging_two_node_umst(scalars.clone(), mask, None);
        let proposed = scalars.clone().add_scalar(5.0);
        let via_mask = umst.apply_policy_mask(proposed.clone());
        let via_all = umst.project_all_scalars(proposed);
        let a: Vec<f32> = via_mask.into_data().value;
        let b: Vec<f32> = via_all.into_data().value;
        assert_eq!(a, b);
        // node0 masked → original; node1 editable → proposed
        assert!((a[SCALAR_TEMPERATURE] - 290.0).abs() < 1e-5);
        assert!((a[f + SCALAR_TEMPERATURE] - 315.0).abs() < 1e-5);
    }

    #[test]
    fn umst_tensors_multi_col_faces_b2_still_unlanded() {
        let dev = device();
        let n = 2usize;
        let scalars = Tensor::<B, 2>::zeros([n, UMST_SCALAR_CHANNEL_COUNT], &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let mut umst = staging_two_node_umst(scalars, mask, None);
        // Two staging face columns — still not a measured B₂ landing.
        umst.faces_b2 = Tensor::from_data(
            Data::new(vec![0i64, 1i64, 1i64, -1i64], Shape::new([2, 2])),
            &dev,
        );
        umst.try_validate_layout()
            .expect("multi-col [2,K] layout ok");
        assert!(!umst.faces_b2_is_staging_placeholder());
        assert_eq!(
            umst.faces_b2_staging_kind(),
            FacesB2StagingKind::MultiColumnUnlanded { cols: 2 }
        );
        assert!(!umst_tensors_b2_cells_landed());
        assert!(!umst_tensors_production_wired());
        assert!(!umst_tensors_master_retick_eligible());
    }

    #[test]
    fn umst_tensors_b1_incidence_accepts_oriented_edges() {
        let dev = device();
        let n = 2usize;
        let scalars = Tensor::<B, 2>::zeros([n, UMST_SCALAR_CHANNEL_COUNT], &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let umst = staging_two_node_umst(scalars, mask, None);
        let b1 = umst.try_b1_incidence().expect("oriented B1");
        assert_eq!(b1.n_edges(), 2);
        assert_eq!(
            umst.faces_b2_staging_kind(),
            FacesB2StagingKind::PlaceholderCols1
        );
    }

    #[test]
    fn umst_tensors_lift_after_dec_staging_witness_gateway_only() {
        let dev = device();
        let n = 2usize;
        let scalars = Tensor::<B, 2>::zeros([n, UMST_SCALAR_CHANNEL_COUNT], &dev);
        let mask = Tensor::<B, 2>::ones([n, 1], &dev);
        let umst = staging_two_node_umst(scalars, mask, None);
        let staging = umst
            .try_as_verified_dec_bundle(SCALAR_HUMIDITY)
            .expect("DEC staging");
        let verified =
            VerifiedUMST::<B, ClausiusDuhemProof>::lift_after_dec_staging_witness(staging, umst);
        assert_eq!(verified.state.active_node_count(), n);
        assert!(verified.state.try_validate_layout().is_ok());
        // Lift morphism ≠ MASTER / production / B₂ flip.
        assert!(!umst_tensors_master_retick_eligible());
        assert!(!umst_tensors_production_wired());
        assert!(!umst_tensors_b2_cells_landed());
    }

    #[test]
    fn umst_tensors_material_composition_tensor_batch_shape() {
        let dev = device();
        let fractions = Tensor::<B, 2>::zeros([4, 3], &dev);
        let comp = MaterialCompositionTensor { fractions };
        assert_eq!(comp.fractions.dims(), [4, 3]);
        // Composition carrier is orthogonal to production / MASTER / B₂ gates.
        assert!(!umst_tensors_production_wired());
        assert_eq!(UMST_TENSORS_CELL_ID, "W29-030-TENSORS");
    }
}
