// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MPL-2.0
//! Scalar physics port — C-ABI thermo snapshot boundary for legacy Haskell consumers.
//!
//! **Honest boundary:** Avrami–Parrott hydration, Powers strength, and cement mix thermodynamics
//! are homed in `umst-cartridge-concrete` (`formulas_port`, `chem_adapter_port`). This executor
//! module records typed absences for legacy FFI literals that lack registry rows until a
//! cartridge re-export lands (follow-up in umst-cartridges, outside this write set).

/// Legacy FFI oracle path (read-only witness for parity tests; not compiled into the port).
pub const LEGACY_SCALAR_PHYSICS_ORACLE: &str =
    "umst/umst-concrete-cartridge/crates/umst-concrete-ffi/src/scalar_physics.rs";

/// Cartridge home for cement scalar physics (hydration + Powers + mix thermo).
pub const CEMENT_SCALAR_PHYSICS_HOME: &str =
    "umst/umst-cartridges/crates/materials/umst-cartridge-concrete";

/// Worklist cell that must land cartridge→runtime re-export before this port computes mix fields.
pub const CEMENT_SCALAR_REEXPORT_CELL: &str = "PPORT-RUNTIME-SCALAR-CARTRIDGE-WIRE";

/// Machine-readable port marker.
pub const SCALAR_PHYSICS_PORT_MARKER: &str = "scalar_physics_port_v2_absent_boundary";

/// Honest physics GREEN posture for this port shard.
pub const PHYSICS_GREEN: bool = false;

/// A legacy scalar quantity still missing registry evidence (charter §3 Absent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScalarPhysicsQuantityAbsent {
    /// Human-readable quantity name (SI semantics in doc).
    pub quantity: &'static str,
    /// Unit label for the measurement still required.
    pub unit: &'static str,
    /// Worklist cell that must supply Derived, Bounded, Measured, or Cited evidence.
    pub follow_up_cell: &'static str,
}

/// Legacy FFI literals from `scalar_physics.rs` / duplicated closure — no registry row on HEAD.
pub const LEGACY_UNGROUNDED_CONSTANTS: &[ScalarPhysicsQuantityAbsent] = &[
    ScalarPhysicsQuantityAbsent {
        quantity: "cement_intrinsic_strength_mpa",
        unit: "MPa",
        follow_up_cell: "CONST-REGISTRY-CEMENT-S-INTRINSIC",
    },
    ScalarPhysicsQuantityAbsent {
        quantity: "cement_reaction_enthalpy",
        unit: "J/kg",
        follow_up_cell: "CONST-REGISTRY-CEMENT-ENTHALPY",
    },
    ScalarPhysicsQuantityAbsent {
        quantity: "hydration_arrhenius_prefactor",
        unit: "s",
        follow_up_cell: "CONST-REGISTRY-HYDRATION-KINETICS",
    },
    ScalarPhysicsQuantityAbsent {
        quantity: "hydration_activation_energy",
        unit: "J/mol",
        follow_up_cell: "CONST-REGISTRY-HYDRATION-KINETICS",
    },
    ScalarPhysicsQuantityAbsent {
        quantity: "hydration_k_ref",
        unit: "1/d",
        follow_up_cell: "CONST-REGISTRY-HYDRATION-AVRAMI",
    },
    ScalarPhysicsQuantityAbsent {
        quantity: "hydration_e_over_r",
        unit: "K",
        follow_up_cell: "CONST-REGISTRY-HYDRATION-AVRAMI",
    },
    ScalarPhysicsQuantityAbsent {
        quantity: "hydration_alpha_max_base",
        unit: "1",
        follow_up_cell: "CONST-REGISTRY-HYDRATION-AVRAMI",
    },
];

/// Refusal when executor callers reach cement material physics before cartridge re-export.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarPhysicsPortRefuse {
    /// Hydration/strength/thermo must come from umst-cartridge-concrete, not umst-runtime.
    MaterialPhysicsHomedInCartridges {
        home_crate_path: &'static str,
        follow_up_cell: &'static str,
    },
    /// A mix thermo call needs a grounded intrinsic strength constant, not a legacy literal.
    UngroundedConstant(ScalarPhysicsQuantityAbsent),
}

impl ScalarPhysicsPortRefuse {
    #[must_use]
    pub const fn material_physics_homed() -> Self {
        Self::MaterialPhysicsHomedInCartridges {
            home_crate_path: CEMENT_SCALAR_PHYSICS_HOME,
            follow_up_cell: CEMENT_SCALAR_REEXPORT_CELL,
        }
    }
}

/// C-ABI struct mirror for Haskell `Storable` consumers (layout only until cartridge wire).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CThermodynamicState {
    pub density: f64,
    pub free_energy: f64,
    pub hydration_degree: f64,
    pub strength: f64,
    pub max_strength: f64,
}

/// Hydration degree α — refused at runtime boundary (cartridge owns Avrami–Parrott).
#[must_use]
pub fn hydration_degree(
    _age_days: f32,
    _temp_c: f32,
    _scm_ratio: f32,
) -> Result<f32, ScalarPhysicsPortRefuse> {
    Err(ScalarPhysicsPortRefuse::material_physics_homed())
}

/// Powers gel-space strength — refused at runtime boundary.
#[must_use]
pub fn strength_powers(
    _wc_ratio: f32,
    _degree_hydration: f32,
    _air_content: f32,
    _intrinsic_strength: f32,
) -> Result<f32, ScalarPhysicsPortRefuse> {
    Err(ScalarPhysicsPortRefuse::material_physics_homed())
}

/// Mix thermo snapshot — refused until intrinsic strength and closure are registry-grounded.
#[must_use]
pub fn thermo_snapshot_from_mix(
    _w_c: f64,
    _alpha: f64,
    _temp: f64,
) -> Result<(), ScalarPhysicsPortRefuse> {
    Err(ScalarPhysicsPortRefuse::UngroundedConstant(
        LEGACY_UNGROUNDED_CONSTANTS[0],
    ))
}

/// C-ABI mix state — refused until [`thermo_snapshot_from_mix`] is cartridge-wired.
#[must_use]
pub fn c_state_from_mix(
    _w_c: f64,
    _alpha: f64,
    _temp: f64,
) -> Result<CThermodynamicState, ScalarPhysicsPortRefuse> {
    Err(ScalarPhysicsPortRefuse::material_physics_homed())
}

#[must_use]
pub const fn legacy_ungrounded_constant_count() -> usize {
    LEGACY_UNGROUNDED_CONSTANTS.len()
}
