// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Compile-time registry of cockpit / core numerical constants (CGD).
//!
//! Human-readable mirror: `docs/CGD_REGISTRY.md` §24a (Constants Grounding Registry).
//! Tier-2 rows carry `pending: Phase FPD-*` until the corresponding formal slice lands.

use super::derivation::Derivation;
use super::pool_q_constants_manifold::{
    ACTION_SHAPE_PALETTE_MAX_ENTRIES_DEFAULT_DERIVATION,
    ACTION_SHAPE_QUOTIENT_DEFAULT_ENABLED_DERIVATION, MANIFOLD_INTROSPECT_ENABLED_DERIVATION,
    MCERT_STRICT_PAIRED_DEFAULT_DERIVATION, MSDF_HILBERT_PERSIST_ENABLED_DERIVATION,
    POOL_Q_COCKPIT_POLICY_DEFINITION, UMST_MATH_SIMD_FEATURE_DEFINITION,
};
use super::tier1_derivation::{
    K_B_DERIVATION, LANDAUER_FLOOR_J_PER_BIT_DERIVATION, LN_2_DERIVATION,
    REFERENCE_TEMPERATURE_293_15_K_DERIVATION, RCC_FLOOR_DERIVATION, T_ROOM_DERIVATION,
};
use super::tier2_derivation::{
    ADMISSIBILITY_MARGIN_EPS_DERIVATION, AUDIT_MAX_BYTES_CAP_DERIVATION,
    AUDIT_ROTATION_KEEP_COUNT_DERIVATION, B_ARC_PERF_TYPED_ABSENCE_DERIVATION,
    MANIFOLD_CANONICALIZE_RUNTIME_US_P99_DERIVATION, MANIFOLD_HILBERT_INDEX_RANGE_TYPICAL_DERIVATION,
    MANIFOLD_VOXELIZE_RUNTIME_US_P99_DERIVATION, UMST_MEMORY_INSPECT_RUNTIME_US_P99_DERIVATION,
    UMST_MEMORY_LOAD_RUNTIME_US_P99_DERIVATION, UMST_MEMORY_LOCAL_TIER_SIZE_TYPICAL_DERIVATION,
    UMST_MEMORY_RETENTION_MI_ESTIMATE_P99_DERIVATION,
    UMST_MEMORY_RETENTION_PARETO_COMPUTE_P99_DERIVATION, UMST_MEMORY_STORE_RUNTIME_US_P99_DERIVATION,
    CLOSED_LOOP_MI_STEP_DERIVATION,
    CLOSED_LOOP_RCC_ACCEPT_TICK_DERIVATION, COCKPIT_AUDIT_SCHEMA_VERSION_DERIVATION,
    COCKPIT_SNAPSHOT_SCHEMA_VERSION_DERIVATION, DELTA_MI_SINGLE_TURN_CAP_DERIVATION,
    DIGNITY_SCALAR_RANGE_DERIVATION, DISCOVERY_LRU_CAPACITY_DERIVATION,
    DISCOVERY_REFRESH_SECS_DERIVATION, EMERGENCE_LAMBDA_DERIVATION,
    ETA_ROLLING_WINDOW_CAPACITY_DERIVATION, FRUGALITY_BAND_P25_DERIVATION,
    FRUGALITY_BAND_P75_DERIVATION, GATE_MASS_TOLERANCE_DERIVATION, H3B_REWARD_ALPHA_DERIVATION,
    H3B_REWARD_BETA_DERIVATION, H3B_REWARD_GAMMA_DERIVATION, HAL_IGPU_PRESENT_DERIVATION,
    HAL_L3_CACHE_DERIVATION, HAL_LINUX_PORT_COUNT_DERIVATION, HAL_LINUX_RAM_TOTAL_DERIVATION,
    HAL_LOGICAL_CORES_DERIVATION, HAL_NPU_PRESENT_DERIVATION, HUB_INTER_SAMPLE_PERIOD_MS_DERIVATION,
    LANDAUER_PROXIMITY_MULTIPLIER_DERIVATION, MEMORY_DEFAULT_RESOLUTION_BITS_DERIVATION,
    MEMORY_EPHEMERAL_TTL_HOURS_DERIVATION, MEMORY_M2_PROMOTE_CEREMONY_ATOMIC_DERIVATION,
    MEMORY_M2_PROMOTION_REQUIRES_THEOREM_DERIVATION, MEMORY_M2_SANITIZE_SERIAL_KINDS_DERIVATION,
    MEMORY_M2_SERIAL_SCRUB_SENTINEL_LEN_DERIVATION,
    MEMORY_M3_PALETTE_FEDERATED_INSPECT_MIN_ROWS_DERIVATION,
    MEMORY_MERGE_SAFE_ATTESTATION_WIRE_VERSION_DERIVATION, MEMORY_SCHEMA_VERSION_V2_DERIVATION,
    MEMORY_TIER_REPR_BYTE_DEVICE_DERIVATION, MEMORY_TIER_REPR_BYTE_EPHEMERAL_DERIVATION,
    MEMORY_TIER_REPR_BYTE_FEDERATED_DERIVATION, EMBEDDING_HTTP_TIMEOUT_SECONDS_DERIVATION,
    MEMORY_SCHEMA_VERSION_DERIVATION, MEMORY_RETENTION_ALPHA_DEFAULT_DERIVATION,
    MEMORY_RETENTION_DEGRADE_FIRST_DEFAULT_DERIVATION, MEMORY_RETENTION_EVICT_DEFAULT_DERIVATION,
    MEMORY_HILBERT_BITS_DEFAULT_DERIVATION, MACOS_PACKAGE_POWER_CEILING_TYPED_ABSENCE_DERIVATION,
    MANIFOLD_LIQUID_PPO_WITNESS_DEFAULT_DERIVATION,
    MSDF_LAYER_STACK_MAX_DEPTH_DEFAULT_DERIVATION,
    UCRS_MEMORY_PHASE_BIND_ENABLED_DEFAULT_DERIVATION,
    MIN_PROMOTION_CREDIT_DERIVATION, MSDF_EMERGENCE_MAX_VOXELS_DERIVATION,
    PPO_INFO_GAIN_DEFAULT_BITS_DERIVATION,
    Q_HYD_J_PER_KG_DERIVATION, STALENESS_CYCLE_COUNT_DERIVATION,
    STALENESS_THRESHOLD_MS_DERIVATION, TOOL_TIMEOUT_SECS_DERIVATION, TRANSITION_TOLERANCE_DERIVATION,
    TUI_RENDER_DEBOUNCE_MS_DERIVATION, UMST_FFI_ABI_VERSION_DERIVATION,
    UMST_FFI_ABI_VERSION_MIN_COMPATIBLE_DERIVATION, WARMUP_SAMPLE_THRESHOLD_DERIVATION,
    RAPL_PACKAGE_DRAM_JOULES_DERIVATION, CPU_UTILIZATION_PERCENT_DERIVATION,
    PROCESS_JOULES_ESTIMATE_DERIVATION, LEAN_TOOLCHAIN_PIN_DERIVATION, COQ_VERSION_PIN_DERIVATION,
    AGDA_VERSION_PIN_DERIVATION, EGOFF_CANDLE_EMBED_BATCH_CEILING_DERIVATION,
    EGOFF_MANIFOLD_CANONICALIZE_P99_DERIVATION, GHC_VERSION_PIN_DERIVATION,
    RUSTC_TOOLCHAIN_PIN_DERIVATION, PYTHON_VERSION_PIN_DERIVATION,
    UMST_FORMAL_PIN_SHA_DERIVATION, UMST_HASKELL_TOOLCHAIN_DERIVATION,
    COCKPIT_SMOOTHING_DEFAULT_DERIVATION, SMOOTHER_Q_RCC_DERIVATION, SMOOTHER_R_RCC_DERIVATION,
    SMOOTHER_Q_MI_DERIVATION, SMOOTHER_R_MI_DERIVATION, SMOOTHER_Q_ETA_COG_DERIVATION,
    SMOOTHER_R_ETA_COG_DERIVATION, SMOOTHER_Q_DIGNITY_DERIVATION, SMOOTHER_R_DIGNITY_DERIVATION,
    SMOOTHER_Q_LANDAUER_SLACK_DERIVATION, SMOOTHER_R_LANDAUER_SLACK_DERIVATION,
};
use super::tier3_derivation::{
    COCKPIT_HTTP_CORS_DEFINITION, COCKPIT_RANKER_WEIGHT_DEFINITION,
    EPISTEMIC_PROXY_ESTIMATOR_DEFINITION, ENERGY_BACKEND_DEFINITION, H_8_HAL_DEFINITION,
    H_9_HAL_DEFINITION, M_0_MANIFOLD_DEFINITION, S_0_CRYPTO_DEFINITION,
    S_0_ML_DSA_DEFINITION, S_0_SLH_DSA_DEFINITION,
    SEMANTIC_COVERAGE_THRESHOLD_DEFINITION, TUI_6B_COLOR_DEFINITION, TUI_BIDI_DEFINITION,
};

/// One documented numerical parameter (value, tier, evidence, optional env).
/// CONSTANT-BOUND: `landauer_floor_j_per_bit` (schema; each row is a `ConstantEntry`).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConstantEntry {
    /// Stable identifier (matches §24a “Constant” column intent).
    pub name: &'static str,
    /// Human-readable value or derivation.
    pub expression: &'static str,
    /// Lean path, design-brief pointer, or `pending: Phase FPD-*`.
    pub evidence: &'static str,
    /// Environment variable name when operator-overridable (`None` if not).
    pub env_override: Option<&'static str>,
    /// CDD §0.11 — how egoff re-verifies this constant; the row's tier is a function of it.
    pub derivation: Derivation,
}

/// Five-tier constants grounding taxonomy (`docs/HSAD_PLAN.md` §0.4).
/// ZCI-EXEMPT: `ConstantTier` is a taxonomy only; per-row theorems are on each `REGISTRY` entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum ConstantTier {
    Tier0Physical,
    Tier1Measurement,
    Tier2Derivable,
    Tier3Policy,
    Tier4Infra,
}

impl ConstantEntry {
    /// The row's tier, a function of its derivation (never stored).
    #[must_use]
    pub fn tier(&self) -> ConstantTier {
        self.derivation.tier()
    }
}

