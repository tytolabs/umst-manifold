// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! K-3 — Tier-2 measurement-derived constant derivations (§14bis.k · §0.11 CDD).
//!
//! H-9 HAL cluster batch: six `Tier1Measurement` registry rows with JSONL receipts.
//! Remaining non-HAL Tier-2 rows stay `Pending` until a later K-3 deepen.

use super::derivation::Derivation;
use super::registry::REGISTRY;
use crate::manifold::csg::Q_HYDRATION_J_PER_KG;
use crate::numeric_tolerance::{
    admissibility_margin_eps_f64, gate_mass_tolerance_kg_m3_f64, transition_tolerance_f64,
};

/// Measurement receipt directory (relative to egoff repo root).
pub const MEASUREMENT_RECEIPTS_DIR: &str = ".umst-ci/measurement-receipts";

/// Methodology anchor prefix for H-9 HAL probes.
pub const HAL_METHODOLOGY_PREFIX: &str = "COCKPIT_DESIGN_BRIEF.md#hal-";

/// `hal_intel_cpu_logical_cores` — /proc/cpuinfo logical core count (H-9).
pub const HAL_LOGICAL_CORES_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/hal_intel_cpu_logical_cores.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#hal-logical-cores",
};

/// `hal_intel_cpu_l3_cache_kb` — host-measured L3 cache size (H-9 sysfs/cpuinfo).
pub const HAL_L3_CACHE_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/hal_intel_cpu_l3_cache_kb.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#hal-l3-cache-measurement",
};

/// `hal_intel_igpu_present_on_dev_host` — Intel DRM vendor probe (H-9).
pub const HAL_IGPU_PRESENT_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/hal_intel_igpu_present_on_dev_host.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#hal-igpu-present",
};

/// `hal_intel_npu_present_on_dev_host` — /sys/class/accel probe (H-9).
pub const HAL_NPU_PRESENT_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/hal_intel_npu_present_on_dev_host.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#hal-npu-present",
};

/// `hal_linux_port_count_on_dev_host` — sysfs net + USB enumeration (H-9).
pub const HAL_LINUX_PORT_COUNT_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/hal_linux_port_count_on_dev_host.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#hal-linux-port-count",
};

/// `hal_linux_ram_total_kb` — /proc/meminfo MemTotal (H-9).
pub const HAL_LINUX_RAM_TOTAL_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/hal_linux_ram_total_kb.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#hal-linux-ram-total",
};

/// `transition_tolerance` — formal Gate.transitionTolerance (SSOT [`transition_tolerance_f64`]).
pub const TRANSITION_TOLERANCE_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Gate.transitionTolerance",
    expected_value: transition_tolerance_f64(),
};

/// `admissibility_margin_eps` — Gate.gateCheckSound witness floor (SSOT [`admissibility_margin_eps_f64`]).
pub const ADMISSIBILITY_MARGIN_EPS_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Gate.gateCheckSound",
    expected_value: admissibility_margin_eps_f64(),
};

/// `gate_mass_tolerance_kg_m3` — Concrete.Gate.δMass_val (SSOT [`gate_mass_tolerance_kg_m3_f64`]).
pub const GATE_MASS_TOLERANCE_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Concrete.Gate.δMass_val",
    expected_value: gate_mass_tolerance_kg_m3_f64(),
};

/// `q_hyd_j_per_kg` — Haskell `qHydration` / formal `Q_hyd` (Helmholtz SDF gate).
pub const Q_HYD_J_PER_KG_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Concrete.Q_hyd_val",
    expected_value: Q_HYDRATION_J_PER_KG,
};

/// K-3 deepen: Tier-1 measurement row (concrete hydration enthalpy scale).
pub const K3_TIER1_MEASUREMENT_ROW_NAMES: &[&str] = &["q_hyd_j_per_kg"];

/// K-3 deepen: Tier-2 gate constants (non-HAL measurement batch).
pub const K3_TIER2_GATE_ROW_NAMES: &[&str] = &[
    "transition_tolerance",
    "admissibility_margin_eps",
    "gate_mass_tolerance_kg_m3",
];

/// K-3 H-9 HAL batch registry row names (6/6 for slice GREEN).
pub const K3_REGISTRY_ROW_NAMES: &[&str] = &[
    "hal_intel_cpu_logical_cores",
    "hal_intel_cpu_l3_cache_kb",
    "hal_intel_igpu_present_on_dev_host",
    "hal_intel_npu_present_on_dev_host",
    "hal_linux_port_count_on_dev_host",
    "hal_linux_ram_total_kb",
];

