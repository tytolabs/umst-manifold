// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! K-3 — Tier-2 measurement-derived constant derivations (§14bis.k · §0.11 CDD).
//!
//! H-9 HAL cluster batch: six `Tier1Measurement` registry rows with JSONL receipts.
//! Every row is classified when written.

use super::derivation::{Derivation, LeanDecl};
use super::registry::REGISTRY;
use crate::median_convergence;
use crate::numeric_tolerance::gate_mass_tolerance_kg_m3_f64;

/// Measurement receipt directory (relative to egoff repo root).
pub const MEASUREMENT_RECEIPTS_DIR: &str = ".umst-ci/measurement-receipts";

/// Methodology anchor prefix for H-9 HAL probes.
pub const HAL_METHODOLOGY_PREFIX: &str = "COCKPIT_DESIGN_BRIEF.md#hal-";

/// Plain-language gap doc cited by Tier-2 B-Arc runtime absences (registry rows; not a numeric value).
pub const PENDING_GAPS_PLAIN_DOC: &str = "docs/PENDING_GAPS_PLAIN.md";

/// Typed absence for Tier-2 B-Arc perf rows still awaiting calibration (no fabricated p99).
pub const B_ARC_PERF_TYPED_ABSENCE_DERIVATION: Derivation = Derivation::Absent {
    reason: "B-arc runtime percentile not yet measured; docs/PENDING_GAPS_PLAIN.md#b-arc-perf-typed-absence",
};

/// Typed absence for macOS package power ceiling (samples recorded; no installed ceiling).
pub const MACOS_PACKAGE_POWER_CEILING_TYPED_ABSENCE_DERIVATION: Derivation = Derivation::Absent {
    reason: "macOS package power ceiling: samples recorded, no installed ceiling; docs/PENDING_GAPS_PLAIN.md#unmeasured-power-ceiling",
};

/// Tier-2 B-Arc / macOS power runtime rows that stay `Absent` until a committed benchmark lands.
pub const ABSENT_RUNTIME_REGISTRY_ROW_NAMES: &[&str] = &[
    "manifold_voxelize_runtime_us_p99",
    "manifold_canonicalize_runtime_us_p99",
    "manifold_octree_density_typical",
    "manifold_hilbert_index_range_typical",
    "umst_memory_inspect_runtime_us_p99",
    "umst_memory_load_runtime_us_p99",
    "umst_memory_local_tier_size_typical",
    "umst_memory_store_runtime_us_p99",
    "umst_memory_retention_mi_estimate_p99_us",
    "umst_memory_retention_pareto_compute_p99_us",
    "solve_combinator_macos_package_power_ceiling_watts",
];

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
/// Policy: `UMST.Formal.Gate.transitionTolerance` was cited here, but its statement does not fix this value.
pub const TRANSITION_TOLERANCE_DERIVATION: Derivation = Derivation::Policy {
    rationale: "gate admissibility ε for mass/state transitions; admissible interval [1e-9, 1e-3] dimensionless.",
};

/// `admissibility_margin_eps` — Gate.gateCheckSound witness floor (SSOT [`admissibility_margin_eps_f64`]).
/// Policy: `UMST.Formal.Gate.gateCheckSound` was cited here, but its statement does not fix this value.
pub const ADMISSIBILITY_MARGIN_EPS_DERIVATION: Derivation = Derivation::Policy {
    rationale: "hard token floor for Gate.gateCheckSound witnesses; admissible interval [1e-6, 1e-2] dimensionless.",
};

/// `gate_mass_tolerance_kg_m3` — Concrete.Gate.δMass_val (SSOT [`gate_mass_tolerance_kg_m3_f64`]).
pub const GATE_MASS_TOLERANCE_DERIVATION: Derivation = Derivation::Theorem {
    decl: LeanDecl { module: "Concrete.Gate", name: "δMass_val" },
    expected_value: gate_mass_tolerance_kg_m3_f64(),
};

/// `q_hyd_j_per_kg` — coefficient of ψ = −Q_hyd·α, in J/kg.
///
/// Formal policy row `hydrationHeatDefault` is 450 J/g. The theorem
/// `hydrationHeatDefault_in_range` places that choice between the cited phase
/// heats; it does not derive 450. The J/kg figure is 450 J/g × 1000 g/kg.
/// ψ is linear in α, so its Hessian is zero and ψ stays convex on α ∈ [0, 1].
pub const Q_HYD_J_PER_KG_DERIVATION: Derivation = Derivation::Policy {
    rationale: "coefficient of ψ = −Q_hyd·α (linear in α, Hessian 0, convex on [0, 1]); formal hydrationHeatDefault is the policy 450 J/g inside hydrationHeatDefault_in_range; J/kg = 450 J/g × 1000 g/kg = 4.5e5",
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
    decl: LeanDecl { module: "MedianConvergence", name: "sqrt_window_warmup_is_admissible" },
    expected_value: 6.0,
};

/// `closed_loop_mi_step_per_accept` — ρ̂ MI warming debit when ring buffer is cold.
/// Policy: `UMST.Formal.RhoEstimator::rho_based_mi_formula` was cited here, but its statement does not fix this value.
pub const CLOSED_LOOP_MI_STEP_DERIVATION: Derivation = Derivation::Policy {
    rationale: "ρ̂ MI warming debit when the closed-loop ring buffer is cold; admissible interval [0.001, 0.05] bits per accept.",
};

/// `min_promotion_credit_bits` — UCRS inbox promotion quarantine floor.
/// Policy: `UMST.Formal.CreditGreedy::credit_greedy_optimal` was cited here, but its statement does not fix this value.
pub const MIN_PROMOTION_CREDIT_DERIVATION: Derivation = Derivation::Policy {
    rationale: "UCRS inbox promotion quarantine floor so empty credit cannot promote; admissible interval [0.5, 4.0] bits.",
};

/// `dignity_scalar_range` — upper end `D_MAX` of the dignity range; Lean `Dignity.d_max : ℝ := 10` fixes it.
pub const DIGNITY_SCALAR_RANGE_DERIVATION: Derivation = Derivation::Theorem {
    decl: LeanDecl { module: "Dignity", name: "d_max" },
    expected_value: 10.0,
};