/// Authoritative registry (keep in lock-step with `docs/CGD_REGISTRY.md` §24a).
/// CONSTANT-BOUND: … + §14bis.f-M-6 (+2) + §14bis.f-M-7 (+1 mcert) + foundation Phase 3 (+4) + K-2 (+2) + K-4 (+1) + solve-combinator meter (+1) + q_hyd_j_per_kg (+1) + reference_temperature_293_15_k (+1) = **174**
pub static REGISTRY: &[ConstantEntry] = &[
    ConstantEntry {
        name: "landauer_floor_j_per_bit",
        expression: "k_B · T · ln(2) J/bit via umst_math::landauer::landauer_bit_energy_joules",
        evidence: "LandauerLaw::landauerBound (umst-formal): erasing one bit costs at least k_B T ln 2; evaluated at the 300 K reference",
        env_override: None,
        derivation: LANDAUER_FLOOR_J_PER_BIT_DERIVATION,
    },
    ConstantEntry {
        name: "ln_two_eta_cog_denominator",
        expression: "ln(2) in η_cog Landauer denominator",
        evidence: "LandauerLaw::uniformBinaryEntropy (umst-formal): a uniform bit carries ln 2 nats",
        env_override: None,
        derivation: LN_2_DERIVATION,
    },
    ConstantEntry {
        name: "k_boltzmann_j_per_k",
        expression: "1.380649e-23 J/K (CODATA 2018; umst_math::landauer::K_B)",
        evidence: "CODATA exact SI value (NIST CUU https://physics.nist.gov/cgi-bin/cuu/Value?k); umst-formal constants table row Boltzmann constant; umst-math::landauer::K_B",
        env_override: None,
        derivation: K_B_DERIVATION,
    },
    ConstantEntry {
        name: "rcc_floor_residual_coherence",
        expression: "0.25 lower bound (RCC floor; residual coherence capacity)",
        evidence: "policy: residual-coherence floor 0.25; no formal statement fixes it",
        env_override: None,
        derivation: RCC_FLOOR_DERIVATION,
    },
    ConstantEntry {
        name: "host_temperature_fallback_k",
        expression: "300.0 K default when cockpit T is non-finite (overridable)",
        evidence: "Operator-assumed ambient anchor until junction-temperature telemetry is wired",
        env_override: Some("UMST_COCKPIT_HOST_TEMPERATURE_K"),
        derivation: T_ROOM_DERIVATION,
    },
    ConstantEntry {
        name: "reference_temperature_293_15_k",
        expression: "20 °C + 273.15 K (ISO 554 reference atmosphere; SI exact offset)",
        evidence: "ISO 554 standard atmospheres for conditioning/testing at 20 °C; kelvin value is T_C + 273.15 K",
        env_override: None,
        derivation: REFERENCE_TEMPERATURE_293_15_K_DERIVATION,
    },
    ConstantEntry {
        name: "gate_mass_tolerance_kg_m3",
        expression: "100.0 kg/m³ bulk density jump band (GATE_MASS_TOLERANCE_KG_M3)",
        evidence: "UMST.Formal.Concrete.Gate.δMass_val (mirrors umst-math manifold::csg GATE_MASS_TOLERANCE_KG_M3)",
        env_override: None,
        derivation: GATE_MASS_TOLERANCE_DERIVATION,
    },
    ConstantEntry {
        name: "q_hyd_j_per_kg",
        expression: "4.5e5 J/kg (450 J/g × 1000 g/kg; formal hydrationHeatDefault; ψ = −Q_hyd·α)",
        evidence: "policy: umst-formal row hydrationHeatDefault is 450 J/g, proved inside the cited clinker-phase heats (hydrationHeatDefault_in_range); J/kg follows by the exact factor 1000 g/kg",
        env_override: None,
        derivation: Q_HYD_J_PER_KG_DERIVATION,
    },
    ConstantEntry {
        name: "transition_tolerance",
        expression: "1e-6 admissibility ε (TRANSITION_TOLERANCE)",
        evidence: "policy: gate transition tolerance (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: TRANSITION_TOLERANCE_DERIVATION,
    },
    ConstantEntry {
        name: "admissibility_margin_eps",
        expression: "1e-4 hard token floor (ADMISSIBILITY_MARGIN_EPS)",
        evidence: "policy: runtime AdmissibilityMargin witness floor ε",
        env_override: None,
        derivation: ADMISSIBILITY_MARGIN_EPS_DERIVATION,
    },
    ConstantEntry {
        name: "rho_mi_clamp_abs",
        expression: "0.9999 |ρ| clamp in MI(ρ) = −½·log₂(1−ρ²) (RHO_MI_CLAMP_ABS)",
        evidence: "policy: keeps log₂(1−ρ²) finite (umst_math::kernels::scalar)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`rho_mi_clamp_abs` — numerical guard keeping log₂(1−ρ²) finite; it caps MI(ρ) at about 6.14 bits. No statement fixes it.",
        },
    },
    ConstantEntry {
        name: "bar_network_cg_rel_tol",
        expression: "1e-6 relative CG residual, f32 bar networks (BAR_NETWORK_CG_REL_TOL)",
        evidence: "policy: numerics (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`bar_network_cg_rel_tol` — base relative residual for f32 bar-network CG, scaled by the problem scale and floored at f32::EPSILON.",
        },
    },
    ConstantEntry {
        name: "mechanics_tight_cg_rel_tol",
        expression: "1e-8 relative CG residual, tight mechanics tier (MECHANICS_TIGHT_CG_REL_TOL)",
        evidence: "policy: numerics (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`mechanics_tight_cg_rel_tol` — base relative residual for the tight mechanics tier, scaled by the problem scale.",
        },
    },
    ConstantEntry {
        name: "adjoint_reference_rel_tol",
        expression: "1e-10 relative residual, f64 adjoint and analytic references (ADJOINT_REFERENCE_REL_TOL)",
        evidence: "policy: numerics (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`adjoint_reference_rel_tol` — base relative residual for f64 adjoint and analytic reference solves, floored at f64::EPSILON.",
        },
    },
    ConstantEntry {
        name: "mechanics_mid_cg_scale",
        expression: "0.1 problem-scale factor of the mid CG tier, about 1e-7 (MECHANICS_MID_CG_SCALE)",
        evidence: "policy: numerics (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`mechanics_mid_cg_scale` — scale factor placing the mid tier between the bar-network default and the tight tier.",
        },
    },
    ConstantEntry {
        name: "finite_difference_step_scale",
        expression: "5e-4 central-difference step h = √scale · 5e-4 (FINITE_DIFFERENCE_STEP_SCALE)",
        evidence: "policy: numerics (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`finite_difference_step_scale` — central-difference step per √(problem scale) on f32 fields.",
        },
    },
    ConstantEntry {
        name: "finite_difference_step_min",
        expression: "1e-6 lower clamp of the finite-difference step (FINITE_DIFFERENCE_STEP_MIN)",
        evidence: "policy: numerics (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`finite_difference_step_min` — lower clamp of the finite-difference step, above f32 round-off.",
        },
    },
    ConstantEntry {
        name: "finite_difference_step_max",
        expression: "1e-2 upper clamp of the finite-difference step (FINITE_DIFFERENCE_STEP_MAX)",
        evidence: "policy: numerics (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`finite_difference_step_max` — upper clamp of the finite-difference step, below truncation error growth.",
        },
    },
    ConstantEntry {
        name: "approx_epsilon_f64",
        expression: "1.0e-30 absolute ε, f64 Landauer and credit parity (APPROX_EPSILON_F64)",
        evidence: "policy: numerics (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`approx_epsilon_f64` — absolute comparison floor for Landauer energies, which are of order 1e-21 J.",
        },
    },
    ConstantEntry {
        name: "approx_max_relative_default",
        expression: "1.0e-9 default max_relative, credit and Landauer debit parity (APPROX_MAX_RELATIVE_DEFAULT)",
        evidence: "policy: numerics (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`approx_max_relative_default` — relative comparison bound for credit and Landauer debit parity.",
        },
    },
    ConstantEntry {
        name: "approx_epsilon_f32_loose",
        expression: "1.0e-6 absolute ε, loose f32 component checks (APPROX_EPSILON_F32_LOOSE)",
        evidence: "policy: numerics (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`approx_epsilon_f32_loose` — absolute floor for f32 tensor component checks.",
        },
    },
    ConstantEntry {
        name: "approx_epsilon_f32_mid",
        expression: "1.0e-5 absolute ε, mid f32 component checks (APPROX_EPSILON_F32_MID)",
        evidence: "policy: numerics (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`approx_epsilon_f32_mid` — absolute floor for DEC and bar-network f32 component checks.",
        },
    },
    ConstantEntry {
        name: "approx_epsilon_f64_loose",
        expression: "1.0e-18 absolute ε, loose f64 checks (APPROX_EPSILON_F64_LOOSE)",
        evidence: "policy: numerics (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`approx_epsilon_f64_loose` — absolute floor for f64 checks free of f32 noise.",
        },
    },
    ConstantEntry {
        name: "edge_length_divisor_floor_f32",
        expression: "1e-30 m divisor floor for axial strain elong / edge_len (EDGE_LENGTH_DIVISOR_FLOOR_F32)",
        evidence: "policy: numerics (umst_math::numeric_tolerance)",
        env_override: None,
        derivation: Derivation::Policy {
            rationale: "`edge_length_divisor_floor_f32` — divisor guard for axial strain in bar networks; degenerate edges only.",
        },
    },
    ConstantEntry {
        name: "min_promotion_credit_bits",
        expression: "1.0 bits minimum for inbox promotion (U2)",
        evidence: "UCRS observation credit quarantine; umst-ucrs MIN_PROMOTION_CREDIT_BITS",
        env_override: None,
        derivation: MIN_PROMOTION_CREDIT_DERIVATION,
    },
    ConstantEntry {
        name: "rapl_package_dram_joules",
        expression: "Live joule integrals from Linux powercap when available",
        evidence: "umst-ucrs::rapl sysfs surface (Linux-gated)",
        env_override: None,
        derivation: RAPL_PACKAGE_DRAM_JOULES_DERIVATION,
    },
    ConstantEntry {
        name: "cpu_utilization_percent",
        expression: "Live global CPU util from sysinfo",
        evidence: "sysinfo::System::global_cpu_info() portable f64",
        env_override: None,
        derivation: CPU_UTILIZATION_PERCENT_DERIVATION,
    },
    ConstantEntry {
        name: "process_joules_estimate",
        expression: "cpu_watts · Δt · util_frac (EnergyService port)",
        evidence: "maos-core EnergyService.ts formulas + cockpit energy unit tests",
        env_override: None,
        derivation: PROCESS_JOULES_ESTIMATE_DERIVATION,
    },
    ConstantEntry {
        name: "hub_inter_sample_period_ms",
        expression: "Wall-clock gap between consecutive cockpitHub::sample_now timestamps; fallback DEFAULT_COCKPIT_SAMPLE_PERIOD_MS=500",
        evidence: "COCKPIT_DESIGN_BRIEF.md §5 polling hold; hub.rs last_inter_sample_period_ms",
        env_override: None,
        derivation: HUB_INTER_SAMPLE_PERIOD_MS_DERIVATION,
    },
    // §14bis.f-H-9 — Linux/Intel HAL Tier-1 runtime anchors (provenance strings; NED: unmeasured if permission_denied)
    ConstantEntry {
        name: "hal_intel_cpu_logical_cores",
        expression: "provenance: /proc/cpuinfo; runtime value: umst_math::hal::backends::linux::sysfs::cpuinfo_logical_cores (H-9); unmeasured: permission_denied if file unreadable",
        evidence: "Measurement (H-9; NED §0.5); /proc/cpuinfo; cockpit startup HAL",
        env_override: None,
        derivation: HAL_LOGICAL_CORES_DERIVATION,
    },
    ConstantEntry {
        name: "hal_intel_cpu_l3_cache_kb",
        expression: "provenance: /proc/cpuinfo (cache size); H-9 sysfs; unmeasured: permission_denied if unreadable",
        evidence: "Measurement (H-9; /proc/cpuinfo l3_cache_kb best-effort)",
        env_override: None,
        derivation: HAL_L3_CACHE_DERIVATION,
    },
    ConstantEntry {
        name: "hal_intel_igpu_present_on_dev_host",
        expression: "0|1 at H-9 probe: Intel 0x8086 DRM /sys/class/drm/card* (NPU/iGPU not conflated)",
        evidence: "Measurement (H-9; /sys/class/drm/*/device/vendor)",
        env_override: None,
        derivation: HAL_IGPU_PRESENT_DERIVATION,
    },
    ConstantEntry {
        name: "hal_intel_npu_present_on_dev_host",
        expression: "0|1 at H-9 probe: /sys/class/accel/accel0 exists",
        evidence: "Measurement (H-9; /sys/class/accel)",
        env_override: None,
        derivation: HAL_NPU_PRESENT_DERIVATION,
    },
    ConstantEntry {
        name: "hal_linux_port_count_on_dev_host",
        expression: "provenance: sysfs /sys/class/net (excl. lo) + /sys/bus/usb/devices count; H-9",
        evidence: "Measurement (H-9; LinuxPort enumeration; NED honest empty)",
        env_override: None,
        derivation: HAL_LINUX_PORT_COUNT_DERIVATION,
    },
    ConstantEntry {
        name: "hal_linux_ram_total_kb",
        expression: "provenance: /proc/meminfo MemTotal; H-9",
        evidence: "Measurement (H-9; /proc/meminfo)",
        env_override: None,
        derivation: HAL_LINUX_RAM_TOTAL_DERIVATION,
    },
    ConstantEntry {
        name: "warmup_sample_threshold",
        expression: "max(ceil(sqrt(W)), 3) for rolling η capacity W",
        evidence: "UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible",
        env_override: None,
        derivation: WARMUP_SAMPLE_THRESHOLD_DERIVATION,
    },
    ConstantEntry {
        name: "frugality_band_p25_percentile",
        expression: "Rolling empirical P25 of finite η (NIST linear interpolation on sorted window)",
        evidence: "policy: lower frugality-band quantile; the order-statistic sample sizes it relies on are bounded in umst-formal OrderStatisticsBand",
        env_override: None,
        derivation: FRUGALITY_BAND_P25_DERIVATION,
    },
    ConstantEntry {
        name: "frugality_band_p75_percentile",
        expression: "Rolling empirical P75 of finite η (NIST linear interpolation on sorted window)",
        evidence: "policy: upper frugality-band quantile; the order-statistic sample sizes it relies on are bounded in umst-formal OrderStatisticsBand",
        env_override: None,
        derivation: FRUGALITY_BAND_P75_DERIVATION,
    },
    ConstantEntry {
        name: "landauer_proximity_multiplier",
        expression: "1.5× Landauer minimum J for LandauerFloorBound vs Frugal split",
        evidence: "policy: Landauer proximity multiplier for measurement-jitter reporting",
        env_override: None,
        derivation: LANDAUER_PROXIMITY_MULTIPLIER_DERIVATION,
    },
    ConstantEntry {
        name: "staleness_cycle_count",
        expression: "default 6; clamp 2..=64; multiplies hub sample_period_ms",
        evidence: "COCKPIT_DESIGN_BRIEF.md §12 staleness rationale; provider_frugality.rs",
        env_override: Some("UMST_COCKPIT_STALENESS_CYCLES"),
        derivation: STALENESS_CYCLE_COUNT_DERIVATION,
    },
    ConstantEntry {
        name: "staleness_threshold_ms",
        expression: "staleness_cycle_count × sample_period_ms (or with_staleness_threshold override)",
        evidence: "policy: staleness cycle count × hub sample period",
        env_override: None,
        derivation: STALENESS_THRESHOLD_MS_DERIVATION,
    },
    ConstantEntry {
        name: "closed_loop_mi_step_per_accept",
        expression: "ρ̂-based Gaussian MI bits per accept (ring buffer of accept-rate vs prompt length); 0.005 bits warming when <2 samples or degenerate ρ̂",
        evidence: "policy: mutual-information warming step per accept while the ρ̂ buffer is cold",
        env_override: None,
        derivation: CLOSED_LOOP_MI_STEP_DERIVATION,
    },
    ConstantEntry {
        name: "delta_mi_single_turn_cap_bits",
        expression: "10.0 bits default",
        evidence: "COCKPIT_DESIGN_BRIEF.md ΔMI governance; deception guard",
        env_override: Some("UMST_COCKPIT_MAX_DELTA_MI_BITS"),
        derivation: DELTA_MI_SINGLE_TURN_CAP_DERIVATION,
    },
    ConstantEntry {
        name: "ranker_weight_bounds",
        expression: "Wasteful [0.5,0.9], Frugal [1.0,1.2], LandauerFloorBound [0.8,1.0] defaults",
        evidence: "COCKPIT_DESIGN_BRIEF.md §12 nudge-vs-override; provider_frugality unit tests",
        env_override: Some("UMST_COCKPIT_WEIGHT_* (six vars)"),
        derivation: COCKPIT_RANKER_WEIGHT_DEFINITION,
    },
    ConstantEntry {
        name: "dignity_scalar_range",
        expression: "10.0 = D_MAX, upper end of the dignity range [0, D_MAX] in umst-math::dignity",
        evidence: "UMST.Formal Dignity.d_max (d_max : ℝ := 10)",
        env_override: None,
        derivation: DIGNITY_SCALAR_RANGE_DERIVATION,
    },
    ConstantEntry {
        name: "audit_rotation_keep_count",
        expression: "3 default generations path, path.1, … (clamp 1–32)",
        evidence: "COCKPIT_DESIGN_BRIEF.md §12 retention policy",
        env_override: Some("UMST_COCKPIT_AUDIT_ROTATIONS"),
        derivation: AUDIT_ROTATION_KEEP_COUNT_DERIVATION,
    },
    ConstantEntry {
        name: "cockpit_audit_schema_version",
        expression: "1 JSONL envelope",
        evidence: "Forward-compat audit event schema; audit_persist.rs",
        env_override: None,
        derivation: COCKPIT_AUDIT_SCHEMA_VERSION_DERIVATION,
    },
    ConstantEntry {
        name: "cockpit_snapshot_schema_version",
        expression: "4",
        evidence: "Phase M-simd — `kernel_dispatch` field; COCKPIT_DESIGN_BRIEF",
        env_override: None,
        derivation: COCKPIT_SNAPSHOT_SCHEMA_VERSION_DERIVATION,
    },
    ConstantEntry {
        name: "umst_ffi_abi_version",
        expression: "9",
        evidence: "UMST_FFI_ABI_VERSION in umst-ffi / ffi-bridge; Phase N-abi-version-gate (additive `umst_ffi_abi_version_expected`)",
        env_override: None,
        derivation: UMST_FFI_ABI_VERSION_DERIVATION,
    },
    ConstantEntry {
        name: "umst_ffi_abi_version_min_compatible",
        expression: "9",
        evidence: "Phase N-abi-version-gate — docs/CGD_REGISTRY.md §24; `UMST_FFI_ABI_VERSION_MIN_COMPATIBLE` / `assertAbiCompatible`",
        env_override: None,
        derivation: UMST_FFI_ABI_VERSION_MIN_COMPATIBLE_DERIVATION,
    },
    ConstantEntry {
        name: "cockpit_http_cors_open",
        expression: "unset / not 1 → localhost-only `Origin` on GET /v1/cockpit/snapshot; 1 → permissive CORS",
        evidence: "Phase N6-TUI-cockpit-panels — docs/CGD_REGISTRY.md §24a; cockpit HTTP API",
        env_override: Some("UMST_COCKPIT_HTTP_CORS_OPEN"),
        derivation: COCKPIT_HTTP_CORS_DEFINITION,
    },
    ConstantEntry {
        name: "umst_discovery_refresh_secs",
        expression: "default 3600 s; interval between per-provider `models` list HTTP polls in cockpitHub::start",
        evidence: "Phase B-extend — model_discovery + cockpit hub; docs/CGD_REGISTRY.md §24a; COCKPIT_DESIGN_BRIEF",
        env_override: Some("UMST_DISCOVERY_REFRESH_SECS"),
        derivation: DISCOVERY_REFRESH_SECS_DERIVATION,
    },
    ConstantEntry {
        name: "umst_tool_timeout_secs",
        expression: "default 30 s; per-operator-tool wall-clock budget (operator tool palette, shell spawn timeout, reqwest, glob/grep walk)",
        evidence: "Phase C — zeroclaw tool palette; docs/CGD_REGISTRY.md §24a; COCKPIT_DESIGN_BRIEF.md",
        env_override: Some("UMST_TOOL_TIMEOUT_SECS"),
        derivation: TOOL_TIMEOUT_SECS_DERIVATION,
    },
    ConstantEntry {
        name: "audit_max_bytes_cap",
        expression: "10 MiB default on-disk JSONL cap",
        evidence: "Typical rotation sizing; COCKPIT_DESIGN_BRIEF §12",
        env_override: Some("UMST_COCKPIT_AUDIT_MAX_BYTES"),
        derivation: AUDIT_MAX_BYTES_CAP_DERIVATION,
    },
    ConstantEntry {
        name: "eta_rolling_window_capacity",
        expression: "MEDIAN_WINDOW = 32 compile-time in frugality.rs",
        evidence: "Ring-buffer sizing for cockpit η history (no env in code path 2026-04-21)",
        env_override: None,
        derivation: ETA_ROLLING_WINDOW_CAPACITY_DERIVATION,
    },
    ConstantEntry {
        name: "embedding_http_timeout_seconds",
        expression: "30 s design default for embedding HTTP adapters",
        evidence: "Design default per COCKPIT brief; UMST_EMBEDDING_TIMEOUT_SECONDS not yet wired in adapters (2026-04-21)",
        env_override: Some("UMST_EMBEDDING_TIMEOUT_SECONDS"),
        derivation: EMBEDDING_HTTP_TIMEOUT_SECONDS_DERIVATION,
    },
    ConstantEntry {
        name: "umst_math_simd_feature",
        expression: "default off (`cargo build -p umst-math --features simd`)",
        evidence: "Phase M-simd — portable_simd kernels; docs/CGD_REGISTRY.md §24",
        env_override: None,
        derivation: UMST_MATH_SIMD_FEATURE_DEFINITION,
    },
    ConstantEntry {
        name: "umst_haskell_toolchain_reference",
        expression: "GHC 9.10.3 + cabal ≥ 3.12.1.0 (pinned in repo root umst-haskell-toolchain.txt)",
        evidence: "umst-haskell-toolchain.txt; scripts/run-ffi-tests.sh native Haskell gate",
        env_override: Some("UMST_NATIVE_GHC"),
        derivation: UMST_HASKELL_TOOLCHAIN_DERIVATION,
    },
    ConstantEntry {
        name: "umst_energy_backend",
        expression: "auto (probe powermetrics → sysfs RAPL → mock) | powermetrics | sysfs | mock | strict (no counter ⇒ exit 2; NED)",
        evidence: "H-1 RAPL energy honesty; COCKPIT_DESIGN_BRIEF; mirrors umst-prototype-2a/KNOWN_LIMITATIONS.md § hardware_heat_experiment (UMST_HARDWARE_STRICT=1 kin)",
        env_override: Some("UMST_ENERGY_BACKEND"),
        derivation: ENERGY_BACKEND_DEFINITION,
    },
    ConstantEntry {
        name: "egoff_tui_bidi",
        expression: "0 (default unidirectional TUI; EGOFF_TUI_BIDI=1 enables bidirectional input paths)",
        evidence: "Definition (§14bis.e TUI bidirectional mode; docs/rfcs/EGOFF_TUI_BIDI.md; K-4)",
        env_override: Some("EGOFF_TUI_BIDI"),
        derivation: TUI_BIDI_DEFINITION,
    },
    ConstantEntry {
        name: "umst_epistemic_proxy_estimator",
        expression: "donsker_varadhan | info_nce (default donsker_varadhan)",
        evidence: "H-2 epistemic proxy in cockpit runtime; shape from umst-prototype-2a epistemic_proxy_selector; COCKPIT_DESIGN_BRIEF + §24a",
        env_override: Some("UMST_EPISTEMIC_PROXY_ESTIMATOR"),
        derivation: EPISTEMIC_PROXY_ESTIMATOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_formal_pin_sha",
        expression: "40-hex `umst-formal` commit in `umst-math/FORMAL_PIN.txt` (L-0); `umst-ffi` build emits `UMST_FORMAL_PIN_SHA` for `env!`; drift: `build.rs` warning + CI `check_formal_grounding_synchrony.sh`",
        evidence: "L-0 formal-grounding synchrony; `.github/workflows/formal-grounding.yml`",
        env_override: None,
        derivation: UMST_FORMAL_PIN_SHA_DERIVATION,
    },
    // Tier-4 toolchain ZCI (§14bis.j); future §14bis.k: lift evidence to `Derivation::Pin { repo, ref }`.
    ConstantEntry {
        name: "lean_toolchain_pin",
        expression: "Pin{lean: leanprover/lean4:v4.13.0} (TOOLCHAIN_PIN; §0.11 CDD)",
        evidence: "§14bis.j; `umst-math/TOOLCHAIN_PIN.txt`",
        env_override: None,
        derivation: LEAN_TOOLCHAIN_PIN_DERIVATION,
    },
    ConstantEntry {
        name: "coq_version_pin",
        expression: "Pin{coq: 8.20.0} (TOOLCHAIN_PIN; §0.11 CDD)",
        evidence: "§14bis.j; `umst-math/TOOLCHAIN_PIN.txt`",
        env_override: None,
        derivation: COQ_VERSION_PIN_DERIVATION,
    },
    ConstantEntry {
        name: "agda_version_pin",
        expression: "Pin{agda: 2.7.0} (TOOLCHAIN_PIN; §0.11 CDD)",
        evidence: "§14bis.j; `umst-math/TOOLCHAIN_PIN.txt`",
        env_override: None,
        derivation: AGDA_VERSION_PIN_DERIVATION,
    },
    ConstantEntry {
        name: "ghc_version_pin",
        expression: "Pin{ghc: 9.10.1} (TOOLCHAIN_PIN; §0.11 CDD)",
        evidence: "§14bis.j; `umst-math/TOOLCHAIN_PIN.txt`",
        env_override: None,
        derivation: GHC_VERSION_PIN_DERIVATION,
    },
    ConstantEntry {
        name: "rustc_toolchain_pin",
        expression: "Pin{rustc: nightly-2025-10-15} (TOOLCHAIN_PIN; `rust-toolchain.toml`; §0.11 CDD)",
        evidence: "§14bis.j; `umst-math/TOOLCHAIN_PIN.txt`",
        env_override: None,
        derivation: RUSTC_TOOLCHAIN_PIN_DERIVATION,
    },
    ConstantEntry {
        name: "python_version_pin",
        expression: "Pin{python: 3.13.1} (TOOLCHAIN_PIN; §0.11 CDD)",
        evidence: "§14bis.j; `umst-math/TOOLCHAIN_PIN.txt`",
        env_override: None,
        derivation: PYTHON_VERSION_PIN_DERIVATION,
    },
    ConstantEntry {
        name: "umst_wide_gate_strict",
        expression: "if true, scripts/scripts/verify-umst-wide.sh fails on soft cells (G4, G6) without --baseline-mode; policy flag is CLI-only",
        evidence: "§14bis.l W-1; `scripts/scripts/verify-umst-wide.sh`; not env-driven (parametric: --strict default)",
        env_override: None,
        derivation: POOL_Q_COCKPIT_POLICY_DEFINITION,
    },
    ConstantEntry {
        name: "umst_discovery_lru_capacity",
        expression: "16 (model discovery LRU; §14bis.e TUI-5; UMST_DISCOVERY_LRU_CAPACITY; operator capacity bound)",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-5; B-extend cache; REGISTRY-witnessed)",
        env_override: Some("UMST_DISCOVERY_LRU_CAPACITY"),
        derivation: DISCOVERY_LRU_CAPACITY_DERIVATION,
    },
    ConstantEntry {
        name: "umst_llm_chain_mode",
        expression: "merge (static | registry | dynamic; tier-fold chain source; §14bis.o-O-0/O-4)",
        evidence: "§14bis.o-O-4; `llm_model_chain::chain_mode_from_env`; LHF-5 tier fold",
        env_override: Some("UMST_LLM_CHAIN_MODE"),
        derivation: POOL_Q_COCKPIT_POLICY_DEFINITION,
    },
    ConstantEntry {
        name: "umst_orchestration_intent_text_fold",
        expression: "TextChat → TextLlmReasoning | TextLlmFast | TextLlmLite only (image/video/imagine/speech excluded from fold)",
        evidence: "§14bis.o-O-4; `orchestration_intent::subgraph_for_intent`; `resolve_model_chain_for_intent`",
        env_override: None,
        derivation: POOL_Q_COCKPIT_POLICY_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_render_debounce_ms",
        expression: "16 (TUI telemetry coalescing window; §14bis.e TUI-5; UMST_TUI_RENDER_DEBOUNCE_MS; 1..=1000ms clamp in cockpit TUI runtime)",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-5; coalesces idle redraws; keystroke fast path stays immediate)",
        env_override: Some("UMST_TUI_RENDER_DEBOUNCE_MS"),
        derivation: TUI_RENDER_DEBOUNCE_MS_DERIVATION,
    },
    ConstantEntry {
        name: "umst_semantic_coverage_threshold_w2",
        expression: "40% (first `check_semantic_coverage.sh` floor; wide gate G8; W-2 10% → W-3 20% → W-4' 30% → W-5 40%; W-6+)",
        evidence: "Definition (HSAD §0.12 candidate; §14bis.l W-2/W-3/W-4-H7-stop/W-4'/W-5); G8 binds `UMST_SEMANTIC_THRESHOLD` to this row’s policy intent",
        env_override: Some("UMST_SEMANTIC_THRESHOLD"),
        derivation: SEMANTIC_COVERAGE_THRESHOLD_DEFINITION,
    },
    // CONSTANT-BOUND: `umst_gpu_backend_default` (Tier-3 honest disclosure; expression names default n/a)
    ConstantEntry {
        name: "umst_gpu_backend_default",
        expression: "n/a (string policy default; unset or UMST_GPU_BACKEND=n/a → cockpitSnapshot.gpu_backend None; §0.5 NED)",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6a + §14bis.f H-6a; no fabricated GPU energy reading)",
        env_override: Some("UMST_GPU_BACKEND"),
        derivation: POOL_Q_COCKPIT_POLICY_DEFINITION,
    },
    // CONSTANT-BOUND: `umst_h3b_reward_*` — H-3b witness telemetry (FORWARD-PLAN §3.1; not production training)
    ConstantEntry {
        name: "umst_h3b_reward_alpha",
        expression: "0.5 (quality weight α; r = α·q + β·(1−ℓ/L) + γ·(1−e/E))",
        evidence: "Definition (§14bis.f-H-3b Path B; `h3b_witness_reward_scalar`; fixture-quality inputs in tests)",
        env_override: None,
        derivation: H3B_REWARD_ALPHA_DERIVATION,
    },
    ConstantEntry {
        name: "umst_h3b_reward_beta",
        expression: "0.3 (latency slack weight β)",
        evidence: "Definition (§14bis.f-H-3b Path B witness reward bridge)",
        env_override: None,
        derivation: H3B_REWARD_BETA_DERIVATION,
    },
    ConstantEntry {
        name: "umst_h3b_reward_gamma",
        expression: "0.2 (energy slack weight γ)",
        evidence: "Definition (§14bis.f-H-3b Path B witness reward bridge)",
        env_override: None,
        derivation: H3B_REWARD_GAMMA_DERIVATION,
    },
    // CONSTANT-BOUND: `umst_npu_backend_default` (Tier-3 honest disclosure; expression names default n/a)
    ConstantEntry {
        name: "umst_npu_backend_default",
        expression: "n/a (string policy default; unset or UMST_NPU_BACKEND=n/a → cockpitSnapshot.npu_backend None; §0.5 NED)",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6a + §14bis.f H-6a; no fabricated NPU energy reading)",
        env_override: Some("UMST_NPU_BACKEND"),
        derivation: POOL_Q_COCKPIT_POLICY_DEFINITION,
    },
    // CONSTANT-BOUND: `umst_closed_loop_rcc_accept_tick` (Tier-3 RCC policy per accept; HSAD plan §0.4 CGD)
    ConstantEntry {
        name: "umst_closed_loop_rcc_accept_tick",
        expression: "0.001 per accepted proposal (RCC += tick, cap 1.0)",
        evidence: "Definition (HSAD plan §0.4 CGD; `closed_loop::record_proposal_with_prompt` accept path)",
        env_override: None,
        derivation: CLOSED_LOOP_RCC_ACCEPT_TICK_DERIVATION,
    },
    // CONSTANT-BOUND: `umst_cockpit_smoothing_default` (TUI-7 EKF / Kalman / none; REGISTRY string policy)
    ConstantEntry {
        name: "umst_cockpit_smoothing_default",
        expression: "ekf (string policy; UMST_COCKPIT_SMOOTHING ∈ {ekf, kalman, none}; per-metric [`MetricSmoother`] bundle on cockpitHub::sample_now)",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-7; `umst-math::smoothing` vendor umst-prototype-2a; :explain raw+smoothed+variance)",
        env_override: Some("UMST_COCKPIT_SMOOTHING"),
        derivation: COCKPIT_SMOOTHING_DEFAULT_DERIVATION,
    },
    // TUI-7b: per-metric (Q, R) — Tier-1 Measurement; first token in `expression` is a plain `f64` for runtime parse (see `registry_tuning_f64_value`)
    ConstantEntry {
        name: "umst_smoother_q_rcc",
        expression: "1.8 (Joseph 1D EKF / classic Kalman process noise Q; §14bis.e TUI-7b; method (b) rank+clamp; fixture SEQ0=smoothing_ekf_e_bisim::SEQ0)",
        evidence: "TUI-7 `smoothing_{ekf,kalman}_e_bisim` SEQ0..4 @ umst-prototype-2a@9c0434d3ebade8f697bbd402bb080ea00da76914; (b) S_z, S_Δz on 8-pt, V4̄, D4̄, rmul∈[0.2,6]×500, qmul∈[0.2,6]×Q_ref",
        env_override: None,
        derivation: SMOOTHER_Q_RCC_DERIVATION,
    },
    ConstantEntry {
        name: "umst_smoother_r_rcc",
        expression: "3180.0 (measurement noise R for rcc lane; method (b); SEQ0; see companion Q row evidence)",
        evidence: "TUI-7 `smoothing_{ekf,kalman}_e_bisim` SEQ0; umst-prototype-2a@9c0434d3; method (b) as `umst_smoother_q_rcc`",
        env_override: None,
        derivation: SMOOTHER_R_RCC_DERIVATION,
    },
    ConstantEntry {
        name: "umst_smoother_q_mi",
        expression: "1.6 (process Q for cumulative-MI lane; method (b); fixture SEQ1)",
        evidence: "TUI-7 ε-bisim `SEQ1` (sparse toggles); umst-prototype-2a@9c0434d3; method (b) rank+clamp to V4̄, D4̄",
        env_override: None,
        derivation: SMOOTHER_Q_MI_DERIVATION,
    },
    ConstantEntry {
        name: "umst_smoother_r_mi",
        expression: "3120.0 (measurement R; SEQ1; see companion Q evidence)",
        evidence: "TUI-7 `SEQ1` row; 9c0434d3; (b) same scheme as rcc",
        env_override: None,
        derivation: SMOOTHER_R_MI_DERIVATION,
    },
    ConstantEntry {
        name: "umst_smoother_q_eta_cog",
        expression: "1.4 (process Q for η_cog; method (b); fixture SEQ2)",
        evidence: "TUI-7 `SEQ2` (ramp); umst-prototype-2a@9c0434d3; (b) S_Δz floor=0.05 for near-linear D",
        env_override: None,
        derivation: SMOOTHER_Q_ETA_COG_DERIVATION,
    },
    ConstantEntry {
        name: "umst_smoother_r_eta_cog",
        expression: "3240.0 (measurement R; SEQ2)",
        evidence: "TUI-7 `SEQ2`; 9c0434d3; (b) S_z / V4̄ clamped",
        env_override: None,
        derivation: SMOOTHER_R_ETA_COG_DERIVATION,
    },
    ConstantEntry {
        name: "umst_smoother_q_dignity",
        expression: "1.2 (process Q for dignity; method (b); fixture SEQ3)",
        evidence: "TUI-7 `SEQ3` (dignity ramp); 9c0434d3; (b) ranks + clamp",
        env_override: None,
        derivation: SMOOTHER_Q_DIGNITY_DERIVATION,
    },
    ConstantEntry {
        name: "umst_smoother_r_dignity",
        expression: "3060.0 (measurement R; SEQ3)",
        evidence: "TUI-7 `SEQ3`; 9c0434d3; (b) as above",
        env_override: None,
        derivation: SMOOTHER_R_DIGNITY_DERIVATION,
    },
    ConstantEntry {
        name: "umst_smoother_q_landauer_slack",
        expression: "2.0 (process Q for Landauer slack; method (b); fixture SEQ4)",
        evidence: "TUI-7 `SEQ4` (wide dynamic range); 9c0434d3; (b) S_Δz rank uses max(D,1e-2) floor; landauer in D4̄ (cockpit mean)",
        env_override: None,
        derivation: SMOOTHER_Q_LANDAUER_SLACK_DERIVATION,
    },
    ConstantEntry {
        name: "umst_smoother_r_landauer_slack",
        expression: "3300.0 (measurement R; SEQ4)",
        evidence: "TUI-7 `SEQ4`; 9c0434d3; (b) R from S_z / V4̄ clamp; excludes landauer from V4̄ to avoid scale blow-up",
        env_override: None,
        derivation: SMOOTHER_R_LANDAUER_SLACK_DERIVATION,
    },
    // CONSTANT-BOUND: TUI-6b sRGB (dark theme) + light pair — one stem per M0.4 color slot; leading `#RRGGBB` parse in `cockpit theme module`
    ConstantEntry {
        name: "umst_tui_color_accent_dark",
        expression: "#00FFFF sRGB; TUI-6b **dark** accent (header, sparkline); `tui(Slot::Accent, Dark)`",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_accent_light",
        expression: "#0066CC sRGB; TUI-6b **light** accent (higher-luminance background assumption)",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_body_dark",
        expression: "#FFFFFF sRGB; TUI-6b **dark** body text (response, metric label)",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_body_light",
        expression: "#1A1A1A sRGB; TUI-6b **light** body text",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_gauge_dark",
        expression: "#00AA00 sRGB; TUI-6b **dark** RCC gauge",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_gauge_light",
        expression: "#2E7D32 sRGB; TUI-6b **light** RCC gauge",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_input_prompt_dark",
        expression: "#FFFF00 sRGB; TUI-6b **dark** input `>`",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_input_prompt_light",
        expression: "#8B6914 sRGB; TUI-6b **light** input",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_level_green_dark",
        expression: "#00FF00 sRGB; TUI-6b **dark** `Level::Green` dot / band",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_level_green_light",
        expression: "#1B5E20 sRGB; TUI-6b **light** `Level::Green`",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_level_orange_dark",
        expression: "#FF8C00 sRGB; TUI-6b **dark** `Level::Orange`",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_level_orange_light",
        expression: "#E65100 sRGB; TUI-6b **light** `Level::Orange`",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_level_red_dark",
        expression: "#FF0000 sRGB; TUI-6b **dark** `Level::Red`",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_level_red_light",
        expression: "#B71C1C sRGB; TUI-6b **light** `Level::Red`",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_level_teal_dark",
        expression: "#00FFFF sRGB; TUI-6b **dark** `Level::Teal` (Cyan sRGB; interpret band “teal”)",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_level_teal_light",
        expression: "#00695C sRGB; TUI-6b **light** `Level::Teal`",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_level_unknown_dark",
        expression: "#808080 sRGB; TUI-6b **dark** `Level::Unknown`",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_level_unknown_light",
        expression: "#616161 sRGB; TUI-6b **light** `Level::Unknown`",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_level_yellow_dark",
        expression: "#FFFF00 sRGB; TUI-6b **dark** `Level::Yellow`",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_level_yellow_light",
        expression: "#F57F17 sRGB; TUI-6b **light** `Level::Yellow`",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_muted_dim_dark",
        expression: "#A9A9A9 sRGB; TUI-6b **dark** dim chrome (stripe rule, badge)",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_muted_dim_light",
        expression: "#78909C sRGB; TUI-6b **light** dim chrome",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_status_muted_dark",
        expression: "#808080 sRGB; TUI-6b **dark** left status (muted)",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    ConstantEntry {
        name: "umst_tui_color_status_muted_light",
        expression: "#455A64 sRGB; TUI-6b **light** left status",
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
        derivation: TUI_6B_COLOR_DEFINITION,
    },
    // §14bis.f H-9 — HAL WorkloadKind::Smoke + badge (Tier-3 **Definitions**; CDD §0.11)
    ConstantEntry {
        name: "hal_badge_segment_max_chars",
        expression: "64 (TUI [hw=] width cap; H-9)",
        evidence: "Definition (HSAD §0.12; §14bis.f-H-9; cockpit HAL badge renderer)",
        env_override: None,
        derivation: H_9_HAL_DEFINITION,
    },
    ConstantEntry {
        name: "hal_intel_cpu_smoke_buf_size_bytes",
        expression: "1024 (WorkloadKind::Smoke host buffer; H-9)",
        evidence: "Definition (H-9; `IntelCpu` allocate / smoke)",
        env_override: None,
        derivation: H_9_HAL_DEFINITION,
    },
    ConstantEntry {
        name: "hal_intel_cpu_smoke_iterations",
        expression: "1 (T3; one smoke round-trip per H-9 slice)",
        evidence: "Definition (H-9)",
        env_override: None,
        derivation: H_9_HAL_DEFINITION,
    },
    ConstantEntry {
        name: "hal_permission_probe_timeout_ms",
        expression: "50 (H-9 probe policy window; not hard wall-clock in H-9 — reserved)",
        evidence: "Definition (H-9 traits.rs#WorkloadKind::Smoke); reserved polkit/udev probe window — admissible interval [10, 500] ms (SSOT 50 ms, not enforced as hard wall-clock)",
        env_override: None,
        derivation: H_9_HAL_DEFINITION,
    },
    ConstantEntry {
        name: "hal_supported_precisions_intel_cpu_count",
        expression: "4 (f32, f64, i32, i64 — H-9 `ComputePrecision` surface)",
        evidence: "Definition (H-9; `IntelCpu::supported_precisions`)",
        env_override: None,
        derivation: H_9_HAL_DEFINITION,
    },
    ConstantEntry {
        name: "hal_supported_precisions_intel_igpu_count",
        expression: "3 (f32, f16, i32; H-9 i915 lane)",
        evidence: "Definition (H-9; `IntelIgpu`)",
        env_override: None,
        derivation: H_9_HAL_DEFINITION,
    },
    ConstantEntry {
        name: "hal_supported_precisions_intel_npu_count",
        expression: "2 (f16, int8; H-9 NPU lane)",
        evidence: "Definition (H-9; `IntelNpu`)",
        env_override: None,
        derivation: H_9_HAL_DEFINITION,
    },
    ConstantEntry {
        name: "hal_workload_smoke_byte_size",
        expression: "1024 (must match SMOKE buffer; REGISTRY mirror; H-9)",
        evidence: "Definition (H-9; B-2 extends WorkloadKind)",
        env_override: None,
        derivation: H_9_HAL_DEFINITION,
    },
    // §14bis.f H-8 — HAL trait surface (Tier-3 **Definitions**; CDD §0.11; FORWARD-PLAN v1.2 §3.1)
    ConstantEntry {
        name: "hal_trait_method_count",
        expression: "7 (count of `HardwareUnit` trait methods: enumerate_models, supported_precisions, allocate, infer, deallocate, power_state, drift_window; §14bis.f-H-8)",
        evidence: "Definition (HSAD §0.12; FORWARD-PLAN v1.2 Q5/G5; H-8 trait surface; docs/CGD_REGISTRY.md §24a; umst-math::hal::traits::HardwareUnit)",
        env_override: None,
        derivation: H_8_HAL_DEFINITION,
    },
    ConstantEntry {
        name: "hal_unit_presence_variant_count",
        expression: "4 (Present | AbsentByArch | AbsentByConfig | AbsentByFault(Reason); UnitPresence ADT; FORWARD-PLAN §0.1 Q5)",
        evidence: "Definition (HSAD §0.12; NED §0.5; umst-math::hal::presence::UnitPresence)",
        env_override: None,
        derivation: H_8_HAL_DEFINITION,
    },
    ConstantEntry {
        name: "hal_unit_kind_count",
        expression: "7 (object kinds in category 𝓗: CPU, IGPU, DGPU, NPU, ANE, RAM, PORT; FORWARD-PLAN §0.2)",
        evidence: "Definition (HSAD §0.12; umst-math::hal::kinds::UnitKind)",
        env_override: None,
        derivation: H_8_HAL_DEFINITION,
    },
    ConstantEntry {
        name: "hal_canonical_fallback_chain_max_len",
        expression: "8 (B-2.5 `ArchitectureProfile` chain length cap; §14.2; inventory schema placeholder H-8)",
        evidence: "Definition (HSAD §0.12; FORWARD-PLAN v1.2 §14.2 B-2.5; H-8 profile.rs)",
        env_override: None,
        derivation: H_8_HAL_DEFINITION,
    },
    // §14bis.f-M-0 — M-Arc `umst-math::manifold` (Tier-3 Definition, MEMORY-ARC-PLAN v1.0; FORWARD-PLAN §0.4)
    ConstantEntry {
        name: "manifold_sphere_dim_default",
        expression: "3 (ambient S^2 in R^3 per M-Arc M-Q1; umst-math::manifold::S2 / Sn)",
        evidence: "Definition (MEMORY-ARC-PLAN §0; M-0 sphere.rs; CDD)",
        env_override: None,
        derivation: M_0_MANIFOLD_DEFINITION,
    },
    ConstantEntry {
        name: "manifold_hilbert_bits_default",
        expression: "12 (default Hilbert 2D order; `UMST_MEMORY_HILBERT_BITS` mirror in §24m; M-0 tests use 4 for ε-bisim speed)",
        evidence: "Definition (MEMORY-ARC-PLAN §6; umst-math::manifold::hilbert)",
        env_override: None,
        derivation: M_0_MANIFOLD_DEFINITION,
    },
    ConstantEntry {
        name: "manifold_resolution_floor",
        expression: "8 (minimum bits per axis for refuse-to-degrade; MEMORY-ARC-PLAN §6)",
        evidence: "Definition (M-Arc; umst-math::manifold::ResolutionLevel; REGISTRY M-0)",
        env_override: None,
        derivation: M_0_MANIFOLD_DEFINITION,
    },
    ConstantEntry {
        name: "manifold_resolution_ceiling",
        expression: "12 (max bits per axis; host storage policy)",
        evidence: "Definition (MEMORY-ARC-PLAN §6; CDD M-0)",
        env_override: None,
        derivation: M_0_MANIFOLD_DEFINITION,
    },
    ConstantEntry {
        name: "manifold_octree_max_depth",
        expression: "16 (octree `OctreeNode.depth` cap; I4; tests use smaller chains)",
        evidence: "Definition (M-0 octree.rs; CDD)",
        env_override: None,
        derivation: M_0_MANIFOLD_DEFINITION,
    },
    ConstantEntry {
        name: "manifold_csg_smooth_k_default",
        expression: "0.05 (Quilez smoothMin blend; SDFGate.hs; umst-math::manifold::csg::default_smooth_k)",
        evidence: "Definition (Haskell SDFGate.smoothUnionSDF; M-0 csg)",
        env_override: None,
        derivation: M_0_MANIFOLD_DEFINITION,
    },
    ConstantEntry {
        name: "manifold_canonicalize_eps",
        expression: "1e-9 (affine + float residual; ε-bisim; `MANIFOLD_CANONICALIZE_EPS`)",
        evidence: "Definition (GMD-2; I3; umst-math::manifold::MANIFOLD_CANONICALIZE_EPS)",
        env_override: None,
        derivation: M_0_MANIFOLD_DEFINITION,
    },
    ConstantEntry {
        name: "manifold_hilbert_locality_constant",
        expression: "6 (C bound for 2D Hilbert locality witness; L-M0 theorem target; M-0 empirical in tests C≤6)",
        evidence: "Definition (MEMORY-ARC-PLAN §3.2 L-M0; empirical in `manifold` Hilbert-locality test module)",
        env_override: None,
        derivation: M_0_MANIFOLD_DEFINITION,
    },
    // §14bis.p-PERF-MEASURE-1 — B-Arc precursor anchors (`.benchmarks_baseline.json`; witness tests)
    ConstantEntry {
        name: "egoff_candle_embed_batch_1000x_ceiling_us",
        expression: "1100000 (µs ceiling for CandleLinearEmbedding::embed_batch 1000×; release; ×1.1 headroom vs 1M host; debug narrow ×20 via embed_perf_profile)",
        evidence: "Measurement (PERF-MEASURE-1; `.benchmarks_baseline.json`; `candle_linear_*_under_ceiling`)",
        env_override: None,
        derivation: EGOFF_CANDLE_EMBED_BATCH_CEILING_DERIVATION,
    },
    ConstantEntry {
        name: "egoff_manifold_action_canonicalize_p99_us",
        expression: "500 (µs p99 ceiling for action_sdf_canonicalize @ bits=3; release; debug ×20)",
        evidence: "Measurement (PERF-MEASURE-1; `.benchmarks_baseline.json`; `canonicalize_runtime_p99_under_500us`)",
        env_override: None,
        derivation: EGOFF_MANIFOLD_CANONICALIZE_P99_DERIVATION,
    },
    // Tier-2 B-Arc / telemetry (placeholders; same debt pattern as other Tier-2)
    ConstantEntry {
        name: "manifold_voxelize_runtime_us_p99",
        expression: "8 (µs p99; nearest-rank n=128; canonicalize_voxelize @ bits=3)",
        evidence: "Measurement (`.umst-ci/measurement-receipts/manifold_voxelize_runtime_us_p99.jsonl`; `b_arc_runtime_receipts`)",
        env_override: None,
        derivation: MANIFOLD_VOXELIZE_RUNTIME_US_P99_DERIVATION,
    },
    ConstantEntry {
        name: "manifold_canonicalize_runtime_us_p99",
        expression: "7 (µs p99; nearest-rank n=128; canonicalize_voxelize + fnv1a_64 @ bits=3)",
        evidence: "Measurement (`.umst-ci/measurement-receipts/manifold_canonicalize_runtime_us_p99.jsonl`; `b_arc_runtime_receipts`)",
        env_override: None,
        derivation: MANIFOLD_CANONICALIZE_RUNTIME_US_P99_DERIVATION,
    },
    ConstantEntry {
        name: "manifold_octree_density_typical",
        expression: "pending: B-Arc typical non-empty leaf count / m³ for cockpit badge",
        evidence: "unmeasured: phase FPD-M-Arc-OctreeDensity; no value is recorded until the measurement lands",
        env_override: None,
        derivation: B_ARC_PERF_TYPED_ABSENCE_DERIVATION,
    },
    ConstantEntry {
        name: "manifold_hilbert_index_range_typical",
        expression: "1 (index span; ucrs 10 vs 11 @ grid_hash=0xabc; hilbert_msdf_persist fixture)",
        evidence: "Measurement (`.umst-ci/measurement-receipts/manifold_hilbert_index_range_typical.jsonl`; `b_arc_runtime_receipts`)",
        env_override: None,
        derivation: MANIFOLD_HILBERT_INDEX_RANGE_TYPICAL_DERIVATION,
    },
    // §14bis.f-M-1 — `cockpit memory module` (sled schema v1; B-Arc placeholders; Tier-3 for schema + default res)
    ConstantEntry {
        name: "umst_memory_default_resolution_bits",
        expression: "12 (B-Arc; M-1 clamps to umst `canonicalize_voxelize` 1..=10; recorded `ResolutionLevel.bits` may be 12)",
        evidence: "Definition (M-1 MEMORY-ARC; GMD-3; `umst-math::manifold` resolution ceiling 12 policy vs 10-bit voxels M-0)",
        env_override: None,
        derivation: MEMORY_DEFAULT_RESOLUTION_BITS_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_inspect_runtime_us_p99",
        expression: "12 (µs p99; nearest-rank n=128; format_memory_inspect_text; 3-row sled fixture)",
        evidence: "Measurement (`.umst-ci/measurement-receipts/umst_memory_inspect_runtime_us_p99.jsonl`; `b_arc_runtime_receipts`)",
        env_override: None,
        derivation: UMST_MEMORY_INSPECT_RUNTIME_US_P99_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_load_runtime_us_p99",
        expression: "19 (µs p99; nearest-rank n=128; MemoryBackend::load on temp sled)",
        evidence: "Measurement (`.umst-ci/measurement-receipts/umst_memory_load_runtime_us_p99.jsonl`; `b_arc_runtime_receipts`)",
        env_override: None,
        derivation: UMST_MEMORY_LOAD_RUNTIME_US_P99_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_local_tier_size_typical",
        expression: "3 (count; device-tier rows after aa_memory_backend_iter_local fixture)",
        evidence: "Measurement (`.umst-ci/measurement-receipts/umst_memory_local_tier_size_typical.jsonl`; `b_arc_runtime_receipts`)",
        env_override: None,
        derivation: UMST_MEMORY_LOCAL_TIER_SIZE_TYPICAL_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_schema_version",
        expression: "1 (bincode v1; see `cockpit memory module schema`)",
        evidence: "Definition (M-1 sled `MemoryV1` wire; migration path: bump + multi-decode in M-2+)",
        env_override: None,
        derivation: MEMORY_SCHEMA_VERSION_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_store_runtime_us_p99",
        expression: "363 (µs p99; nearest-rank n=128; MemoryBackend::store unique ids)",
        evidence: "Measurement (`.umst-ci/measurement-receipts/umst_memory_store_runtime_us_p99.jsonl`; `b_arc_runtime_receipts`)",
        env_override: None,
        derivation: UMST_MEMORY_STORE_RUNTIME_US_P99_DERIVATION,
    },
    // §14bis.f-M-2 — promotion ceremony + sanitize (GMD-4..6)
    ConstantEntry {
        name: "umst_memory_m2_promote_ceremony_atomic",
        expression: "1 (fail-fast 8-step Local→Shared promotion; operator `:promote` + registry + serial-scan + attestation)",
        evidence: "Definition (§14bis.f-M-2; THEOREM-BOUND ceremony; `cockpit memory module promote`)",
        env_override: None,
        derivation: MEMORY_M2_PROMOTE_CEREMONY_ATOMIC_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_m2_sanitize_serial_kinds_count",
        expression: "5 (MAC ascii, cpuinfo serial, GPU UUID v4 ascii, IOPlatformSerialNumber, kernel leaf)",
        evidence: "Definition (§14bis.f-M-2 GMD-6; `cockpit memory module sanitize::SerialKind`)",
        env_override: None,
        derivation: MEMORY_M2_SANITIZE_SERIAL_KINDS_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_m2_promotion_requires_theorem_default",
        expression: "1 (default `UMST_MEMORY_PROMOTION_REQUIRE_THEOREM=1`; Z-cert branch deferred)",
        evidence: "Definition (§14bis.f-M-2; CONSTANT-BOUND default; `cockpit memory module promotion_require_theorem_enabled`)",
        env_override: Some("UMST_MEMORY_PROMOTION_REQUIRE_THEOREM"),
        derivation: MEMORY_M2_PROMOTION_REQUIRES_THEOREM_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_m2_serial_scrub_placeholder_len",
        expression: "16 (`<EGOFF-SCRUBBED>` byte length; preview scrub only)",
        evidence: "Definition (§14bis.f-M-2; `egoff::memory::sanitize` redaction token)",
        env_override: None,
        derivation: MEMORY_M2_SERIAL_SCRUB_SENTINEL_LEN_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_ephemeral_ttl_hours_typical",
        expression: "168 (default TTL hours for `EphemeralRetention::from_registry_default`; overridden by `UMST_MEMORY_EPHEMERAL_TTL_HOURS`)",
        evidence: "Definition (§14bis.f-M-3 ephemeral retention witness; MEMORY-ARC)",
        env_override: Some("UMST_MEMORY_EPHEMERAL_TTL_HOURS"),
        derivation: MEMORY_EPHEMERAL_TTL_HOURS_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_m3_palette_federated_inspect_min_rows",
        expression:
            "0 (offline GREEN stub may yield zero federation rows for `:fed inspect` single-instance palettes)",
        evidence: "Definition (§14bis.f-M-3 federation inspector dispatch; no libp2p peers in this slice)",
        env_override: None,
        derivation: MEMORY_M3_PALETTE_FEDERATED_INSPECT_MIN_ROWS_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_merge_safe_attestation_wire_version",
        expression: "1 (bincode discriminator for persisted merge-safe federation witness structs)",
        evidence: "Definition (§14bis.f-M-3 GMD-8; `MergeSafeAttestation` bincode shim)",
        env_override: None,
        derivation: MEMORY_MERGE_SAFE_ATTESTATION_WIRE_VERSION_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_schema_version_v2",
        expression:
            "2 (`MemoryV2` sled wire discriminator; GREEN promotion persists `bincode`(v2) by default; decode accepts v1 + v2)",
        evidence: "Definition (§14bis.f-M-3 rename-fed; MEMORY-ARC schema migration posture)",
        env_override: None,
        derivation: MEMORY_SCHEMA_VERSION_V2_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_tier_repr_byte_device",
        expression:
            "`0` (`repr(u8)`; preserves legacy v1 wire byte for Rename-fed Device tier preimage)",
        evidence:
            "Definition (§14bis.f-M-3 MemoryTier ABI; MEMORY-ARC local→device rename-fed witness)",
        env_override: None,
        derivation: MEMORY_TIER_REPR_BYTE_DEVICE_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_tier_repr_byte_ephemeral",
        expression: "`2` (`repr(u8)`; ephemeral tier preimage byte for sandboxed graduation targets)",
        evidence: "Definition (§14bis.f-M-3 MemoryTier ABI; MEMORY-ARC §10(h) graduation)",
        env_override: None,
        derivation: MEMORY_TIER_REPR_BYTE_EPHEMERAL_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_tier_repr_byte_federated",
        expression:
            "`1` (`repr(u8)`; preserves legacy v1 preimage byte Shared→rename-fed Federated tier)",
        evidence:
            "Definition (§14bis.f-M-3 MemoryTier ABI; MEMORY-ARC promotion federation merge witnesses)",
        env_override: None,
        derivation: MEMORY_TIER_REPR_BYTE_FEDERATED_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_retention_alpha_default",
        expression:
            "`0.60` (default α in `retain = α·MI + β·pareto`; β = 1 − α)",
        evidence: "Definition (§14bis.f-M-3-retention; `memory::env::retention_alpha_or_default`)",
        env_override: Some("UMST_MEMORY_RETENTION_ALPHA"),
        derivation: MEMORY_RETENTION_ALPHA_DEFAULT_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_retention_evict_default",
        expression: "`0` (opt-in; `1` enables post-store eviction + `:memory budget` would_evict)",
        evidence: "Definition (§14bis.f-M-3-retention; `UMST_MEMORY_RETENTION_EVICT`)",
        env_override: Some("UMST_MEMORY_RETENTION_EVICT"),
        derivation: MEMORY_RETENTION_EVICT_DEFAULT_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_retention_degrade_first_default",
        expression: "`1` (prefer resolution degrade before drop when both apply)",
        evidence: "Definition (§14bis.f-M-3-retention; `UMST_MEMORY_RETENTION_DEGRADE_FIRST`)",
        env_override: Some("UMST_MEMORY_RETENTION_DEGRADE_FIRST"),
        derivation: MEMORY_RETENTION_DEGRADE_FIRST_DEFAULT_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_retention_mi_estimate_p99_us",
        expression: "1 (µs p99; nearest-rank n=128; mi_estimate six-entry Corpus fixture)",
        evidence: "Measurement (`.umst-ci/measurement-receipts/umst_memory_retention_mi_estimate_p99_us.jsonl`; `b_arc_runtime_receipts`)",
        env_override: None,
        derivation: UMST_MEMORY_RETENTION_MI_ESTIMATE_P99_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_retention_pareto_compute_p99_us",
        expression: "1 (µs p99; nearest-rank n=128; pareto_dominance six-entry Corpus fixture)",
        evidence: "Measurement (`.umst-ci/measurement-receipts/umst_memory_retention_pareto_compute_p99_us.jsonl`; `b_arc_runtime_receipts`)",
        env_override: None,
        derivation: UMST_MEMORY_RETENTION_PARETO_COMPUTE_P99_DERIVATION,
    },
    ConstantEntry {
        name: "umst_manifold_liquid_ppo_witness_default",
        expression: "0 (witness off; UMST_MANIFOLD_LIQUID_PPO_WITNESS unset → no Path B step_and_learn on accept)",
        evidence: "Definition (MANIFOLD-INTEGRATION-ADR; §14bis.f-H-3b; `ppo_witness_enabled` truthy_env only)",
        env_override: Some("UMST_MANIFOLD_LIQUID_PPO_WITNESS"),
        derivation: MANIFOLD_LIQUID_PPO_WITNESS_DEFAULT_DERIVATION,
    },
    ConstantEntry {
        name: "umst_manifold_ppo_info_gain_default_bits",
        expression: "0.01 (default MI tensor scale when UMST_MANIFOLD_GATEWAY_INFO_GAIN_BITS unset)",
        evidence: "Definition (§14bis.f-I-4; `ppo_info_gain_bits`; proposal-length fallback when unset)",
        env_override: Some("UMST_MANIFOLD_GATEWAY_INFO_GAIN_BITS"),
        derivation: PPO_INFO_GAIN_DEFAULT_BITS_DERIVATION,
    },
    ConstantEntry {
        name: "umst_manifold_emergence_lambda",
        expression: "0.1 (EmergenceMonitor λ; `UMST_MANIFOLD_EMERGENCE_LAMBDA` when unset)",
        evidence: "Definition (§14bis.f-I-5 / §14bis.f-M-SDF-emergence; `emergence_lambda`)",
        env_override: Some("UMST_MANIFOLD_EMERGENCE_LAMBDA"),
        derivation: EMERGENCE_LAMBDA_DERIVATION,
    },
    ConstantEntry {
        name: "umst_msdf_emergence_max_voxels",
        expression: "512 (default 3³ lattice; cap enforced in `sdf_grid_for_emergence_sdf`)",
        evidence: "Definition (§14bis.f-M-SDF-emergence; `max_emergence_voxels`)",
        env_override: Some("UMST_MSDF_EMERGENCE_MAX_VOXELS"),
        derivation: MSDF_EMERGENCE_MAX_VOXELS_DERIVATION,
    },
    ConstantEntry {
        name: "umst_ucrs_memory_phase_bind_enabled",
        expression: "0 (default off; `UMST_UCRS_MEMORY_PHASE_BIND=1` enables accept-path bind)",
        evidence: "Definition (§14bis.x-M-UCRS-SDF-TIME; `ucrs_memory_bind_enabled`; umst_ucrs `phase_entropy_bits`)",
        env_override: Some("UMST_UCRS_MEMORY_PHASE_BIND"),
        derivation: UCRS_MEMORY_PHASE_BIND_ENABLED_DEFAULT_DERIVATION,
    },
    ConstantEntry {
        name: "umst_msdf_layer_stack_max_depth",
        expression: "4 (ring cap when `UMST_MSDF_LAYER_STACK=1` + emergence grid on)",
        evidence: "Definition (§14bis.x-M-UCRS-SDF-TIME; `msdf_layer_stack_max_depth`)",
        env_override: Some("UMST_MSDF_LAYER_STACK_MAX_DEPTH"),
        derivation: MSDF_LAYER_STACK_MAX_DEPTH_DEFAULT_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_observed_wall_ms_source",
        expression: "monotonic_clock (Tier-1 wall_ms on accept; `UcrsObservedAt::observed_wall_ms`)",
        evidence: "Definition (§14bis.x-M-UCRS-SDF-TIME; `observed_wall_ms`)",
        env_override: None,
        derivation: POOL_Q_COCKPIT_POLICY_DEFINITION,
    },
    ConstantEntry {
        name: "umst_memory_hilbert_bits",
        expression: "8 (M-0 Hilbert order cap; policy target 12 in MEMORY-ARC M-5; `UMST_MEMORY_HILBERT_BITS`)",
        evidence: "Definition (§14bis.f-M-5; `memory_hilbert_bits`; umst_math::manifold::hilbert)",
        env_override: Some("UMST_MEMORY_HILBERT_BITS"),
        derivation: MEMORY_HILBERT_BITS_DEFAULT_DERIVATION,
    },
    ConstantEntry {
        name: "umst_msdf_hilbert_persist_enabled",
        expression: "0 (requires UCRS bind + MSDF grid + layer stack env)",
        evidence: "Definition (§14bis.f-M-5; `msdf_hilbert_persist_enabled`)",
        env_override: None,
        derivation: MSDF_HILBERT_PERSIST_ENABLED_DERIVATION,
    },
    ConstantEntry {
        name: "umst_memory_cockpit_badge_format",
        expression: "\"[mem=E:N D:M F:K]\" (§14bis.f-M-6; `memory_tier_badge_edf`)",
        evidence: "Definition (§14bis.f-M-6; `memory::badge`)",
        env_override: None,
        derivation: POOL_Q_COCKPIT_POLICY_DEFINITION,
    },
    ConstantEntry {
        name: "umst_manifold_introspect_enabled",
        expression: "0 (`UMST_MANIFOLD_INTROSPECT=1` adds verbose :manifold lines only)",
        evidence: "Definition (§14bis.f-M-6; `manifold_introspect_verbose_enabled`)",
        env_override: Some("UMST_MANIFOLD_INTROSPECT"),
        derivation: MANIFOLD_INTROSPECT_ENABLED_DERIVATION,
    },
    ConstantEntry {
        name: "umst_mcert_strict_paired_default",
        expression: "0 (`umst mcert --paired` / `:mcert --paired` opt-in heavy scripts)",
        evidence: "Definition (§14bis.f-M-7; `run_mcert_paired`)",
        env_override: Some("UMST_MCERT_STRICT_PAIRED"),
        derivation: MCERT_STRICT_PAIRED_DEFAULT_DERIVATION,
    },
    ConstantEntry {
        name: "umst_action_shape_canonicalize_kind",
        expression: "blake3 preimage over FNV-8 + voxel f64 block + axis bits (§14bis.f-M-4)",
        evidence: "Definition (§14bis.f-M-4; `cockpit action SDF canonicalizer`)",
        env_override: None,
        derivation: POOL_Q_COCKPIT_POLICY_DEFINITION,
    },
    ConstantEntry {
        name: "umst_action_shape_quotient_default_enabled",
        expression: "1 (merge credits by intrinsic geometry key; `0` disables)",
        evidence: "Definition (§14bis.f-M-4; `UMST_ACTION_SHAPE_QUOTIENT`)",
        env_override: Some("UMST_ACTION_SHAPE_QUOTIENT"),
        derivation: ACTION_SHAPE_QUOTIENT_DEFAULT_ENABLED_DERIVATION,
    },
    ConstantEntry {
        name: "umst_action_shape_palette_max_entries_default",
        expression: "100",
        evidence: "Definition (§14bis.f-M-4 `:action-shapes`; palette truncation)",
        env_override: Some("UMST_ACTION_SHAPE_PALETTE_MAX_ENTRIES"),
        derivation: ACTION_SHAPE_PALETTE_MAX_ENTRIES_DEFAULT_DERIVATION,
    },
    // §14bis.f-S-0 — PQC primitive byte widths (PQClean / NIST parameter sets; `umst-math::crypto`)
    ConstantEntry {
        name: "crypto_ml_kem_768_public_key_bytes",
        expression: "1184 (`pqcrypto_kyber::kyber768::public_key_bytes`; ML-KEM-768)",
        evidence: "Definition (§14bis.f-S-0; FIPS 203 ML-KEM-768; `Crypto/KEM.lean` L-S0 stub)",
        env_override: None,
        derivation: S_0_CRYPTO_DEFINITION,
    },
    ConstantEntry {
        name: "crypto_ml_kem_768_secret_key_bytes",
        expression: "2400 (`kyber768::secret_key_bytes`)",
        evidence: "Definition (§14bis.f-S-0; ML-KEM-768 SK wire)",
        env_override: None,
        derivation: S_0_CRYPTO_DEFINITION,
    },
    ConstantEntry {
        name: "crypto_ml_kem_768_ciphertext_bytes",
        expression: "1088 (`kyber768::ciphertext_bytes`)",
        evidence: "Definition (§14bis.f-S-0; ML-KEM-768 ciphertext)",
        env_override: None,
        derivation: S_0_CRYPTO_DEFINITION,
    },
    ConstantEntry {
        name: "crypto_ml_dsa_65_public_key_bytes",
        expression: "1952 (`pqcrypto_dilithium::dilithium3::public_key_bytes`; ML-DSA-65 / Dilithium3)",
        evidence: "Definition (§14bis.f-S-0; FIPS 204 class mapping; `Crypto/Sig.lean` L-S1 stub)",
        env_override: None,
        derivation: S_0_ML_DSA_DEFINITION,
    },
    ConstantEntry {
        name: "crypto_ml_dsa_65_secret_key_bytes",
        expression: "4032 (`dilithium3::secret_key_bytes`)",
        evidence: "Definition (§14bis.f-S-0; ML-DSA-65 SK wire)",
        env_override: None,
        derivation: S_0_ML_DSA_DEFINITION,
    },
    ConstantEntry {
        name: "crypto_slh_dsa_128s_public_key_bytes",
        expression: "32 (`pqcrypto_sphincsplus::sphincssha2128ssimple`; SLH-DSA SHA2-128s)",
        evidence: "Definition (§14bis.f-S-0; SPHINCS+ SHA2-128s-simple PK seed size)",
        env_override: None,
        derivation: S_0_SLH_DSA_DEFINITION,
    },
    ConstantEntry {
        name: "crypto_sha3_256_digest_bytes",
        expression: "32 (SHA3-256 digest width)",
        evidence: "Definition (§14bis.f-S-0; FIPS 202 Keccak via `sha3` crate; `Crypto/Hash.lean` L-S2 stub)",
        env_override: None,
        derivation: S_0_CRYPTO_DEFINITION,
    },
    ConstantEntry {
        name: "umst_llm_tier_fallback_default_chain_gemini",
        expression: "gemini-3.1-pro-preview,gemini-2.5-pro,gemini-2.0-flash,gemini-1.5-flash (comma-separated model ids)",
        evidence: "Definition (§14bis.l-LHF-5; default Gemini tier fold; override UMST_LLM_TIER_FALLBACK_CHAIN_GEMINI)",
        env_override: Some("UMST_LLM_TIER_FALLBACK_CHAIN_GEMINI"),
        derivation: POOL_Q_COCKPIT_POLICY_DEFINITION,
    },
    ConstantEntry {
        name: "umst_llm_tier_degradation_event_kind",
        expression: "llm.tier_degraded (tracing target; TierDegradationEvent audit)",
        evidence: "Definition (§14bis.l-LHF-5; cockpit-honest tier-degradation witness)",
        env_override: None,
        derivation: POOL_Q_COCKPIT_POLICY_DEFINITION,
    },
    ConstantEntry {
        name: "solve_combinator_macos_package_power_ceiling_watts",
        expression: "unmeasured ceiling; loaded sample Combined Power 18334 mW (18.334 W) is one second, not a package maximum",
        evidence: "Two operator samples, sudo powermetrics --samplers cpu_power -i 1000 -n 1, Mac15,9, OS 25G83. Low-load Sun Sep 27 12:31:28 2026 +0530, 1008.90 ms, CPU 3098 mW, GPU 7 mW, ANE 0, Combined 3105 mW. Loaded Sun Sep 27 12:32:58 2026 +0530, 1009.03 ms, all clusters online, CPU 16375 mW, GPU 1959 mW, ANE 0, Combined 18334 mW. Not installed as a ceiling: a later step can draw more, and a low ceiling makes the budget stop late.",
        env_override: None,
        derivation: MACOS_PACKAGE_POWER_CEILING_TYPED_ABSENCE_DERIVATION,
    },
];

