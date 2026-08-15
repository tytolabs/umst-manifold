// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP Manifesto §1 — phantom-typed Burn tensor carriers ([`Field`]).
//!
//! Staging vocabulary for THMC / fracture migration (P3): solvers still accept naked
//! [`burn::tensor::Tensor`] at call sites. This module introduces compile-time space
//! witnesses without breaking Burn APIs; P3.1–P3.7 schedule in
//! `old/residuals/residuals/migration-2026-07-20/fp_p3_thmc_field_migration_plan.md`.
//!
//! Prior art: [`super::dec_typestate::B1Incidence`] (topology) and
//! [`super::dec_typestate::ScalarChannelIdx`] (scalar layout).
//!
//! # Migration
//!
//! P3.1 wrapped `ThmcState` plan fields; solvers unwrap via [`Field::as_tensor`] / [`Field::into_tensor`]
//! at kernel boundaries. P3.2–P3.7 schedule in `old/residuals/residuals/migration-2026-07-20/fp_p3_thmc_field_migration_plan.md`.

use std::marker::PhantomData;

use burn::tensor::{backend::Backend, Tensor};

use super::material_transition::ReactionExtentKineticsSpec;
pub use super::solver_unwrap_inventory::{
    P3_SOLVER_UNWRAP_BOUNDARY_AUDIT_COMPLETE, P3_SOLVER_UNWRAP_BOUNDARY_OPEN,
    P3_SOLVER_UNWRAP_SITES_CLOSED, P3_SOLVER_UNWRAP_SITES_NAMED_OPEN,
};

/// FP P3 migration posture — phantom-typed carriers on disk; solver unwrap boundary still open.
pub const P3_MIGRATION_POSTURE: &str = "PHANTOM_TYPED_STAGING";

/// Whether rank-1+ `TensorAlgebra` impl over [`Field`] carriers is closed.
pub const P3_TENSOR_ALGEBRA_IMPL_LANDED: bool = true;

/// Whether P3.1 wrapped `ThmcState` plan fields landed in production solver paths.
pub const P3_PLAN_FIELD_WRAP_LANDED: bool = true;

/// Errors surfaced by [`Field`] layout validators (total public API).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldError {
    /// Tensor rank does not match the compile-time `D` witness on [`Field`].
    RankMismatch { expected: usize, found: usize },
    /// Batch dimension does not match the caller-supplied contract.
    BatchDimMismatch { expected: usize, found: usize },
    /// Nodal dimension does not match the caller-supplied contract.
    NodeDimMismatch { expected: usize, found: usize },
    /// Trailing feature dimension does not match the caller-supplied contract.
    LastDimMismatch { expected: usize, found: usize },
}

/// Compile-time metadata for phantom space markers.
pub trait FieldSpace {
    /// Rust identifier for the phantom marker (cross-ref slice-3b ledger).
    const MARKER_NAME: &'static str;
    /// Burn tensor rank `D` for [`Field<B, Self, D>`].
    const TENSOR_RANK: usize;
}

/// One census row for phantom-typed field carriers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldCensusRow {
    /// Phantom marker name in this module.
    pub marker_name: &'static str,
    /// Burn tensor rank.
    pub tensor_rank: u8,
    /// Typical shape note (not enforced at runtime).
    pub typical_shape_note: &'static str,
    /// Matching slice-3b ledger sub-id when aligned with [`crate::runtime::atoms_tensor_lift_ops`].
    pub ledger_sub_id: Option<&'static str>,
}

