// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Compile-time grounded numerics for umst-math kernels (fleet N_float SSOT).
//!
//! Literals live here with a typed [`Derivation`]; call sites use [`GroundedConst::value`] or `*_F64`
//! shims — not bare decimals in physics modules.

#![allow(missing_docs)]

/// How a registry numeric is re-verified (CDD-shaped; not free-text evidence alone).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Derivation {
    /// Mechanised theorem id (Lean path).
    Theorem {
        /// Lean theorem identifier.
        theorem_id: &'static str,
    },
    /// Calibrated / operator measurement anchor.
    Measured {
        /// Design brief or calibration cite.
        anchor: &'static str,
    },
    /// Authority document (CODATA, IUPAC, SI, etc.).
    Cited {
        /// Human-readable authority label.
        authority: &'static str,
    },
}

/// One compile-time numeric with units and typed derivation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroundedConst<T: Copy> {
    /// Stable registry name.
    pub name: &'static str,
    /// Numeric value in [`Self::units`].
    pub value: T,
    /// SI or dimensionless unit label.
    pub units: &'static str,
    /// Typed derivation (registers the literal for fleet N_float).
    pub derivation: Derivation,
}

/// CODATA 2018 Boltzmann constant (J/K).
pub const K_BOLTZMANN_CODATA_J_PER_K: GroundedConst<f64> = GroundedConst {
    name: "k_boltzmann_j_per_k",
    value: 1.380_649e-23,
    units: "J/K",
    derivation: Derivation::Cited {
        authority: "CODATA 2018; umst-math::landauer::K_B",
    },
};
pub const K_BOLTZMANN_CODATA_J_PER_K_F64: f64 = K_BOLTZMANN_CODATA_J_PER_K.value;

/// Cockpit / Landauer ambient anchor when telemetry is absent (K).
pub const HOST_TEMPERATURE_FALLBACK_K: GroundedConst<f64> = GroundedConst {
    name: "host_temperature_fallback_k",
    value: 300.0,
    units: "K",
    derivation: Derivation::Measured {
        anchor: "constants::registry host_temperature_fallback_k; operator ambient until junction-T wired",
    },
};
pub const HOST_TEMPERATURE_FALLBACK_K_F64: f64 = HOST_TEMPERATURE_FALLBACK_K.value;

/// IUPAC / laboratory 20 °C reference (K) for Landauer energy budgets in tests and combinator witnesses.
pub const REFERENCE_TEMPERATURE_293_15_K: GroundedConst<f64> = GroundedConst {
    name: "reference_temperature_293_15_k",
    value: 293.15,
    units: "K",
    derivation: Derivation::Cited {
        authority: "20 °C laboratory reference (293.15 K)",
    },
};
pub const REFERENCE_TEMPERATURE_293_15_K_F64: f64 = REFERENCE_TEMPERATURE_293_15_K.value;

/// Admissibility transition ε (`UMST.Formal.Gate.transitionTolerance`).
pub const TRANSITION_TOLERANCE: GroundedConst<f64> = GroundedConst {
    name: "transition_tolerance",
    value: 1e-6,
    units: "—",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.Gate.transitionTolerance",
    },
};
pub const TRANSITION_TOLERANCE_F64: f64 = TRANSITION_TOLERANCE.value;

/// Runtime admissibility margin floor ε.
pub const ADMISSIBILITY_MARGIN_EPS: GroundedConst<f64> = GroundedConst {
    name: "admissibility_margin_eps",
    value: 1e-4,
    units: "—",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.Gate.gateCheckSound",
    },
};
pub const ADMISSIBILITY_MARGIN_EPS_F64: f64 = ADMISSIBILITY_MARGIN_EPS.value;

/// Bulk density jump band (kg/m³) — mirrors `gate_mass_tolerance_kg_m3` CGD row.
pub const GATE_MASS_TOLERANCE_KG_M3: GroundedConst<f64> = GroundedConst {
    name: "gate_mass_tolerance_kg_m3",
    value: 100.0,
    units: "kg/m³",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.Concrete.Gate.δMass_val",
    },
};
pub const GATE_MASS_TOLERANCE_KG_M3_F64: f64 = GATE_MASS_TOLERANCE_KG_M3.value;

