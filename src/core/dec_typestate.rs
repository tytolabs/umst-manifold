// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Phase 1 §1C staging: DEC incidence and scalar-channel typestates.
//!
//! Makes invalid B₁ layout and out-of-range scalar channel indices unrepresentable or
//! rejectable via [`Result`] — no panics on the public API. Existing [`super::umst_schema`]
//! `SCALAR_*` literals remain the layout SSOT until a later migration pass.
//!
//! ## Honest fences (W29-022)
//!
//! - [`B1_WITNESS_LANDED`], [`SCALAR_CHANNEL_WITNESS_LANDED`], [`STAGING_BUNDLE_LANDED`] — cold
//!   staging pre-checks landed.
//! - [`B2_WITNESS_LANDED`], [`GATEWAY_MIGRATION_LANDED`] stay **false** — B₂ face incidence and
//!   Kleisli hot-path migration remain **open** (refuse-closed stubs only).
//! - [`PHYSICS_GREEN`], [`PRODUCTION_WIRED`], [`MASTER`], and [`OP5_PASS`] stay **false** — no
//!   invent GREEN / production / MASTER / OP-5. See [`dec_typestate_posture_probe`].

use core::fmt;
use std::error::Error;
use std::marker::PhantomData;

use burn::tensor::{backend::Backend, Int, Tensor};

use super::umst_schema::UMST_SCALAR_CHANNEL_COUNT;

/// W29 deepen cell — DEC typestate honesty (no invent GREEN / PRODUCTION / MASTER).
pub const W29_DEC_TYPESTATE_DEEPEN_CELL: &str = "W29-022-DEC_TYPESTATE";

/// Slice identifier for Phase 1 §1C DEC typestate staging.
pub const SLICE_ID: &str = "phase-1c-dec-typestate";

/// Formal gate catalog surface (hand-aligned to [`crate::ai::formal::FormalReject`]).
pub const CATALOG_ID: &str = "umst.gate.dec_typestate";

/// Honest posture — witnesses landed; gateway migration **open**.
pub const POSTURE_TAG: &str = "DEC_TYPESTATE_STAGING_PARTIAL";

/// B₁ incidence witness (`[2, E]` layout) landed.
pub const B1_WITNESS_LANDED: bool = true;

/// Runtime + const-generic scalar channel witnesses landed.
pub const SCALAR_CHANNEL_WITNESS_LANDED: bool = true;

/// Staging [`VerifiedUMST`] cold-assemble bundle landed.
pub const STAGING_BUNDLE_LANDED: bool = true;

/// B₂ face incidence typestate — **not** landed (placeholder `[2, 1]` still common).
pub const B2_WITNESS_LANDED: bool = false;

/// Gateway hot-path migration from staging → proof-carrying bundle — **open**.
pub const GATEWAY_MIGRATION_LANDED: bool = false;

/// Honest physics posture — staging typestate only; not a DEC physics GREEN claim.
pub const PHYSICS_GREEN: bool = false;

/// Honest production DEC typestate gateway path — **false** until measured live eval.
pub const PRODUCTION_WIRED: bool = false;

/// Honest master / fleet-complete posture — **false** at staging slice.
pub const MASTER: bool = false;

/// Honest OP-5 pass claim — **false** until measured operator-5 fleet sign-off.
pub const OP5_PASS: bool = false;

/// Fence facet inventory size (landed + open gaps).
pub const DEC_TYPESTATE_FENCE_FACET_COUNT: usize = 9;

/// Wired facets today (3/9 measured: B₁, scalar channel, staging bundle).
pub const DEC_TYPESTATE_FENCE_WIRED_COUNT: usize = 3;

/// Open (unwired) fence facets — must match inventory gaps.
pub const DEC_TYPESTATE_FENCE_OPEN_COUNT: usize =
    DEC_TYPESTATE_FENCE_FACET_COUNT - DEC_TYPESTATE_FENCE_WIRED_COUNT;

/// Honest deepen fence for meta / fleet probes.
pub const HONEST_FENCE: &str =
    "b1_witness_landed=true|scalar_channel_witness_landed=true|staging_bundle_landed=true|\
     b2_witness_landed=false|gateway_migration_landed=false|production_wired=false|\
     physics_green=false|master=false|op5_pass=false";

/// One facet of the DEC typestate production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecTypestateFenceFacet {
    pub facet: &'static str,
    pub wired: bool,
    pub owning_slice: &'static str,
}

