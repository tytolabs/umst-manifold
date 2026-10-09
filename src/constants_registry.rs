// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Compile-time constants registry for manifold solvers: each row names its value, the evidence it
//! rests on and its typed derivation; migrated rows point at a single Rust `const` or a `umst-math`
//! re-export. THMC reaction-extent floats stay in their solver until cartridge calibration lands.

pub use umst_math::constants::derivation::Derivation;

/// One grounded numerical parameter (pure FP: copy types only).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroundedConst<T: Copy> {
    pub name: &'static str,
    pub value: T,
    pub evidence: &'static str,
    /// How the value is known, in the umst-math registry's terms (`Theorem`, `Definition`, `Policy`, `Absent`, …).
    /// `None` when the value pins `umst_constants` by code (the formal table row is its derivation) and for a
    /// test fixture literal, which grounds nothing.
    pub derivation: Option<Derivation>,
}

/// Q1-hex f32 PCG lane relative tolerance (SSOT: `hex_elasticity::HEX_PCG_REL_TOL_F32`).
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub const HEX_PCG_REL_TOL_F32_GROUNDED: GroundedConst<f32> = GroundedConst {
    name: "hex_pcg_rel_tol_f32",
    value: crate::physics::hex_elasticity::HEX_PCG_REL_TOL_F32,
    evidence: "src/physics/hex_elasticity.rs — attainable κ·ε floor (arm-A 9×8×2, 2026-06-10)",
    derivation: Some(Derivation::Policy {
        rationale: "stopping rule of the f32 Q1-hex PCG lane: the attainable condition-number times f32 epsilon floor on the arm-A 9x8x2 mesh (2026-06-10); a solver choice, not a property of the material",
    }),
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
    derivation: Some(Derivation::Policy {
        rationale: "stopping rule of the f64 Striatus PCG lane (re-grounded 2026-06-10); a solver choice, not a property of the material",
    }),
};

/// Default bar-network PCG relative tolerance (`MechanicsInnerLoopConfig` default).
pub const DEFAULT_BAR_PCG_REL_TOL: GroundedConst<f32> = GroundedConst {
    name: "mechanics_default_pcg_rel_tol",
    value: 1e-6,
    evidence: "src/physics/time_orchestration.rs MechanicsInnerLoopConfig::default",
    derivation: Some(Derivation::Policy {
        rationale: "default relative residual of the bar-network PCG (MechanicsInnerLoopConfig::default); a solver choice, not a property of the material",
    }),
};

/// Dense monolithic THMC stacked-DOF cap (SSOT: `thmc_residual::THMC_DENSE_NEWTON_MAX_STACKED_DOFS`).
#[cfg(feature = "thmc-coupled")]
pub const THMC_DENSE_NEWTON_MAX_STACKED_DOFS_GROUNDED: GroundedConst<usize> = GroundedConst {
    name: "thmc_dense_newton_max_stacked_dofs",
    value: crate::physics::solvers::THMC_DENSE_NEWTON_MAX_STACKED_DOFS,
    evidence: "src/physics/solvers/thmc_residual.rs post-3394b96",
    derivation: Some(Derivation::Policy {
        rationale: "largest stacked THMC system the dense monolithic Newton factorises; above it the sparse path runs; a memory and time bound, not physics",
    }),
};

/// Boltzmann constant (J/K), SI 2019 exact — re-export from `umst-math` when `math-constants` is on;
/// `umst_math::landauer::K_B` reads the generated formal table (`umst_constants::BOLTZMANN`).
#[cfg(feature = "math-constants")]
pub const K_BOLTZMANN_CODATA: GroundedConst<f64> = GroundedConst {
    name: "k_boltzmann_j_per_k",
    value: umst_math::constants::registry::K_BOLTZMANN_J_PER_K,
    evidence: "umst-math::constants::registry::K_BOLTZMANN_J_PER_K = umst_constants::BOLTZMANN (SI 2019 exact)",
    derivation: None,
};

/// Landauer bit energy at 300 K (J/bit): `k_B T ln 2`, the floor `Constants.SIBridge.landauerBoundSI` fixes
/// (umst-formal; the erase case of the second law with the exact SI `k_B`; Lean, Coq, Agda, Haskell parity).
/// `k_B` is `umst_math::landauer::K_B`, which reads the generated formal table (`umst_constants::BOLTZMANN`).
pub const LANDAUER_BIT_ENERGY_300K_J: GroundedConst<f64> = GroundedConst {
    name: "landauer_bit_energy_300k_j",
    value: umst_math::landauer::K_B * 300.0 * std::f64::consts::LN_2,
    evidence: "Constants.SIBridge.landauerBoundSI (umst-formal): erasing one bit at T costs at least k_B T ln 2 J; evaluated at T = 300 K with k_B = umst_constants::BOLTZMANN",
    derivation: None,
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

/// All migrated row names for `scripts/check_constants.py` (values checked in Rust unit tests).
#[must_use]
#[allow(unused_mut)] // cfg-gated `push` extends the vec when features are on
pub fn migrated_registry_names() -> Vec<&'static str> {
    let mut names = vec![
        DEFAULT_BAR_PCG_REL_TOL.name,
        LANDAUER_BIT_ENERGY_300K_J.name,
    ];
    #[cfg(any(
        feature = "topology-density-evolution",
        feature = "mechanics-voigt-cauchy"
    ))]
    {
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
    fn migrated_registry_names_always_include_bar_and_landauer() {
        let names = migrated_registry_names();
        assert!(names.contains(&"mechanics_default_pcg_rel_tol"));
        assert!(names.contains(&"landauer_bit_energy_300k_j"));
        assert!(names.len() >= 2);
    }

    #[test]
    fn default_bar_tols_match_orchestration() {
        let d = crate::physics::time_orchestration::MechanicsInnerLoopConfig::default();
        assert_eq!(DEFAULT_BAR_PCG_REL_TOL.value, d.pcg_tolerance);
    }

    #[test]
    fn landauer_300k_matches_runtime_helper() {
        let runtime = crate::constants::landauer_bit_energy_joules(300.0);
        assert!((LANDAUER_BIT_ENERGY_300K_J.value - runtime).abs() < 1e-30);
        // the umst-math Landauer kernel (k_B from the formal table) at the 300 K reference gives the same floor
        let t = ordered_float::NotNan::new(300.0).expect("finite");
        let kernel = umst_math::landauer::landauer_bit_energy_joules(t).into_inner();
        assert!((LANDAUER_BIT_ENERGY_300K_J.value - kernel).abs() <= 4.0 * f64::EPSILON * kernel);
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
    }
}
