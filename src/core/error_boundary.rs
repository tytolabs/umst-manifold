// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! IO-adjacent and gateway boundary error types (FP manifesto §4).
//!
//! Distinct from [`crate::physics::error::PhysicsError`]: these surface at the UMST writeback /
//! policy gateway without routing filesystem or catalog IO through the physics core.
//!
//! ## Honest fences (W29-024)
//!
//! - Typed variants for [`ApplyPhysicsError`], [`CbfReject`], and [`CatalogIoError`] are landed.
//! - Unified [`std::error::Error`] impls are landed for all three enums (`source` for DEC nest).
//! - Legacy `From<String>` shims remain required at gateway call sites.
//! - Live gateway eval and production gateway wiring remain **open**.
//! - [`error_boundary_physics_green`], [`error_boundary_production_wired`], and
//!   [`error_boundary_master_wired`] stay **false** — no invent GREEN / production / MASTER.
//!
//! See [`error_boundary_posture_probe`].

/// W29 deepen wave step — error boundary honesty (no invent GREEN).
pub const ERROR_BOUNDARY_W29_WAVE_STEP: &str = "W29-024-ERROR_BOUNDARY";

/// Honest posture — typed + `Error` landed; production gateway flip **open**.
pub const POSTURE_TAG: &str = "BOUNDARY_TYPED_STD_ERROR_PARTIAL";

/// Honest deepen fence for meta / fleet probes.
pub const HONEST_FENCE: &str = "typed_variants_landed=true|legacy_string_shims_required=true|std_error_impl_landed=true|gateway_eval_measured=false|production_wired=false|physics_green=false|master_wired=false";

/// Whether typed boundary enums are landed for apply-physics / CBF / catalog IO.
pub const TYPED_VARIANTS_LANDED: bool = true;

/// Whether legacy `From<String>` shims are still required at gateway call sites.
pub const LEGACY_STRING_SHIMS_REQUIRED: bool = true;

/// Whether unified `std::error::Error` impls are landed across all three enums.
pub const STD_ERROR_IMPL_LANDED: bool = true;

/// Whether a live gateway eval has measured boundary flip readiness.
pub const GATEWAY_EVAL_MEASURED: bool = false;

/// Honest physics GREEN claim — structural typed boundary only; no invent GREEN.
pub const ERROR_BOUNDARY_PHYSICS_GREEN: bool = false;

/// Count of non-legacy [`ApplyPhysicsError`] variants (excludes [`ApplyPhysicsError::LegacyDetail`]).
pub const APPLY_PHYSICS_TYPED_VARIANT_COUNT: usize = 5;

/// Count of non-legacy [`CbfReject`] variants (excludes [`CbfReject::LegacyDetail`]).
pub const CBF_TYPED_VARIANT_COUNT: usize = 2;

/// Count of non-legacy [`CatalogIoError`] variants (excludes [`CatalogIoError::LegacyDetail`]).
pub const CATALOG_IO_TYPED_VARIANT_COUNT: usize = 3;

/// Total boundary kinds guarded by this module.
pub const BOUNDARY_KIND_COUNT: usize = 3;

use core::fmt;
use std::error::Error;

use crate::core::dec_typestate::DecTypestateError;

/// Which IO-adjacent boundary an error enum guards.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BoundaryKind {
    /// [`crate::core::apply_physics::apply_physics_to_umst`] UMST writeback.
    ApplyPhysics,
    /// [`crate::ai::cbf::ThermodynamicCBF`] admissibility checks.
    Cbf,
    /// Lean `catalog.json` traceability partition reads.
    CatalogIo,
}

impl BoundaryKind {
    /// Stable static tag for probes / logs (not a GREEN claim).
    #[must_use]
    pub const fn as_static_tag(self) -> &'static str {
        match self {
            Self::ApplyPhysics => "apply_physics",
            Self::Cbf => "cbf",
            Self::CatalogIo => "catalog_io",
        }
    }

    /// Exhaustive kind enumeration for fence tests.
    #[must_use]
    pub const fn all() -> [Self; BOUNDARY_KIND_COUNT] {
        [Self::ApplyPhysics, Self::Cbf, Self::CatalogIo]
    }
}