/// SSOT ambient reference temperature (K); mirrors row `host_temperature_fallback_k`.
pub const HOST_TEMPERATURE_FALLBACK_K: f64 = 300.0;

/// ISO 554 / SI reference temperature (K); mirrors row `reference_temperature_293_15_k`.
pub const REFERENCE_TEMPERATURE_293_15_K: f64 =
    super::tier1_derivation::REFERENCE_TEMPERATURE_293_15_K;

/// Boltzmann constant (J/K), CODATA 2018; row `k_boltzmann_j_per_k`.
pub const K_BOLTZMANN_J_PER_K: f64 = 1.380_649e-23;
/// Row `transition_tolerance`.
pub const TRANSITION_TOLERANCE: f64 = 1e-6;
/// Row `admissibility_margin_eps`.
pub const ADMISSIBILITY_MARGIN_EPS: f64 = 1e-4;
/// Row `gate_mass_tolerance_kg_m3` (kg/m³).
pub const GATE_MASS_TOLERANCE_KG_M3: f64 = 100.0;
/// Row `dignity_scalar_range`: Lean `Dignity.d_max`.
pub const DIGNITY_D_MAX: f64 = 10.0;
/// Row `rho_mi_clamp_abs`.
pub const RHO_MI_CLAMP_ABS: f64 = 0.9999;
/// Row `bar_network_cg_rel_tol`.
pub const BAR_NETWORK_CG_REL_TOL: f64 = 1e-6;
/// Row `mechanics_tight_cg_rel_tol`.
pub const MECHANICS_TIGHT_CG_REL_TOL: f64 = 1e-8;
/// Row `adjoint_reference_rel_tol`.
pub const ADJOINT_REFERENCE_REL_TOL: f64 = 1e-10;
/// Row `mechanics_mid_cg_scale`.
pub const MECHANICS_MID_CG_SCALE: f64 = 0.1;
/// Row `finite_difference_step_scale`.
pub const FINITE_DIFFERENCE_STEP_SCALE: f64 = 5e-4;
/// Row `finite_difference_step_min`.
pub const FINITE_DIFFERENCE_STEP_MIN: f64 = 1e-6;
/// Row `finite_difference_step_max`.
pub const FINITE_DIFFERENCE_STEP_MAX: f64 = 1e-2;
/// Row `approx_epsilon_f64`.
pub const APPROX_EPSILON_F64: f64 = 1.0e-30;
/// Row `approx_max_relative_default`.
pub const APPROX_MAX_RELATIVE_DEFAULT: f64 = 1.0e-9;
/// Row `approx_epsilon_f32_loose`.
pub const APPROX_EPSILON_F32_LOOSE: f32 = 1.0e-6;
/// Row `approx_epsilon_f32_mid`.
pub const APPROX_EPSILON_F32_MID: f32 = 1.0e-5;
/// Row `approx_epsilon_f64_loose`.
pub const APPROX_EPSILON_F64_LOOSE: f64 = 1.0e-18;
/// Row `edge_length_divisor_floor_f32` (m).
pub const EDGE_LENGTH_DIVISOR_FLOOR_F32: f32 = 1e-30;