/// DEC typestate fence facet inventory (honest posture SSOT).
pub const DEC_TYPESTATE_FENCE_FACETS: &[DecTypestateFenceFacet] = &[
    DecTypestateFenceFacet {
        facet: "b1_incidence_witness",
        wired: true,
        owning_slice: W29_DEC_TYPESTATE_DEEPEN_CELL,
    },
    DecTypestateFenceFacet {
        facet: "scalar_channel_witness",
        wired: true,
        owning_slice: W29_DEC_TYPESTATE_DEEPEN_CELL,
    },
    DecTypestateFenceFacet {
        facet: "staging_bundle_cold_assemble",
        wired: true,
        owning_slice: W29_DEC_TYPESTATE_DEEPEN_CELL,
    },
    DecTypestateFenceFacet {
        facet: "b2_face_incidence_witness",
        wired: false,
        owning_slice: "deferred-b2-typestate",
    },
    DecTypestateFenceFacet {
        facet: "gateway_hot_path_migration",
        wired: false,
        owning_slice: "deferred-kleisli-hot-path",
    },
    DecTypestateFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
    DecTypestateFenceFacet {
        facet: "physics_green",
        wired: false,
        owning_slice: "deferred-physics-oracle",
    },
    DecTypestateFenceFacet {
        facet: "master_orchestrator_pin",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
    DecTypestateFenceFacet {
        facet: "op5_pass",
        wired: false,
        owning_slice: "deferred-op5-fleet-signoff",
    },
];

const _: () = assert!(B1_WITNESS_LANDED);
const _: () = assert!(SCALAR_CHANNEL_WITNESS_LANDED);
const _: () = assert!(STAGING_BUNDLE_LANDED);
const _: () = assert!(!B2_WITNESS_LANDED);
const _: () = assert!(!GATEWAY_MIGRATION_LANDED);
const _: () = assert!(!PHYSICS_GREEN);
const _: () = assert!(!PRODUCTION_WIRED);
const _: () = assert!(!MASTER);
const _: () = assert!(!OP5_PASS);
const _: () = assert!(!dec_typestate_production_wired());
const _: () = assert!(!dec_typestate_physics_green());
const _: () = assert!(!dec_typestate_master_wired());
const _: () = assert!(!dec_typestate_op5_pass());
const _: () = assert!(
    DEC_TYPESTATE_FENCE_OPEN_COUNT
        == DEC_TYPESTATE_FENCE_FACET_COUNT - DEC_TYPESTATE_FENCE_WIRED_COUNT
);

/// Honest production DEC typestate gateway path — **false** until measured live eval.
#[must_use]
pub const fn dec_typestate_production_wired() -> bool {
    false
}

/// Honest physics GREEN claim — **false** at staging typestate slice.
#[must_use]
pub const fn dec_typestate_physics_green() -> bool {
    false
}

/// Honest master-tier wiring — **false** until fleet sign-off.
#[must_use]
pub const fn dec_typestate_master_wired() -> bool {
    false
}

/// Honest OP-5 pass claim — **false** until measured operator-5 fleet sign-off.
#[must_use]
pub const fn dec_typestate_op5_pass() -> bool {
    false
}

/// Count wired DEC typestate fence facets (must match [`DEC_TYPESTATE_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn dec_typestate_fence_wired_count() -> usize {
    DEC_TYPESTATE_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Count open (unwired) DEC typestate fence facets.
#[must_use]
pub fn dec_typestate_fence_open_count() -> usize {
    DEC_TYPESTATE_FENCE_FACETS
        .iter()
        .filter(|f| !f.wired)
        .count()
}

/// Open fence facet names (stable inventory order).
#[must_use]
pub fn dec_typestate_open_facet_names() -> Vec<&'static str> {
    DEC_TYPESTATE_FENCE_FACETS
        .iter()
        .filter(|f| !f.wired)
        .map(|f| f.facet)
        .collect()
}

/// Typed probe for DEC typestate posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecTypestatePostureProbe {
    pub deepen_cell: &'static str,
    pub slice_id: &'static str,
    pub catalog_id: &'static str,
    pub posture_tag: &'static str,
    pub b1_witness_landed: bool,
    pub scalar_channel_witness_landed: bool,
    pub staging_bundle_landed: bool,
    pub b2_witness_landed: bool,
    pub gateway_migration_landed: bool,
    pub channel_count: usize,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub production_wired: bool,
    pub physics_green: bool,
    pub master: bool,
    pub op5_pass: bool,
    pub honest_fence: &'static str,
}