/// Honest production gateway wiring — **false** until measured live eval.
#[must_use]
pub const fn error_boundary_production_wired() -> bool {
    false
}

/// Honest master-tier wiring — **false** until fleet sign-off.
#[must_use]
pub const fn error_boundary_master_wired() -> bool {
    false
}

/// Honest physics GREEN — **false**; typed boundary is structural only.
#[must_use]
pub const fn error_boundary_physics_green() -> bool {
    ERROR_BOUNDARY_PHYSICS_GREEN
}

/// Honest gateway-eval measured flag — **false** until live eval lands.
#[must_use]
pub const fn error_boundary_gateway_eval_measured() -> bool {
    GATEWAY_EVAL_MEASURED
}

/// Whether unified `std::error::Error` impls are landed (structural; not gateway GREEN).
#[must_use]
pub const fn error_boundary_std_error_impl_landed() -> bool {
    STD_ERROR_IMPL_LANDED
}

/// Compile-time fence — production flip not authorized at posture tier.
const _: () = assert!(!error_boundary_production_wired());

/// Compile-time fence — master flip not authorized at posture tier.
const _: () = assert!(!error_boundary_master_wired());

/// Compile-time fence — physics GREEN not authorized at posture tier.
const _: () = assert!(!error_boundary_physics_green());

/// Compile-time fence — gateway eval not measured at posture tier.
const _: () = assert!(!error_boundary_gateway_eval_measured());

/// Compile-time fence — typed variants claimed landed.
const _: () = assert!(TYPED_VARIANTS_LANDED);

/// Compile-time fence — legacy shims still required.
const _: () = assert!(LEGACY_STRING_SHIMS_REQUIRED);

/// Compile-time fence — std::error::Error unified impl landed at W29-024 deepen.
const _: () = assert!(STD_ERROR_IMPL_LANDED);
const _: () = assert!(error_boundary_std_error_impl_landed());

/// Typed probe for error-boundary posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorBoundaryPostureProbe {
    pub wave_step: &'static str,
    pub posture_tag: &'static str,
    pub typed_variants_landed: bool,
    pub legacy_string_shims_required: bool,
    pub std_error_impl_landed: bool,
    pub gateway_eval_measured: bool,
    pub apply_physics_typed_variant_count: usize,
    pub cbf_typed_variant_count: usize,
    pub catalog_io_typed_variant_count: usize,
    pub boundary_kind_count: usize,
    pub production_wired: bool,
    pub master_wired: bool,
    pub physics_green: bool,
    pub honest_fence: &'static str,
}

/// Build introspection probe for error-boundary done-when checks.
#[must_use]
pub const fn error_boundary_posture_probe() -> ErrorBoundaryPostureProbe {
    ErrorBoundaryPostureProbe {
        wave_step: ERROR_BOUNDARY_W29_WAVE_STEP,
        posture_tag: POSTURE_TAG,
        typed_variants_landed: TYPED_VARIANTS_LANDED,
        legacy_string_shims_required: LEGACY_STRING_SHIMS_REQUIRED,
        std_error_impl_landed: STD_ERROR_IMPL_LANDED,
        gateway_eval_measured: GATEWAY_EVAL_MEASURED,
        apply_physics_typed_variant_count: APPLY_PHYSICS_TYPED_VARIANT_COUNT,
        cbf_typed_variant_count: CBF_TYPED_VARIANT_COUNT,
        catalog_io_typed_variant_count: CATALOG_IO_TYPED_VARIANT_COUNT,
        boundary_kind_count: BOUNDARY_KIND_COUNT,
        production_wired: error_boundary_production_wired(),
        master_wired: error_boundary_master_wired(),
        physics_green: error_boundary_physics_green(),
        honest_fence: HONEST_FENCE,
    }
}