/// Upper UX scale for dignity scalar (`D_MAX` in Lean).
pub const DIGNITY_D_MAX: GroundedConst<f64> = GroundedConst {
    name: "dignity_scalar_d_max",
    value: 10.0,
    units: "RCC·bits scale",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.Dignity::dignity_monotone_under_mi_gain",
    },
};
pub const DIGNITY_D_MAX_F64: f64 = DIGNITY_D_MAX.value;

/// Pearson |ρ| clamp for finite `log₂(1−ρ²)` (ρ estimator).
pub const RHO_MI_CLAMP_ABS: GroundedConst<f64> = GroundedConst {
    name: "rho_mi_clamp_abs",
    value: 0.9999,
    units: "—",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.RhoEstimator::rho_based_mi_formula",
    },
};
pub const RHO_MI_CLAMP_ABS_F64: f64 = RHO_MI_CLAMP_ABS.value;

/// Default bar-network PCG relative tolerance @ unit scale.
pub const BAR_NETWORK_CG_REL_TOL: GroundedConst<f64> = GroundedConst {
    name: "bar_network_cg_rel_tol",
    value: 1e-6,
    units: "—",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.SolverComposition::bar_network_cg_tol",
    },
};
pub const BAR_NETWORK_CG_REL_TOL_F64: f64 = BAR_NETWORK_CG_REL_TOL.value;

/// Tighter mechanics / hex regression relative tolerance @ unit scale.
pub const MECHANICS_TIGHT_CG_REL_TOL: GroundedConst<f64> = GroundedConst {
    name: "mechanics_tight_cg_rel_tol",
    value: 1e-8,
    units: "—",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.SolverComposition::mechanics_tight_cg_tol",
    },
};
pub const MECHANICS_TIGHT_CG_REL_TOL_F64: f64 = MECHANICS_TIGHT_CG_REL_TOL.value;

/// Adjoint / analytic reference relative tolerance @ unit scale.
pub const ADJOINT_REFERENCE_REL_TOL: GroundedConst<f64> = GroundedConst {
    name: "adjoint_reference_rel_tol",
    value: 1e-10,
    units: "—",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.SolverComposition::adjoint_reference_tol",
    },
};
pub const ADJOINT_REFERENCE_REL_TOL_F64: f64 = ADJOINT_REFERENCE_REL_TOL.value;

/// Symmetric finite-difference step scale (√scale · 5e-4).
pub const FINITE_DIFFERENCE_STEP_SCALE: GroundedConst<f64> = GroundedConst {
    name: "finite_difference_step_scale",
    value: 5e-4,
    units: "—",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.NumericTolerance::finite_difference_step",
    },
};
pub const FINITE_DIFFERENCE_STEP_SCALE_F64: f64 = FINITE_DIFFERENCE_STEP_SCALE.value;

/// FD step lower clamp.
pub const FINITE_DIFFERENCE_STEP_MIN: GroundedConst<f64> = GroundedConst {
    name: "finite_difference_step_min",
    value: 1e-6,
    units: "—",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.NumericTolerance::finite_difference_step",
    },
};
pub const FINITE_DIFFERENCE_STEP_MIN_F64: f64 = FINITE_DIFFERENCE_STEP_MIN.value;

/// FD step upper clamp.
pub const FINITE_DIFFERENCE_STEP_MAX: GroundedConst<f64> = GroundedConst {
    name: "finite_difference_step_max",
    value: 1e-2,
    units: "—",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.NumericTolerance::finite_difference_step",
    },
};
pub const FINITE_DIFFERENCE_STEP_MAX_F64: f64 = FINITE_DIFFERENCE_STEP_MAX.value;

/// `approx` / unit-test absolute epsilon for f64 Landauer and credit parity.
pub const APPROX_EPSILON_F64: GroundedConst<f64> = GroundedConst {
    name: "approx_epsilon_f64",
    value: 1.0e-30,
    units: "—",
    derivation: Derivation::Measured {
        anchor: "numeric_tolerance::APPROX_EPSILON_F64 regression floor",
    },
};
pub const APPROX_EPSILON_F64_VALUE: f64 = APPROX_EPSILON_F64.value;

/// Default `max_relative` for credit / Landauer debit parity tests.
pub const APPROX_MAX_RELATIVE_DEFAULT: GroundedConst<f64> = GroundedConst {
    name: "approx_max_relative_default",
    value: 1.0e-9,
    units: "—",
    derivation: Derivation::Measured {
        anchor: "numeric_tolerance::APPROX_MAX_RELATIVE_DEFAULT",
    },
};
pub const APPROX_MAX_RELATIVE_DEFAULT_F64: f64 = APPROX_MAX_RELATIVE_DEFAULT.value;

