// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! K-3 — Tier-2 measurement-derived constant derivations (§14bis.k · §0.11 CDD).
//!
//! H-9 HAL cluster batch: six `Tier1Measurement` registry rows with JSONL receipts.
//! Remaining non-HAL Tier-2 rows stay `Pending` until a later K-3 deepen.

use super::derivation::Derivation;
use super::registry::REGISTRY;
use crate::info_entropy::MIN_NEGENTROPY_FLOOR_BITS;
use crate::manifold::csg::Q_HYDRATION_J_PER_KG;
use crate::dignity::D_MAX;
use crate::median_convergence;
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

// --- K-5 frugality / UCRS policy batch (§14bis.k deepen) ---

/// Cockpit rolling-η window capacity for warmup reference (W = 32 → threshold 6).
pub const WARMUP_REFERENCE_WINDOW_CAPACITY: usize = 32;

/// SSOT mirror: `egoff/src/closed_loop.rs` `CLOSED_LOOP_MI_WARMING_STEP_BITS`.
pub const CLOSED_LOOP_MI_WARMING_STEP_BITS: f64 = 0.005;

/// SSOT mirror: `umst-ucrs/Rust/src/observation.rs` `MIN_PROMOTION_CREDIT_BITS`.
pub const MIN_PROMOTION_CREDIT_BITS: f64 = 1.0;

/// SSOT mirror: `egoff/src/provider_frugality.rs` staleness default (COCKPIT_DESIGN_BRIEF §12).
pub const DEFAULT_STALENESS_CYCLE_COUNT: f64 = 6.0;

/// `warmup_sample_threshold` — `max(3, ⌈√W⌉)` at reference **W = 32**.
pub const WARMUP_SAMPLE_THRESHOLD_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible",
    expected_value: 6.0,
};

/// `closed_loop_mi_step_per_accept` — ρ̂ MI warming debit when ring buffer is cold.
pub const CLOSED_LOOP_MI_STEP_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.RhoEstimator::rho_based_mi_formula",
    expected_value: CLOSED_LOOP_MI_WARMING_STEP_BITS,
};

/// `min_promotion_credit_bits` — UCRS inbox promotion quarantine floor.
pub const MIN_PROMOTION_CREDIT_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.CreditGreedy::credit_greedy_optimal",
    expected_value: MIN_PROMOTION_CREDIT_BITS,
};

/// `dignity_scalar_range` — operator UX upper bound (`D_MAX`).
pub const DIGNITY_SCALAR_RANGE_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Dignity::dignity_monotone_under_mi_gain",
    expected_value: D_MAX,
};

/// `staleness_cycle_count` — default ranker staleness cycles (Tier-3 policy).
pub const STALENESS_CYCLE_COUNT_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.EtaCog::eta_cog_nonneg",
    expected_value: DEFAULT_STALENESS_CYCLE_COUNT,
};

/// K-5 batch registry row names.
pub const K5_REGISTRY_ROW_NAMES: &[&str] = &[
    "warmup_sample_threshold",
    "closed_loop_mi_step_per_accept",
    "min_promotion_credit_bits",
    "dignity_scalar_range",
    "staleness_cycle_count",
];

// --- K-5b cockpit schema / ΔMI policy batch (§14bis.k deepen wave 2) ---

/// SSOT mirror: `egoff/src/cockpit/frugality.rs` `DEFAULT_MAX_DELTA_MI_BITS`.
pub const DEFAULT_MAX_DELTA_MI_BITS: f64 = 10.0;

/// SSOT mirror: `egoff/src/cockpit/audit_persist.rs` `DEFAULT_ROTATION_SLOTS`.
pub const DEFAULT_AUDIT_ROTATION_SLOTS: f64 = 3.0;

/// SSOT mirror: `egoff/src/cockpit/audit_persist.rs` `AUDIT_SCHEMA_VERSION`.
pub const COCKPIT_AUDIT_SCHEMA_VERSION_DEFAULT: f64 = 1.0;

/// SSOT mirror: `egoff/src/cockpit/hub.rs` snapshot `schema_version` (Phase M-simd).
pub const COCKPIT_SNAPSHOT_SCHEMA_VERSION_DEFAULT: f64 = 4.0;

/// `delta_mi_single_turn_cap_bits` — per-turn ΔMI ceiling (deception guard).
pub const DELTA_MI_SINGLE_TURN_CAP_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Dignity::dignity_monotone_under_mi_gain",
    expected_value: DEFAULT_MAX_DELTA_MI_BITS,
};