/// Frozen field carrier census — honest inventory, not production-wired claim.
pub const FIELD_CENSUS_ROWS: &[FieldCensusRow] = &[
    FieldCensusRow {
        marker_name: "Temperature",
        tensor_rank: 3,
        typical_shape_note: "[B, N, F_T]",
        ledger_sub_id: Some("R-ATOMS-F1-T"),
    },
    FieldCensusRow {
        marker_name: "Humidity",
        tensor_rank: 3,
        typical_shape_note: "[B, N, F_h]",
        ledger_sub_id: Some("R-ATOMS-F1-H"),
    },
    FieldCensusRow {
        marker_name: "Displacement",
        tensor_rank: 3,
        typical_shape_note: "[B, N, 3]",
        ledger_sub_id: Some("R-ATOMS-F1-u"),
    },
    FieldCensusRow {
        marker_name: "BodyForce",
        tensor_rank: 3,
        typical_shape_note: "[B, N, 3]",
        ledger_sub_id: None,
    },
    FieldCensusRow {
        marker_name: "BoundaryMask",
        tensor_rank: 3,
        typical_shape_note: "[B, N, 3]",
        ledger_sub_id: None,
    },
    FieldCensusRow {
        marker_name: "Damage",
        tensor_rank: 3,
        typical_shape_note: "[B, N, 1]",
        ledger_sub_id: Some("R-ATOMS-F1-d"),
    },
    FieldCensusRow {
        marker_name: "ReactionExtent",
        tensor_rank: 3,
        typical_shape_note: "[B, N, F_alpha]",
        ledger_sub_id: Some("R-ATOMS-F1-alpha"),
    },
    FieldCensusRow {
        marker_name: "SmallStrain",
        tensor_rank: 4,
        typical_shape_note: "[B, N, 3, 3]",
        ledger_sub_id: Some("R-ATOMS-F1-eps"),
    },
    FieldCensusRow {
        marker_name: "CauchyStress",
        tensor_rank: 4,
        typical_shape_note: "[B, N, 3, 3]",
        ledger_sub_id: None,
    },
    FieldCensusRow {
        marker_name: "FractureEnergy",
        tensor_rank: 3,
        typical_shape_note: "[B, N, 1]",
        ledger_sub_id: None,
    },
    FieldCensusRow {
        marker_name: "NodalDensity",
        tensor_rank: 3,
        typical_shape_note: "[B, N, 1]",
        ledger_sub_id: None,
    },
    FieldCensusRow {
        marker_name: "Velocity",
        tensor_rank: 3,
        typical_shape_note: "[B, N, 3]",
        ledger_sub_id: None,
    },
    FieldCensusRow {
        marker_name: "Acceleration",
        tensor_rank: 3,
        typical_shape_note: "[B, N, 3]",
        ledger_sub_id: None,
    },
    FieldCensusRow {
        marker_name: "ScalarPressure",
        tensor_rank: 3,
        typical_shape_note: "[B, N, 1]",
        ledger_sub_id: None,
    },
    FieldCensusRow {
        marker_name: "Stiffness",
        tensor_rank: 3,
        typical_shape_note: "[B, N, 2]",
        ledger_sub_id: None,
    },
];

/// Honest deepen summary for fleet / meta hygiene.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldDepthSummary {
    pub migration_posture: &'static str,
    pub tensor_algebra_impl_landed: bool,
    pub solver_unwrap_boundary_open: bool,
    pub plan_field_wrap_landed: bool,
    pub census_row_count: usize,
    pub ledger_aligned_row_count: usize,
}

/// Frozen depth summary — phantom staging landed; algebra / solver wrap still open.
#[must_use]
pub const fn field_depth_summary() -> FieldDepthSummary {
    let mut ledger_aligned = 0;
    let mut i = 0;
    while i < FIELD_CENSUS_ROWS.len() {
        if FIELD_CENSUS_ROWS[i].ledger_sub_id.is_some() {
            ledger_aligned += 1;
        }
        i += 1;
    }
    FieldDepthSummary {
        migration_posture: P3_MIGRATION_POSTURE,
        tensor_algebra_impl_landed: P3_TENSOR_ALGEBRA_IMPL_LANDED,
        solver_unwrap_boundary_open: P3_SOLVER_UNWRAP_BOUNDARY_OPEN,
        plan_field_wrap_landed: P3_PLAN_FIELD_WRAP_LANDED,
        census_row_count: FIELD_CENSUS_ROWS.len(),
        ledger_aligned_row_count: ledger_aligned,
    }
}

/// Phantom space marker: nodal temperature field \(T\) — shape `[B, N, F_T]`, kelvin.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; layout SSOT is `umst_schema::SCALAR_TEMPERATURE`.
#[derive(Clone, Copy, Debug)]
pub struct Temperature;
impl FieldSpace for Temperature {
    const MARKER_NAME: &'static str = "Temperature";
    const TENSOR_RANK: usize = 3;
}

/// Phantom space marker: pore-fluid / humidity proxy \(h\) — shape `[B, N, F_h]`, typically `[0, 1]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; layout SSOT is `umst_schema::SCALAR_HUMIDITY`.
#[derive(Clone, Copy, Debug)]
pub struct Humidity;
impl FieldSpace for Humidity {
    const MARKER_NAME: &'static str = "Humidity";
    const TENSOR_RANK: usize = 3;
}

/// Phantom space marker: mechanical displacement \(\mathbf u\) — shape `[B, N, 3]`, SI metres.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; vector channel SSOT is `vector_features[*, 0, *]`.
#[derive(Clone, Copy, Debug)]
pub struct Displacement;
impl FieldSpace for Displacement {
    const MARKER_NAME: &'static str = "Displacement";
    const TENSOR_RANK: usize = 3;
}