/// Build introspection probe for DEC typestate done-when checks.
#[must_use]
pub const fn dec_typestate_posture_probe() -> DecTypestatePostureProbe {
    DecTypestatePostureProbe {
        deepen_cell: W29_DEC_TYPESTATE_DEEPEN_CELL,
        slice_id: SLICE_ID,
        catalog_id: CATALOG_ID,
        posture_tag: POSTURE_TAG,
        b1_witness_landed: B1_WITNESS_LANDED,
        scalar_channel_witness_landed: SCALAR_CHANNEL_WITNESS_LANDED,
        staging_bundle_landed: STAGING_BUNDLE_LANDED,
        b2_witness_landed: B2_WITNESS_LANDED,
        gateway_migration_landed: GATEWAY_MIGRATION_LANDED,
        channel_count: UMST_SCALAR_CHANNEL_COUNT,
        fence_facet_count: DEC_TYPESTATE_FENCE_FACET_COUNT,
        fence_wired_count: DEC_TYPESTATE_FENCE_WIRED_COUNT,
        production_wired: dec_typestate_production_wired(),
        physics_green: dec_typestate_physics_green(),
        master: dec_typestate_master_wired(),
        op5_pass: dec_typestate_op5_pass(),
        honest_fence: HONEST_FENCE,
    }
}

/// Staging witnesses landed with production / GREEN / MASTER gateway path honestly open.
#[must_use]
pub fn dec_typestate_posture_honest(probe: &DecTypestatePostureProbe) -> bool {
    probe.deepen_cell == W29_DEC_TYPESTATE_DEEPEN_CELL
        && probe.slice_id == SLICE_ID
        && probe.catalog_id == CATALOG_ID
        && probe.posture_tag == POSTURE_TAG
        && probe.b1_witness_landed
        && probe.scalar_channel_witness_landed
        && probe.staging_bundle_landed
        && !probe.b2_witness_landed
        && !probe.gateway_migration_landed
        && probe.channel_count == UMST_SCALAR_CHANNEL_COUNT
        && probe.fence_facet_count == DEC_TYPESTATE_FENCE_FACET_COUNT
        && probe.fence_wired_count == DEC_TYPESTATE_FENCE_WIRED_COUNT
        && !probe.production_wired
        && !probe.physics_green
        && !probe.master
        && !probe.op5_pass
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("b2_witness_landed=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("op5_pass=false")
}

/// Validate DEC typestate posture honesty — fail closed on fake production / GREEN claims.
pub fn validate_dec_typestate_posture_honesty() -> Result<(), &'static str> {
    let probe = dec_typestate_posture_probe();
    if probe.production_wired || dec_typestate_production_wired() || PRODUCTION_WIRED {
        return Err("dec_typestate_production_wired must stay false until gateway migration");
    }
    if probe.physics_green || dec_typestate_physics_green() || PHYSICS_GREEN {
        return Err("dec_typestate PHYSICS_GREEN must stay false at staging slice");
    }
    if probe.master || dec_typestate_master_wired() || MASTER {
        return Err("dec_typestate MASTER must stay false until fleet sign-off");
    }
    if probe.op5_pass || dec_typestate_op5_pass() || OP5_PASS {
        return Err("dec_typestate OP5_PASS must stay false until measured OP-5 sign-off");
    }
    if probe.b2_witness_landed || B2_WITNESS_LANDED {
        return Err("b2_witness_landed must stay false until B2 typestate lands");
    }
    if probe.gateway_migration_landed || GATEWAY_MIGRATION_LANDED {
        return Err("gateway_migration_landed must stay false until Kleisli hot-path wire");
    }
    if dec_typestate_fence_wired_count() != DEC_TYPESTATE_FENCE_WIRED_COUNT {
        return Err("dec_typestate_fence_wired_count drifted from DEC_TYPESTATE_FENCE_WIRED_COUNT");
    }
    if dec_typestate_fence_open_count() != DEC_TYPESTATE_FENCE_OPEN_COUNT {
        return Err("dec_typestate_fence_open_count drifted from DEC_TYPESTATE_FENCE_OPEN_COUNT");
    }
    if DEC_TYPESTATE_FENCE_FACETS.len() != DEC_TYPESTATE_FENCE_FACET_COUNT {
        return Err(
            "DEC_TYPESTATE_FENCE_FACETS length drifted from DEC_TYPESTATE_FENCE_FACET_COUNT",
        );
    }
    if !dec_typestate_posture_honest(&probe) {
        return Err("dec_typestate_posture_honest failed");
    }
    Ok(())
}

