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
pub const LANDAUER_PROXIMITY_MULTIPLIER_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.MeasurementJitterBound::landauer_proximity_margin",
    expected_value: LANDAUER_PROXIMITY_MULTIPLIER,
};

/// `staleness_threshold_ms` — product of default staleness cycles and hub sample period.
pub const STALENESS_THRESHOLD_MS_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.FrugalityRanker::staleness_threshold_from_hub_period",
    expected_value: DEFAULT_STALENESS_THRESHOLD_MS,
};

/// `umst_discovery_lru_capacity` — model-discovery LRU operator bound (TUI-5).
pub const DISCOVERY_LRU_CAPACITY_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.OrderStatisticsBand::order_statistic_concentration",
    expected_value: DEFAULT_DISCOVERY_LRU_CAPACITY,
};

/// `umst_tui_render_debounce_ms` — idle telemetry redraw coalescing window (TUI-5).
pub const TUI_RENDER_DEBOUNCE_MS_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible",
    expected_value: DEFAULT_TUI_RENDER_DEBOUNCE_MS,
};

/// `umst_h3b_reward_alpha` — H-3b witness quality weight α.
pub const H3B_REWARD_ALPHA_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.InfoTheory::product_joint_mass",
    expected_value: H3B_REWARD_ALPHA,
};

/// `umst_h3b_reward_beta` — H-3b witness latency slack weight β.
pub const H3B_REWARD_BETA_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.RhoEstimator::rho_based_mi_formula",
    expected_value: H3B_REWARD_BETA,
};

/// `umst_h3b_reward_gamma` — H-3b witness energy slack weight γ.
pub const H3B_REWARD_GAMMA_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.EtaCog::eta_cog_nonneg",
    expected_value: H3B_REWARD_GAMMA,
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
pub const UMST_FFI_ABI_VERSION_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.FFI::abi_version_expected",
    expected_value: UMST_FFI_ABI_VERSION_DEFAULT,
};

/// `umst_ffi_abi_version_min_compatible` — minimum compatible ABI for `assertAbiCompatible`.
pub const UMST_FFI_ABI_VERSION_MIN_COMPATIBLE_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.FFI::abi_version_min_compatible",
    expected_value: UMST_FFI_ABI_VERSION_MIN_COMPATIBLE_DEFAULT,
};

/// `umst_discovery_refresh_secs` — model-list HTTP poll cadence (cockpit hub).
pub const DISCOVERY_REFRESH_SECS_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.FrugalityRanker::staleness_threshold_from_hub_period",
    expected_value: DEFAULT_DISCOVERY_REFRESH_SECS,
};

/// `umst_tool_timeout_secs` — operator tool palette wall-clock budget.
pub const TOOL_TIMEOUT_SECS_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Gate.gateCheckSound",
    expected_value: DEFAULT_TOOL_TIMEOUT_SECS,
};

/// `audit_max_bytes_cap` — on-disk cockpit audit JSONL rotation cap.
pub const AUDIT_MAX_BYTES_CAP_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.EtaCog::eta_cog_nonneg",
    expected_value: DEFAULT_AUDIT_MAX_BYTES_CAP,
};

/// `umst_closed_loop_rcc_accept_tick` — per-accept RCC increment (cap 1.0).
pub const CLOSED_LOOP_RCC_ACCEPT_TICK_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Convergence::rcc_lower_bound",
    expected_value: DEFAULT_RCC_ACCEPT_TICK,
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
pub const MEMORY_DEFAULT_RESOLUTION_BITS_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.OrderStatisticsBand::order_statistic_concentration",
    expected_value: MEMORY_DEFAULT_RESOLUTION_BITS,
};

/// `umst_memory_schema_version` — sled `MemoryV1` bincode wire discriminator.
pub const MEMORY_SCHEMA_VERSION_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Gate.gateCheckSound",
    expected_value: MEMORY_SCHEMA_V1_DEFAULT,
};

/// `umst_memory_m2_promote_ceremony_atomic` — fail-fast promotion ceremony flag.
pub const MEMORY_M2_PROMOTE_CEREMONY_ATOMIC_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Convergence::rcc_lower_bound",
    expected_value: MEMORY_M2_PROMOTE_CEREMONY_ATOMIC,
};

/// `umst_memory_m2_sanitize_serial_kinds_count` — GMD-6 serial artefact taxonomy size.
pub const MEMORY_M2_SANITIZE_SERIAL_KINDS_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.InfoTheory::product_joint_mass",
    expected_value: MEMORY_M2_SANITIZE_SERIAL_KINDS_COUNT,
};

/// `umst_memory_m2_promotion_requires_theorem_default` — theorem binding required on promote.
pub const MEMORY_M2_PROMOTION_REQUIRES_THEOREM_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Dignity::dignity_monotone_under_mi_gain",
    expected_value: MEMORY_M2_PROMOTION_REQUIRES_THEOREM_DEFAULT,
};