/// Phantom space marker: nodal body-force density \(\mathbf f\) — shape `[B, N, 3]` (FP XS-6).
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; vector channel SSOT is `vector_features[*, 1, *]` (body-force slot).
#[derive(Clone, Copy, Debug)]
pub struct BodyForce;
impl FieldSpace for BodyForce {
    const MARKER_NAME: &'static str = "BodyForce";
    const TENSOR_RANK: usize = 3;
}

/// Phantom space marker: Dirichlet DOF mask — shape `[B, N, 3]` (FP XS-5).
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; boundary mask is caller-owned `[B, N, 3]` per-DOF constraint channel.
#[derive(Clone, Copy, Debug)]
pub struct BoundaryMask;
impl FieldSpace for BoundaryMask {
    const MARKER_NAME: &'static str = "BoundaryMask";
    const TENSOR_RANK: usize = 3;
}

/// Phantom space marker: phase-field / continuum damage \(d\) — shape `[B, N, 1]` (or `[B, N, F_d]`).
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; layout SSOT is `umst_schema::SCALAR_DAMAGE`.
#[derive(Clone, Copy, Debug)]
pub struct Damage;
impl FieldSpace for Damage {
    const MARKER_NAME: &'static str = "Damage";
    const TENSOR_RANK: usize = 3;
}

/// Phantom space marker: chemical reaction extent \(\alpha\) — shape `[B, N, F_\alpha]`, clipped to `[0, 1]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; THMC chemical channel (P3.1 migration target).
#[derive(Clone, Copy, Debug)]
pub struct ReactionExtent;
impl FieldSpace for ReactionExtent {
    const MARKER_NAME: &'static str = "ReactionExtent";
    const TENSOR_RANK: usize = 3;
}

/// Phantom space marker: symmetric small strain ε — shape `[B, N, 3, 3]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; fracture AT2 strain rank/shape SSOT is `[B, N, 3, 3]` symmetric tensor layout.
#[derive(Clone, Copy, Debug)]
pub struct SmallStrain;
impl FieldSpace for SmallStrain {
    const MARKER_NAME: &'static str = "SmallStrain";
    const TENSOR_RANK: usize = 4;
}

/// Phantom space marker: nodal Cauchy stress σ — shape `[B, N, 3, 3]`, symmetric, Pa (FP XS-9a).
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; mechanics equilibrium σ rank/shape SSOT is `[B, N, 3, 3]` (distinct from [`SmallStrain`] despite shared rank).
#[derive(Clone, Copy, Debug)]
pub struct CauchyStress;
impl FieldSpace for CauchyStress {
    const MARKER_NAME: &'static str = "CauchyStress";
    const TENSOR_RANK: usize = 4;
}

/// Phantom space marker: fracture energy release rate \(G_c\) — shape `[B, N, 1]`, J/m².
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; AT2 fracture toughness nodal field SSOT is `[B, N, 1]` (distinct from [`Damage`] despite shared rank).
#[derive(Clone, Copy, Debug)]
pub struct FractureEnergy;
impl FieldSpace for FractureEnergy {
    const MARKER_NAME: &'static str = "FractureEnergy";
    const TENSOR_RANK: usize = 3;
}

/// Phantom space marker: nodal stiffness / modulus pair \([E_\mathrm{young}, \nu]\) — shape `[B, N, 2]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; bar-network mechanics SSOT is `[B, N, 2]` with columns `[E, ν]` (distinct from [`Damage`] / [`ReactionExtent`] despite shared rank-3).
#[derive(Clone, Copy, Debug)]
pub struct Stiffness;
impl FieldSpace for Stiffness {
    const MARKER_NAME: &'static str = "Stiffness";
    const TENSOR_RANK: usize = 3;
}

/// Phantom space marker: nodal pseudo-density ρ — shape `[B, N, 1]` (topology diffusion).
#[derive(Clone, Copy, Debug)]
pub struct NodalDensity;
impl FieldSpace for NodalDensity {
    const MARKER_NAME: &'static str = "NodalDensity";
    const TENSOR_RANK: usize = 3;
}

/// Phantom space marker: nodal velocity **u̇** — shape `[B, N, 3]`.
#[derive(Clone, Copy, Debug)]
pub struct Velocity;
impl FieldSpace for Velocity {
    const MARKER_NAME: &'static str = "Velocity";
    const TENSOR_RANK: usize = 3;
}