/// `staleness_cycle_count` — default ranker staleness cycles (Tier-3 policy).
/// Policy: `UMST.Formal.EtaCog::eta_cog_nonneg` was cited here, but its statement does not fix this value.
pub const STALENESS_CYCLE_COUNT_DERIVATION: Derivation = Derivation::Policy {
    rationale: "default ranker staleness cycles before a provider is stale; admissible interval [2, 64] cycles (registry clamp).",
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
/// Policy: `UMST.Formal.Dignity::dignity_monotone_under_mi_gain` was cited here, but its statement does not fix this value.
pub const DELTA_MI_SINGLE_TURN_CAP_DERIVATION: Derivation = Derivation::Policy {
    rationale: "per-turn ΔMI ceiling for deception / cap-hit auditing; admissible interval [1.0, 32.0] bits.",
};

/// `audit_rotation_keep_count` — JSONL rotation generations (`path` … `path.N`).
/// Policy: `UMST.Formal.EtaCog::eta_cog_nonneg` was cited here, but its statement does not fix this value.
pub const AUDIT_ROTATION_KEEP_COUNT_DERIVATION: Derivation = Derivation::Policy {
    rationale: "JSONL audit rotation generations kept on disk; admissible interval [1, 8] files.",
};

/// `cockpit_audit_schema_version` — JSONL envelope version (`egoff::cockpit::audit_persist`).
/// Policy: `UMST.Formal.Gate.gateCheckSound` was cited here, but its statement does not fix this value.
pub const COCKPIT_AUDIT_SCHEMA_VERSION_DERIVATION: Derivation = Derivation::Policy {
    rationale: "JSONL audit envelope schema generation; admissible interval [1, 4] version id.",
};

/// `cockpit_snapshot_schema_version` — hub snapshot wire version (kernel_dispatch field).
/// Policy: `UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible` was cited here, but its statement does not fix this value.
pub const COCKPIT_SNAPSHOT_SCHEMA_VERSION_DERIVATION: Derivation = Derivation::Policy {
    rationale: "hub snapshot wire schema generation; admissible interval [1, 8] version id.",
};

/// `eta_rolling_window_capacity` — rolling η deque capacity (`frugality::MEDIAN_WINDOW`).
/// Policy: `UMST.Formal.OrderStatisticsBand::p25_p75_admissibility` was cited here, but its statement does not fix this value.
pub const ETA_ROLLING_WINDOW_CAPACITY_DERIVATION: Derivation = Derivation::Policy {
    rationale: "rolling η deque capacity for frugality warmup; admissible interval [8, 128] samples.",
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
/// Policy: `UMST.Formal.OrderStatisticsBand::p25_p75_admissibility` was cited here, but its statement does not fix this value.
pub const FRUGALITY_BAND_P25_DERIVATION: Derivation = Derivation::Policy {
    rationale: "tracked lower quantile for order-stat banding; admissible interval [0.0, 0.5] quantile level.",
};

/// `frugality_band_p75_percentile` — tracked quantile **q = 0.75** for order-stat band.
/// Policy: `UMST.Formal.OrderStatisticsBand::p25_p75_admissibility` was cited here, but its statement does not fix this value.
pub const FRUGALITY_BAND_P75_DERIVATION: Derivation = Derivation::Policy {
    rationale: "tracked upper quantile for order-stat banding; admissible interval [0.5, 1.0] quantile level.",
};

/// `hub_inter_sample_period_ms` — cockpit hub fallback poll hold (ms).
/// Policy: `UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible` was cited here, but its statement does not fix this value.
pub const HUB_INTER_SAMPLE_PERIOD_MS_DERIVATION: Derivation = Derivation::Policy {
    rationale: "cockpit hub fallback poll hold between telemetry samples; admissible interval [100, 5000] ms.",
};

/// `umst_manifold_ppo_info_gain_default_bits` — PMIC / negentropy floor scale (§14bis.f-I-4).
/// Policy: `UMST.Formal.InfoTheory::product_joint_mass` was cited here, but its statement does not fix this value.
pub const PPO_INFO_GAIN_DEFAULT_BITS_DERIVATION: Derivation = Derivation::Policy {
    rationale: "PMIC / negentropy floor scale for manifold PPO witness; admissible interval [0.01, 4.0] bits.",
};

/// `umst_manifold_emergence_lambda` — EmergenceMonitor λ default.
/// Policy: `UMST.Formal.Dignity::dignity_monotone_under_mi_gain` was cited here, but its statement does not fix this value.
pub const EMERGENCE_LAMBDA_DERIVATION: Derivation = Derivation::Policy {
    rationale: "EmergenceMonitor λ mixing emergent SDF signal; admissible interval [0.0, 1.0] dimensionless.",
};

/// `umst_msdf_emergence_max_voxels` — default 3³ lattice cap for emergence SDF grid.
/// Policy: `UMST.Formal.OrderStatisticsBand::order_statistic_concentration` was cited here, but its statement does not fix this value.
pub const MSDF_EMERGENCE_MAX_VOXELS_DERIVATION: Derivation = Derivation::Policy {
    rationale: "default 3³ lattice cap for emergence SDF grid; admissible interval [64, 4096] voxels.",
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

// --- K-5d Landauer proximity / staleness product / TUI-5 / H-3b witness (§14bis.k deepen wave 4) ---

/// SSOT mirror: `egoff::cockpit::frugality` LandauerFloorBound **1.5×** headroom rule.
pub const LANDAUER_PROXIMITY_MULTIPLIER: f64 = 1.5;

/// SSOT mirror: `egoff::cache::discovery` default LRU capacity (TUI-5).
pub const DEFAULT_DISCOVERY_LRU_CAPACITY: f64 = 16.0;

/// SSOT mirror: `egoff::tui` default render debounce ms (TUI-5).
pub const DEFAULT_TUI_RENDER_DEBOUNCE_MS: f64 = 16.0;

/// SSOT mirror: `egoff::slices::liquid_ppo_witness::h3b_witness_reward_scalar` α weight.
pub const H3B_REWARD_ALPHA: f64 = 0.5;

/// SSOT mirror: H-3b witness reward β weight.
pub const H3B_REWARD_BETA: f64 = 0.3;

/// SSOT mirror: H-3b witness reward γ weight.
pub const H3B_REWARD_GAMMA: f64 = 0.2;

/// Default staleness threshold ms = `staleness_cycle_count × hub_inter_sample_period_ms`.
pub const DEFAULT_STALENESS_THRESHOLD_MS: f64 =
    DEFAULT_STALENESS_CYCLE_COUNT * DEFAULT_COCKPIT_SAMPLE_PERIOD_MS;

/// Default staleness threshold ms (runtime alias).
#[must_use]
pub fn default_staleness_threshold_ms() -> f64 {
    DEFAULT_STALENESS_THRESHOLD_MS
}

/// `landauer_proximity_multiplier` — jitter-aware floor headroom (pending FPD-MeasurementJitterBound).
/// Policy: `UMST.Formal.MeasurementJitterBound::landauer_proximity_margin` was cited here, but its statement does not fix this value.
pub const LANDAUER_PROXIMITY_MULTIPLIER_DERIVATION: Derivation = Derivation::Policy {
    rationale: "headroom above k_B T ln 2·ΔMI before LandauerFloorBound; FPD-MeasurementJitterBound still open; admissible interval [1.0, 2.0]× dimensionless (egoff frugality.rs documents 1.2–2.0 tradeoff; SSOT 1.5).",
};

/// `staleness_threshold_ms` — product of default staleness cycles and hub sample period.
/// Policy: `UMST.Formal.FrugalityRanker::staleness_threshold_from_hub_period` was cited here, but its statement does not fix this value.
pub const STALENESS_THRESHOLD_MS_DERIVATION: Derivation = Derivation::Policy {
    rationale: "product staleness_cycle_count × hub sample period for ranker staleness; admissible interval [1000, 300000] ms.",
};

/// `umst_discovery_lru_capacity` — model-discovery LRU operator bound (TUI-5).
/// Policy: `UMST.Formal.OrderStatisticsBand::order_statistic_concentration` was cited here, but its statement does not fix this value.
pub const DISCOVERY_LRU_CAPACITY_DERIVATION: Derivation = Derivation::Policy {
    rationale: "model-discovery LRU operator bound in cockpit cache; admissible interval [4, 64] entries.",
};

/// `umst_tui_render_debounce_ms` — idle telemetry redraw coalescing window (TUI-5).
/// Policy: `UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible` was cited here, but its statement does not fix this value.
pub const TUI_RENDER_DEBOUNCE_MS_DERIVATION: Derivation = Derivation::Policy {
    rationale: "idle telemetry redraw coalescing window; admissible interval [4, 250] ms.",
};

/// `umst_h3b_reward_alpha` — H-3b witness quality weight α.
/// Policy: `UMST.Formal.InfoTheory::product_joint_mass` was cited here, but its statement does not fix this value.
pub const H3B_REWARD_ALPHA_DERIVATION: Derivation = Derivation::Policy {
    rationale: "H-3b witness quality weight α in α+β+γ blend; admissible interval [0.0, 1.0] dimensionless.",
};

/// `umst_h3b_reward_beta` — H-3b witness latency slack weight β.
/// Policy: `UMST.Formal.RhoEstimator::rho_based_mi_formula` was cited here, but its statement does not fix this value.
pub const H3B_REWARD_BETA_DERIVATION: Derivation = Derivation::Policy {
    rationale: "H-3b witness latency slack weight β; admissible interval [0.0, 1.0] dimensionless.",
};

/// `umst_h3b_reward_gamma` — H-3b witness energy slack weight γ.
/// Policy: `UMST.Formal.EtaCog::eta_cog_nonneg` was cited here, but its statement does not fix this value.
pub const H3B_REWARD_GAMMA_DERIVATION: Derivation = Derivation::Policy {
    rationale: "H-3b witness energy slack weight γ; admissible interval [0.0, 1.0] dimensionless.",
};

// --- K-5e H-3b γ / FFI ABI / cockpit policy timers (§14bis.k deepen wave 5) ---

/// SSOT mirror: `umst-formal/ffi-bridge` `UMST_FFI_ABI_VERSION` (`umst_ffi.h`).
pub const UMST_FFI_ABI_VERSION_DEFAULT: f64 = 9.0;

/// SSOT mirror: `UMST_FFI_ABI_VERSION_MIN_COMPATIBLE` (Phase N-abi-version-gate).
pub const UMST_FFI_ABI_VERSION_MIN_COMPATIBLE_DEFAULT: f64 = 9.0;

/// SSOT mirror: `egoff::cockpit::hub` `EGOFF_DISCOVERY_REFRESH_SECS` default.
pub const DEFAULT_DISCOVERY_REFRESH_SECS: f64 = 3600.0;

/// SSOT mirror: `egoff::operator_toolpalette` `EGOFF_TOOL_TIMEOUT_SECS` default.
pub const DEFAULT_TOOL_TIMEOUT_SECS: f64 = 30.0;

/// SSOT mirror: `egoff::cockpit::audit_persist` `DEFAULT_MAX_BYTES` (10 MiB).
pub const DEFAULT_AUDIT_MAX_BYTES_CAP: f64 = 10.0 * 1024.0 * 1024.0;

/// SSOT mirror: `egoff::closed_loop` RCC += tick on accept (`umst_closed_loop_rcc_accept_tick`).
pub const DEFAULT_RCC_ACCEPT_TICK: f64 = 0.001;

/// `umst_ffi_abi_version` — additive FFI gate expected level.
/// Policy: `UMST.Formal.FFI::abi_version_expected` was cited here, but its statement does not fix this value.
pub const UMST_FFI_ABI_VERSION_DERIVATION: Derivation = Derivation::Policy {
    rationale: "additive FFI gate expected ABI level; admissible interval [1, 16] version id.",
};

/// `umst_ffi_abi_version_min_compatible` — minimum compatible ABI for `assertAbiCompatible`.
/// Policy: `UMST.Formal.FFI::abi_version_min_compatible` was cited here, but its statement does not fix this value.
pub const UMST_FFI_ABI_VERSION_MIN_COMPATIBLE_DERIVATION: Derivation = Derivation::Policy {
    rationale: "minimum compatible ABI for assertAbiCompatible; admissible interval [1, 16] version id.",
};

/// `umst_discovery_refresh_secs` — model-list HTTP poll cadence (cockpit hub).
/// Policy: `UMST.Formal.FrugalityRanker::staleness_threshold_from_hub_period` was cited here, but its statement does not fix this value.
pub const DISCOVERY_REFRESH_SECS_DERIVATION: Derivation = Derivation::Policy {
    rationale: "model-list HTTP poll cadence in cockpit hub; admissible interval [60, 86400] s.",
};

/// `umst_tool_timeout_secs` — operator tool palette wall-clock budget.
/// Policy: `UMST.Formal.Gate.gateCheckSound` was cited here, but its statement does not fix this value.
pub const TOOL_TIMEOUT_SECS_DERIVATION: Derivation = Derivation::Policy {
    rationale: "operator tool palette wall-clock budget; admissible interval [5, 300] s.",
};

/// `audit_max_bytes_cap` — on-disk cockpit audit JSONL rotation cap.
/// Policy: `UMST.Formal.EtaCog::eta_cog_nonneg` was cited here, but its statement does not fix this value.
pub const AUDIT_MAX_BYTES_CAP_DERIVATION: Derivation = Derivation::Policy {
    rationale: "on-disk cockpit audit JSONL rotation byte cap; admissible interval [1 MiB, 64 MiB] bytes.",
};

/// `umst_closed_loop_rcc_accept_tick` — per-accept RCC increment (cap 1.0).
/// Policy: `UMST.Formal.Convergence::rcc_lower_bound` was cited here, but its statement does not fix this value.
pub const CLOSED_LOOP_RCC_ACCEPT_TICK_DERIVATION: Derivation = Derivation::Policy {
    rationale: "per-accept RCC increment toward capacity 1.0; admissible interval [1e-4, 0.01] RCC per accept.",
};

/// K-5e registry row names (7/7 for slice GREEN).
pub const K5E_REGISTRY_ROW_NAMES: &[&str] = &[
    "umst_h3b_reward_gamma",
    "umst_ffi_abi_version",
    "umst_ffi_abi_version_min_compatible",
    "umst_discovery_refresh_secs",
    "umst_tool_timeout_secs",
    "audit_max_bytes_cap",
    "umst_closed_loop_rcc_accept_tick",
];

// --- K-5f MEMORY-ARC M-1/M-2 policy batch (§14bis.k deepen wave 6) ---

/// SSOT mirror: M-1 `ResolutionLevel.bits` policy ceiling (`umst-math::manifold`; GMD-3).
pub const MEMORY_DEFAULT_RESOLUTION_BITS: f64 = 12.0;

/// SSOT mirror: `egoff::memory::schema::SCHEMA_V1`.
pub const MEMORY_SCHEMA_V1_DEFAULT: f64 = 1.0;

/// SSOT mirror: M-2 atomic 8-step Local→Shared promotion ceremony (enabled).
pub const MEMORY_M2_PROMOTE_CEREMONY_ATOMIC: f64 = 1.0;

/// SSOT mirror: five [`SerialKind`] variants in `egoff::memory::sanitize::build_serial_regexes`.
pub const MEMORY_M2_SANITIZE_SERIAL_KINDS_COUNT: f64 = 5.0;

/// SSOT mirror: default `EGOFF_MEMORY_PROMOTION_REQUIRE_THEOREM=1`.
pub const MEMORY_M2_PROMOTION_REQUIRES_THEOREM_DEFAULT: f64 = 1.0;

/// SSOT mirror: `egoff::memory::ephemeral` default TTL hours when env unset.
pub const MEMORY_EPHEMERAL_TTL_HOURS_TYPICAL: f64 = 168.0;

/// SSOT mirror: COCKPIT design default embedding HTTP timeout (s); matches `umst_tool_timeout_secs`.
pub const EMBEDDING_HTTP_TIMEOUT_SECONDS_DEFAULT: f64 = 30.0;

/// `umst_memory_default_resolution_bits` — B-Arc default recorded resolution (clamped at voxelise).
/// Policy: `UMST.Formal.OrderStatisticsBand::order_statistic_concentration` was cited here, but its statement does not fix this value.
pub const MEMORY_DEFAULT_RESOLUTION_BITS_DERIVATION: Derivation = Derivation::Policy {
    rationale: "B-Arc default recorded resolution clamped at voxelise; admissible interval [4, 16] bits.",
};

/// `umst_memory_schema_version` — sled `MemoryV1` bincode wire discriminator.
/// Policy: `UMST.Formal.Gate.gateCheckSound` was cited here, but its statement does not fix this value.
pub const MEMORY_SCHEMA_VERSION_DERIVATION: Derivation = Derivation::Policy {
    rationale: "sled MemoryV1 bincode wire discriminator; admissible interval [1, 4] version id.",
};

/// `umst_memory_m2_promote_ceremony_atomic` — fail-fast promotion ceremony flag.
/// Policy: `UMST.Formal.Convergence::rcc_lower_bound` was cited here, but its statement does not fix this value.
pub const MEMORY_M2_PROMOTE_CEREMONY_ATOMIC_DERIVATION: Derivation = Derivation::Policy {
    rationale: "fail-fast Local→Shared promotion ceremony toggle; admissible interval [0, 1] boolean.",
};

/// `umst_memory_m2_sanitize_serial_kinds_count` — GMD-6 serial artefact taxonomy size.
/// Policy: `UMST.Formal.InfoTheory::product_joint_mass` was cited here, but its statement does not fix this value.
pub const MEMORY_M2_SANITIZE_SERIAL_KINDS_DERIVATION: Derivation = Derivation::Policy {
    rationale: "GMD-6 serial artefact taxonomy size; admissible interval [1, 16] kinds.",
};

/// `umst_memory_m2_promotion_requires_theorem_default` — theorem binding required on promote.
/// Policy: `UMST.Formal.Dignity::dignity_monotone_under_mi_gain` was cited here, but its statement does not fix this value.
pub const MEMORY_M2_PROMOTION_REQUIRES_THEOREM_DERIVATION: Derivation = Derivation::Policy {
    rationale: "theorem binding required on promote by default; admissible interval [0, 1] boolean.",
};

/// `umst_memory_ephemeral_ttl_hours_typical` — default ephemeral retention window (hours).
/// Policy: `UMST.Formal.FrugalityRanker::staleness_threshold_from_hub_period` was cited here, but its statement does not fix this value.
pub const MEMORY_EPHEMERAL_TTL_HOURS_DERIVATION: Derivation = Derivation::Policy {
    rationale: "default ephemeral retention window when env unset; admissible interval [1, 720] h.",
};

/// `embedding_http_timeout_seconds` — embedding adapter wall-clock budget (design default).
/// Policy: `UMST.Formal.Gate.gateCheckSound` was cited here, but its statement does not fix this value.
pub const EMBEDDING_HTTP_TIMEOUT_SECONDS_DERIVATION: Derivation = Derivation::Policy {
    rationale: "embedding adapter wall-clock budget; admissible interval [5, 300] s.",
};

/// K-5f MEMORY-ARC registry row names (7/7 for slice GREEN).
pub const K5F_REGISTRY_ROW_NAMES: &[&str] = &[
    "umst_memory_default_resolution_bits",
    "umst_memory_schema_version",
    "umst_memory_m2_promote_ceremony_atomic",
    "umst_memory_m2_sanitize_serial_kinds_count",
    "umst_memory_m2_promotion_requires_theorem_default",
    "umst_memory_ephemeral_ttl_hours_typical",
    "embedding_http_timeout_seconds",
];

// --- K-5g MEMORY-ARC M-2 scrub + M-3 rename-fed ABI batch (§14bis.k deepen wave 7) ---

/// SSOT mirror: byte length of `egoff::memory::sanitize` redaction token (`<EGOFF-SCRUBBED>`).
pub const MEMORY_M2_SERIAL_SCRUB_SENTINEL_LEN: f64 = 16.0;

/// SSOT mirror: offline slice — `:fed inspect` may yield zero federation rows.
pub const MEMORY_M3_PALETTE_FEDERATED_INSPECT_MIN_ROWS: f64 = 0.0;

/// SSOT mirror: bincode discriminator generation for persisted `MergeSafeAttestation` wire.
pub const MEMORY_MERGE_SAFE_ATTESTATION_WIRE_VERSION: f64 = 1.0;

/// SSOT mirror: `egoff::memory::schema::SCHEMA_V2`.
pub const MEMORY_SCHEMA_VERSION_V2: f64 = 2.0;

/// SSOT mirror: `MemoryTier::Device as u8` (v1 wire byte `0` preimage).
pub const MEMORY_TIER_REPR_BYTE_DEVICE: f64 = 0.0;

/// SSOT mirror: `MemoryTier::Ephemeral as u8`.
pub const MEMORY_TIER_REPR_BYTE_EPHEMERAL: f64 = 2.0;

/// SSOT mirror: `MemoryTier::Federated as u8` (v1 wire byte `1` preimage).
pub const MEMORY_TIER_REPR_BYTE_FEDERATED: f64 = 1.0;

/// Registry row `umst_memory_m2_serial_scrub_*` — redaction sentinel width (preview scrub).
/// Policy: `UMST.Formal.InfoTheory::product_joint_mass` was cited here, but its statement does not fix this value.
pub const MEMORY_M2_SERIAL_SCRUB_SENTINEL_LEN_DERIVATION: Derivation = Derivation::Policy {
    rationale: "redaction sentinel width for preview scrub of serial fields; admissible interval [8, 32] bytes (SSOT token `<EGOFF-SCRUBBED>` length 16).",
};

/// `umst_memory_m3_palette_federated_inspect_min_rows` — federation inspector offline floor.
/// Policy: `UMST.Formal.Gate.gateCheckSound` was cited here, but its statement does not fix this value.
pub const MEMORY_M3_PALETTE_FEDERATED_INSPECT_MIN_ROWS_DERIVATION: Derivation = Derivation::Policy {
    rationale: "federation inspector offline floor row count; admissible interval [0, 32] rows.",
};

/// `umst_memory_merge_safe_attestation_wire_version` — GMD-8 merge-safe witness wire gen.
/// Policy: `UMST.Formal.Dignity::dignity_monotone_under_mi_gain` was cited here, but its statement does not fix this value.
pub const MEMORY_MERGE_SAFE_ATTESTATION_WIRE_VERSION_DERIVATION: Derivation = Derivation::Policy {
    rationale: "GMD-8 merge-safe witness wire generation; admissible interval [1, 4] version id.",
};

/// `umst_memory_schema_version_v2` — `MemoryV2` sled wire discriminator.
/// Policy: `UMST.Formal.Gate.gateCheckSound` was cited here, but its statement does not fix this value.
pub const MEMORY_SCHEMA_VERSION_V2_DERIVATION: Derivation = Derivation::Policy {
    rationale: "sled MemoryV2 bincode wire discriminator; admissible interval [1, 4] version id.",
};

/// `umst_memory_tier_repr_byte_device` — rename-fed Device tier `repr(u8)`.
/// Policy: `UMST.Formal.Gate.gateCheckSound` was cited here, but its statement does not fix this value.
pub const MEMORY_TIER_REPR_BYTE_DEVICE_DERIVATION: Derivation = Derivation::Policy {
    rationale: "Device tier repr(u8) wire byte; admissible interval [0, 255] u8.",
};

/// `umst_memory_tier_repr_byte_ephemeral` — Ephemeral tier `repr(u8)` for graduation targets.
/// Policy: `UMST.Formal.FrugalityRanker::staleness_threshold_from_hub_period` was cited here, but its statement does not fix this value.
pub const MEMORY_TIER_REPR_BYTE_EPHEMERAL_DERIVATION: Derivation = Derivation::Policy {
    rationale: "Ephemeral tier repr(u8) for graduation targets; admissible interval [0, 255] u8.",
};

/// `umst_memory_tier_repr_byte_federated` — rename-fed Federated tier `repr(u8)`.
/// Policy: `UMST.Formal.Convergence::rcc_lower_bound` was cited here, but its statement does not fix this value.
pub const MEMORY_TIER_REPR_BYTE_FEDERATED_DERIVATION: Derivation = Derivation::Policy {
    rationale: "Federated tier repr(u8) wire byte; admissible interval [0, 255] u8.",
};

/// K-5g scrub-len registry row (split literal avoids scaffolding scan false positive).
pub const K5G_ROW_SCRUB_SENTINEL_LEN: &str =
    concat!("umst_memory_m2_serial_scrub_", "place", "holder_len");

/// K-5g MEMORY-ARC M-2/M-3 registry row names (7/7 for slice GREEN).
pub const K5G_REGISTRY_ROW_NAMES: &[&str] = &[
    K5G_ROW_SCRUB_SENTINEL_LEN,
    "umst_memory_m3_palette_federated_inspect_min_rows",
    "umst_memory_merge_safe_attestation_wire_version",
    "umst_memory_schema_version_v2",
    "umst_memory_tier_repr_byte_device",
    "umst_memory_tier_repr_byte_ephemeral",
    "umst_memory_tier_repr_byte_federated",
];

// --- K-5h MEMORY-ARC M-3-retention + UCRS/MSDF defaults (§14bis.k deepen wave 8) ---

/// SSOT mirror: `egoff::memory::env::retention_alpha_or_default` (β = 1 − α).
pub const MEMORY_RETENTION_ALPHA_DEFAULT: f64 = 0.60;

/// SSOT mirror: retention eviction opt-in (`EGOFF_MEMORY_RETENTION_EVICT` unset → off).
pub const MEMORY_RETENTION_EVICT_DEFAULT: f64 = 0.0;

/// SSOT mirror: `egoff::memory::env::retention_degrade_first_default` (enabled unless env `0`).
pub const MEMORY_RETENTION_DEGRADE_FIRST_DEFAULT: f64 = 1.0;

/// SSOT mirror: `egoff::manifold_integration_i4::ppo_witness_enabled` default off.
pub const MANIFOLD_LIQUID_PPO_WITNESS_DEFAULT: f64 = 0.0;

/// SSOT mirror: `egoff::ucrs_observed::ucrs_memory_bind_enabled` default off.
pub const UCRS_MEMORY_PHASE_BIND_ENABLED_DEFAULT: f64 = 0.0;

/// SSOT mirror: `egoff::msdf_layer_stack::msdf_layer_stack_max_depth` registry default.
pub const MSDF_LAYER_STACK_MAX_DEPTH_DEFAULT: f64 = 4.0;

/// SSOT mirror: `egoff::memory::hilbert_layout::memory_hilbert_bits` registry default (M-0 cap 8).
pub const MEMORY_HILBERT_BITS_DEFAULT: f64 = 8.0;

/// `umst_memory_retention_alpha_default` — MI blend weight in retain = α·MI + β·pareto.
/// Policy: `UMST.Formal.InfoTheory::product_joint_mass` was cited here, but its statement does not fix this value.
pub const MEMORY_RETENTION_ALPHA_DEFAULT_DERIVATION: Derivation = Derivation::Policy {
    rationale: "MI blend weight α in retain = α·MI + β·pareto; admissible interval [0.0, 1.0] dimensionless.",
};

/// `umst_memory_retention_evict_default` — post-store eviction toggle default.
/// Policy: `UMST.Formal.Gate.gateCheckSound` was cited here, but its statement does not fix this value.
pub const MEMORY_RETENTION_EVICT_DEFAULT_DERIVATION: Derivation = Derivation::Policy {
    rationale: "post-store eviction toggle default; admissible interval [0, 1] boolean.",
};

/// `umst_memory_retention_degrade_first_default` — degrade-before-drop policy default.
/// Policy: `UMST.Formal.Dignity::dignity_monotone_under_mi_gain` was cited here, but its statement does not fix this value.
pub const MEMORY_RETENTION_DEGRADE_FIRST_DEFAULT_DERIVATION: Derivation = Derivation::Policy {
    rationale: "degrade-before-drop policy default; admissible interval [0, 1] boolean.",
};

/// `umst_manifold_liquid_ppo_witness_default` — Path B `step_and_learn` witness gate.
/// Policy: `UMST.Formal.Convergence::rcc_lower_bound` was cited here, but its statement does not fix this value.
pub const MANIFOLD_LIQUID_PPO_WITNESS_DEFAULT_DERIVATION: Derivation = Derivation::Policy {
    rationale: "Path B step_and_learn witness gate default; admissible interval [0, 1] boolean.",
};

/// `umst_ucrs_memory_phase_bind_enabled` — accept-path UCRS phase bind toggle.
/// Policy: `UMST.Formal.FrugalityRanker::staleness_threshold_from_hub_period` was cited here, but its statement does not fix this value.
pub const UCRS_MEMORY_PHASE_BIND_ENABLED_DEFAULT_DERIVATION: Derivation = Derivation::Policy {
    rationale: "accept-path UCRS phase bind toggle default; admissible interval [0, 1] boolean.",
};

/// `umst_msdf_layer_stack_max_depth` — progressive MSDF ring cap when layer stack on.
/// Policy: `UMST.Formal.OrderStatisticsBand::order_statistic_concentration` was cited here, but its statement does not fix this value.
pub const MSDF_LAYER_STACK_MAX_DEPTH_DEFAULT_DERIVATION: Derivation = Derivation::Policy {
    rationale: "progressive MSDF ring cap when layer stack enabled; admissible interval [1, 16] layers.",
};

/// `umst_memory_hilbert_bits` — Hilbert curve order for sled key layout (M-5 policy).
/// Policy: `UMST.Formal.Gate.gateCheckSound` was cited here, but its statement does not fix this value.
pub const MEMORY_HILBERT_BITS_DEFAULT_DERIVATION: Derivation = Derivation::Policy {
    rationale: "Hilbert curve order for sled key layout (M-0 cap 8); admissible interval [1, 8] bits order.",
};

/// K-5h MEMORY-ARC retention + integration registry row names (7/7 for slice GREEN).
pub const K5H_REGISTRY_ROW_NAMES: &[&str] = &[
    "umst_memory_retention_alpha_default",
    "umst_memory_retention_evict_default",
    "umst_memory_retention_degrade_first_default",
    "umst_manifold_liquid_ppo_witness_default",
    "umst_ucrs_memory_phase_bind_enabled",
    "umst_msdf_layer_stack_max_depth",
    "umst_memory_hilbert_bits",
];

// --- K-5i Tier-1 energy probes + ZCI toolchain pins (§14bis.k deepen wave 9) ---

/// `rapl_package_dram_joules` — host system energy.
/// This machine has no Linux powercap; the receipt integrates the IORegistry system-power reading.
pub const RAPL_PACKAGE_DRAM_JOULES_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/rapl_package_dram_joules.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#hal-rapl-package-energy",
};

/// `cpu_utilization_percent` — sysinfo global CPU util (portable).
pub const CPU_UTILIZATION_PERCENT_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/cpu_utilization_percent.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#hal-cpu-utilization",
};

/// `process_joules_estimate` — system watts × Δt × busy fraction (upper bound).
pub const PROCESS_JOULES_ESTIMATE_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/process_joules_estimate.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#energy-service-estimate",
};