/// `audit_rotation_keep_count` — JSONL rotation generations (`path` … `path.N`).
pub const AUDIT_ROTATION_KEEP_COUNT_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.EtaCog::eta_cog_nonneg",
    expected_value: DEFAULT_AUDIT_ROTATION_SLOTS,
};

/// `cockpit_audit_schema_version` — JSONL envelope version (`egoff::cockpit::audit_persist`).
pub const COCKPIT_AUDIT_SCHEMA_VERSION_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Gate.gateCheckSound",
    expected_value: COCKPIT_AUDIT_SCHEMA_VERSION_DEFAULT,
};

/// `cockpit_snapshot_schema_version` — hub snapshot wire version (kernel_dispatch field).
pub const COCKPIT_SNAPSHOT_SCHEMA_VERSION_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible",
    expected_value: COCKPIT_SNAPSHOT_SCHEMA_VERSION_DEFAULT,
};

/// `eta_rolling_window_capacity` — rolling η deque capacity (`frugality::MEDIAN_WINDOW`).
pub const ETA_ROLLING_WINDOW_CAPACITY_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.OrderStatisticsBand::p25_p75_admissibility",
    expected_value: WARMUP_REFERENCE_WINDOW_CAPACITY as f64,
};

/// K-5b cockpit policy batch registry row names.
pub const K5B_REGISTRY_ROW_NAMES: &[&str] = &[
    "delta_mi_single_turn_cap_bits",
    "audit_rotation_keep_count",
    "cockpit_audit_schema_version",
    "cockpit_snapshot_schema_version",
    "eta_rolling_window_capacity",
];

// --- K-5c order-stats / hub / manifold PPO defaults (§14bis.k deepen wave 3) ---

/// SSOT mirror: `egoff::cockpit::hub::DEFAULT_COCKPIT_SAMPLE_PERIOD_MS`.
pub const DEFAULT_COCKPIT_SAMPLE_PERIOD_MS: f64 = 500.0;

/// SSOT mirror: `umst_manifold::core::emergence::DEFAULT_EMERGENCE_LAMBDA`.
pub const DEFAULT_EMERGENCE_LAMBDA: f64 = 0.1;

/// SSOT mirror: `umst_manifold::core::emergence::DEFAULT_MAX_EMERGENCE_VOXELS`.
pub const DEFAULT_MAX_EMERGENCE_VOXELS: f64 = 512.0;

/// NIST quantile level for rolling η P25 band (registry metadata).
pub const FRUGALITY_BAND_P25_QUANTILE: f64 = 0.25;

/// NIST quantile level for rolling η P75 band (registry metadata).
pub const FRUGALITY_BAND_P75_QUANTILE: f64 = 0.75;

/// `frugality_band_p25_percentile` — tracked quantile **q = 0.25** for order-stat band.
pub const FRUGALITY_BAND_P25_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.OrderStatisticsBand::p25_p75_admissibility",
    expected_value: FRUGALITY_BAND_P25_QUANTILE,
};

/// `frugality_band_p75_percentile` — tracked quantile **q = 0.75** for order-stat band.
pub const FRUGALITY_BAND_P75_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.OrderStatisticsBand::p25_p75_admissibility",
    expected_value: FRUGALITY_BAND_P75_QUANTILE,
};

/// `hub_inter_sample_period_ms` — cockpit hub fallback poll hold (ms).
pub const HUB_INTER_SAMPLE_PERIOD_MS_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible",
    expected_value: DEFAULT_COCKPIT_SAMPLE_PERIOD_MS,
};

/// `umst_manifold_ppo_info_gain_default_bits` — PMIC / negentropy floor scale (§14bis.f-I-4).
pub const PPO_INFO_GAIN_DEFAULT_BITS_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.InfoTheory::product_joint_mass",
    expected_value: MIN_NEGENTROPY_FLOOR_BITS,
};

/// `umst_manifold_emergence_lambda` — EmergenceMonitor λ default.
pub const EMERGENCE_LAMBDA_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Dignity::dignity_monotone_under_mi_gain",
    expected_value: DEFAULT_EMERGENCE_LAMBDA,
};

/// `umst_msdf_emergence_max_voxels` — default 3³ lattice cap for emergence SDF grid.
pub const MSDF_EMERGENCE_MAX_VOXELS_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.OrderStatisticsBand::order_statistic_concentration",
    expected_value: DEFAULT_MAX_EMERGENCE_VOXELS,
};