/// Phantom space marker: nodal acceleration **ü** — shape `[B, N, 3]`.
#[derive(Clone, Copy, Debug)]
pub struct Acceleration;
impl FieldSpace for Acceleration {
    const MARKER_NAME: &'static str = "Acceleration";
    const TENSOR_RANK: usize = 3;
}

/// Phantom space marker: nodal scalar pressure — shape `[B, N, 1]`.
#[derive(Clone, Copy, Debug)]
pub struct ScalarPressure;
impl FieldSpace for ScalarPressure {
    const MARKER_NAME: &'static str = "ScalarPressure";
    const TENSOR_RANK: usize = 3;
}

/// Phantom-typed tensor carrier: physical meaning encoded at compile time via `Space`.
///
/// Uses `PhantomData<fn() -> Space>` so the space witness is invariant (not covariant),
/// preventing accidental subtyping between distinct material quantities that share rank/shape.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Pure newtype over Burn `Tensor`; no new physics claim beyond caller layout contracts.
#[derive(Clone, Debug)]
pub struct Field<B: Backend, Space, const D: usize> {
    tensor: Tensor<B, D>,
    _space: PhantomData<fn() -> Space>,
}

impl<B: Backend, Space, const D: usize> Field<B, Space, D> {
    /// Wrap an existing Burn tensor (layout contracts remain caller-owned).
    ///
    /// formal_anchor: NONE
    /// formal_status: Structural
    /// formal_anchor_rationale: Opaque constructor; does not validate shape or units.
    #[inline]
    #[must_use]
    pub fn new(tensor: Tensor<B, D>) -> Self {
        Self {
            tensor,
            _space: PhantomData,
        }
    }

    /// Borrow the underlying Burn tensor for kernel / solver ops.
    ///
    /// formal_anchor: NONE
    /// formal_status: Structural
    /// formal_anchor_rationale: Explicit escape hatch to Burn APIs; preserves staging boundary.
    #[inline]
    pub fn as_tensor(&self) -> &Tensor<B, D> {
        &self.tensor
    }

    /// Consume and return the underlying Burn tensor.
    ///
    /// formal_anchor: NONE
    /// formal_status: Structural
    /// formal_anchor_rationale: Explicit escape hatch to Burn APIs; preserves staging boundary.
    #[inline]
    pub fn into_tensor(self) -> Tensor<B, D> {
        self.tensor
    }

    /// Map the inner tensor while preserving the space witness.
    ///
    /// formal_anchor: NONE
    /// formal_status: Structural
    /// formal_anchor_rationale: Functorial map over carrier; space marker unchanged by construction.
    #[inline]
    #[must_use]
    pub fn map(self, f: impl FnOnce(Tensor<B, D>) -> Tensor<B, D>) -> Self {
        Self::new(f(self.tensor))
    }

    /// Shape dimensions of the underlying Burn tensor.
    #[inline]
    pub fn dims(&self) -> [usize; D] {
        self.tensor.dims()
    }
}

impl<B: Backend, Space: FieldSpace> Field<B, Space, 3> {
    /// Validate `[batch, nodes, last_dim]` and wrap as a rank-3 field carrier.
    pub fn try_new_batch_nodes_last(
        tensor: Tensor<B, 3>,
        batch: usize,
        nodes: usize,
        last_dim: usize,
    ) -> Result<Self, FieldError> {
        if Space::TENSOR_RANK != 3 {
            return Err(FieldError::RankMismatch {
                expected: Space::TENSOR_RANK,
                found: 3,
            });
        }
        let dims = tensor.dims();
        if dims[0] != batch {
            return Err(FieldError::BatchDimMismatch {
                expected: batch,
                found: dims[0],
            });
        }
        if dims[1] != nodes {
            return Err(FieldError::NodeDimMismatch {
                expected: nodes,
                found: dims[1],
            });
        }
        if dims[2] != last_dim {
            return Err(FieldError::LastDimMismatch {
                expected: last_dim,
                found: dims[2],
            });
        }
        Ok(Self::new(tensor))
    }
}