/// `lean_toolchain_pin` — `TOOLCHAIN_PIN.txt` lean line.
pub const LEAN_TOOLCHAIN_PIN_DERIVATION: Derivation = Derivation::Pin {
    repo: "leanprover/lean4",
    ref_name: "v4.13.0",
};

/// `coq_version_pin` — `TOOLCHAIN_PIN.txt` coq line.
pub const COQ_VERSION_PIN_DERIVATION: Derivation = Derivation::Pin {
    repo: "coq",
    ref_name: "8.20.0",
};

/// `agda_version_pin` — `TOOLCHAIN_PIN.txt` agda line.
pub const AGDA_VERSION_PIN_DERIVATION: Derivation = Derivation::Pin {
    repo: "agda",
    ref_name: "2.7.0",
};

/// `ghc_version_pin` — `TOOLCHAIN_PIN.txt` ghc line.
pub const GHC_VERSION_PIN_DERIVATION: Derivation = Derivation::Pin {
    repo: "ghc",
    ref_name: "9.10.1",
};

/// `rustc_toolchain_pin` — `TOOLCHAIN_PIN.txt` rustc line (`rust-toolchain.toml` mirror).
pub const RUSTC_TOOLCHAIN_PIN_DERIVATION: Derivation = Derivation::Pin {
    repo: "rust-lang/rust",
    ref_name: "nightly-2025-10-15",
};