/// Typed boundary + `Error` landed; production/master/GREEN paths honestly open.
#[must_use]
pub fn error_boundary_posture_honest(probe: &ErrorBoundaryPostureProbe) -> bool {
    probe.wave_step == ERROR_BOUNDARY_W29_WAVE_STEP
        && probe.posture_tag == POSTURE_TAG
        && probe.typed_variants_landed
        && probe.legacy_string_shims_required
        && probe.std_error_impl_landed
        && !probe.gateway_eval_measured
        && probe.apply_physics_typed_variant_count == APPLY_PHYSICS_TYPED_VARIANT_COUNT
        && probe.cbf_typed_variant_count == CBF_TYPED_VARIANT_COUNT
        && probe.catalog_io_typed_variant_count == CATALOG_IO_TYPED_VARIANT_COUNT
        && probe.boundary_kind_count == BOUNDARY_KIND_COUNT
        && !probe.production_wired
        && !probe.master_wired
        && !probe.physics_green
        && probe.honest_fence.contains("typed_variants_landed=true")
        && probe
            .honest_fence
            .contains("legacy_string_shims_required=true")
        && probe.honest_fence.contains("std_error_impl_landed=true")
        && probe.honest_fence.contains("gateway_eval_measured=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master_wired=false")
        && !probe.posture_tag.to_ascii_lowercase().contains("green")
        && !probe.posture_tag.to_ascii_lowercase().contains("master")
}

/// Validate error-boundary posture honesty — fail closed on fake production / GREEN claims.
pub fn validate_error_boundary_posture_honesty() -> Result<(), &'static str> {
    let probe = error_boundary_posture_probe();
    if probe.production_wired || error_boundary_production_wired() {
        return Err("error_boundary_production_wired must stay false until gateway measured");
    }
    if probe.master_wired || error_boundary_master_wired() {
        return Err("error_boundary_master_wired must stay false until fleet sign-off");
    }
    if probe.physics_green || error_boundary_physics_green() {
        return Err("error_boundary_physics_green must stay false — no invent GREEN");
    }
    if probe.gateway_eval_measured || error_boundary_gateway_eval_measured() {
        return Err("gateway_eval_measured must stay false until live gateway eval");
    }
    if !probe.std_error_impl_landed
        || !STD_ERROR_IMPL_LANDED
        || !error_boundary_std_error_impl_landed()
    {
        return Err("STD_ERROR_IMPL_LANDED must stay true after W29-024 Error deepen");
    }
    if !probe.typed_variants_landed {
        return Err("typed_variants_landed must stay true at W29-024");
    }
    if !probe.legacy_string_shims_required {
        return Err("legacy_string_shims_required must stay true until gateway migrates");
    }
    if probe.apply_physics_typed_variant_count != APPLY_PHYSICS_TYPED_VARIANT_COUNT
        || probe.cbf_typed_variant_count != CBF_TYPED_VARIANT_COUNT
        || probe.catalog_io_typed_variant_count != CATALOG_IO_TYPED_VARIANT_COUNT
    {
        return Err("typed variant counts drifted from W29-024 fence constants");
    }
    if probe.boundary_kind_count != BOUNDARY_KIND_COUNT {
        return Err("BOUNDARY_KIND_COUNT must stay 3 at W29-024");
    }
    if !error_boundary_posture_honest(&probe) {
        return Err("error_boundary_posture_honest failed");
    }
    Ok(())
}

/// Count typed (non-legacy) arms for apply-physics — exhaustive match fence.
#[must_use]
pub fn apply_physics_typed_arm_count(err: &ApplyPhysicsError) -> Option<usize> {
    match err {
        ApplyPhysicsError::DecTypestate { .. }
        | ApplyPhysicsError::ScalarFeaturesTooSmallForDamage { .. }
        | ApplyPhysicsError::DamageWidthMismatch { .. }
        | ApplyPhysicsError::ScalarFeaturesTooSmallForTemperature { .. }
        | ApplyPhysicsError::TemperatureWidthMismatch { .. } => {
            Some(APPLY_PHYSICS_TYPED_VARIANT_COUNT)
        }
        ApplyPhysicsError::LegacyDetail { .. } => None,
    }
}

/// Count typed (non-legacy) arms for CBF — exhaustive match fence.
#[must_use]
pub fn cbf_typed_arm_count(err: &CbfReject) -> Option<usize> {
    match err {
        CbfReject::InsufficientGlobalEnergyCredit { .. }
        | CbfReject::ClausiusDuhemViolation { .. } => Some(CBF_TYPED_VARIANT_COUNT),
        CbfReject::LegacyDetail { .. } => None,
    }
}

