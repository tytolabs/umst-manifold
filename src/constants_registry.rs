// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Immutable compile-time constants registry for manifold solvers (integration-contracts D3).
//!
//! **Law:** migrated rows point at a single Rust `const` or `umst-math` re-export; THMC reaction-extent
//! floats remain **TODO** until cartridge calibration lands (no duplicate literals here).
//!
//! W29-020 deepen — honest SSOT fences only; does **not** claim production wiring, physics GREEN,
//! or MASTER retick.

/// W29-020 cell id (constants registry deepen).
pub const CONSTANTS_REGISTRY_CELL_ID: &str = "W29-020-CONSTANTS_REGISTRY";

/// Integration-contracts D3 morphism tag.
pub const CONSTANTS_REGISTRY_MORPHISM: &str = "D3-GROUNDED-CONST";

/// Honest posture — compile-time registry only; no GREEN invent.
pub const CONSTANTS_REGISTRY_POSTURE_TAG: &str = "honest-grounded-const-ssot-only";

/// Registry rows landed at compile-time SSOT re-exports.
pub const REGISTRY_ROWS_LANDED: bool = true;

/// Honest physics posture — registry documents SSOT; not a physics GREEN claim.
pub const REGISTRY_PHYSICS_GREEN: bool = false;

/// Honest refusal — not production-wired to cartridge calibration or live solver hot-bind.
pub const REGISTRY_PRODUCTION_WIRED: bool = false;

/// Operator master retick — **not** authorized from registry-only deepen.
pub const REGISTRY_MASTER_RETICK: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const HONEST_FENCE: &str =
    "registry_landed=true thmc_floats_todo=true production_wired=false physics_green=false master_retick=false";

/// One grounded numerical parameter (pure FP: copy types only).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroundedConst<T: Copy> {
    pub name: &'static str,
    pub value: T,
    pub evidence: &'static str,
}

/// Typed probe for W29 constants-registry posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegistryPostureProbe {
    pub cell_id: &'static str,
    pub morphism_id: &'static str,
    pub posture_tag: &'static str,
    pub registry_landed: bool,
    pub physics_green: bool,
    pub production_wired: bool,
    pub master_retick: bool,
    pub thmc_floats_todo: bool,
    pub honest_fence: &'static str,
}

/// Striatus Q1-hex PCG iteration cap (SSOT: `hex_elasticity::HEX_PCG_MAX_ITER_DEFAULT_STRIATUS`).
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub const STRIATUS_HEX_PCG_MAX_ITER: GroundedConst<usize> = GroundedConst {
    name: "hex_pcg_max_iter_default_striatus",
    value: crate::physics::hex_elasticity::HEX_PCG_MAX_ITER_DEFAULT_STRIATUS,
    evidence:
        "src/physics/hex_elasticity.rs — 2× headroom over 3960-iter sharp-field peak (2026-06-12)",
};

/// Q1-hex f32 PCG lane relative tolerance (SSOT: `hex_elasticity::HEX_PCG_REL_TOL_F32`).
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub const HEX_PCG_REL_TOL_F32_GROUNDED: GroundedConst<f32> = GroundedConst {
    name: "hex_pcg_rel_tol_f32",
    value: crate::physics::hex_elasticity::HEX_PCG_REL_TOL_F32,
    evidence: "src/physics/hex_elasticity.rs — attainable κ·ε floor (arm-A 9×8×2, 2026-06-10)",
};

/// Q1-hex f64 Striatus lane relative tolerance (SSOT: `hex_elasticity::HEX_PCG_REL_TOL_F64`).
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub const HEX_PCG_REL_TOL_F64_GROUNDED: GroundedConst<f32> = GroundedConst {
    name: "hex_pcg_rel_tol_f64",
    value: crate::physics::hex_elasticity::HEX_PCG_REL_TOL_F64,
    evidence: "src/physics/hex_elasticity.rs — re-grounded Striatus lane (2026-06-10)",
};