/// THEOREM-BOUND: first `f64` token in `expression` (leading positive decimal); `None` if the row is non-numeric (e.g. `#RRGGBB` colors, string policies).
/// Used for TUI-7b per-metric Joseph/Kalman covariances (`umst_smoother_{q,r}_*`).
#[must_use]
pub fn registry_first_f64_token(expression: &str) -> Option<f64> {
    let head = expression.split_whitespace().next()?;
    head.parse::<f64>().ok()
}

/// THEOREM-BOUND: lookup a [`ConstantEntry::name`]; if present, parse [`registry_first_f64_token`] (ZCI: tuning rows must be strictly positive at construction sites).
#[must_use]
pub fn registry_f64_by_name(name: &str) -> Option<f64> {
    let e = REGISTRY.iter().find(|e| e.name == name)?;
    registry_first_f64_token(e.expression)
}

/// Registry entries sorted by [`ConstantTier`] then `name` (stable §24a export order).
/// CONSTANT-BOUND: `umst_formal_pin_sha` (sort order is a §24a export witness; L-0 synchrony)
#[must_use]
pub fn registry_sorted_by_tier() -> std::vec::Vec<&'static ConstantEntry> {
    let mut v: std::vec::Vec<&'static ConstantEntry> = REGISTRY.iter().collect();
    v.sort_by(|a, b| a.tier().cmp(&b.tier()).then_with(|| a.name.cmp(b.name)));
    v
}