/// Count typed (non-legacy) arms for catalog IO — exhaustive match fence.
#[must_use]
pub fn catalog_io_typed_arm_count(err: &CatalogIoError) -> Option<usize> {
    match err {
        CatalogIoError::Read { .. }
        | CatalogIoError::Json { .. }
        | CatalogIoError::MissingModulesArray => Some(CATALOG_IO_TYPED_VARIANT_COUNT),
        CatalogIoError::LegacyDetail { .. } => None,
    }
}

/// Failures from [`crate::core::apply_physics::apply_physics_to_umst`] UMST writeback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApplyPhysicsError {
    /// DEC typestate witness rejected on the live UMST bundle.
    DecTypestate {
        context: &'static str,
        source: DecTypestateError,
    },
    /// `scalar_features` width cannot index the damage channel.
    ScalarFeaturesTooSmallForDamage { width: usize, required_index: usize },
    /// Sparse damage tensor node count disagrees with UMST layout.
    DamageWidthMismatch {
        damage_width: usize,
        umst_nodes: usize,
    },
    /// `scalar_features` width cannot index the temperature channel.
    ScalarFeaturesTooSmallForTemperature { width: usize, required_index: usize },
    /// Temperature delta tensor node count disagrees with UMST layout.
    TemperatureWidthMismatch {
        delta_width: usize,
        umst_nodes: usize,
    },
    /// Legacy string shim for callers still bridging `Err(String)`.
    LegacyDetail { detail: String },
}

impl ApplyPhysicsError {
    /// Which boundary this error guards.
    #[must_use]
    pub const fn boundary_kind(&self) -> BoundaryKind {
        BoundaryKind::ApplyPhysics
    }

    /// Whether this variant is the legacy `From<String>` shim.
    #[must_use]
    pub const fn is_legacy_shim(&self) -> bool {
        matches!(self, Self::LegacyDetail { .. })
    }

    /// Stable variant tag for logs / probes (not a GREEN claim).
    #[must_use]
    pub const fn variant_tag(&self) -> &'static str {
        match self {
            Self::DecTypestate { .. } => "dec_typestate",
            Self::ScalarFeaturesTooSmallForDamage { .. } => "scalar_features_too_small_for_damage",
            Self::DamageWidthMismatch { .. } => "damage_width_mismatch",
            Self::ScalarFeaturesTooSmallForTemperature { .. } => {
                "scalar_features_too_small_for_temperature"
            }
            Self::TemperatureWidthMismatch { .. } => "temperature_width_mismatch",
            Self::LegacyDetail { .. } => "legacy_detail",
        }
    }
}

impl fmt::Display for ApplyPhysicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplyPhysicsError::DecTypestate { context, source } => {
                write!(f, "apply_physics_to_umst: {context}: {source:?}")
            }
            ApplyPhysicsError::ScalarFeaturesTooSmallForDamage {
                width,
                required_index,
            } => write!(
                f,
                "apply_physics_to_umst: scalar_features width {width} too small for SCALAR_DAMAGE={required_index}"
            ),
            ApplyPhysicsError::DamageWidthMismatch {
                damage_width,
                umst_nodes,
            } => write!(
                f,
                "apply_physics_to_umst: damage width {damage_width} != UMST nodes {umst_nodes}"
            ),
            ApplyPhysicsError::ScalarFeaturesTooSmallForTemperature {
                width,
                required_index,
            } => write!(
                f,
                "apply_physics_to_umst: scalar_features width {width} too small for SCALAR_TEMPERATURE={required_index}"
            ),
            ApplyPhysicsError::TemperatureWidthMismatch {
                delta_width,
                umst_nodes,
            } => write!(
                f,
                "apply_physics_to_umst: temperature_delta width {delta_width} != UMST nodes {umst_nodes}"
            ),
            ApplyPhysicsError::LegacyDetail { detail } => f.write_str(detail),
        }
    }
}

impl From<String> for ApplyPhysicsError {
    fn from(detail: String) -> Self {
        ApplyPhysicsError::LegacyDetail { detail }
    }
}