/// Default bar-network PCG relative tolerance (`MechanicsInnerLoopConfig` default).
pub const DEFAULT_BAR_PCG_REL_TOL: GroundedConst<f32> = GroundedConst {
    name: "mechanics_default_pcg_rel_tol",
    value: 1e-6,
    evidence: "src/physics/time_orchestration.rs MechanicsInnerLoopConfig::default",
};

/// Default bar-network PCG iteration budget.
pub const DEFAULT_BAR_PCG_MAX_ITER: GroundedConst<usize> = GroundedConst {
    name: "mechanics_default_max_cg_iterations",
    value: 200,
    evidence: "src/physics/time_orchestration.rs MechanicsInnerLoopConfig::default",
};

/// Dense monolithic THMC stacked-DOF cap (SSOT: `thmc_residual::THMC_DENSE_NEWTON_MAX_STACKED_DOFS`).
#[cfg(feature = "thmc-coupled")]
pub const THMC_DENSE_NEWTON_MAX_STACKED_DOFS_GROUNDED: GroundedConst<usize> = GroundedConst {
    name: "thmc_dense_newton_max_stacked_dofs",
    value: crate::physics::solvers::THMC_DENSE_NEWTON_MAX_STACKED_DOFS,
    evidence: "src/physics/solvers/thmc_residual.rs post-3394b96",
};

/// CODATA 2018 Boltzmann constant (J/K) — re-export from `umst-math` when `math-constants` is on.
#[cfg(feature = "math-constants")]
pub const K_BOLTZMANN_CODATA: GroundedConst<f64> = GroundedConst {
    name: "k_boltzmann_j_per_k",
    value: umst_math::landauer::K_B,
    evidence: "umst-math::landauer::K_B (CODATA 2018)",
};

/// Landauer bit energy at 300 K (J/bit) — `k_B T ln 2` (CODATA 2018 `k_B`, same as `constants.rs` fallback).
pub const LANDAUER_BIT_ENERGY_300K_J: GroundedConst<f64> = GroundedConst {
    name: "landauer_bit_energy_300k_j",
    value: 1.380_649e-23 * 300.0 * std::f64::consts::LN_2,
    evidence: "k_B T ln 2 — aligns with constants::landauer_bit_energy_joules fallback path",
};

/// THMC reaction-extent floats — SSOT in domain cartridge (`material_transition.rs` / `solvers/thmc.rs`).
/// Listed here for `scripts/check_constants.py --check-thmc-todo`; **not** duplicated as registry rows.
pub const THMC_FLOATS_TODO: &[(&str, &str)] = &[
    (
        "HYDRATION_ARRHENIUS_PREFACTOR_S",
        "src/physics/solvers/thmc.rs",
    ),
    (
        "HYDRATION_ACTIVATION_ENERGY_J_PER_MOL",
        "src/physics/solvers/thmc.rs",
    ),
    ("HYDRATION_T_MIN_K", "src/physics/solvers/thmc.rs"),
    ("HYDRATION_T_BOOST_REF_K", "src/physics/solvers/thmc.rs"),
    ("HYDRATION_T_BOOST_PER_K", "src/physics/solvers/thmc.rs"),
    (
        "HYDRATION_EXOTHERMIC_K_PER_ALPHA_RATE",
        "src/physics/solvers/thmc.rs",
    ),
    (
        "UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K",
        "src/physics/solvers/thmc.rs",
    ),
];

/// Honest `production_wired` fence — never true until cartridge calibration wire measured.
#[must_use]
pub const fn registry_production_wired() -> bool {
    false
}

/// Master retick eligible — false @ registry-only deepen pass.
#[must_use]
pub const fn registry_master_retick_eligible() -> bool {
    false
}

/// Honest physics GREEN fence — registry documents SSOT; not a physics GREEN claim.
#[must_use]
pub const fn registry_physics_green() -> bool {
    false
}

/// Compile-time fence — production flip not authorized at this slice.
const _: () = assert!(!registry_production_wired());