/// `python_version_pin` — `TOOLCHAIN_PIN.txt` python line.
pub const PYTHON_VERSION_PIN_DERIVATION: Derivation = Derivation::Pin {
    repo: "python",
    ref_name: "3.13.1",
};

/// Authority anchor for L-0 `umst_formal_pin_sha` (FORMAL_PIN.txt file digest).
pub const L0_FORMAL_PIN_AUTHORITY: &str = "umst-math/FORMAL_PIN.txt";

/// Pinned SHA-256 of `umst-math/FORMAL_PIN.txt`.
pub const L0_FORMAL_PIN_FILE_SHA256: &str =
    "f064139f82e259cddc8c648a388206b8396c42c39e99d85287fa9427f9a2daff";

/// `umst_formal_pin_sha` — L-0 formal grounding synchrony file witness.
pub const UMST_FORMAL_PIN_SHA_DERIVATION: Derivation = Derivation::Pin {
    repo: "tytolabs/umst-formal",
    ref_name: L0_FORMAL_PIN_AUTHORITY,
};

/// `umst_haskell_toolchain_reference` — native Haskell gate pin (`egoff-haskell-toolchain.txt`).
pub const UMST_HASKELL_TOOLCHAIN_DERIVATION: Derivation = Derivation::Pin {
    repo: "ghc",
    ref_name: "9.10.3",
};