impl Error for ApplyPhysicsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ApplyPhysicsError::DecTypestate { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Failures from [`crate::ai::cbf::ThermodynamicCBF`] admissibility checks.
#[derive(Clone, Debug, PartialEq)]
pub enum CbfReject {
    /// Landauer erasure cost exceeds the agent's remaining energy credit.
    InsufficientGlobalEnergyCredit { required_j: f64, available_j: f64 },
    /// Clausius–Duhem inequality violated after Landauer debit.
    ClausiusDuhemViolation { generalized_entropy: f64 },
    /// Legacy string shim for callers still bridging `Err(String)`.
    LegacyDetail { detail: String },
}

impl CbfReject {
    /// Which boundary this rejection guards.
    #[must_use]
    pub const fn boundary_kind(&self) -> BoundaryKind {
        BoundaryKind::Cbf
    }

    /// Whether this variant is the legacy `From<String>` shim.
    #[must_use]
    pub const fn is_legacy_shim(&self) -> bool {
        matches!(self, Self::LegacyDetail { .. })
    }

    /// Stable variant tag for logs / probes (not a GREEN claim).
    #[must_use]
    pub const fn variant_tag(&self) -> &'static str {
        match self {
            Self::InsufficientGlobalEnergyCredit { .. } => "insufficient_global_energy_credit",
            Self::ClausiusDuhemViolation { .. } => "clausius_duhem_violation",
            Self::LegacyDetail { .. } => "legacy_detail",
        }
    }
}

impl fmt::Display for CbfReject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CbfReject::InsufficientGlobalEnergyCredit {
                required_j,
                available_j,
            } => write!(
                f,
                "REJECTED: Insufficient Global Energy Credit. Required {required_j} J, Available {available_j} J."
            ),
            CbfReject::ClausiusDuhemViolation {
                generalized_entropy,
            } => write!(
                f,
                "REJECTED: Clausius-Duhem Violation. Generalized entropy {generalized_entropy} < 0."
            ),
            CbfReject::LegacyDetail { detail } => f.write_str(detail),
        }
    }
}

impl From<String> for CbfReject {
    fn from(detail: String) -> Self {
        CbfReject::LegacyDetail { detail }
    }
}

impl Error for CbfReject {}

/// Failures reading Lean `catalog.json` for traceability partition (FP §4 IO boundary).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CatalogIoError {
    /// Filesystem read of `catalog.json` failed.
    Read { detail: String },
    /// JSON parse of `catalog.json` failed.
    Json { detail: String },
    /// Top-level `modules` array absent or not an array.
    MissingModulesArray,
    /// Legacy string shim for callers still bridging `Err(String)`.
    LegacyDetail { detail: String },
}

impl CatalogIoError {
    /// Which boundary this error guards.
    #[must_use]
    pub const fn boundary_kind(&self) -> BoundaryKind {
        BoundaryKind::CatalogIo
    }

    /// Whether this variant is the legacy `From<String>` shim.
    #[must_use]
    pub const fn is_legacy_shim(&self) -> bool {
        matches!(self, Self::LegacyDetail { .. })
    }

    /// Stable variant tag for logs / probes (not a GREEN claim).
    #[must_use]
    pub const fn variant_tag(&self) -> &'static str {
        match self {
            Self::Read { .. } => "read",
            Self::Json { .. } => "json",
            Self::MissingModulesArray => "missing_modules_array",
            Self::LegacyDetail { .. } => "legacy_detail",
        }
    }
}

impl fmt::Display for CatalogIoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CatalogIoError::Read { detail } | CatalogIoError::Json { detail } => {
                f.write_str(detail)
            }
            CatalogIoError::MissingModulesArray => {
                f.write_str("catalog.json missing modules array")
            }
            CatalogIoError::LegacyDetail { detail } => f.write_str(detail),
        }
    }
}