/// K-5c registry row names (6/6 for slice GREEN).
pub const K5C_REGISTRY_ROW_NAMES: &[&str] = &[
    "frugality_band_p25_percentile",
    "frugality_band_p75_percentile",
    "hub_inter_sample_period_ms",
    "umst_manifold_ppo_info_gain_default_bits",
    "umst_manifold_emergence_lambda",
    "umst_msdf_emergence_max_voxels",
];

/// Count K-5 rows with non-`Pending` derivation.
#[must_use]
pub fn k5_backfilled_count() -> usize {
    K5_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5 REGISTRY backfill landed.
#[must_use]
pub fn k5_backfill_landed() -> bool {
    k5_backfilled_count() == K5_REGISTRY_ROW_NAMES.len()
}

/// Reference-window warmup threshold (numeric SSOT for registry `expected_value`).
#[must_use]
pub fn warmup_threshold_at_reference_window() -> f64 {
    median_convergence::sqrt_window_threshold(WARMUP_REFERENCE_WINDOW_CAPACITY) as f64
}

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
        "warmup_sample_threshold" => Some(WARMUP_SAMPLE_THRESHOLD_DERIVATION),
        "closed_loop_mi_step_per_accept" => Some(CLOSED_LOOP_MI_STEP_DERIVATION),
        "min_promotion_credit_bits" => Some(MIN_PROMOTION_CREDIT_DERIVATION),
        "dignity_scalar_range" => Some(DIGNITY_SCALAR_RANGE_DERIVATION),
        "staleness_cycle_count" => Some(STALENESS_CYCLE_COUNT_DERIVATION),
        "delta_mi_single_turn_cap_bits" => Some(DELTA_MI_SINGLE_TURN_CAP_DERIVATION),
        "audit_rotation_keep_count" => Some(AUDIT_ROTATION_KEEP_COUNT_DERIVATION),
        "cockpit_audit_schema_version" => Some(COCKPIT_AUDIT_SCHEMA_VERSION_DERIVATION),
        "cockpit_snapshot_schema_version" => Some(COCKPIT_SNAPSHOT_SCHEMA_VERSION_DERIVATION),
        "eta_rolling_window_capacity" => Some(ETA_ROLLING_WINDOW_CAPACITY_DERIVATION),
        "frugality_band_p25_percentile" => Some(FRUGALITY_BAND_P25_DERIVATION),
        "frugality_band_p75_percentile" => Some(FRUGALITY_BAND_P75_DERIVATION),
        "hub_inter_sample_period_ms" => Some(HUB_INTER_SAMPLE_PERIOD_MS_DERIVATION),
        "umst_manifold_ppo_info_gain_default_bits" => Some(PPO_INFO_GAIN_DEFAULT_BITS_DERIVATION),
        "umst_manifold_emergence_lambda" => Some(EMERGENCE_LAMBDA_DERIVATION),
        "umst_msdf_emergence_max_voxels" => Some(MSDF_EMERGENCE_MAX_VOXELS_DERIVATION),
        _ => None,
    }
}

/// Count K-5c rows with non-`Pending` derivation.
#[must_use]
pub fn k5c_backfilled_count() -> usize {
    K5C_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5c REGISTRY backfill landed.
#[must_use]
pub fn k5c_backfill_landed() -> bool {
    k5c_backfilled_count() == K5C_REGISTRY_ROW_NAMES.len()
}

/// Count K-5b rows with non-`Pending` derivation.
#[must_use]
pub fn k5b_backfilled_count() -> usize {
    K5B_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5b REGISTRY backfill landed.
#[must_use]
pub fn k5b_backfill_landed() -> bool {
    k5b_backfilled_count() == K5B_REGISTRY_ROW_NAMES.len()
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

    #[test]
    fn k5_warmup_reference_matches_median_convergence() {
        assert_eq!(
            WARMUP_SAMPLE_THRESHOLD_DERIVATION,
            Derivation::Theorem {
                theorem_id: "UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible",
                expected_value: warmup_threshold_at_reference_window(),
            }
        );
    }

    #[test]
    fn k5_registry_rows_backfilled() {
        assert!(k5_backfill_landed());
        for name in K5_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5: {name} must be backfilled"
            );
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
        }
    }

    #[test]
    fn k5b_registry_rows_backfilled() {
        assert!(k5b_backfill_landed());
        for name in K5B_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5b: {name} must be backfilled"
            );
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
        }
    }

    #[test]
    fn k5c_registry_rows_backfilled() {
        assert!(k5c_backfill_landed());
        for name in K5C_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5c: {name} must be backfilled"
            );
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
            assert_eq!(entry.derivation.label(), "Theorem");
        }
    }
}