/// Errors surfaced by DEC typestate constructors (total public API).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecTypestateError {
    /// `edges_b1` must have shape `[2, E]` (primal node→edge incidence).
    B1WrongRowCount { rows: usize },
    /// Scalar channel index is outside the pinned layout contract.
    ScalarChannelOutOfRange { index: usize, channel_count: usize },
    /// Nodal scalar tensor width does not match the compile-time layout witness.
    ScalarWidthMismatch { expected: usize, found: usize },
    /// B₂ face incidence typestate is not landed — refuse closed.
    B2NotLanded,
    /// Gateway hot-path migration from staging → proof-carrying bundle is open.
    GatewayMigrationOpen,
}

impl fmt::Display for DecTypestateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecTypestateError::B1WrongRowCount { rows } => {
                write!(f, "B1 incidence must have 2 rows, found {rows}")
            }
            DecTypestateError::ScalarChannelOutOfRange {
                index,
                channel_count,
            } => write!(
                f,
                "scalar channel {index} out of range (layout count {channel_count})"
            ),
            DecTypestateError::ScalarWidthMismatch { expected, found } => write!(
                f,
                "scalar_features width {found} != pinned layout {expected}"
            ),
            DecTypestateError::B2NotLanded => write!(
                f,
                "B2 face incidence typestate not landed (b2_witness_landed=false)"
            ),
            DecTypestateError::GatewayMigrationOpen => write!(
                f,
                "DEC typestate gateway migration open (gateway_migration_landed=false)"
            ),
        }
    }
}

impl Error for DecTypestateError {}

/// Oriented primal **B₁** incidence: nodes → edges, shape `[2, E]`.
#[derive(Clone, Debug)]
pub struct B1Incidence<B: Backend> {
    tensor: Tensor<B, 2, Int>,
}

impl<B: Backend> B1Incidence<B> {
    /// Validate `edges_b1` layout `[2, E]` and wrap as a typed incidence carrier.
    pub fn try_new(edges_b1: Tensor<B, 2, Int>) -> Result<Self, DecTypestateError> {
        let rows = edges_b1.dims()[0];
        if rows != 2 {
            return Err(DecTypestateError::B1WrongRowCount { rows });
        }
        Ok(Self { tensor: edges_b1 })
    }

    /// Borrow the underlying Burn incidence tensor.
    #[inline]
    pub fn as_tensor(&self) -> &Tensor<B, 2, Int> {
        &self.tensor
    }

    /// Consume and return the underlying Burn incidence tensor.
    #[inline]
    pub fn into_tensor(self) -> Tensor<B, 2, Int> {
        self.tensor
    }

    /// Number of oriented edges `E` in the incidence matrix.
    #[inline]
    pub fn n_edges(&self) -> usize {
        self.tensor.dims()[1]
    }

    /// View as [`crate::physics::topology::EdgeTopology`] for existing DEC call sites.
    #[inline]
    pub fn to_edge_topology(&self) -> crate::physics::topology::EdgeTopology<B> {
        crate::physics::topology::EdgeTopology::new(self.tensor.clone())
    }
}

/// Refuse-closed **B₂** face incidence stub — typestate **not** landed.
///
/// Placeholder layouts (often `[2, 1]`) must not mint a witness while
/// [`B2_WITNESS_LANDED`] is false. [`B2Incidence::try_new`] always returns
/// [`DecTypestateError::B2NotLanded`].
#[derive(Clone, Debug)]
pub struct B2Incidence<B: Backend> {
    _phantom: PhantomData<B>,
}

impl<B: Backend> B2Incidence<B> {
    /// Refuse closed — B₂ face incidence typestate is not landed.
    pub fn try_new(_faces_b2: Tensor<B, 2, Int>) -> Result<Self, DecTypestateError> {
        let _ = B2_WITNESS_LANDED;
        Err(DecTypestateError::B2NotLanded)
    }
}

/// Compile-time scalar channel index validated against [`UMST_SCALAR_CHANNEL_COUNT`].
///
/// When `N >= UMST_SCALAR_CHANNEL_COUNT`, [`ScalarChannel::try_index`] returns `None` — the
/// invalid index cannot be obtained without falling back to raw `usize`.
#[derive(Clone, Copy, Debug)]
pub struct ScalarChannel<const N: usize>(PhantomData<()>);

impl<const N: usize> ScalarChannel<N> {
    /// Construct a typed channel witness (const-generic index is the payload).
    #[must_use]
    pub const fn new() -> Self {
        Self(PhantomData)
    }

    /// `Some(N)` only when `N` is inside the pinned layout contract.
    pub const fn try_index() -> Option<usize> {
        if N < UMST_SCALAR_CHANNEL_COUNT {
            Some(N)
        } else {
            None
        }
    }