impl From<std::io::Error> for CatalogIoError {
    fn from(err: std::io::Error) -> Self {
        CatalogIoError::Read {
            detail: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for CatalogIoError {
    fn from(err: serde_json::Error) -> Self {
        CatalogIoError::Json {
            detail: err.to_string(),
        }
    }
}

impl From<String> for CatalogIoError {
    fn from(detail: String) -> Self {
        CatalogIoError::LegacyDetail { detail }
    }
}

impl Error for CatalogIoError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_physics_dec_typestate_display_preserves_legacy_wording() {
        let err = ApplyPhysicsError::DecTypestate {
            context: "invalid SCALAR_DAMAGE channel",
            source: DecTypestateError::ScalarChannelOutOfRange {
                index: 99,
                channel_count: 8,
            },
        };
        assert_eq!(
            err.to_string(),
            "apply_physics_to_umst: invalid SCALAR_DAMAGE channel: ScalarChannelOutOfRange { index: 99, channel_count: 8 }"
        );
        assert_eq!(err.variant_tag(), "dec_typestate");
        assert_eq!(
            apply_physics_typed_arm_count(&err),
            Some(APPLY_PHYSICS_TYPED_VARIANT_COUNT)
        );
    }

    #[test]
    fn apply_physics_damage_width_display_preserves_legacy_wording() {
        let err = ApplyPhysicsError::DamageWidthMismatch {
            damage_width: 3,
            umst_nodes: 5,
        };
        assert_eq!(
            err.to_string(),
            "apply_physics_to_umst: damage width 3 != UMST nodes 5"
        );
        assert_eq!(err.variant_tag(), "damage_width_mismatch");
    }

    #[test]
    fn cbf_insufficient_credit_display_preserves_legacy_wording() {
        let err = CbfReject::InsufficientGlobalEnergyCredit {
            required_j: 1.5,
            available_j: 0.25,
        };
        assert_eq!(
            err.to_string(),
            "REJECTED: Insufficient Global Energy Credit. Required 1.5 J, Available 0.25 J."
        );
        assert_eq!(err.variant_tag(), "insufficient_global_energy_credit");
        assert_eq!(cbf_typed_arm_count(&err), Some(CBF_TYPED_VARIANT_COUNT));
    }

    #[test]
    fn cbf_clausius_duhem_display_preserves_legacy_wording() {
        let err = CbfReject::ClausiusDuhemViolation {
            generalized_entropy: -0.01,
        };
        assert_eq!(
            err.to_string(),
            "REJECTED: Clausius-Duhem Violation. Generalized entropy -0.01 < 0."
        );
        assert_eq!(err.variant_tag(), "clausius_duhem_violation");
    }

    #[test]
    fn catalog_io_missing_modules_display_preserves_legacy_wording() {
        assert_eq!(
            CatalogIoError::MissingModulesArray.to_string(),
            "catalog.json missing modules array"
        );
        assert_eq!(
            CatalogIoError::MissingModulesArray.variant_tag(),
            "missing_modules_array"
        );
        assert_eq!(
            catalog_io_typed_arm_count(&CatalogIoError::MissingModulesArray),
            Some(CATALOG_IO_TYPED_VARIANT_COUNT)
        );
    }

    #[test]
    fn catalog_io_from_string_shim_round_trip() {
        let err = CatalogIoError::from("legacy catalog detail".to_string());
        assert_eq!(err.to_string(), "legacy catalog detail");
        assert!(err.is_legacy_shim());
        assert_eq!(err.variant_tag(), "legacy_detail");
        assert_eq!(catalog_io_typed_arm_count(&err), None);
    }

    #[test]
    fn apply_physics_scalar_features_too_small_display_preserves_legacy_wording() {
        let err = ApplyPhysicsError::ScalarFeaturesTooSmallForDamage {
            width: 2,
            required_index: 4,
        };
        assert_eq!(
            err.to_string(),
            "apply_physics_to_umst: scalar_features width 2 too small for SCALAR_DAMAGE=4"
        );
    }

    #[test]
    fn apply_physics_temperature_width_display_preserves_legacy_wording() {
        let err = ApplyPhysicsError::TemperatureWidthMismatch {
            delta_width: 7,
            umst_nodes: 9,
        };
        assert_eq!(
            err.to_string(),
            "apply_physics_to_umst: temperature_delta width 7 != UMST nodes 9"
        );
        assert_eq!(err.variant_tag(), "temperature_width_mismatch");
    }

    #[test]
    fn apply_physics_temperature_scalar_features_display_preserves_legacy_wording() {
        let err = ApplyPhysicsError::ScalarFeaturesTooSmallForTemperature {
            width: 3,
            required_index: 5,
        };
        assert_eq!(
            err.to_string(),
            "apply_physics_to_umst: scalar_features width 3 too small for SCALAR_TEMPERATURE=5"
        );
        assert_eq!(
            err.variant_tag(),
            "scalar_features_too_small_for_temperature"
        );
    }

    #[test]
    fn boundary_kind_helpers_route_to_correct_fence() {
        let apply = ApplyPhysicsError::DamageWidthMismatch {
            damage_width: 1,
            umst_nodes: 2,
        };
        let cbf = CbfReject::ClausiusDuhemViolation {
            generalized_entropy: -1.0,
        };
        let catalog = CatalogIoError::MissingModulesArray;
        assert_eq!(apply.boundary_kind(), BoundaryKind::ApplyPhysics);
        assert_eq!(cbf.boundary_kind(), BoundaryKind::Cbf);
        assert_eq!(catalog.boundary_kind(), BoundaryKind::CatalogIo);
        assert!(!apply.is_legacy_shim());
        assert!(!cbf.is_legacy_shim());
        assert!(!catalog.is_legacy_shim());
        assert!(ApplyPhysicsError::from("x".to_string()).is_legacy_shim());
        assert!(CbfReject::from("x".to_string()).is_legacy_shim());
        assert!(CatalogIoError::from("x".to_string()).is_legacy_shim());
        assert_eq!(
            apply_physics_typed_arm_count(&ApplyPhysicsError::from("x".to_string())),
            None
        );
        assert_eq!(cbf_typed_arm_count(&CbfReject::from("x".to_string())), None);
    }

    #[test]
    fn boundary_kind_static_tags_and_exhaustive_all() {
        let all = BoundaryKind::all();
        assert_eq!(all.len(), BOUNDARY_KIND_COUNT);
        assert_eq!(all[0].as_static_tag(), "apply_physics");
        assert_eq!(all[1].as_static_tag(), "cbf");
        assert_eq!(all[2].as_static_tag(), "catalog_io");
    }

    #[test]
    fn catalog_io_from_io_and_json_map_to_typed_variants() {
        let io_err = CatalogIoError::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "missing catalog.json",
        ));
        assert!(matches!(io_err, CatalogIoError::Read { .. }));
        assert_eq!(io_err.variant_tag(), "read");
        assert!(!io_err.is_legacy_shim());
        assert_eq!(
            catalog_io_typed_arm_count(&io_err),
            Some(CATALOG_IO_TYPED_VARIANT_COUNT)
        );

        let json_err = CatalogIoError::from(
            serde_json::from_str::<serde_json::Value>("not-json").unwrap_err(),
        );
        assert!(matches!(json_err, CatalogIoError::Json { .. }));
        assert_eq!(json_err.variant_tag(), "json");
        assert!(!json_err.is_legacy_shim());
    }

