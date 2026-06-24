//! Compile-time Landauer / CODATA constant registry (P3 axiom slice).
//!
//! Pure: no `std::fs`, no env reads. Numeric SSOT delegates to [`crate::landauer`].

use crate::landauer::K_B;

/// Reference ambient temperature (K) for derived Landauer rows (cockpit fallback anchor).
pub const HOST_TEMPERATURE_REFERENCE_K: f64 = 300.0;

/// One CODATA-grounded or Landauer-derived physical constant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LandauerConst {
    /// Stable registry identifier.
    pub name: &'static str,
    /// Numeric value in SI units given by [`Self::si_unit`].
    pub value: f64,
    /// SI unit label (human-facing).
    pub si_unit: &'static str,
    /// CODATA revision, Lean path, or derivation note.
    pub provenance: &'static str,
}

/// Immutable Landauer/CODATA registry (compile-time slice; no mutation).
#[derive(Debug)]
pub struct LandauerRegistry;

impl LandauerRegistry {
    /// Authoritative rows for Landauer accounting and η_cog denominators.
    pub const ENTRIES: &'static [LandauerConst] = &[
        LandauerConst {
            name: "k_boltzmann_j_per_k",
            value: K_B,
            si_unit: "J/K",
            provenance: "CODATA 2018; umst-math::landauer::K_B",
        },
        LandauerConst {
            name: "ln_two",
            value: std::f64::consts::LN_2,
            si_unit: "1",
            provenance: "UMST.Formal.Real.log_two_pos (ln 2 positivity chain)",
        },
        LandauerConst {
            name: "host_temperature_reference_k",
            value: HOST_TEMPERATURE_REFERENCE_K,
            si_unit: "K",
            provenance: "Operator-assumed ambient anchor (constants::registry host_temperature_fallback_k)",
        },
        LandauerConst {
            name: "landauer_bit_energy_300k_j",
            value: K_B * HOST_TEMPERATURE_REFERENCE_K * std::f64::consts::LN_2,
            si_unit: "J/bit",
            provenance: "k_B T ln 2 at 300 K; UMST.FormalDoubleSlit.LandauerBound",
        },
    ];

    /// Lookup a row by [`LandauerConst::name`].
    #[must_use]
    pub fn get(name: &str) -> Option<&'static LandauerConst> {
        Self::ENTRIES.iter().find(|e| e.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::landauer::landauer_bit_energy_joules;
    use ordered_float::NotNan;

    #[test]
    fn codata_landauer_rows_match_landauer_ssot() {
        let k_b = LandauerRegistry::get("k_boltzmann_j_per_k").expect("k_B row");
        assert!((k_b.value - 1.380_649e-23).abs() < 1e-30);
        assert_eq!(k_b.value, K_B);

        let ln2 = LandauerRegistry::get("ln_two").expect("ln2 row");
        assert!((ln2.value - std::f64::consts::LN_2).abs() < f64::EPSILON);

        let e300 = LandauerRegistry::get("landauer_bit_energy_300k_j").expect("300K row");
        let expected = landauer_bit_energy_joules(NotNan::new(300.0).unwrap()).into_inner();
        assert!((e300.value - expected).abs() < 1e-30);
    }
}