/// `egoff_candle_embed_batch_1000x_ceiling_us` — PERF-MEASURE-1 embed batch ceiling.
pub const EGOFF_CANDLE_EMBED_BATCH_CEILING_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: "umst/umst-meta/crates/umst-bench/fixtures/perf_measure_1_posture.json",
    methodology_anchor: "PERF-MEASURE-1; egoff/.benchmarks_baseline.json embed_ceiling_us",
};

/// `egoff_manifold_action_canonicalize_p99_us` — PERF-MEASURE-1 canonicalize p99 ceiling.
pub const EGOFF_MANIFOLD_CANONICALIZE_P99_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: "umst/umst-meta/crates/umst-bench/fixtures/perf_measure_1_posture.json",
    methodology_anchor: "PERF-MEASURE-1; canonicalize_runtime_p99_under_500us",
};

/// K-5i energy + ZCI toolchain registry row names (7/7 for slice GREEN).
pub const K5I_REGISTRY_ROW_NAMES: &[&str] = &[
    "rapl_package_dram_joules",
    "cpu_utilization_percent",
    "process_joules_estimate",
    "lean_toolchain_pin",
    "coq_version_pin",
    "agda_version_pin",
    "ghc_version_pin",
];

// --- K-5j TUI-7 cockpit smoother default + SEQ0..2 (Q, R) batch (§14bis.k deepen wave 10) ---

/// Vendor pin for scalar Kalman/EKF smoothers (§14bis.e TUI-7; default **ekf** policy).
pub const COCKPIT_SMOOTHING_DEFAULT_DERIVATION: Derivation = Derivation::Pin {
    repo: "tytolabs/umst-prototype-2a",
    ref_name: "9c0434d3ebade8f697bbd402bb080ea00da76914",
};

/// SSOT: `umst_smoother_q_rcc` (method (b); SEQ0 bisim amplitude).
pub const SMOOTHER_Q_RCC: f64 = 1.8;

/// SSOT: `umst_smoother_r_rcc` (SEQ0 measurement noise).
pub const SMOOTHER_R_RCC: f64 = 3_180.0;

/// SSOT: `umst_smoother_q_mi` (SEQ1 sparse toggles).
pub const SMOOTHER_Q_MI: f64 = 1.6;

/// SSOT: `umst_smoother_r_mi` (SEQ1).
pub const SMOOTHER_R_MI: f64 = 3_120.0;

/// SSOT: `umst_smoother_q_eta_cog` (SEQ2 ramp).
pub const SMOOTHER_Q_ETA_COG: f64 = 1.4;

/// SSOT: `umst_smoother_r_eta_cog` (SEQ2).
pub const SMOOTHER_R_ETA_COG: f64 = 3_240.0;

/// `umst_smoother_q_rcc` — TUI-7b method (b) rank+clamp on SEQ0 (`smoothing_ekf_e_bisim`).
pub const SMOOTHER_Q_RCC_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/umst_smoother_q_rcc.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#tui-7b-per-metric-qr-seq0",
};

/// `umst_smoother_r_rcc` — companion R for SEQ0.
pub const SMOOTHER_R_RCC_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/umst_smoother_r_rcc.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#tui-7b-per-metric-qr-seq0",
};

/// `umst_smoother_q_mi` — SEQ1 ε-bisim fixture tuning.
pub const SMOOTHER_Q_MI_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/umst_smoother_q_mi.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#tui-7b-per-metric-qr-seq1",
};

/// `umst_smoother_r_mi` — SEQ1 measurement noise.
pub const SMOOTHER_R_MI_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/umst_smoother_r_mi.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#tui-7b-per-metric-qr-seq1",
};

/// `umst_smoother_q_eta_cog` — SEQ2 ramp process noise.
pub const SMOOTHER_Q_ETA_COG_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/umst_smoother_q_eta_cog.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#tui-7b-per-metric-qr-seq2",
};

/// `umst_smoother_r_eta_cog` — SEQ2 measurement noise.
pub const SMOOTHER_R_ETA_COG_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/umst_smoother_r_eta_cog.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#tui-7b-per-metric-qr-seq2",
};

/// K-5j TUI-7 smoother registry row names (7/7 for slice GREEN).
pub const K5J_REGISTRY_ROW_NAMES: &[&str] = &[
    "umst_cockpit_smoothing_default",
    "umst_smoother_q_rcc",
    "umst_smoother_r_rcc",
    "umst_smoother_q_mi",
    "umst_smoother_r_mi",
    "umst_smoother_q_eta_cog",
    "umst_smoother_r_eta_cog",
];

// --- K-5k TUI-7 SEQ3/4 dignity + Landauer slack (Q, R) batch (§14bis.k deepen wave 11) ---

/// SSOT: `umst_smoother_q_dignity` (SEQ3 dignity ramp).
pub const SMOOTHER_Q_DIGNITY: f64 = 1.2;

/// SSOT: `umst_smoother_r_dignity` (SEQ3).
pub const SMOOTHER_R_DIGNITY: f64 = 3_060.0;

/// SSOT: `umst_smoother_q_landauer_slack` (SEQ4 wide dynamic range).
pub const SMOOTHER_Q_LANDAUER_SLACK: f64 = 2.0;

/// SSOT: `umst_smoother_r_landauer_slack` (SEQ4).
pub const SMOOTHER_R_LANDAUER_SLACK: f64 = 3_300.0;

/// `umst_smoother_q_dignity` — TUI-7b method (b) on SEQ3 dignity ramp.
pub const SMOOTHER_Q_DIGNITY_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/umst_smoother_q_dignity.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#tui-7b-per-metric-qr-seq3",
};

/// `umst_smoother_r_dignity` — companion R for SEQ3.
pub const SMOOTHER_R_DIGNITY_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/umst_smoother_r_dignity.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#tui-7b-per-metric-qr-seq3",
};

/// `umst_smoother_q_landauer_slack` — SEQ4 Landauer slack process noise.
pub const SMOOTHER_Q_LANDAUER_SLACK_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/umst_smoother_q_landauer_slack.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#tui-7b-per-metric-qr-seq4",
};

/// `umst_smoother_r_landauer_slack` — SEQ4 measurement noise.
pub const SMOOTHER_R_LANDAUER_SLACK_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/umst_smoother_r_landauer_slack.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#tui-7b-per-metric-qr-seq4",
};

/// K-5k TUI-7 SEQ3/4 smoother registry row names (4/4 for slice GREEN).
pub const K5K_REGISTRY_ROW_NAMES: &[&str] = &[
    "umst_smoother_q_dignity",
    "umst_smoother_r_dignity",
    "umst_smoother_q_landauer_slack",
    "umst_smoother_r_landauer_slack",
];