    #[test]
    fn error_boundary_posture_metadata_locked() {
        assert_eq!(ERROR_BOUNDARY_W29_WAVE_STEP, "W29-024-ERROR_BOUNDARY");
        assert_eq!(POSTURE_TAG, "BOUNDARY_TYPED_STD_ERROR_PARTIAL");
        assert!(TYPED_VARIANTS_LANDED);
        assert!(LEGACY_STRING_SHIMS_REQUIRED);
        assert!(STD_ERROR_IMPL_LANDED);
        assert!(error_boundary_std_error_impl_landed());
        assert!(!GATEWAY_EVAL_MEASURED);
        assert!(!ERROR_BOUNDARY_PHYSICS_GREEN);
        assert_eq!(APPLY_PHYSICS_TYPED_VARIANT_COUNT, 5);
        assert_eq!(CBF_TYPED_VARIANT_COUNT, 2);
        assert_eq!(CATALOG_IO_TYPED_VARIANT_COUNT, 3);
        assert_eq!(BOUNDARY_KIND_COUNT, 3);
        assert!(!error_boundary_production_wired());
        assert!(!error_boundary_master_wired());
        assert!(!error_boundary_physics_green());
        assert!(!error_boundary_gateway_eval_measured());
        assert!(HONEST_FENCE.contains("physics_green=false"));
        assert!(HONEST_FENCE.contains("gateway_eval_measured=false"));
        assert!(HONEST_FENCE.contains("master_wired=false"));
        assert!(HONEST_FENCE.contains("std_error_impl_landed=true"));
    }