/// Lookup a K-3 batch derivation by registry row `name`.
#[must_use]
pub fn derivation_for_registry_row(name: &str) -> Option<Derivation> {
    match name {
        "hal_intel_cpu_logical_cores" => Some(HAL_LOGICAL_CORES_DERIVATION),
        "hal_intel_cpu_l3_cache_kb" => Some(HAL_L3_CACHE_DERIVATION),
        "hal_intel_igpu_present_on_dev_host" => Some(HAL_IGPU_PRESENT_DERIVATION),
        "hal_intel_npu_present_on_dev_host" => Some(HAL_NPU_PRESENT_DERIVATION),
        "hal_linux_port_count_on_dev_host" => Some(HAL_LINUX_PORT_COUNT_DERIVATION),
        "hal_linux_ram_total_kb" => Some(HAL_LINUX_RAM_TOTAL_DERIVATION),
        "transition_tolerance" => Some(TRANSITION_TOLERANCE_DERIVATION),
        "admissibility_margin_eps" => Some(ADMISSIBILITY_MARGIN_EPS_DERIVATION),
        "gate_mass_tolerance_kg_m3" => Some(GATE_MASS_TOLERANCE_DERIVATION),
        "q_hyd_j_per_kg" => Some(Q_HYD_J_PER_KG_DERIVATION),
        _ => None,
    }
}

/// Count K-3 gate-deepen rows with non-`Pending` derivation.
#[must_use]
pub fn k3_tier2_gate_backfilled_count() -> usize {
    K3_TIER2_GATE_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// Count K-3 batch rows with non-`Pending` derivation.
#[must_use]
pub fn k3_backfilled_count() -> usize {
    K3_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-3 pilot scaffold landed — one measurement row + receipt path wired (legacy alias).
#[must_use]
pub fn k3_measurement_pilot_landed() -> bool {
    k3_batch_landed()
}

/// K-3 H-9 HAL batch landed — every batch row `Derivation::Measurement`.
#[must_use]
pub fn k3_batch_landed() -> bool {
    k3_backfilled_count() == K3_REGISTRY_ROW_NAMES.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn k3_batch_derivations_are_measurement() {
        for name in K3_REGISTRY_ROW_NAMES {
            let d = derivation_for_registry_row(name).expect("lookup");
            assert_eq!(d.label(), "Measurement");
            assert!(!d.is_pending());
        }
    }

    #[test]
    fn k3_batch_registry_rows_backfilled() {
        assert!(k3_batch_landed());
        assert_eq!(k3_backfilled_count(), K3_REGISTRY_ROW_NAMES.len());
        for name in K3_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
        }
    }

    #[test]
    fn k3_tier2_gate_derivation_matches_registry_and_ssot() {
        assert_eq!(k3_tier2_gate_backfilled_count(), K3_TIER2_GATE_ROW_NAMES.len());
        for (name, expected) in [
            (
                "transition_tolerance",
                TRANSITION_TOLERANCE_DERIVATION,
            ),
            (
                "admissibility_margin_eps",
                ADMISSIBILITY_MARGIN_EPS_DERIVATION,
            ),
            (
                "gate_mass_tolerance_kg_m3",
                GATE_MASS_TOLERANCE_DERIVATION,
            ),
        ] {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == name)
                .expect("registry row");
            assert_eq!(entry.derivation, expected);
            assert!(!entry.derivation.is_pending());
        }
        assert_eq!(
            TRANSITION_TOLERANCE_DERIVATION,
            Derivation::Theorem {
                theorem_id: "UMST.Formal.Gate.transitionTolerance",
                expected_value: transition_tolerance_f64(),
            }
        );
        assert_eq!(
            ADMISSIBILITY_MARGIN_EPS_DERIVATION,
            Derivation::Theorem {
                theorem_id: "UMST.Formal.Gate.gateCheckSound",
                expected_value: admissibility_margin_eps_f64(),
            }
        );
        assert_eq!(
            GATE_MASS_TOLERANCE_DERIVATION,
            Derivation::Theorem {
                theorem_id: "UMST.Formal.Concrete.Gate.δMass_val",
                expected_value: gate_mass_tolerance_kg_m3_f64(),
            }
        );
    }

    #[test]
    fn k3_tier1_q_hyd_derivation_matches_registry_and_ssot() {
        let entry = REGISTRY
            .iter()
            .find(|e| e.name == "q_hyd_j_per_kg")
            .expect("registry row");
        assert_eq!(entry.derivation, Q_HYD_J_PER_KG_DERIVATION);
        assert_eq!(
            Q_HYD_J_PER_KG_DERIVATION,
            Derivation::Theorem {
                theorem_id: "UMST.Concrete.Q_hyd_val",
                expected_value: Q_HYDRATION_J_PER_KG,
            }
        );
        assert!((Q_HYDRATION_J_PER_KG - 450.0).abs() < f64::EPSILON);
    }
}
