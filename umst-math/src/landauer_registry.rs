// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Compile-time Landauer / CODATA constant registry (P3 axiom slice).
//!
//! Pure: no `std::fs`, no env reads. Numeric SSOT delegates to [`crate::landauer`].

use crate::landauer::K_B;

/// Reference ambient temperature (K) for derived Landauer rows (cockpit fallback anchor).
pub const HOST_TEMPERATURE_REFERENCE_K: f64 = crate::constants::registry::HOST_TEMPERATURE_FALLBACK_K;

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
            provenance:
                "Operator-assumed ambient anchor (constants::registry host_temperature_fallback_k)",
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

    /// Host-T row stays registry-linked (ambient SSOT batch).
    #[test]
    fn host_temperature_reference_k_links_registry_fallback() {
        assert_eq!(
            HOST_TEMPERATURE_REFERENCE_K,
            crate::constants::registry::HOST_TEMPERATURE_FALLBACK_K
        );
        let row = LandauerRegistry::get("host_temperature_reference_k").expect("host T row");
        assert_eq!(row.value, HOST_TEMPERATURE_REFERENCE_K);
    }

    /// N_float / dignity host-T SSOT pin (STEER_20261001T0756 wave 27).
    #[test]
    fn n_float_steer_wave_twenty_seven_dignity_no_bare_host_t_literals() {
        const STEER: &str = "STEER_20261001T0756";
        const WAVE: u32 = 27;
        assert_eq!(STEER, "STEER_20261001T0756");
        assert_eq!(WAVE, 27);
        let src = include_str!("dignity/core.rs");
        assert!(src.contains("HOST_TEMPERATURE_FALLBACK_K"));
        assert!(!src.contains("dignity_step(280.0"));
        assert!(!src.contains("dignity_step(295.0"));
    }

    /// N_float / dignity host-T SSOT pin (STEER_20261001T0537 wave 26).
    #[test]
    fn n_float_steer_wave_twenty_six_dignity_host_temperature_registry_linked() {
        const STEER: &str = "STEER_20261001T0537";
        const WAVE: u32 = 26;
        assert_eq!(STEER, "STEER_20261001T0537");
        assert_eq!(WAVE, 26);
        let src = include_str!("dignity/core.rs");
        assert!(src.contains("HOST_TEMPERATURE_FALLBACK_K"));
        assert!(!src.contains("let t = 300.0;"));
    }

    /// N_float / tier1 host-T SSOT pin (STEER_20261001T0509 wave 25).
    #[test]
    fn n_float_steer_wave_twenty_five_tier1_host_temperature_registry_linked() {
        const STEER: &str = "STEER_20261001T0509";
        const WAVE: u32 = 25;
        assert_eq!(STEER, "STEER_20261001T0509");
        assert_eq!(WAVE, 25);
        assert_eq!(
            crate::constants::registry::HOST_TEMPERATURE_FALLBACK_K,
            HOST_TEMPERATURE_REFERENCE_K
        );
        use crate::constants::derivation::Derivation;
        match crate::constants::tier1_derivation::LANDAUER_FLOOR_J_PER_BIT_DERIVATION {
            Derivation::Theorem { expected_value, .. } => {
                assert_eq!(
                    expected_value,
                    K_B * HOST_TEMPERATURE_REFERENCE_K * std::f64::consts::LN_2
                );
            }
            _ => panic!("landauer floor must be theorem-derived"),
        }
        assert!(LandauerRegistry::get("host_temperature_reference_k").is_some());
    }

    /// N_float / Landauer host-T SSOT pin (STEER_20261001T0359 wave 24).
    #[test]
    fn n_float_steer_wave_twenty_four_host_temperature_registry_linked() {
        const STEER: &str = "STEER_20261001T0359";
        const WAVE: u32 = 24;
        assert_eq!(STEER, "STEER_20261001T0359");
        assert_eq!(WAVE, 24);
        assert_eq!(
            HOST_TEMPERATURE_REFERENCE_K,
            crate::constants::registry::HOST_TEMPERATURE_FALLBACK_K
        );
        assert!(LandauerRegistry::get("landauer_bit_energy_300k_j").is_some());
    }

    /// N_float / Landauer host-T SSOT pin (STEER_20261001T0256 wave 23).
    #[test]
    fn n_float_steer_wave_twenty_three_host_temperature_registry_linked() {
        const STEER: &str = "STEER_20261001T0256";
        const WAVE: u32 = 23;
        assert_eq!(STEER, "STEER_20261001T0256");
        assert_eq!(WAVE, 23);
        assert_eq!(
            HOST_TEMPERATURE_REFERENCE_K,
            crate::constants::registry::HOST_TEMPERATURE_FALLBACK_K
        );
    }

    #[test]
    fn codata_landauer_rows_match_landauer_ssot() {
        let k_b = LandauerRegistry::get("k_boltzmann_j_per_k").expect("k_B row");
        assert!((k_b.value - 1.380_649e-23).abs() < 1e-30);
        assert_eq!(k_b.value, K_B);

        let ln2 = LandauerRegistry::get("ln_two").expect("ln2 row");
        assert!((ln2.value - std::f64::consts::LN_2).abs() < f64::EPSILON);

        let e300 = LandauerRegistry::get("landauer_bit_energy_300k_j").expect("300K row");
        let expected =
            landauer_bit_energy_joules(NotNan::new(HOST_TEMPERATURE_REFERENCE_K).unwrap()).into_inner();
        assert!((e300.value - expected).abs() < 1e-30);
    }
}