    /// Const-generic channel index or [`DecTypestateError::ScalarChannelOutOfRange`].
    pub const fn index_or_err() -> Result<usize, DecTypestateError> {
        match Self::try_index() {
            Some(idx) => Ok(idx),
            None => Err(DecTypestateError::ScalarChannelOutOfRange {
                index: N,
                channel_count: UMST_SCALAR_CHANNEL_COUNT,
            }),
        }
    }
}

/// Runtime-validated scalar channel index (staging alternative to raw `usize`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScalarChannelIdx(usize);

impl ScalarChannelIdx {
    /// Reject indices outside `0 .. UMST_SCALAR_CHANNEL_COUNT`.
    pub fn try_new(index: usize) -> Result<Self, DecTypestateError> {
        if index >= UMST_SCALAR_CHANNEL_COUNT {
            return Err(DecTypestateError::ScalarChannelOutOfRange {
                index,
                channel_count: UMST_SCALAR_CHANNEL_COUNT,
            });
        }
        Ok(Self(index))
    }

    /// Lift a compile-time witness into a runtime-validated index.
    pub const fn from_const<const N: usize>() -> Result<Self, DecTypestateError> {
        match ScalarChannel::<N>::try_index() {
            Some(idx) => Ok(Self(idx)),
            None => Err(DecTypestateError::ScalarChannelOutOfRange {
                index: N,
                channel_count: UMST_SCALAR_CHANNEL_COUNT,
            }),
        }
    }

    /// Channel column index for tensor slicing / projection.
    #[inline]
    pub fn index(self) -> usize {
        self.0
    }
}

/// Channel selector for [`super::tensors::UnifiedMaterialStateTensor::project_scalar_channel`] /
/// [`super::tensors::UnifiedMaterialStateTensor::write_scalar_channel`].
///
/// [`usize`] preserves legacy call sites; [`ScalarChannelIdx`] carries a layout-validated index.
pub trait ScalarChannelSelector {
    /// Scalar column index into `scalar_features`.
    fn scalar_channel_index(&self) -> usize;
}

impl ScalarChannelSelector for usize {
    #[inline]
    fn scalar_channel_index(&self) -> usize {
        *self
    }
}

impl ScalarChannelSelector for ScalarChannelIdx {
    #[inline]
    fn scalar_channel_index(&self) -> usize {
        self.index()
    }
}

impl<const N: usize> ScalarChannelSelector for ScalarChannel<N> {
    #[inline]
    fn scalar_channel_index(&self) -> usize {
        N
    }
}

/// Staging DEC + scalar-layout bundle (`math-verified-umst-typestate`).
///
/// Composes a validated [`B1Incidence`] with a runtime [`ScalarChannelIdx`], pinned to
/// [`UMST_SCALAR_CHANNEL_COUNT`] at compile time via [`ScalarChannel`]. Distinct from the
/// proof-carrying gateway type [`super::tensors::VerifiedUMST`].
#[derive(Clone, Debug)]
pub struct VerifiedUMST<B: Backend> {
    b1: B1Incidence<B>,
    channel: ScalarChannelIdx,
    _layout: PhantomData<ScalarChannel<UMST_SCALAR_CHANNEL_COUNT>>,
}

impl<B: Backend> VerifiedUMST<B> {
    /// Pinned nodal scalar width from [`UMST_SCALAR_CHANNEL_COUNT`].
    pub const CHANNEL_COUNT: usize = UMST_SCALAR_CHANNEL_COUNT;

    /// Validate incidence layout, scalar tensor width, and channel index; assemble staging bundle.
    pub fn try_assemble(
        edges_b1: Tensor<B, 2, Int>,
        scalar_cols: usize,
        channel: usize,
    ) -> Result<Self, DecTypestateError> {
        let channel = ScalarChannelIdx::try_new(channel)?;
        Self::try_assemble_with_idx(edges_b1, scalar_cols, channel)
    }

    /// Assemble with a pre-validated [`ScalarChannelIdx`].
    pub fn try_assemble_with_idx(
        edges_b1: Tensor<B, 2, Int>,
        scalar_cols: usize,
        channel: ScalarChannelIdx,
    ) -> Result<Self, DecTypestateError> {
        if scalar_cols != Self::CHANNEL_COUNT {
            return Err(DecTypestateError::ScalarWidthMismatch {
                expected: Self::CHANNEL_COUNT,
                found: scalar_cols,
            });
        }
        let b1 = B1Incidence::try_new(edges_b1)?;
        Ok(Self {
            b1,
            channel,
            _layout: PhantomData,
        })
    }