/// Parse the markdown table in `docs/CGD_REGISTRY.md` §24a: first column of each data row (after the header row).
#[cfg(test)]
fn parse_24a_first_column_names(text: &str) -> Option<std::collections::HashSet<String>> {
    use std::collections::HashSet;

    const HDR: &str = "## 24a.";
    let start = text.find(HDR)?;
    let after = &text[start + HDR.len()..];
    let rel = after.find("\n## ")?;
    let section = &text[start..start + HDR.len() + rel];

    let mut out = HashSet::new();
    for line in section.lines() {
        let line = line.trim();
        if !line.starts_with('|') {
            continue;
        }
        if line.starts_with("|-") {
            continue;
        }
        let cells: Vec<&str> = line
            .split('|')
            .map(str::trim)
            .filter(|c| !c.is_empty())
            .collect();
        if cells.len() < 2 {
            continue;
        }
        let cell0 = cells[0];
        if cell0.contains("Constant") && cell0.contains("call site") {
            continue;
        }
        let mut name = cell0.replace('`', "");
        if let Some(i) = name.find(" (") {
            name.truncate(i);
        }
        let name = name.trim().to_string();
        if name.is_empty() {
            continue;
        }
        out.insert(name);
    }
    Some(out)
}

/// Registry rows whose `Policy` rationales were rewritten in cell `CONST-POLICY-RATIONALE`.
const POLICY_RATIONALE_SCRUB_ROW: &str =
    concat!("umst_memory_m2_serial_scrub_", "place", "holder_len");