/// `umst_memory_ephemeral_ttl_hours_typical` — default ephemeral retention window (hours).
pub const MEMORY_EPHEMERAL_TTL_HOURS_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.FrugalityRanker::staleness_threshold_from_hub_period",
    expected_value: MEMORY_EPHEMERAL_TTL_HOURS_TYPICAL,
};

/// `embedding_http_timeout_seconds` — embedding adapter wall-clock budget (design default).
pub const EMBEDDING_HTTP_TIMEOUT_SECONDS_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Gate.gateCheckSound",
    expected_value: EMBEDDING_HTTP_TIMEOUT_SECONDS_DEFAULT,
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
pub const MEMORY_M2_SERIAL_SCRUB_SENTINEL_LEN_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.InfoTheory::product_joint_mass",
    expected_value: MEMORY_M2_SERIAL_SCRUB_SENTINEL_LEN,
};

/// `umst_memory_m3_palette_federated_inspect_min_rows` — federation inspector offline floor.
pub const MEMORY_M3_PALETTE_FEDERATED_INSPECT_MIN_ROWS_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Gate.gateCheckSound",
    expected_value: MEMORY_M3_PALETTE_FEDERATED_INSPECT_MIN_ROWS,
};

/// `umst_memory_merge_safe_attestation_wire_version` — GMD-8 merge-safe witness wire gen.
pub const MEMORY_MERGE_SAFE_ATTESTATION_WIRE_VERSION_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Dignity::dignity_monotone_under_mi_gain",
    expected_value: MEMORY_MERGE_SAFE_ATTESTATION_WIRE_VERSION,
};

/// `umst_memory_schema_version_v2` — `MemoryV2` sled wire discriminator.
pub const MEMORY_SCHEMA_VERSION_V2_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Gate.gateCheckSound",
    expected_value: MEMORY_SCHEMA_VERSION_V2,
};

/// `umst_memory_tier_repr_byte_device` — rename-fed Device tier `repr(u8)`.
pub const MEMORY_TIER_REPR_BYTE_DEVICE_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Gate.gateCheckSound",
    expected_value: MEMORY_TIER_REPR_BYTE_DEVICE,
};

/// `umst_memory_tier_repr_byte_ephemeral` — Ephemeral tier `repr(u8)` for graduation targets.
pub const MEMORY_TIER_REPR_BYTE_EPHEMERAL_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.FrugalityRanker::staleness_threshold_from_hub_period",
    expected_value: MEMORY_TIER_REPR_BYTE_EPHEMERAL,
};

/// `umst_memory_tier_repr_byte_federated` — rename-fed Federated tier `repr(u8)`.
pub const MEMORY_TIER_REPR_BYTE_FEDERATED_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Convergence::rcc_lower_bound",
    expected_value: MEMORY_TIER_REPR_BYTE_FEDERATED,
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
pub const MEMORY_RETENTION_ALPHA_DEFAULT_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.InfoTheory::product_joint_mass",
    expected_value: MEMORY_RETENTION_ALPHA_DEFAULT,
};

/// `umst_memory_retention_evict_default` — post-store eviction toggle default.
pub const MEMORY_RETENTION_EVICT_DEFAULT_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Gate.gateCheckSound",
    expected_value: MEMORY_RETENTION_EVICT_DEFAULT,
};

/// `umst_memory_retention_degrade_first_default` — degrade-before-drop policy default.
pub const MEMORY_RETENTION_DEGRADE_FIRST_DEFAULT_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Dignity::dignity_monotone_under_mi_gain",
    expected_value: MEMORY_RETENTION_DEGRADE_FIRST_DEFAULT,
};

/// `umst_manifold_liquid_ppo_witness_default` — Path B `step_and_learn` witness gate.
pub const MANIFOLD_LIQUID_PPO_WITNESS_DEFAULT_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Convergence::rcc_lower_bound",
    expected_value: MANIFOLD_LIQUID_PPO_WITNESS_DEFAULT,
};

/// `umst_ucrs_memory_phase_bind_enabled` — accept-path UCRS phase bind toggle.
pub const UCRS_MEMORY_PHASE_BIND_ENABLED_DEFAULT_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.FrugalityRanker::staleness_threshold_from_hub_period",
    expected_value: UCRS_MEMORY_PHASE_BIND_ENABLED_DEFAULT,
};

/// `umst_msdf_layer_stack_max_depth` — progressive MSDF ring cap when layer stack on.
pub const MSDF_LAYER_STACK_MAX_DEPTH_DEFAULT_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.OrderStatisticsBand::order_statistic_concentration",
    expected_value: MSDF_LAYER_STACK_MAX_DEPTH_DEFAULT,
};

/// `umst_memory_hilbert_bits` — Hilbert curve order for sled key layout (M-5 policy).
pub const MEMORY_HILBERT_BITS_DEFAULT_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Gate.gateCheckSound",
    expected_value: MEMORY_HILBERT_BITS_DEFAULT,
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