    #[test]
    fn error_boundary_posture_probe_typed_landed_not_production() {
        let probe = error_boundary_posture_probe();
        assert_eq!(probe.wave_step, "W29-024-ERROR_BOUNDARY");
        assert!(probe.typed_variants_landed);
        assert!(probe.legacy_string_shims_required);
        assert!(probe.std_error_impl_landed);
        assert!(!probe.gateway_eval_measured);
        assert!(!probe.production_wired);
        assert!(!probe.master_wired);
        assert!(!probe.physics_green);
        assert_eq!(probe.boundary_kind_count, 3);
        assert!(error_boundary_posture_honest(&probe));
    }

    #[test]
    fn error_boundary_posture_tag_honest_not_green() {
        assert!(POSTURE_TAG.contains("PARTIAL"));
        assert!(POSTURE_TAG.contains("STD_ERROR"));
        assert!(!POSTURE_TAG.to_ascii_lowercase().contains("green"));
        assert!(!POSTURE_TAG.to_ascii_lowercase().contains("master"));
        assert!(HONEST_FENCE.contains("production_wired=false"));
        assert!(HONEST_FENCE.contains("master_wired=false"));
        assert!(HONEST_FENCE.contains("physics_green=false"));
        assert!(!HONEST_FENCE.contains("physics_green=true"));
        assert!(!HONEST_FENCE.contains("production_wired=true"));
    }

    #[test]
    fn error_boundary_validate_posture_honesty() {
        assert!(validate_error_boundary_posture_honesty().is_ok());
        assert!(!error_boundary_production_wired());
        assert!(!error_boundary_master_wired());
        assert!(!error_boundary_physics_green());
    }

    #[test]
    fn error_boundary_refuse_fake_green_probe() {
        let mut probe = error_boundary_posture_probe();
        probe.physics_green = true;
        assert!(!error_boundary_posture_honest(&probe));
        probe = error_boundary_posture_probe();
        probe.production_wired = true;
        assert!(!error_boundary_posture_honest(&probe));
        probe = error_boundary_posture_probe();
        probe.master_wired = true;
        assert!(!error_boundary_posture_honest(&probe));
        probe = error_boundary_posture_probe();
        probe.gateway_eval_measured = true;
        assert!(!error_boundary_posture_honest(&probe));
        probe = error_boundary_posture_probe();
        probe.std_error_impl_landed = false;
        assert!(!error_boundary_posture_honest(&probe));
    }

    #[test]
    fn error_boundary_honest_fence_no_green_invent() {
        let fence = HONEST_FENCE.to_ascii_lowercase();
        assert!(!fence.contains("physics_green=true"));
        assert!(!fence.contains("production_wired=true"));
        assert!(!fence.contains("master_wired=true"));
        assert!(!fence.contains("gateway_eval_measured=true"));
        assert!(!POSTURE_TAG.to_ascii_lowercase().contains("green"));
        validate_error_boundary_posture_honesty().expect("posture must stay honest at W29-024");
    }

    #[test]
    fn error_boundary_std_error_trait_and_source_chain() {
        let nested = ApplyPhysicsError::DecTypestate {
            context: "invalid SCALAR_DAMAGE channel",
            source: DecTypestateError::ScalarChannelOutOfRange {
                index: 99,
                channel_count: 8,
            },
        };
        let as_err: &dyn Error = &nested;
        assert_eq!(
            as_err.to_string(),
            "apply_physics_to_umst: invalid SCALAR_DAMAGE channel: ScalarChannelOutOfRange { index: 99, channel_count: 8 }"
        );
        let src = as_err.source().expect("DecTypestate must expose source");
        assert!(src.to_string().contains("scalar channel 99 out of range"));

        let width = ApplyPhysicsError::DamageWidthMismatch {
            damage_width: 1,
            umst_nodes: 2,
        };
        assert!((&width as &dyn Error).source().is_none());

        let cbf: &dyn Error = &CbfReject::ClausiusDuhemViolation {
            generalized_entropy: -0.5,
        };
        assert!(cbf.source().is_none());
        assert!(cbf.to_string().contains("Clausius-Duhem"));

        let catalog: &dyn Error = &CatalogIoError::MissingModulesArray;
        assert!(catalog.source().is_none());
        assert_eq!(catalog.to_string(), "catalog.json missing modules array");
        assert!(STD_ERROR_IMPL_LANDED);
        assert!(error_boundary_std_error_impl_landed());
    }
}