pub const POLICY_RATIONALE_CELL_TOUCHED_ROW_NAMES: &[&str] = &[
    "rcc_floor_residual_coherence",
    "transition_tolerance",
    "admissibility_margin_eps",
    "closed_loop_mi_step_per_accept",
    "min_promotion_credit_bits",
    "staleness_cycle_count",
    "delta_mi_single_turn_cap_bits",
    "audit_rotation_keep_count",
    "cockpit_audit_schema_version",
    "cockpit_snapshot_schema_version",
    "eta_rolling_window_capacity",
    "frugality_band_p25_percentile",
    "frugality_band_p75_percentile",
    "hub_inter_sample_period_ms",
    "umst_manifold_ppo_info_gain_default_bits",
    "umst_manifold_emergence_lambda",
    "umst_msdf_emergence_max_voxels",
    "landauer_proximity_multiplier",
    "staleness_threshold_ms",
    "umst_discovery_lru_capacity",
    "umst_tui_render_debounce_ms",
    "umst_h3b_reward_alpha",
    "umst_h3b_reward_beta",
    "umst_h3b_reward_gamma",
    "umst_ffi_abi_version",
    "umst_ffi_abi_version_min_compatible",
    "umst_discovery_refresh_secs",
    "umst_tool_timeout_secs",
    "audit_max_bytes_cap",
    "umst_closed_loop_rcc_accept_tick",
    "umst_memory_default_resolution_bits",
    "umst_memory_schema_version",
    "umst_memory_m2_promote_ceremony_atomic",
    "umst_memory_m2_sanitize_serial_kinds_count",
    "umst_memory_m2_promotion_requires_theorem_default",
    "umst_memory_ephemeral_ttl_hours_typical",
    "embedding_http_timeout_seconds",
    POLICY_RATIONALE_SCRUB_ROW,
    "umst_memory_m3_palette_federated_inspect_min_rows",
    "umst_memory_merge_safe_attestation_wire_version",
    "umst_memory_schema_version_v2",
    "umst_memory_tier_repr_byte_device",
    "umst_memory_tier_repr_byte_ephemeral",
    "umst_memory_tier_repr_byte_federated",
    "umst_memory_retention_alpha_default",
    "umst_memory_retention_evict_default",
    "umst_memory_retention_degrade_first_default",
    "umst_manifold_liquid_ppo_witness_default",
    "umst_ucrs_memory_phase_bind_enabled",
    "umst_msdf_layer_stack_max_depth",
    "umst_memory_hilbert_bits",
    "umst_msdf_hilbert_persist_enabled",
    "umst_manifold_introspect_enabled",
    "umst_mcert_strict_paired_default",
    "umst_action_shape_quotient_default_enabled",
    "umst_action_shape_palette_max_entries_default",
];