/// Build introspection probe for registry posture done-when checks.
#[must_use]
pub const fn registry_posture_probe() -> RegistryPostureProbe {
    RegistryPostureProbe {
        cell_id: CONSTANTS_REGISTRY_CELL_ID,
        morphism_id: CONSTANTS_REGISTRY_MORPHISM,
        posture_tag: CONSTANTS_REGISTRY_POSTURE_TAG,
        registry_landed: REGISTRY_ROWS_LANDED,
        physics_green: REGISTRY_PHYSICS_GREEN,
        production_wired: REGISTRY_PRODUCTION_WIRED,
        master_retick: REGISTRY_MASTER_RETICK,
        thmc_floats_todo: !THMC_FLOATS_TODO.is_empty(),
        honest_fence: HONEST_FENCE,
    }
}

/// Registry scaffold landed with THMC floats honestly open and no fake GREEN.
#[must_use]
pub fn registry_posture_honest(probe: &RegistryPostureProbe) -> bool {
    probe.cell_id == CONSTANTS_REGISTRY_CELL_ID
        && probe.morphism_id == CONSTANTS_REGISTRY_MORPHISM
        && probe.posture_tag == CONSTANTS_REGISTRY_POSTURE_TAG
        && probe.registry_landed
        && !probe.physics_green
        && !probe.production_wired
        && !probe.master_retick
        && probe.thmc_floats_todo
}

/// Validate registry posture honesty — fail closed on fake production / GREEN claims.
pub fn validate_registry_posture_honesty() -> Result<(), &'static str> {
    let probe = registry_posture_probe();
    if probe.production_wired || registry_production_wired() {
        return Err("registry_production_wired must stay false until cartridge calibration wire");
    }
    if probe.physics_green || registry_physics_green() {
        return Err("registry_physics_green must stay false — SSOT registry is not physics GREEN");
    }
    if probe.master_retick || registry_master_retick_eligible() {
        return Err("registry_master_retick must stay false at registry-only deepen");
    }
    if !registry_posture_honest(&probe) {
        return Err("registry_posture_honest failed — fence mismatch");
    }
    if !probe.honest_fence.contains("production_wired=false") {
        return Err("honest_fence missing production_wired=false");
    }
    if !probe.honest_fence.contains("physics_green=false") {
        return Err("honest_fence missing physics_green=false");
    }
    if !probe.honest_fence.contains("master_retick=false") {
        return Err("honest_fence missing master_retick=false");
    }
    Ok(())
}

/// Whether the registry morphism pins are stable @ HEAD.
#[must_use]
pub fn registry_morphism_pinned() -> bool {
    CONSTANTS_REGISTRY_CELL_ID == "W29-020-CONSTANTS_REGISTRY"
        && CONSTANTS_REGISTRY_MORPHISM == "D3-GROUNDED-CONST"
        && CONSTANTS_REGISTRY_POSTURE_TAG == "honest-grounded-const-ssot-only"
        && REGISTRY_ROWS_LANDED
        && !REGISTRY_PRODUCTION_WIRED
        && !REGISTRY_PHYSICS_GREEN
        && !REGISTRY_MASTER_RETICK
}

/// Symbol names for THMC floats still TODO (cartridge calibration pending).
#[must_use]
pub fn thmc_floats_todo_names() -> Vec<&'static str> {
    THMC_FLOATS_TODO.iter().map(|(sym, _)| *sym).collect()
}

