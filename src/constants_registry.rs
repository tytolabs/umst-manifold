// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Immutable compile-time constants registry for manifold solvers (integration-contracts D3).
//!
//! **Law:** migrated rows point at a single Rust `const` or `umst-math` re-export; THMC reaction-extent
//! floats remain **TODO** until cartridge calibration lands (no duplicate literals here).

/// One grounded numerical parameter (pure FP: copy types only).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroundedConst<T: Copy> {
    pub name: &'static str,
    pub value: T,
    pub evidence: &'static str,
}

/// Striatus Q1-hex PCG iteration cap (SSOT: `q1_hex_elasticity::HEX_PCG_MAX_ITER_DEFAULT_STRIATUS`).
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub const STRIATUS_HEX_PCG_MAX_ITER: GroundedConst<usize> = GroundedConst {
    name: "hex_pcg_max_iter_default_striatus",
    value: crate::physics::q1_hex_elasticity::HEX_PCG_MAX_ITER_DEFAULT_STRIATUS,
    evidence: "src/physics/q1_hex_elasticity.rs — 2× headroom over 3960-iter sharp-field peak (2026-06-12)",
};

/// Q1-hex f32 PCG lane relative tolerance (SSOT: `q1_hex_elasticity::HEX_PCG_REL_TOL_F32`).
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub const HEX_PCG_REL_TOL_F32_GROUNDED: GroundedConst<f32> = GroundedConst {
    name: "hex_pcg_rel_tol_f32",
    value: crate::physics::q1_hex_elasticity::HEX_PCG_REL_TOL_F32,
    evidence: "src/physics/q1_hex_elasticity.rs — attainable κ·ε floor (arm-A 9×8×2, 2026-06-10)",
};

/// Q1-hex f64 Striatus lane relative tolerance (SSOT: `q1_hex_elasticity::HEX_PCG_REL_TOL_F64`).
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub const HEX_PCG_REL_TOL_F64_GROUNDED: GroundedConst<f32> = GroundedConst {
    name: "hex_pcg_rel_tol_f64",
    value: crate::physics::q1_hex_elasticity::HEX_PCG_REL_TOL_F64,
    evidence: "src/physics/q1_hex_elasticity.rs — re-grounded Striatus lane (2026-06-10)",
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

/// THMC reaction-extent floats — SSOT in domain cartridge (`material_transition.rs`).
pub const THMC_FLOATS_TODO: &[(&str, &str)] = &[(
    "UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K",
    "src/physics/solvers/thmc.rs",
)];

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
            crate::physics::q1_hex_elasticity::HEX_PCG_REL_TOL_F32
        );
        assert_eq!(
            HEX_PCG_REL_TOL_F64_GROUNDED.value,
            crate::physics::q1_hex_elasticity::HEX_PCG_REL_TOL_F64
        );
        assert_eq!(
            STRIATUS_HEX_PCG_MAX_ITER.value,
            crate::physics::q1_hex_elasticity::HEX_PCG_MAX_ITER_DEFAULT_STRIATUS
        );
    }
}