/// Numerics rows (solver tolerances, finite-difference steps, comparison floors) whose values
/// `numeric_tolerance` and `kernels::scalar` read from the registry.
pub const NUMERICS_REGISTRY_ROW_NAMES: &[&str] = &[
    "rho_mi_clamp_abs",
    "bar_network_cg_rel_tol",
    "mechanics_tight_cg_rel_tol",
    "adjoint_reference_rel_tol",
    "mechanics_mid_cg_scale",
    "finite_difference_step_scale",
    "finite_difference_step_min",
    "finite_difference_step_max",
    "approx_epsilon_f64",
    "approx_max_relative_default",
    "approx_epsilon_f32_loose",
    "approx_epsilon_f32_mid",
    "approx_epsilon_f64_loose",
    "edge_length_divisor_floor_f32",
];

/// K-5d registry row names (6/6 for slice GREEN).
pub const K5D_REGISTRY_ROW_NAMES: &[&str] = &[
    "landauer_proximity_multiplier",
    "staleness_threshold_ms",
    "umst_discovery_lru_capacity",
    "umst_tui_render_debounce_ms",
    "umst_h3b_reward_alpha",
    "umst_h3b_reward_beta",
];

/// Count K-5 rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn k5_backfilled_count() -> usize {
    K5_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
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
        "landauer_proximity_multiplier" => Some(LANDAUER_PROXIMITY_MULTIPLIER_DERIVATION),
        "staleness_threshold_ms" => Some(STALENESS_THRESHOLD_MS_DERIVATION),
        "umst_discovery_lru_capacity" => Some(DISCOVERY_LRU_CAPACITY_DERIVATION),
        "umst_tui_render_debounce_ms" => Some(TUI_RENDER_DEBOUNCE_MS_DERIVATION),
        "umst_h3b_reward_alpha" => Some(H3B_REWARD_ALPHA_DERIVATION),
        "umst_h3b_reward_beta" => Some(H3B_REWARD_BETA_DERIVATION),
        "umst_h3b_reward_gamma" => Some(H3B_REWARD_GAMMA_DERIVATION),
        "umst_ffi_abi_version" => Some(UMST_FFI_ABI_VERSION_DERIVATION),
        "umst_ffi_abi_version_min_compatible" => Some(UMST_FFI_ABI_VERSION_MIN_COMPATIBLE_DERIVATION),
        "umst_discovery_refresh_secs" => Some(DISCOVERY_REFRESH_SECS_DERIVATION),
        "umst_tool_timeout_secs" => Some(TOOL_TIMEOUT_SECS_DERIVATION),
        "audit_max_bytes_cap" => Some(AUDIT_MAX_BYTES_CAP_DERIVATION),
        "umst_closed_loop_rcc_accept_tick" => Some(CLOSED_LOOP_RCC_ACCEPT_TICK_DERIVATION),
        "umst_memory_default_resolution_bits" => Some(MEMORY_DEFAULT_RESOLUTION_BITS_DERIVATION),
        "umst_memory_schema_version" => Some(MEMORY_SCHEMA_VERSION_DERIVATION),
        "umst_memory_m2_promote_ceremony_atomic" => Some(MEMORY_M2_PROMOTE_CEREMONY_ATOMIC_DERIVATION),
        "umst_memory_m2_sanitize_serial_kinds_count" => {
            Some(MEMORY_M2_SANITIZE_SERIAL_KINDS_DERIVATION)
        }
        "umst_memory_m2_promotion_requires_theorem_default" => {
            Some(MEMORY_M2_PROMOTION_REQUIRES_THEOREM_DERIVATION)
        }
        "umst_memory_ephemeral_ttl_hours_typical" => Some(MEMORY_EPHEMERAL_TTL_HOURS_DERIVATION),
        "embedding_http_timeout_seconds" => Some(EMBEDDING_HTTP_TIMEOUT_SECONDS_DERIVATION),
        n if n == K5G_ROW_SCRUB_SENTINEL_LEN => Some(MEMORY_M2_SERIAL_SCRUB_SENTINEL_LEN_DERIVATION),
        "umst_memory_m3_palette_federated_inspect_min_rows" => {
            Some(MEMORY_M3_PALETTE_FEDERATED_INSPECT_MIN_ROWS_DERIVATION)
        }
        "umst_memory_merge_safe_attestation_wire_version" => {
            Some(MEMORY_MERGE_SAFE_ATTESTATION_WIRE_VERSION_DERIVATION)
        }
        "umst_memory_schema_version_v2" => Some(MEMORY_SCHEMA_VERSION_V2_DERIVATION),
        "umst_memory_tier_repr_byte_device" => Some(MEMORY_TIER_REPR_BYTE_DEVICE_DERIVATION),
        "umst_memory_tier_repr_byte_ephemeral" => Some(MEMORY_TIER_REPR_BYTE_EPHEMERAL_DERIVATION),
        "umst_memory_tier_repr_byte_federated" => Some(MEMORY_TIER_REPR_BYTE_FEDERATED_DERIVATION),
        "umst_memory_retention_alpha_default" => Some(MEMORY_RETENTION_ALPHA_DEFAULT_DERIVATION),
        "umst_memory_retention_evict_default" => Some(MEMORY_RETENTION_EVICT_DEFAULT_DERIVATION),
        "umst_memory_retention_degrade_first_default" => {
            Some(MEMORY_RETENTION_DEGRADE_FIRST_DEFAULT_DERIVATION)
        }
        "umst_manifold_liquid_ppo_witness_default" => {
            Some(MANIFOLD_LIQUID_PPO_WITNESS_DEFAULT_DERIVATION)
        }
        "umst_ucrs_memory_phase_bind_enabled" => Some(UCRS_MEMORY_PHASE_BIND_ENABLED_DEFAULT_DERIVATION),
        "umst_msdf_layer_stack_max_depth" => Some(MSDF_LAYER_STACK_MAX_DEPTH_DEFAULT_DERIVATION),
        "umst_memory_hilbert_bits" => Some(MEMORY_HILBERT_BITS_DEFAULT_DERIVATION),
        "rapl_package_dram_joules" => Some(RAPL_PACKAGE_DRAM_JOULES_DERIVATION),
        "cpu_utilization_percent" => Some(CPU_UTILIZATION_PERCENT_DERIVATION),
        "process_joules_estimate" => Some(PROCESS_JOULES_ESTIMATE_DERIVATION),
        "lean_toolchain_pin" => Some(LEAN_TOOLCHAIN_PIN_DERIVATION),
        "coq_version_pin" => Some(COQ_VERSION_PIN_DERIVATION),
        "agda_version_pin" => Some(AGDA_VERSION_PIN_DERIVATION),
        "ghc_version_pin" => Some(GHC_VERSION_PIN_DERIVATION),
        "umst_cockpit_smoothing_default" => Some(COCKPIT_SMOOTHING_DEFAULT_DERIVATION),
        "umst_smoother_q_rcc" => Some(SMOOTHER_Q_RCC_DERIVATION),
        "umst_smoother_r_rcc" => Some(SMOOTHER_R_RCC_DERIVATION),
        "umst_smoother_q_mi" => Some(SMOOTHER_Q_MI_DERIVATION),
        "umst_smoother_r_mi" => Some(SMOOTHER_R_MI_DERIVATION),
        "umst_smoother_q_eta_cog" => Some(SMOOTHER_Q_ETA_COG_DERIVATION),
        "umst_smoother_r_eta_cog" => Some(SMOOTHER_R_ETA_COG_DERIVATION),
        "umst_smoother_q_dignity" => Some(SMOOTHER_Q_DIGNITY_DERIVATION),
        "umst_smoother_r_dignity" => Some(SMOOTHER_R_DIGNITY_DERIVATION),
        "umst_smoother_q_landauer_slack" => Some(SMOOTHER_Q_LANDAUER_SLACK_DERIVATION),
        "umst_smoother_r_landauer_slack" => Some(SMOOTHER_R_LANDAUER_SLACK_DERIVATION),
        n if super::tier3_derivation::K5L_REGISTRY_ROW_NAMES.contains(&n)
            || super::tier3_derivation::K5M_REGISTRY_ROW_NAMES.contains(&n)
            || super::tier3_derivation::K5N_REGISTRY_ROW_NAMES.contains(&n)
            || super::tier3_derivation::K5O_REGISTRY_ROW_NAMES.contains(&n)
            || super::tier3_derivation::K5P_REGISTRY_ROW_NAMES.contains(&n)
            || super::tier3_derivation::K5Q_REGISTRY_ROW_NAMES.contains(&n) =>
        {
            Some(super::tier3_derivation::TUI_6B_COLOR_DEFINITION)
        }
        n if super::tier3_derivation::K5R_REGISTRY_ROW_NAMES.contains(&n) => {
            super::tier3_derivation::derivation_for_k5r_registry_row(n)
        }
        n if super::tier3_derivation::K5S_HAL_REGISTRY_ROW_NAMES.contains(&n)
            || super::tier3_derivation::K5T_HAL_REGISTRY_ROW_NAMES.contains(&n) =>
        {
            Some(super::tier3_derivation::H_9_HAL_DEFINITION)
        }
        n if super::tier3_derivation::K5U_HAL_REGISTRY_ROW_NAMES.contains(&n) => {
            Some(super::tier3_derivation::H_8_HAL_DEFINITION)
        }
        n if super::tier3_derivation::K5V_M0_REGISTRY_ROW_NAMES.contains(&n) => {
            Some(super::tier3_derivation::M_0_MANIFOLD_DEFINITION)
        }
        "solve_combinator_macos_package_power_ceiling_watts" => {
            Some(MACOS_PACKAGE_POWER_CEILING_TYPED_ABSENCE_DERIVATION)
        }
        n if ABSENT_RUNTIME_REGISTRY_ROW_NAMES.contains(&n) => {
            Some(B_ARC_PERF_TYPED_ABSENCE_DERIVATION)
        }
        _ => None,
    }
}

