//! Constant provenance taxonomy (foundation Phase 3).

/// How a load-bearing numeric constant is grounded.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstantProvenance {
    /// Derived from a mechanised theorem (Lean path in evidence).
    Derived(&'static str),
    /// Measured / calibrated with UCRS stamp.
    Measured(&'static str),
    /// Physical constant (e.g. k_B).
    Grounded(&'static str),
    /// Load-bearing without evidence — CI must fail.
    Ungrounded,
}

impl ConstantProvenance {
    #[must_use]
    pub fn is_grounded(self) -> bool {
        !matches!(self, Self::Ungrounded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_path_constants_all_grounded() {
        for (name, prov) in GATE_PATH_CONSTANTS {
            assert!(
                prov.is_grounded(),
                "UNGROUNDED load-bearing gate constant: {name}"
            );
        }
    }
}

/// Gate-path constants that must be grounded for admissibility CBF.
pub const GATE_PATH_CONSTANTS: &[(&'static str, ConstantProvenance)] = &[
    (
        "gate_mass_tolerance_kg_m3",
        ConstantProvenance::Measured(
            "bulk mix calibration band; mirrors umst-math GATE_MASS_TOLERANCE_KG_M3",
        ),
    ),
    (
        "transition_tolerance",
        ConstantProvenance::Derived("UMST.Formal.Gate.transitionTolerance"),
    ),
    (
        "admissibility_margin_eps",
        ConstantProvenance::Derived("runtime gate ε floor; AdmissibilityMargin ADMISSIBILITY_MARGIN_EPS"),
    ),
    (
        "min_promotion_credit_bits",
        ConstantProvenance::Measured("UCRS promotion quarantine; umst-ucrs observation.rs"),
    ),
];