/// Loose f32 component checks (tensor spot tests).
pub const APPROX_EPSILON_F32_LOOSE: GroundedConst<f32> = GroundedConst {
    name: "approx_epsilon_f32_loose",
    value: 1.0e-6,
    units: "—",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.NumericTolerance::approx_epsilon_f32_loose",
    },
};
pub const APPROX_EPSILON_F32_LOOSE_F32: f32 = APPROX_EPSILON_F32_LOOSE.value;

/// Mid f32 component checks (DEC / bar-network spot tests).
pub const APPROX_EPSILON_F32_MID: GroundedConst<f32> = GroundedConst {
    name: "approx_epsilon_f32_mid",
    value: 1.0e-5,
    units: "—",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.NumericTolerance::approx_epsilon_f32_mid",
    },
};
pub const APPROX_EPSILON_F32_MID_F32: f32 = APPROX_EPSILON_F32_MID.value;

/// Loose f64 checks where f32 noise is absent.
pub const APPROX_EPSILON_F64_LOOSE: GroundedConst<f64> = GroundedConst {
    name: "approx_epsilon_f64_loose",
    value: 1.0e-18,
    units: "—",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.NumericTolerance::approx_epsilon_f64_loose",
    },
};
pub const APPROX_EPSILON_F64_LOOSE_F64: f64 = APPROX_EPSILON_F64_LOOSE.value;

/// Bar-network mid-tier scale factor (0.1 × default relative → ~1e-7 @ unit).
pub const MECHANICS_MID_CG_SCALE: GroundedConst<f64> = GroundedConst {
    name: "mechanics_mid_cg_scale",
    value: 0.1,
    units: "—",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.SolverComposition::mechanics_mid_cg_tol",
    },
};
pub const MECHANICS_MID_CG_SCALE_F64: f64 = MECHANICS_MID_CG_SCALE.value;

/// Edge-length divisor floor for axial strain in bar networks.
pub const EDGE_LENGTH_DIVISOR_FLOOR: GroundedConst<f32> = GroundedConst {
    name: "edge_length_divisor_floor_f32",
    value: 1.0e-30,
    units: "m",
    derivation: Derivation::Theorem {
        theorem_id: "UMST.Formal.NumericTolerance::edge_length_divisor_floor",
    },
};
pub const EDGE_LENGTH_DIVISOR_FLOOR_F32: f32 = EDGE_LENGTH_DIVISOR_FLOOR.value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grounded_landauer_k_boltzmann_matches_codata_row() {
        assert_eq!(crate::landauer::K_B, K_BOLTZMANN_CODATA_J_PER_K_F64);
        assert!(K_BOLTZMANN_CODATA_J_PER_K.derivation == Derivation::Cited {
            authority: "CODATA 2018; umst-math::landauer::K_B",
        });
    }

    #[test]
    fn grounded_host_temperature_matches_registry_row() {
        let row = HOST_TEMPERATURE_FALLBACK_K;
        assert_eq!(row.value, HOST_TEMPERATURE_FALLBACK_K_F64);
        assert!(matches!(row.derivation, Derivation::Measured { .. }));
    }

    #[test]
    fn grounded_registry_links_dignity_and_tolerance_ssot() {
        assert_eq!(crate::dignity::core::D_MAX, DIGNITY_D_MAX_F64);
        assert_eq!(
            crate::numeric_tolerance::transition_tolerance_f64(),
            TRANSITION_TOLERANCE_F64
        );
        assert_eq!(
            crate::numeric_tolerance::APPROX_EPSILON_F32_LOOSE,
            APPROX_EPSILON_F32_LOOSE_F32
        );
        assert_eq!(
            crate::numeric_tolerance::EDGE_LENGTH_DIVISOR_FLOOR_F32,
            EDGE_LENGTH_DIVISOR_FLOOR_F32
        );
    }

    #[cfg(feature = "math-constants")]
    #[test]
    fn grounded_registry_links_landauer_ambient_row() {
        assert_eq!(
            crate::landauer_registry::HOST_TEMPERATURE_REFERENCE_K,
            HOST_TEMPERATURE_FALLBACK_K_F64
        );
    }
}