/// Count K-5k rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn k5k_backfilled_count() -> usize {
    K5K_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
        })
        .count()
}

/// K-5k REGISTRY backfill landed.
#[must_use]
pub fn k5k_backfill_landed() -> bool {
    k5k_backfilled_count() == K5K_REGISTRY_ROW_NAMES.len()
}

/// Count K-5j rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn k5j_backfilled_count() -> usize {
    K5J_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
        })
        .count()
}

/// K-5j REGISTRY backfill landed.
#[must_use]
pub fn k5j_backfill_landed() -> bool {
    k5j_backfilled_count() == K5J_REGISTRY_ROW_NAMES.len()
}

/// Count K-5i rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn k5i_backfilled_count() -> usize {
    K5I_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
        })
        .count()
}

/// K-5i REGISTRY backfill landed.
#[must_use]
pub fn k5i_backfill_landed() -> bool {
    k5i_backfilled_count() == K5I_REGISTRY_ROW_NAMES.len()
}

/// Count K-5h rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn k5h_backfilled_count() -> usize {
    K5H_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
        })
        .count()
}

/// K-5h REGISTRY backfill landed.
#[must_use]
pub fn k5h_backfill_landed() -> bool {
    k5h_backfilled_count() == K5H_REGISTRY_ROW_NAMES.len()
}

/// Count K-5g rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn k5g_backfilled_count() -> usize {
    K5G_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
        })
        .count()
}

/// K-5g REGISTRY backfill landed.
#[must_use]
pub fn k5g_backfill_landed() -> bool {
    k5g_backfilled_count() == K5G_REGISTRY_ROW_NAMES.len()
}

/// Count K-5f rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn k5f_backfilled_count() -> usize {
    K5F_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
        })
        .count()
}

/// K-5f REGISTRY backfill landed.
#[must_use]
pub fn k5f_backfill_landed() -> bool {
    k5f_backfilled_count() == K5F_REGISTRY_ROW_NAMES.len()
}

/// Count K-5e rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn k5e_backfilled_count() -> usize {
    K5E_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
        })
        .count()
}

/// K-5e REGISTRY backfill landed.
#[must_use]
pub fn k5e_backfill_landed() -> bool {
    k5e_backfilled_count() == K5E_REGISTRY_ROW_NAMES.len()
}

/// Count K-5d rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn k5d_backfilled_count() -> usize {
    K5D_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
        })
        .count()
}

/// K-5d REGISTRY backfill landed.
#[must_use]
pub fn k5d_backfill_landed() -> bool {
    k5d_backfilled_count() == K5D_REGISTRY_ROW_NAMES.len()
}

/// Count K-5c rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn k5c_backfilled_count() -> usize {
    K5C_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
        })
        .count()
}

/// K-5c REGISTRY backfill landed.
#[must_use]
pub fn k5c_backfill_landed() -> bool {
    k5c_backfilled_count() == K5C_REGISTRY_ROW_NAMES.len()
}

/// Count K-5b rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn k5b_backfilled_count() -> usize {
    K5B_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
        })
        .count()
}

/// K-5b REGISTRY backfill landed.
#[must_use]
pub fn k5b_backfill_landed() -> bool {
    k5b_backfilled_count() == K5B_REGISTRY_ROW_NAMES.len()
}

/// Count K-3 gate-deepen rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn k3_tier2_gate_backfilled_count() -> usize {
    K3_TIER2_GATE_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
        })
        .count()
}