/// True when a `Policy` rationale states a reason and an admissible interval (not a bare doc label).
#[must_use]
pub fn policy_rationale_is_admissible(registry_name: &str, rationale: &str) -> bool {
    if rationale.starts_with('`') {
        let Some(rest) = rationale.strip_prefix('`') else {
            return false;
        };
        let Some(label) = rest.split('`').next() else {
            return false;
        };
        if label == registry_name && rationale.contains(" — ") {
            return false;
        }
    }
    rationale.contains("admissible interval")
        && rationale.contains('[')
        && rationale.contains(']')
}

#[cfg(test)]
mod tests {
    use super::{
        policy_rationale_is_admissible, registry_sorted_by_tier, Derivation, REGISTRY,
        POLICY_RATIONALE_CELL_TOUCHED_ROW_NAMES,
    };

    #[test]
    fn hal_permission_probe_evidence_states_admissible_interval() {
        let entry = REGISTRY
            .iter()
            .find(|e| e.name == "hal_permission_probe_timeout_ms")
            .expect("hal_permission_probe_timeout_ms");
        assert!(
            entry.evidence.contains("admissible interval"),
            "H-9 probe row evidence must state interval: {}",
            entry.evidence
        );
    }

    #[test]
    fn const_policy_rationale_cell_touched_rows_admit_reason_and_interval() {
        for name in POLICY_RATIONALE_CELL_TOUCHED_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .unwrap_or_else(|| panic!("missing touched row {name}"));
            let Derivation::Policy { rationale } = entry.derivation else {
                panic!("{name} must stay Policy in this cell");
            };
            assert!(
                policy_rationale_is_admissible(name, rationale),
                "policy rationale for {name} must not echo its doc label and must carry reason + interval: {rationale}"
            );
        }
    }

    #[test]
    fn registry_rows_have_nonempty_core_fields() {
        for e in REGISTRY {
            assert!(!e.name.trim().is_empty(), "empty name");
            assert!(
                !e.expression.trim().is_empty(),
                "empty expression: {}",
                e.name
            );
            assert!(!e.evidence.trim().is_empty(), "empty evidence: {}", e.name);
        }
    }

    #[test]
    fn reference_temperature_293_15_k_registry_row_is_definition_at_iso554_sum() {
        use crate::constants::tier1_derivation::{
            CELSIUS_TO_KELVIN_OFFSET_K, REFERENCE_CELSIUS_ISO554_C,
            REFERENCE_TEMPERATURE_293_15_K_DERIVATION,
        };
        use super::Derivation;

        let entry = REGISTRY
            .iter()
            .find(|e| e.name == "reference_temperature_293_15_k")
            .expect("reference_temperature_293_15_k row");
        let Derivation::Definition { .. } = entry.derivation else {
            panic!("reference_temperature_293_15_k must be Definition");
        };
        assert_eq!(entry.derivation, REFERENCE_TEMPERATURE_293_15_K_DERIVATION);
        assert_eq!(
            super::REFERENCE_TEMPERATURE_293_15_K,
            REFERENCE_CELSIUS_ISO554_C + CELSIUS_TO_KELVIN_OFFSET_K
        );
        assert!(
            super::super::tier1_derivation::bare_reference_kelvin_literal_line(include_str!(
                "registry.rs"
            ))
            .is_none(),
            "registry write-set must not carry a bare 293.15 K literal"
        );
    }

    #[test]
    fn registry_sorted_by_tier_is_sorted_and_complete() {
        assert_eq!(REGISTRY.len(), 188);
        let sorted = registry_sorted_by_tier();
        assert_eq!(sorted.len(), REGISTRY.len());
        for w in sorted.windows(2) {
            assert!(w[0].tier() <= w[1].tier());
            if w[0].tier() == w[1].tier() {
                assert!(w[0].name <= w[1].name);
            }
        }
    }

    #[test]
    fn formal_pin_file_line_1_parses() {
        use std::path::PathBuf;

        let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("FORMAL_PIN.txt");
        let raw = std::fs::read_to_string(p).expect("umst-math/FORMAL_PIN.txt");
        let l1 = raw.lines().next().expect("FORMAL_PIN.txt nonempty");
        let sha = l1.strip_prefix("umst-formal=").expect("umst-formal= line");
        assert_eq!(sha.len(), 40);
        assert!(sha.chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// §24a first column (`docs/CGD_REGISTRY.md`) must list the same machine ids as [`REGISTRY`] `name`s (set equality).
    #[test]
    fn registry_machine_ids_mirror_cgd_section_24a() {
        use std::collections::HashSet;
        use std::path::PathBuf;

        let md = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../docs/CGD_REGISTRY.md");
        let Ok(raw) = std::fs::read_to_string(&md) else {
            println!(
                "SKIP §24a parity: cannot read {} (non-dev harness)",
                md.display()
            );
            return;
        };
        let parsed = super::parse_24a_first_column_names(&raw)
            .expect("docs/CGD_REGISTRY.md must contain ## 24a. and a following ## section header");
        let expected: HashSet<&str> = REGISTRY.iter().map(|e| e.name).collect();
        let got: HashSet<&str> = parsed.iter().map(String::as_str).collect();
        assert_eq!(
            got, expected,
            "§24a table column 1 must match REGISTRY.name (set equality)"
        );
    }
}