    /// Assemble using a compile-time channel witness [`ScalarChannel<N>`].
    ///
    /// Rejects `N >= UMST_SCALAR_CHANNEL_COUNT` before incidence wrap.
    pub fn try_assemble_const_channel<const N: usize>(
        edges_b1: Tensor<B, 2, Int>,
        scalar_cols: usize,
    ) -> Result<Self, DecTypestateError> {
        let channel = ScalarChannelIdx::from_const::<N>()?;
        Self::try_assemble_with_idx(edges_b1, scalar_cols, channel)
    }

    /// Refuse closed — staging bundle is not a gateway hot-path migration witness.
    ///
    /// Call sites that need proof-carrying [`super::tensors::VerifiedUMST`] must wait until
    /// [`GATEWAY_MIGRATION_LANDED`] flips under measured Kleisli wire.
    pub fn try_promote_to_gateway(&self) -> Result<(), DecTypestateError> {
        let _ = (self, GATEWAY_MIGRATION_LANDED);
        Err(DecTypestateError::GatewayMigrationOpen)
    }

    /// Borrow the validated **B₁** incidence carrier.
    #[inline]
    pub fn b1(&self) -> &B1Incidence<B> {
        &self.b1
    }

    /// Active scalar channel index inside the pinned layout.
    #[inline]
    pub fn channel(&self) -> ScalarChannelIdx {
        self.channel
    }