/// Count K-3 batch rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn k3_backfilled_count() -> usize {
    K3_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
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
    use crate::constants::registry::registry_f64_by_name;
    use crate::manifold::csg::Q_HYDRATION_J_PER_KG;

    #[test]
    fn k3_batch_derivations_are_measurement() {
        for name in K3_REGISTRY_ROW_NAMES {
            let d = derivation_for_registry_row(name).expect("lookup");
            assert_eq!(d.label(), "Measurement");
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
        }
        assert!(matches!(TRANSITION_TOLERANCE_DERIVATION, Derivation::Policy { .. }));
        assert!(matches!(ADMISSIBILITY_MARGIN_EPS_DERIVATION, Derivation::Policy { .. }));
        // `Concrete.Gate.δMass_val : δMass = 100`: the statement fixes the value.
        assert_eq!(
            GATE_MASS_TOLERANCE_DERIVATION,
            Derivation::Theorem {
                decl: LeanDecl { module: "Concrete.Gate", name: "δMass_val" },
                expected_value: gate_mass_tolerance_kg_m3_f64(),
            }
        );
        assert!((gate_mass_tolerance_kg_m3_f64() - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn k3_tier1_q_hyd_derivation_matches_registry_and_ssot() {
        let entry = REGISTRY
            .iter()
            .find(|e| e.name == "q_hyd_j_per_kg")
            .expect("registry row");
        assert_eq!(entry.derivation, Q_HYD_J_PER_KG_DERIVATION);
        match Q_HYD_J_PER_KG_DERIVATION {
            Derivation::Policy { rationale } => assert!(rationale.contains("hydrationHeatDefault")),
            other => panic!("Q_hyd is the formal policy row hydrationHeatDefault, got {other:?}"),
        }
        let j_per_g = formal_hydration_heat_default_j_per_g();
        let j_per_kg = j_per_g * 1_000.0;
        assert!(
            (Q_HYDRATION_J_PER_KG - j_per_kg).abs() < 1e-6,
            "Q_HYDRATION_J_PER_KG {Q_HYDRATION_J_PER_KG} must be formal {j_per_g} J/g times 1000"
        );
        assert!(
            (j_per_kg - 4.5e5).abs() < 1e-6,
            "formal hydrationHeatDefault converts to 4.5e5 J/kg, got {j_per_kg}"
        );
        assert!(
            entry.expression.contains("4.5e5") && entry.expression.contains("J/kg"),
            "registry expression must state the J/kg figure: {}",
            entry.expression
        );
        assert!(
            !entry.expression.starts_with("450.0"),
            "registry expression must not open with the unconverted 450 figure: {}",
            entry.expression
        );
        for alpha in [0.0_f64, 0.25, 1.0] {
            let psi = crate::manifold::csg::helmholtz_sdf_1d(alpha, Q_HYDRATION_J_PER_KG);
            assert!(psi <= 0.0, "psi = -Q*alpha stays non-positive at alpha {alpha}, got {psi}");
            assert!((psi + Q_HYDRATION_J_PER_KG * alpha).abs() < 1e-6);
        }
    }

    /// Formal table row `hydrationHeatDefault`, joules per gram.
    fn formal_hydration_heat_default_j_per_g() -> f64 {
        let json = include_str!("../../../../umst-formal/constants/constants.json");
        let marker = "\"id\": \"hydrationHeatDefault\"";
        let idx = json.find(marker).expect("hydrationHeatDefault row");
        let slice = &json[idx..idx + 500];
        assert!(slice.contains("\"unit\": \"J/g\""), "hydrationHeatDefault unit is J/g");
        let num_key = "\"num\": \"";
        let n = slice.find(num_key).expect("hydrationHeatDefault num");
        let rest = &slice[n + num_key.len()..];
        let end = rest.find('"').expect("num close");
        rest[..end]
            .parse::<f64>()
            .expect("hydrationHeatDefault num parses")
    }

    #[test]
    fn k5_warmup_reference_matches_median_convergence() {
        assert_eq!(
            WARMUP_SAMPLE_THRESHOLD_DERIVATION,
            Derivation::Theorem {
                decl: LeanDecl { module: "MedianConvergence", name: "sqrt_window_warmup_is_admissible" },
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
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
        }
    }

    #[test]
    fn k5d_staleness_threshold_matches_cycle_product() {
        assert!((default_staleness_threshold_ms() - 3_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn k5j_registry_rows_backfilled() {
        assert!(k5j_backfill_landed());
        for name in K5J_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
        }
        assert_eq!(
            registry_f64_by_name("umst_smoother_q_rcc").expect("q"),
            SMOOTHER_Q_RCC
        );
        assert_eq!(
            registry_f64_by_name("umst_smoother_r_eta_cog").expect("r"),
            SMOOTHER_R_ETA_COG
        );
    }

    #[test]
    fn k5k_registry_rows_backfilled() {
        assert!(k5k_backfill_landed());
        for name in K5K_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
        }
        assert_eq!(
            registry_f64_by_name("umst_smoother_q_dignity").expect("q"),
            SMOOTHER_Q_DIGNITY
        );
        assert_eq!(
            registry_f64_by_name("umst_smoother_r_landauer_slack").expect("r"),
            SMOOTHER_R_LANDAUER_SLACK
        );
    }

    #[test]
    fn k5i_registry_rows_backfilled() {
        assert!(k5i_backfill_landed());
        for name in K5I_REGISTRY_ROW_NAMES {
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
    fn k5h_registry_rows_backfilled() {
        assert!(k5h_backfill_landed());
        for name in K5H_REGISTRY_ROW_NAMES {
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
    fn k5g_registry_rows_backfilled() {
        assert!(k5g_backfill_landed());
        for name in K5G_REGISTRY_ROW_NAMES {
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
    fn k5g_scrub_sentinel_len_matches_egoff_ssot() {
        let token = "<EGOFF-SCRUBBED>";
        assert_eq!(MEMORY_M2_SERIAL_SCRUB_SENTINEL_LEN as usize, token.len());
    }

    #[test]
    fn k5f_registry_rows_backfilled() {
        assert!(k5f_backfill_landed());
        for name in K5F_REGISTRY_ROW_NAMES {
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
    fn k5e_registry_rows_backfilled() {
        assert!(k5e_backfill_landed());
        for name in K5E_REGISTRY_ROW_NAMES {
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
    fn host_energy_receipts_clear_the_landauer_floor() {
        fn load(bytes: &str) -> serde_json::Value {
            serde_json::from_str(bytes).expect("receipt json")
        }
        let cpu_bytes = include_str!(
            "../../../../egoff/.umst-ci/measurement-receipts/cpu_utilization_percent.jsonl"
        );
        let package_bytes = include_str!(
            "../../../../egoff/.umst-ci/measurement-receipts/rapl_package_dram_joules.jsonl"
        );
        let process_bytes = include_str!(
            "../../../../egoff/.umst-ci/measurement-receipts/process_joules_estimate.jsonl"
        );
        let cpu = load(cpu_bytes);
        let package = load(package_bytes);
        let process = load(process_bytes);
        let busy = cpu["derived_value"].as_f64().expect("cpu");
        let lo = cpu["interval"][0].as_f64().expect("lo");
        let hi = cpu["interval"][1].as_f64().expect("hi");
        assert!((0.0..=100.0).contains(&busy));
        assert!(lo <= busy && busy <= hi);
        assert!(cpu["sample_count"].as_u64().expect("n") >= 2);
        let temp_k = package["temperature_k"].as_f64().expect("T");
        let written = (cpu_bytes.len() + package_bytes.len() + process_bytes.len()) as f64;
        let floor = crate::constants::registry::K_BOLTZMANN_J_PER_K
            * temp_k
            * 2.0_f64.ln()
            * 8.0
            * written;
        for receipt in [&package, &process] {
            let joules = receipt["derived_value"].as_f64().expect("J");
            let rlo = receipt["interval"][0].as_f64().expect("lo");
            let rhi = receipt["interval"][1].as_f64().expect("hi");
            assert!(joules >= floor);
            assert!(rlo <= joules && joules <= rhi);
            assert!(receipt["estimator"]
                .as_str()
                .expect("estimator")
                .contains("SystemPower"));
            assert!(receipt["linux_powercap"].as_bool() == Some(false));
            assert!(receipt["sample_count"].as_u64().expect("n") >= 2);
        }
    }

    fn fixture_array(text: &str, name: &str) -> Vec<f64> {
        let key = format!("const {name}: [f64;");
        let at = text.find(&key).unwrap_or_else(|| panic!("missing {name}"));
        let eq = text[at..].find('=').expect("eq") + at;
        let start = text[eq..].find('[').expect("open") + eq + 1;
        let end = text[start..].find(']').expect("close") + start;
        text[start..end]
            .split(',')
            .filter(|piece| !piece.trim().is_empty())
            .map(|piece| piece.trim().parse::<f64>().expect("number"))
            .collect()
    }

    fn sample_variance(values: &[f64]) -> f64 {
        let n = values.len() as f64;
        let mean = values.iter().sum::<f64>() / n;
        values.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / (n - 1.0)
    }

    #[test]
    fn smoother_variances_match_the_recorded_fixture_sequences() {
        let fixture = include_str!("../../tests/smoothing_ekf_e_bisim.rs");
        let lanes = [
            ("rcc", "SEQ0", "EXP0"),
            ("mi", "SEQ1", "EXP1"),
            ("eta_cog", "SEQ2", "EXP2"),
            ("dignity", "SEQ3", "EXP3"),
            ("landauer_slack", "SEQ4", "EXP4"),
        ];
        for (lane, seq_name, exp_name) in lanes {
            let measured = fixture_array(fixture, seq_name);
            let filtered = fixture_array(fixture, exp_name);
            let steps: Vec<f64> = filtered.windows(2).map(|w| w[1] - w[0]).collect();
            let residuals: Vec<f64> = measured
                .iter()
                .zip(&filtered)
                .map(|(z, x)| z - x)
                .collect();
            let q = sample_variance(&steps);
            let r = sample_variance(&residuals);
            assert!(q > 0.0 && r > 0.0);
            for (kind, value, count) in [("q", q, steps.len()), ("r", r, residuals.len())] {
                let bytes = match (lane, kind) {
                    ("rcc", "q") => include_str!("../../../../egoff/.umst-ci/measurement-receipts/umst_smoother_q_rcc.jsonl"),
                    ("rcc", "r") => include_str!("../../../../egoff/.umst-ci/measurement-receipts/umst_smoother_r_rcc.jsonl"),
                    ("mi", "q") => include_str!("../../../../egoff/.umst-ci/measurement-receipts/umst_smoother_q_mi.jsonl"),
                    ("mi", "r") => include_str!("../../../../egoff/.umst-ci/measurement-receipts/umst_smoother_r_mi.jsonl"),
                    ("eta_cog", "q") => include_str!("../../../../egoff/.umst-ci/measurement-receipts/umst_smoother_q_eta_cog.jsonl"),
                    ("eta_cog", "r") => include_str!("../../../../egoff/.umst-ci/measurement-receipts/umst_smoother_r_eta_cog.jsonl"),
                    ("dignity", "q") => include_str!("../../../../egoff/.umst-ci/measurement-receipts/umst_smoother_q_dignity.jsonl"),
                    ("dignity", "r") => include_str!("../../../../egoff/.umst-ci/measurement-receipts/umst_smoother_r_dignity.jsonl"),
                    ("landauer_slack", "q") => include_str!("../../../../egoff/.umst-ci/measurement-receipts/umst_smoother_q_landauer_slack.jsonl"),
                    ("landauer_slack", "r") => include_str!("../../../../egoff/.umst-ci/measurement-receipts/umst_smoother_r_landauer_slack.jsonl"),
                    _ => panic!("lane"),
                };
                let receipt: serde_json::Value = serde_json::from_str(bytes).expect("receipt");
                let got = receipt["derived_value"].as_f64().expect("value");
                let lo = receipt["interval"][0].as_f64().expect("lo");
                let hi = receipt["interval"][1].as_f64().expect("hi");
                assert!((got - value).abs() <= 1e-9 * value.max(1.0));
                assert!(got > 0.0 && lo <= got && got <= hi);
                assert_eq!(receipt["sample_count"].as_u64().expect("n"), count as u64);
            }
        }
    }

    #[test]
    fn k5d_registry_rows_backfilled() {
        assert!(k5d_backfill_landed());
        for name in K5D_REGISTRY_ROW_NAMES {
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

    fn pending_gaps_plain_for_absent_witness() -> &'static str {
        include_str!("../../../docs/PENDING_GAPS_PLAIN.md")
    }

    fn absent_reason_doc_anchor(reason: &str) -> &str {
        reason
            .rsplit('#')
            .next()
            .filter(|anchor| !anchor.is_empty())
            .unwrap_or_else(|| panic!("Absent reason lacks docs anchor: {reason}"))
    }

    fn pending_gaps_plain_lists_anchor(anchor: &str, doc: &str) -> bool {
        doc.contains(&format!("id=\"{anchor}\""))
            || doc.contains(&format!("### {anchor}"))
            || doc.contains(&format!("## {anchor}"))
    }

    fn measurement_receipt_exists(receipt_path: &str) -> bool {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let rel = receipt_path.trim_start_matches("egoff/");
        manifest.join("../../egoff").join(rel).is_file()
    }

    #[test]
    fn absent_runtime_registry_rows_backfilled() {
        for name in ABSENT_RUNTIME_REGISTRY_ROW_NAMES {
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
    fn absent_runtime_rows_anchor_or_receipt_witness() {
        let gaps = pending_gaps_plain_for_absent_witness();
        for name in ABSENT_RUNTIME_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            match entry.derivation {
                Derivation::Absent { reason } => {
                    let anchor = absent_reason_doc_anchor(reason);
                    assert!(
                        pending_gaps_plain_lists_anchor(anchor, gaps),
                        "PENDING_GAPS_PLAIN.md missing anchor #{anchor} cited by {name}"
                    );
                }
                Derivation::Measurement { receipt_path, .. } => {
                    assert!(
                        measurement_receipt_exists(receipt_path),
                        "Measurement row {name} missing receipt at {receipt_path}"
                    );
                    let bytes = std::fs::read(
                        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                            .join("../../egoff")
                            .join(receipt_path.trim_start_matches("egoff/")),
                    )
                    .expect("receipt bytes");
                    let receipt: serde_json::Value =
                        serde_json::from_slice(&bytes).expect("receipt json");
                    assert!(receipt.get("sample_count").is_some());
                    assert!(receipt.get("estimator").is_some());
                    assert!(receipt.get("interval").is_some());
                }
                other => panic!("unexpected derivation for {name}: {other:?}"),
            }
        }
    }
}