impl<B: Backend, Space: FieldSpace> Field<B, Space, 4> {
    /// Validate `[batch, nodes, d3, d4]` and wrap as a rank-4 field carrier.
    pub fn try_new_batch_nodes_d3_d4(
        tensor: Tensor<B, 4>,
        batch: usize,
        nodes: usize,
        d3: usize,
        d4: usize,
    ) -> Result<Self, FieldError> {
        if Space::TENSOR_RANK != 4 {
            return Err(FieldError::RankMismatch {
                expected: Space::TENSOR_RANK,
                found: 4,
            });
        }
        let dims = tensor.dims();
        if dims[0] != batch {
            return Err(FieldError::BatchDimMismatch {
                expected: batch,
                found: dims[0],
            });
        }
        if dims[1] != nodes {
            return Err(FieldError::NodeDimMismatch {
                expected: nodes,
                found: dims[1],
            });
        }
        if dims[2] != d3 {
            return Err(FieldError::LastDimMismatch {
                expected: d3,
                found: dims[2],
            });
        }
        if dims[3] != d4 {
            return Err(FieldError::LastDimMismatch {
                expected: d4,
                found: dims[3],
            });
        }
        Ok(Self::new(tensor))
    }
}

/// Lookup census row by phantom marker name.
#[must_use]
pub fn field_census_row(marker_name: &str) -> Option<&'static FieldCensusRow> {
    FIELD_CENSUS_ROWS
        .iter()
        .find(|row| row.marker_name == marker_name)
}

/// Temperature plan field — `[B, N, F_T]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`Temperature`] witness.
pub type TemperatureField<B> = Field<B, Temperature, 3>;
/// Humidity plan field — `[B, N, F_h]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`Humidity`] witness.
pub type HumidityField<B> = Field<B, Humidity, 3>;
/// Displacement plan field — `[B, N, 3]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`Displacement`] witness.
pub type DisplacementField<B> = Field<B, Displacement, 3>;
pub type BodyForceField<B> = Field<B, BodyForce, 3>;
pub type BoundaryMaskField<B> = Field<B, BoundaryMask, 3>;
/// Damage plan field — `[B, N, 1]` (typical).
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`Damage`] witness.
pub type DamageField<B> = Field<B, Damage, 3>;
/// Reaction-extent plan field — `[B, N, F_\alpha]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`ReactionExtent`] witness.
pub type ReactionExtentField<B> = Field<B, ReactionExtent, 3>;
/// Small-strain tensor field — `[B, N, 3, 3]` symmetric ε.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-4 alias for [`Field`] with [`SmallStrain`] witness.
pub type SmallStrainField<B> = Field<B, SmallStrain, 4>;
/// Cauchy-stress tensor field — `[B, N, 3, 3]` symmetric σ.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-4 alias for [`Field`] with [`CauchyStress`] witness.
pub type CauchyStressField<B> = Field<B, CauchyStress, 4>;
/// Fracture-energy plan field — `[B, N, 1]` \(G_c\).
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`FractureEnergy`] witness.
pub type FractureEnergyField<B> = Field<B, FractureEnergy, 3>;
/// Stiffness / modulus plan field — `[B, N, 2]` with columns `[E_\mathrm{young}, \nu]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`Stiffness`] witness.
pub type StiffnessField<B> = Field<B, Stiffness, 3>;
pub type NodalDensityField<B> = Field<B, NodalDensity, 3>;
pub type VelocityField<B> = Field<B, Velocity, 3>;
pub type AccelerationField<B> = Field<B, Acceleration, 3>;
pub type ScalarPressureField<B> = Field<B, ScalarPressure, 3>;

/// Frozen damage mask at THMC step entry — distinct from live `state.damage` after fracture.
#[derive(Clone, Debug)]
pub struct StepEntryDamageMask<B: Backend>(DamageField<B>);

impl<B: Backend> StepEntryDamageMask<B> {
    #[inline]
    #[must_use]
    pub fn from_damage_field(damage: DamageField<B>) -> Self {
        Self(damage)
    }

    #[must_use]
    pub fn from_step_entry_damage(state_damage: &DamageField<B>, batch: usize, n: usize) -> Self {
        let damage_tensor = state_damage.as_tensor();
        let tensor = match damage_tensor.dims()[2] {
            1 => damage_tensor.clone(),
            _ => damage_tensor.clone().slice([0..batch, 0..n, 0..1]),
        };
        Self(Field::new(tensor))
    }

    #[deprecated(since = "0.2.0", note = "use from_step_entry_damage — FP P3.2")]
    #[inline]
    #[must_use]
    pub fn from_tensor(tensor: Tensor<B, 3>) -> Self {
        Self(Field::new(tensor))
    }

    #[inline]
    pub fn as_tensor(&self) -> &Tensor<B, 3> {
        self.0.as_tensor()
    }

    #[inline]
    pub fn as_damage_field(&self) -> &DamageField<B> {
        &self.0
    }

    #[inline]
    pub fn into_damage_field(self) -> DamageField<B> {
        self.0
    }
}