    /// Consume and return the underlying incidence tensor.
    #[inline]
    pub fn into_b1(self) -> B1Incidence<B> {
        self.b1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::Tensor;
    use burn_ndarray::NdArray;

    type B = NdArray;

    #[test]
    fn dec_typestate_posture_metadata_locked() {
        assert_eq!(W29_DEC_TYPESTATE_DEEPEN_CELL, "W29-022-DEC_TYPESTATE");
        assert_eq!(SLICE_ID, "phase-1c-dec-typestate");
        assert_eq!(CATALOG_ID, "umst.gate.dec_typestate");
        assert_eq!(POSTURE_TAG, "DEC_TYPESTATE_STAGING_PARTIAL");
        assert!(B1_WITNESS_LANDED);
        assert!(SCALAR_CHANNEL_WITNESS_LANDED);
        assert!(STAGING_BUNDLE_LANDED);
        assert!(!B2_WITNESS_LANDED);
        assert!(!GATEWAY_MIGRATION_LANDED);
        assert!(!PHYSICS_GREEN);
        assert!(!PRODUCTION_WIRED);
        assert!(!MASTER);
        assert!(!OP5_PASS);
        assert!(!dec_typestate_production_wired());
        assert!(!dec_typestate_physics_green());
        assert!(!dec_typestate_master_wired());
        assert!(!dec_typestate_op5_pass());
        assert!(HONEST_FENCE.contains("production_wired=false"));
        assert!(HONEST_FENCE.contains("physics_green=false"));
        assert!(HONEST_FENCE.contains("master=false"));
        assert!(HONEST_FENCE.contains("op5_pass=false"));
        assert_eq!(DEC_TYPESTATE_FENCE_FACET_COUNT, 9);
        assert_eq!(DEC_TYPESTATE_FENCE_WIRED_COUNT, 3);
        assert_eq!(DEC_TYPESTATE_FENCE_OPEN_COUNT, 6);
        assert_eq!(
            DEC_TYPESTATE_FENCE_FACETS.len(),
            DEC_TYPESTATE_FENCE_FACET_COUNT
        );
        assert_eq!(
            dec_typestate_fence_wired_count(),
            DEC_TYPESTATE_FENCE_WIRED_COUNT
        );
        assert_eq!(
            dec_typestate_fence_open_count(),
            DEC_TYPESTATE_FENCE_OPEN_COUNT
        );
    }

    #[test]
    fn dec_typestate_posture_honest_staging_not_production() {
        let probe = dec_typestate_posture_probe();
        assert_eq!(probe.deepen_cell, W29_DEC_TYPESTATE_DEEPEN_CELL);
        assert!(dec_typestate_posture_honest(&probe));
        assert!(!probe.production_wired);
        assert!(!probe.physics_green);
        assert!(!probe.master);
        assert!(!probe.op5_pass);
        assert!(!probe.b2_witness_landed);
        assert!(!probe.gateway_migration_landed);
        assert_eq!(probe.channel_count, UMST_SCALAR_CHANNEL_COUNT);
        assert_eq!(probe.fence_wired_count, 3);
        assert_eq!(probe.fence_facet_count, 9);
        validate_dec_typestate_posture_honesty().expect("posture honesty");
    }

    #[test]
    fn dec_typestate_fence_facets_refuse_open_gaps() {
        let open = dec_typestate_open_facet_names();
        assert!(open.contains(&"b2_face_incidence_witness"));
        assert!(open.contains(&"gateway_hot_path_migration"));
        assert!(open.contains(&"production_wired"));
        assert!(open.contains(&"physics_green"));
        assert!(open.contains(&"master_orchestrator_pin"));
        assert!(open.contains(&"op5_pass"));
        assert_eq!(open.len(), DEC_TYPESTATE_FENCE_OPEN_COUNT);
    }

    #[test]
    fn dec_typestate_error_display_preserves_detail() {
        let err = DecTypestateError::B1WrongRowCount { rows: 3 };
        assert!(err.to_string().contains('3'));
        let err = DecTypestateError::ScalarChannelOutOfRange {
            index: 9,
            channel_count: UMST_SCALAR_CHANNEL_COUNT,
        };
        assert!(err.to_string().contains('9'));
        let err = DecTypestateError::ScalarWidthMismatch {
            expected: 5,
            found: 3,
        };
        assert!(err.to_string().contains("3"));
        assert!(err.to_string().contains("5"));
        let err = DecTypestateError::B2NotLanded;
        assert!(err.to_string().contains("b2_witness_landed=false"));
        assert!(err.source().is_none());
        let err = DecTypestateError::GatewayMigrationOpen;
        assert!(err.to_string().contains("gateway_migration_landed=false"));
    }

    #[test]
    fn dec_scalar_channel_runtime_rejects_out_of_range_index() {
        let err = ScalarChannelIdx::try_new(UMST_SCALAR_CHANNEL_COUNT).unwrap_err();
        assert_eq!(
            err,
            DecTypestateError::ScalarChannelOutOfRange {
                index: UMST_SCALAR_CHANNEL_COUNT,
                channel_count: UMST_SCALAR_CHANNEL_COUNT,
            }
        );
        assert!(ScalarChannelIdx::try_new(UMST_SCALAR_CHANNEL_COUNT + 1).is_err());
        assert_eq!(ScalarChannelIdx::try_new(0).unwrap().index(), 0);
    }

    #[test]
    fn dec_scalar_channel_const_generic_makes_overflow_unrepresentable() {
        assert_eq!(ScalarChannel::<0>::try_index(), Some(0));
        assert_eq!(
            ScalarChannel::<{ UMST_SCALAR_CHANNEL_COUNT - 1 }>::try_index(),
            Some(UMST_SCALAR_CHANNEL_COUNT - 1)
        );
        assert!(ScalarChannel::<{ UMST_SCALAR_CHANNEL_COUNT }>::try_index().is_none());
        assert!(ScalarChannel::<{ UMST_SCALAR_CHANNEL_COUNT + 1 }>::try_index().is_none());
        assert_eq!(ScalarChannel::<0>::index_or_err(), Ok(0));
        assert!(ScalarChannel::<{ UMST_SCALAR_CHANNEL_COUNT }>::index_or_err().is_err());
        assert!(ScalarChannelIdx::from_const::<{ UMST_SCALAR_CHANNEL_COUNT }>().is_err());
    }

    #[test]
    fn dec_b1_incidence_rejects_non_two_row_layout() {
        let device = Default::default();
        let bad: Tensor<B, 2, Int> = Tensor::zeros([3, 4], &device);
        let err = B1Incidence::try_new(bad).unwrap_err();
        assert_eq!(err, DecTypestateError::B1WrongRowCount { rows: 3 });

        let ok: Tensor<B, 2, Int> = Tensor::zeros([2, 5], &device);
        assert_eq!(B1Incidence::try_new(ok).unwrap().n_edges(), 5);
    }

    #[test]
    fn dec_b2_incidence_refuse_closed_while_not_landed() {
        let device = Default::default();
        // Common placeholder layout must not mint a B₂ witness.
        let placeholder: Tensor<B, 2, Int> = Tensor::zeros([2, 1], &device);
        let err = B2Incidence::<B>::try_new(placeholder).unwrap_err();
        assert_eq!(err, DecTypestateError::B2NotLanded);
        assert!(!B2_WITNESS_LANDED);
    }

    #[test]
    fn dec_verified_umst_staging_assembles_b1_and_scalar_channel() {
        let device = Default::default();
        let edges: Tensor<B, 2, Int> = Tensor::zeros([2, 2], &device);
        let staging =
            VerifiedUMST::try_assemble(edges, UMST_SCALAR_CHANNEL_COUNT, 0).expect("valid staging");
        assert_eq!(staging.b1().n_edges(), 2);
        assert_eq!(staging.channel().index(), 0);
        assert_eq!(VerifiedUMST::<B>::CHANNEL_COUNT, UMST_SCALAR_CHANNEL_COUNT);
        assert_eq!(
            staging.try_promote_to_gateway().unwrap_err(),
            DecTypestateError::GatewayMigrationOpen
        );
    }

    #[test]
    fn dec_verified_umst_try_assemble_const_channel_rejects_overflow() {
        let device = Default::default();
        let edges: Tensor<B, 2, Int> = Tensor::zeros([2, 2], &device);
        let ok = VerifiedUMST::<B>::try_assemble_const_channel::<0>(
            edges.clone(),
            UMST_SCALAR_CHANNEL_COUNT,
        )
        .expect("const channel 0");
        assert_eq!(ok.channel().index(), 0);
        let err = VerifiedUMST::<B>::try_assemble_const_channel::<{ UMST_SCALAR_CHANNEL_COUNT }>(
            edges,
            UMST_SCALAR_CHANNEL_COUNT,
        )
        .unwrap_err();
        assert_eq!(
            err,
            DecTypestateError::ScalarChannelOutOfRange {
                index: UMST_SCALAR_CHANNEL_COUNT,
                channel_count: UMST_SCALAR_CHANNEL_COUNT,
            }
        );
    }

    #[test]
    fn dec_verified_umst_staging_rejects_scalar_width_mismatch() {
        let device = Default::default();
        let edges: Tensor<B, 2, Int> = Tensor::zeros([2, 2], &device);

        let too_narrow =
            VerifiedUMST::try_assemble(edges.clone(), UMST_SCALAR_CHANNEL_COUNT - 1, 0)
                .unwrap_err();
        assert_eq!(
            too_narrow,
            DecTypestateError::ScalarWidthMismatch {
                expected: UMST_SCALAR_CHANNEL_COUNT,
                found: UMST_SCALAR_CHANNEL_COUNT - 1,
            }
        );

        let too_wide =
            VerifiedUMST::try_assemble(edges, UMST_SCALAR_CHANNEL_COUNT + 1, 0).unwrap_err();
        assert_eq!(
            too_wide,
            DecTypestateError::ScalarWidthMismatch {
                expected: UMST_SCALAR_CHANNEL_COUNT,
                found: UMST_SCALAR_CHANNEL_COUNT + 1,
            }
        );
    }

    #[test]
    fn dec_verified_umst_staging_rejects_bad_b1_and_channel() {
        let device = Default::default();
        let bad_b1: Tensor<B, 2, Int> = Tensor::zeros([1, 2], &device);
        let err = VerifiedUMST::try_assemble(bad_b1, UMST_SCALAR_CHANNEL_COUNT, 0).unwrap_err();
        assert_eq!(err, DecTypestateError::B1WrongRowCount { rows: 1 });

        let edges: Tensor<B, 2, Int> = Tensor::zeros([2, 2], &device);
        let err =
            VerifiedUMST::try_assemble(edges, UMST_SCALAR_CHANNEL_COUNT, UMST_SCALAR_CHANNEL_COUNT)
                .unwrap_err();
        assert_eq!(
            err,
            DecTypestateError::ScalarChannelOutOfRange {
                index: UMST_SCALAR_CHANNEL_COUNT,
                channel_count: UMST_SCALAR_CHANNEL_COUNT,
            }
        );
    }

    #[test]
    fn dec_scalar_channel_selector_const_generic_delegates_index() {
        let ch = ScalarChannel::<0>::new();
        assert_eq!(ch.scalar_channel_index(), 0);
        assert_eq!(
            ScalarChannelIdx::from_const::<0>()
                .unwrap()
                .scalar_channel_index(),
            0
        );
        assert_eq!(0usize.scalar_channel_index(), 0);
    }

    #[test]
    fn dec_typestate_fake_production_probe_fails_honesty() {
        let mut probe = dec_typestate_posture_probe();
        probe.production_wired = true;
        assert!(!dec_typestate_posture_honest(&probe));
        probe = dec_typestate_posture_probe();
        probe.physics_green = true;
        assert!(!dec_typestate_posture_honest(&probe));
        probe = dec_typestate_posture_probe();
        probe.master = true;
        assert!(!dec_typestate_posture_honest(&probe));
        probe = dec_typestate_posture_probe();
        probe.op5_pass = true;
        assert!(!dec_typestate_posture_honest(&probe));
        probe = dec_typestate_posture_probe();
        probe.b2_witness_landed = true;
        assert!(!dec_typestate_posture_honest(&probe));
    }
}