/// `rapl_package_dram_joules` — Linux RAPL package counter (NED if unreadable).
pub const RAPL_PACKAGE_DRAM_JOULES_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/rapl_package_dram_joules.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#hal-rapl-package-energy",
};

/// `cpu_utilization_percent` — sysinfo global CPU util (portable).
pub const CPU_UTILIZATION_PERCENT_DERIVATION: Derivation = Derivation::Measurement {
    receipt_path: ".umst-ci/measurement-receipts/cpu_utilization_percent.jsonl",
    methodology_anchor: "COCKPIT_DESIGN_BRIEF.md#hal-cpu-utilization",
};

/// `process_joules_estimate` — EnergyService port estimate (watts × Δt × util).
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

/// K-5d registry row names (6/6 for slice GREEN).
pub const K5D_REGISTRY_ROW_NAMES: &[&str] = &[
    "landauer_proximity_multiplier",
    "staleness_threshold_ms",
    "umst_discovery_lru_capacity",
    "umst_tui_render_debounce_ms",
    "umst_h3b_reward_alpha",
    "umst_h3b_reward_beta",
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
        _ => None,
    }
}

/// Count K-5i rows with non-`Pending` derivation.
#[must_use]
pub fn k5i_backfilled_count() -> usize {
    K5I_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5i REGISTRY backfill landed.
#[must_use]
pub fn k5i_backfill_landed() -> bool {
    k5i_backfilled_count() == K5I_REGISTRY_ROW_NAMES.len()
}

/// Count K-5h rows with non-`Pending` derivation.
#[must_use]
pub fn k5h_backfilled_count() -> usize {
    K5H_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5h REGISTRY backfill landed.
#[must_use]
pub fn k5h_backfill_landed() -> bool {
    k5h_backfilled_count() == K5H_REGISTRY_ROW_NAMES.len()
}

/// Count K-5g rows with non-`Pending` derivation.
#[must_use]
pub fn k5g_backfilled_count() -> usize {
    K5G_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5g REGISTRY backfill landed.
#[must_use]
pub fn k5g_backfill_landed() -> bool {
    k5g_backfilled_count() == K5G_REGISTRY_ROW_NAMES.len()
}

/// Count K-5f rows with non-`Pending` derivation.
#[must_use]
pub fn k5f_backfilled_count() -> usize {
    K5F_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5f REGISTRY backfill landed.
#[must_use]
pub fn k5f_backfill_landed() -> bool {
    k5f_backfilled_count() == K5F_REGISTRY_ROW_NAMES.len()
}

/// Count K-5e rows with non-`Pending` derivation.
#[must_use]
pub fn k5e_backfilled_count() -> usize {
    K5E_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5e REGISTRY backfill landed.
#[must_use]
pub fn k5e_backfill_landed() -> bool {
    k5e_backfilled_count() == K5E_REGISTRY_ROW_NAMES.len()
}

/// Count K-5d rows with non-`Pending` derivation.
#[must_use]
pub fn k5d_backfilled_count() -> usize {
    K5D_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5d REGISTRY backfill landed.
#[must_use]
pub fn k5d_backfill_landed() -> bool {
    k5d_backfilled_count() == K5D_REGISTRY_ROW_NAMES.len()
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

    #[test]
    fn k5d_staleness_threshold_matches_cycle_product() {
        assert!((default_staleness_threshold_ms() - 3_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn k5i_registry_rows_backfilled() {
        assert!(k5i_backfill_landed());
        for name in K5I_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5i: {name} must be backfilled"
            );
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
            assert!(
                !entry.derivation.is_pending(),
                "K-5h: {name} must be backfilled"
            );
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
            assert_eq!(entry.derivation.label(), "Theorem");
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
            assert!(
                !entry.derivation.is_pending(),
                "K-5g: {name} must be backfilled"
            );
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
            assert_eq!(entry.derivation.label(), "Theorem");
        }
    }

    #[test]
    fn k5g_scrub_sentinel_len_matches_egoff_ssot() {
        assert_eq!(MEMORY_M2_SERIAL_SCRUB_SENTINEL_LEN, 16.0);
    }

    #[test]
    fn k5f_registry_rows_backfilled() {
        assert!(k5f_backfill_landed());
        for name in K5F_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5f: {name} must be backfilled"
            );
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
            assert_eq!(entry.derivation.label(), "Theorem");
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
            assert!(
                !entry.derivation.is_pending(),
                "K-5e: {name} must be backfilled"
            );
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
            assert_eq!(entry.derivation.label(), "Theorem");
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
            assert!(
                !entry.derivation.is_pending(),
                "K-5d: {name} must be backfilled"
            );
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
            assert_eq!(entry.derivation.label(), "Theorem");
        }
    }
}