impl<B: Backend> FractureEnergyField<B> {
    /// Zero-filled fracture-energy field.
    #[must_use]
    pub fn zeros(dims: [usize; 3], device: &B::Device) -> Self {
        Field::new(Tensor::<B, 3>::zeros(dims, device))
    }

    /// Wrap an existing fracture-energy tensor.
    #[inline]
    #[must_use]
    pub fn from_tensor(tensor: Tensor<B, 3>) -> Self {
        Field::new(tensor)
    }
}

impl<B: Backend> StiffnessField<B> {
    /// Zero-filled stiffness field.
    #[must_use]
    pub fn zeros(dims: [usize; 3], device: &B::Device) -> Self {
        Field::new(Tensor::<B, 3>::zeros(dims, device))
    }

    /// Wrap an existing `[B, N, 2]` stiffness tensor.
    #[inline]
    #[must_use]
    pub fn from_tensor(tensor: Tensor<B, 3>) -> Self {
        Field::new(tensor)
    }

    /// Canonical bar-network assembly: `cat([e_young, nu], dim=2)` → `[B, N, 2]`.
    ///
    /// `e_young` and `nu` are typically `[B, N, 1]` nodal channels.
    #[inline]
    #[must_use]
    pub fn from_e_nu_cat(e_young: Tensor<B, 3>, nu: Tensor<B, 3>) -> Self {
        Field::new(Tensor::cat(vec![e_young, nu], 2))
    }

    /// THMC reaction-extent stiffness centralizer: \(E = \alpha \cdot E_\mathrm{scale}\), \(\nu\) uniform.
    ///
    /// `alpha_bn1` is the clipped `[B, N, 1]` reaction-extent channel (caller-owned slice/clamp).
    #[inline]
    #[must_use]
    pub fn from_alpha_kinetics(
        alpha_bn1: Tensor<B, 3>,
        spec: &ReactionExtentKineticsSpec,
        device: &B::Device,
    ) -> Self {
        let [batch, n, _] = alpha_bn1.dims();
        let stiffness_e = alpha_bn1.mul_scalar(spec.stiffness_e_scale_pa);
        let stiffness_nu =
            Tensor::<B, 3>::zeros([batch, n, 1], device).add_scalar(spec.stiffness_nu);
        Self::from_e_nu_cat(stiffness_e, stiffness_nu)
    }
}

impl<B: Backend> BodyForceField<B> {
    #[must_use]
    pub fn zeros(dims: [usize; 3], device: &B::Device) -> Self {
        Field::new(Tensor::<B, 3>::zeros(dims, device))
    }

    #[inline]
    #[must_use]
    pub fn from_tensor(tensor: Tensor<B, 3>) -> Self {
        Field::new(tensor)
    }
}

impl<B: Backend> BoundaryMaskField<B> {
    #[must_use]
    pub fn zeros(dims: [usize; 3], device: &B::Device) -> Self {
        Field::new(Tensor::<B, 3>::zeros(dims, device))
    }

    #[inline]
    #[must_use]
    pub fn from_tensor(tensor: Tensor<B, 3>) -> Self {
        Field::new(tensor)
    }
}

impl<B: Backend> SmallStrainField<B> {
    /// Zero-filled small-strain field.
    #[must_use]
    pub fn zeros(dims: [usize; 4], device: &B::Device) -> Self {
        Field::new(Tensor::<B, 4>::zeros(dims, device))
    }

    /// Wrap an existing symmetric strain tensor.
    #[inline]
    #[must_use]
    pub fn from_tensor(tensor: Tensor<B, 4>) -> Self {
        Field::new(tensor)
    }
}

impl<B: Backend> CauchyStressField<B> {
    /// Zero-filled Cauchy-stress field.
    #[must_use]
    pub fn zeros(dims: [usize; 4], device: &B::Device) -> Self {
        Field::new(Tensor::<B, 4>::zeros(dims, device))
    }

    /// Wrap an existing symmetric Cauchy-stress tensor.
    #[inline]
    #[must_use]
    pub fn from_tensor(tensor: Tensor<B, 4>) -> Self {
        Field::new(tensor)
    }
}

#[cfg(test)]
mod tests {
    use burn::tensor::Tensor;
    use burn_ndarray::NdArray;

    use super::*;

    type B = NdArray;

    #[test]
    fn field_newtype_round_trips_tensor() {
        let device = Default::default();
        let raw = Tensor::<B, 3>::ones([1, 4, 1], &device);
        let field: TemperatureField<B> = Field::new(raw.clone());
        assert_eq!(field.as_tensor().dims(), [1, 4, 1]);
        assert_eq!(field.clone().into_tensor().dims(), raw.dims());
    }