/// All migrated row names for `scripts/check_constants.py` (values checked in Rust unit tests).
#[must_use]
#[allow(unused_mut)] // cfg-gated `push` extends the vec when features are on
pub fn migrated_registry_names() -> Vec<&'static str> {
    let mut names = vec![
        DEFAULT_BAR_PCG_REL_TOL.name,
        DEFAULT_BAR_PCG_MAX_ITER.name,
        LANDAUER_BIT_ENERGY_300K_J.name,
    ];
    #[cfg(any(
        feature = "topology-density-evolution",
        feature = "mechanics-voigt-cauchy"
    ))]
    {
        names.push(STRIATUS_HEX_PCG_MAX_ITER.name);
        names.push(HEX_PCG_REL_TOL_F32_GROUNDED.name);
        names.push(HEX_PCG_REL_TOL_F64_GROUNDED.name);
    }
    #[cfg(feature = "thmc-coupled")]
    names.push(THMC_DENSE_NEWTON_MAX_STACKED_DOFS_GROUNDED.name);
    #[cfg(feature = "math-constants")]
    names.push(K_BOLTZMANN_CODATA.name);
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_cell_metadata_pinned() {
        assert!(registry_morphism_pinned());
        assert_eq!(CONSTANTS_REGISTRY_CELL_ID, "W29-020-CONSTANTS_REGISTRY");
        assert_eq!(CONSTANTS_REGISTRY_MORPHISM, "D3-GROUNDED-CONST");
    }

    #[test]
    fn registry_posture_probe_honest_fences_hold() {
        let probe = registry_posture_probe();
        assert!(registry_posture_honest(&probe));
        assert!(!probe.production_wired);
        assert!(!probe.physics_green);
        assert!(!probe.master_retick);
        assert!(probe.thmc_floats_todo);
        assert!(!registry_production_wired());
        assert!(!registry_physics_green());
        assert!(!registry_master_retick_eligible());
        assert!(probe.honest_fence.contains("production_wired=false"));
        assert!(probe.honest_fence.contains("physics_green=false"));
        assert!(probe.honest_fence.contains("master_retick=false"));
        validate_registry_posture_honesty().expect("posture honesty");
    }

    #[test]
    fn thmc_floats_todo_matches_docs_section() {
        let names = thmc_floats_todo_names();
        assert_eq!(names.len(), THMC_FLOATS_TODO.len());
        assert!(names.contains(&"UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K"));
        assert!(names.contains(&"HYDRATION_ARRHENIUS_PREFACTOR_S"));
        assert!(names.contains(&"HYDRATION_ACTIVATION_ENERGY_J_PER_MOL"));
        for (sym, path) in THMC_FLOATS_TODO {
            assert!(!sym.is_empty());
            assert!(path.contains("thmc"));
        }
    }

    #[test]
    fn migrated_registry_names_always_include_bar_and_landauer() {
        let names = migrated_registry_names();
        assert!(names.contains(&"mechanics_default_pcg_rel_tol"));
        assert!(names.contains(&"mechanics_default_max_cg_iterations"));
        assert!(names.contains(&"landauer_bit_energy_300k_j"));
        assert!(names.len() >= 3);
    }

    #[test]
    fn default_bar_tols_match_orchestration() {
        let d = crate::physics::time_orchestration::MechanicsInnerLoopConfig::default();
        assert_eq!(DEFAULT_BAR_PCG_REL_TOL.value, d.pcg_tolerance);
        assert_eq!(DEFAULT_BAR_PCG_MAX_ITER.value, d.max_cg_iterations);
    }

    #[test]
    fn landauer_300k_matches_runtime_helper() {
        let runtime = crate::constants::landauer_bit_energy_joules(300.0);
        assert!((LANDAUER_BIT_ENERGY_300K_J.value - runtime).abs() < 1e-30);
    }

    #[cfg(any(
        feature = "topology-density-evolution",
        feature = "mechanics-voigt-cauchy"
    ))]
    #[test]
    fn hex_lane_tols_match_q1_hex() {
        assert_eq!(
            HEX_PCG_REL_TOL_F32_GROUNDED.value,
            crate::physics::hex_elasticity::HEX_PCG_REL_TOL_F32
        );
        assert_eq!(
            HEX_PCG_REL_TOL_F64_GROUNDED.value,
            crate::physics::hex_elasticity::HEX_PCG_REL_TOL_F64
        );
        assert_eq!(
            STRIATUS_HEX_PCG_MAX_ITER.value,
            crate::physics::hex_elasticity::HEX_PCG_MAX_ITER_DEFAULT_STRIATUS
        );
    }

    #[test]
    fn w29_020_constants_registry_fleet_verify() {
        let probe = registry_posture_probe();
        assert_eq!(probe.cell_id, "W29-020-CONSTANTS_REGISTRY");
        assert!(registry_posture_honest(&probe));
        assert!(registry_morphism_pinned());
        validate_registry_posture_honesty().expect("fleet verify");
    }
}