    #[test]
    fn field_map_preserves_space_marker() {
        let device = Default::default();
        let raw = Tensor::<B, 3>::zeros([1, 2, 1], &device);
        let scaled = Field::<B, Humidity, 3>::new(raw).map(|t| t.add_scalar(0.5_f32));
        assert!((scaled.as_tensor().clone().into_data().value[0] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn distinct_space_markers_are_separate_types() {
        fn accept_temperature(_: TemperatureField<B>) {}
        fn accept_damage(_: DamageField<B>) {}

        let device = Default::default();
        let raw = Tensor::<B, 3>::zeros([1, 2, 1], &device);
        accept_temperature(Field::new(raw.clone()));
        accept_damage(Field::new(raw));
    }

    #[test]
    fn cauchy_stress_field_distinct_from_small_strain() {
        fn accept_sigma(_: CauchyStressField<B>) {}
        fn accept_eps(_: SmallStrainField<B>) {}

        let device = Default::default();
        let raw = Tensor::<B, 4>::zeros([1, 2, 3, 3], &device);
        accept_sigma(CauchyStressField::from_tensor(raw.clone()));
        accept_eps(SmallStrainField::from_tensor(raw));
    }

    #[test]
    fn small_strain_field_distinct_from_damage() {
        fn accept_strain(_: SmallStrainField<B>) {}
        fn accept_damage(_: DamageField<B>) {}

        let device = Default::default();
        let strain_raw = Tensor::<B, 4>::zeros([1, 2, 3, 3], &device);
        let damage_raw = Tensor::<B, 3>::zeros([1, 2, 1], &device);
        accept_strain(Field::new(strain_raw));
        accept_damage(Field::new(damage_raw));
    }

    #[test]
    fn fracture_energy_field_distinct_from_damage() {
        fn accept_gc(_: FractureEnergyField<B>) {}
        fn accept_damage(_: DamageField<B>) {}

        let device = Default::default();
        let gc_raw = Tensor::<B, 3>::zeros([1, 2, 1], &device);
        let damage_raw = Tensor::<B, 3>::zeros([1, 2, 1], &device);
        accept_gc(FractureEnergyField::from_tensor(gc_raw));
        accept_damage(Field::new(damage_raw));
    }

    #[test]
    fn stiffness_from_alpha_kinetics_matches_e_nu_cat() {
        use crate::core::material_transition::ReactionExtentKineticsSpec;

        let device = Default::default();
        let spec = ReactionExtentKineticsSpec {
            stiffness_e_scale_pa: 30e9,
            stiffness_nu: 0.2,
            ..ReactionExtentKineticsSpec::substrate_neutral()
        };
        let alpha = Tensor::<B, 3>::from_floats([[[0.5], [0.75]]], &device);
        let central = StiffnessField::from_alpha_kinetics(alpha.clone(), &spec, &device);
        let manual = StiffnessField::from_e_nu_cat(
            alpha.mul_scalar(spec.stiffness_e_scale_pa),
            Tensor::<B, 3>::zeros([1, 2, 1], &device).add_scalar(spec.stiffness_nu),
        );
        assert_eq!(central.as_tensor().dims(), [1, 2, 2]);
        let c = central.as_tensor().clone().into_data();
        let m = manual.as_tensor().clone().into_data();
        for (a, b) in c.value.iter().zip(m.value.iter()) {
            assert!((a - b).abs() < 1e-3, "mismatch: {a} vs {b}");
        }
    }

    #[test]
    fn body_force_field_distinct_from_displacement_and_boundary_mask() {
        fn accept_body_force(_: BodyForceField<B>) {}
        fn accept_displacement(_: DisplacementField<B>) {}
        fn accept_boundary_mask(_: BoundaryMaskField<B>) {}

        let device = Default::default();
        let raw = Tensor::<B, 3>::zeros([1, 2, 3], &device);
        accept_body_force(BodyForceField::from_tensor(raw.clone()));
        accept_displacement(Field::new(raw.clone()));
        accept_boundary_mask(BoundaryMaskField::from_tensor(raw));
    }

    #[test]
    fn stiffness_field_distinct_from_damage_and_reaction_extent() {
        fn accept_stiffness(_: StiffnessField<B>) {}
        fn accept_damage(_: DamageField<B>) {}
        fn accept_alpha(_: ReactionExtentField<B>) {}

        let device = Default::default();
        let stiff = StiffnessField::from_e_nu_cat(
            Tensor::<B, 3>::zeros([1, 2, 1], &device),
            Tensor::<B, 3>::zeros([1, 2, 1], &device),
        );
        accept_stiffness(stiff);
        accept_damage(Field::new(Tensor::<B, 3>::zeros([1, 2, 1], &device)));
        accept_alpha(Field::new(Tensor::<B, 3>::zeros([1, 2, 1], &device)));
    }

    #[test]
    fn field_honesty_fences_not_production_wired() {
        let summary = field_depth_summary();
        assert_eq!(summary.migration_posture, "PHANTOM_TYPED_STAGING");
        assert!(summary.tensor_algebra_impl_landed);
        assert!(summary.solver_unwrap_boundary_open);
        assert!(summary.plan_field_wrap_landed);
        assert_eq!(summary.census_row_count, 11);
        assert_eq!(summary.ledger_aligned_row_count, 6);
        assert!(P3_TENSOR_ALGEBRA_IMPL_LANDED);
        assert!(P3_SOLVER_UNWRAP_BOUNDARY_OPEN);
        assert!(P3_PLAN_FIELD_WRAP_LANDED);
    }

    #[test]
    fn field_census_rows_align_with_field_space_trait() {
        assert_eq!(FIELD_CENSUS_ROWS.len(), 11);
        assert_eq!(Temperature::MARKER_NAME, "Temperature");
        assert_eq!(Temperature::TENSOR_RANK, 3);
        assert_eq!(SmallStrain::TENSOR_RANK, 4);
        for row in FIELD_CENSUS_ROWS {
            let found = field_census_row(row.marker_name).expect("census lookup");
            assert_eq!(found.marker_name, row.marker_name);
            assert_eq!(
                found.tensor_rank as usize,
                match row.marker_name {
                    "SmallStrain" | "CauchyStress" => 4,
                    _ => 3,
                }
            );
        }
    }

    #[test]
    fn field_try_new_batch_nodes_last_validates_layout() {
        let device = Default::default();
        let ok = Tensor::<B, 3>::zeros([2, 5, 1], &device);
        let field = DamageField::<B>::try_new_batch_nodes_last(ok, 2, 5, 1).expect("valid");
        assert_eq!(field.dims(), [2, 5, 1]);

        let bad_batch = Tensor::<B, 3>::zeros([3, 5, 1], &device);
        assert!(matches!(
            DamageField::<B>::try_new_batch_nodes_last(bad_batch, 2, 5, 1),
            Err(FieldError::BatchDimMismatch {
                expected: 2,
                found: 3,
            })
        ));
    }

    #[test]
    fn field_try_new_rank4_validates_layout() {
        let device = Default::default();
        let ok = Tensor::<B, 4>::zeros([1, 3, 3, 3], &device);
        let field =
            SmallStrainField::<B>::try_new_batch_nodes_d3_d4(ok, 1, 3, 3, 3).expect("valid");
        assert_eq!(field.dims(), [1, 3, 3, 3]);

        let bad_nodes = Tensor::<B, 4>::zeros([1, 4, 3, 3], &device);
        assert!(matches!(
            SmallStrainField::<B>::try_new_batch_nodes_d3_d4(bad_nodes, 1, 3, 3, 3),
            Err(FieldError::NodeDimMismatch {
                expected: 3,
                found: 4,
            })
        ));
    }

    #[test]
    fn step_entry_damage_mask_slices_multi_channel_damage() {
        let device = Default::default();
        let multi = Tensor::<B, 3>::from_floats([[[0.1, 0.9], [0.2, 0.8]]], &device);
        let damage: DamageField<B> = Field::new(multi);
        let mask = StepEntryDamageMask::from_step_entry_damage(&damage, 1, 2);
        assert_eq!(mask.as_tensor().dims(), [1, 2, 1]);
        let data = mask.as_tensor().clone().into_data();
        assert!((data.value[0] - 0.1).abs() < 1e-6);
        assert!((data.value[1] - 0.2).abs() < 1e-6);
    }

    #[test]
    fn step_entry_damage_mask_preserves_single_channel() {
        let device = Default::default();
        let single = Tensor::<B, 3>::from_floats([[[0.42], [0.58]]], &device);
        let damage: DamageField<B> = Field::new(single);
        let mask = StepEntryDamageMask::from_step_entry_damage(&damage, 1, 2);
        assert_eq!(mask.as_tensor().dims(), [1, 2, 1]);
        let data = mask.as_tensor().clone().into_data();
        assert!((data.value[0] - 0.42).abs() < 1e-6);
        assert!((data.value[1] - 0.58).abs() < 1e-6);
    }
}
