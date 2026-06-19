//! Compile-time registry of cockpit / core numerical constants (CGD).
//!
//! Human-readable mirror: `egoff/egoffimprov.md` §24a (Constants Grounding Registry).
//! Tier-2 rows carry `pending: Phase FPD-*` until the corresponding formal slice lands.

/// One documented numerical parameter (value, tier, evidence, optional env).
/// CONSTANT-BOUND: `landauer_floor_j_per_bit` (schema; each row is a `ConstantEntry`).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstantEntry {
    /// Stable identifier (matches §24a “Constant” column intent).
    pub name: &'static str,
    /// Human-readable value or derivation.
    pub expression: &'static str,
    /// CGD tier (see `egoffplan.md` §0.4).
    pub tier: ConstantTier,
    /// Lean path, design-brief pointer, or `pending: Phase FPD-*`.
    pub evidence: &'static str,
    /// Environment variable name when operator-overridable (`None` if not).
    pub env_override: Option<&'static str>,
}

/// Five-tier constants grounding taxonomy (`egoffplan.md` §0.4).
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

/// Authoritative registry (keep in lock-step with `egoff/egoffimprov.md` §24a).
/// CONSTANT-BOUND: … + §14bis.f-M-6 (+2) + §14bis.f-M-7 (+1 mcert) = **162** (mirror `egoff/egoffimprov.md` §24a)
pub static REGISTRY: &[ConstantEntry] = &[
    ConstantEntry {
        name: "landauer_floor_j_per_bit",
        expression: "k_B · T · ln(2) J/bit via umst_math::landauer::landauer_bit_energy_joules",
        tier: ConstantTier::Tier0Physical,
        evidence: "UMST.FormalDoubleSlit.LandauerBound + UMST.Formal.EtaCog::etaDenom_pos",
        env_override: None,
    },
    ConstantEntry {
        name: "ln_two_eta_cog_denominator",
        expression: "ln(2) in η_cog Landauer denominator",
        tier: ConstantTier::Tier0Physical,
        evidence: "UMST.Formal.Real.log_two_pos (ln 2 positivity chain)",
        env_override: None,
    },
    ConstantEntry {
        name: "host_temperature_fallback_k",
        expression: "300.0 K default when cockpit T is non-finite (overridable)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "Operator-assumed ambient anchor until junction-temperature telemetry is wired",
        env_override: Some("UMST_COCKPIT_HOST_TEMPERATURE_K"),
    },
    ConstantEntry {
        name: "rapl_package_dram_joules",
        expression: "Live joule integrals from Linux powercap when available",
        tier: ConstantTier::Tier1Measurement,
        evidence: "umst-ucrs::rapl sysfs surface (Linux-gated)",
        env_override: None,
    },
    ConstantEntry {
        name: "cpu_utilization_percent",
        expression: "Live global CPU util from sysinfo",
        tier: ConstantTier::Tier1Measurement,
        evidence: "sysinfo::System::global_cpu_info() portable f64",
        env_override: None,
    },
    ConstantEntry {
        name: "process_joules_estimate",
        expression: "cpu_watts · Δt · util_frac (EnergyService port)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "maos-core EnergyService.ts formulas + egoff cockpit energy unit tests",
        env_override: None,
    },
    ConstantEntry {
        name: "hub_inter_sample_period_ms",
        expression: "Wall-clock gap between consecutive CockpitHub::sample_now timestamps; fallback DEFAULT_COCKPIT_SAMPLE_PERIOD_MS=500",
        tier: ConstantTier::Tier1Measurement,
        evidence: "COCKPIT_DESIGN_BRIEF.md §5 polling hold; hub.rs last_inter_sample_period_ms",
        env_override: None,
    },
    // §14bis.f-H-9 — Linux/Intel HAL Tier-1 runtime anchors (provenance strings; NED: unmeasured if permission_denied)
    ConstantEntry {
        name: "hal_intel_cpu_logical_cores",
        expression: "provenance: /proc/cpuinfo; runtime value: umst_math::hal::backends::linux::sysfs::cpuinfo_logical_cores (H-9); unmeasured: permission_denied if file unreadable",
        tier: ConstantTier::Tier1Measurement,
        evidence: "Measurement (H-9; NED §0.5); /proc/cpuinfo; egoff startup HAL",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_intel_cpu_l3_cache_kb",
        expression: "provenance: /proc/cpuinfo (cache size); H-9 sysfs; unmeasured: permission_denied if unreadable",
        tier: ConstantTier::Tier1Measurement,
        evidence: "Measurement (H-9; /proc/cpuinfo l3_cache_kb best-effort)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_intel_igpu_present_on_dev_host",
        expression: "0|1 at H-9 probe: Intel 0x8086 DRM /sys/class/drm/card* (NPU/iGPU not conflated)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "Measurement (H-9; /sys/class/drm/*/device/vendor)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_intel_npu_present_on_dev_host",
        expression: "0|1 at H-9 probe: /sys/class/accel/accel0 exists",
        tier: ConstantTier::Tier1Measurement,
        evidence: "Measurement (H-9; /sys/class/accel)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_linux_port_count_on_dev_host",
        expression: "provenance: sysfs /sys/class/net (excl. lo) + /sys/bus/usb/devices count; H-9",
        tier: ConstantTier::Tier1Measurement,
        evidence: "Measurement (H-9; LinuxPort enumeration; NED honest empty)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_linux_ram_total_kb",
        expression: "provenance: /proc/meminfo MemTotal; H-9",
        tier: ConstantTier::Tier1Measurement,
        evidence: "Measurement (H-9; /proc/meminfo)",
        env_override: None,
    },
    ConstantEntry {
        name: "warmup_sample_threshold",
        expression: "max(ceil(sqrt(W)), 3) for rolling η capacity W",
        tier: ConstantTier::Tier2Derivable,
        evidence: "UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible",
        env_override: None,
    },
    ConstantEntry {
        name: "frugality_band_p25_percentile",
        expression: "Rolling empirical P25 of finite η (NIST linear interpolation on sorted window)",
        tier: ConstantTier::Tier2Derivable,
        evidence: "UMST.Formal.OrderStatisticsBand::p25_p75_admissibility",
        env_override: None,
    },
    ConstantEntry {
        name: "frugality_band_p75_percentile",
        expression: "Rolling empirical P75 of finite η (NIST linear interpolation on sorted window)",
        tier: ConstantTier::Tier2Derivable,
        evidence: "UMST.Formal.OrderStatisticsBand::p25_p75_admissibility",
        env_override: None,
    },
    ConstantEntry {
        name: "landauer_proximity_multiplier",
        expression: "1.5× Landauer minimum J for LandauerFloorBound vs Frugal split",
        tier: ConstantTier::Tier2Derivable,
        evidence: "pending: Phase FPD-MeasurementJitterBound",
        env_override: None,
    },
    ConstantEntry {
        name: "staleness_cycle_count",
        expression: "default 6; clamp 2..=64; multiplies hub sample_period_ms",
        tier: ConstantTier::Tier3Policy,
        evidence: "COCKPIT_DESIGN_BRIEF.md §12 staleness rationale; provider_frugality.rs",
        env_override: Some("UMST_COCKPIT_STALENESS_CYCLES"),
    },
    ConstantEntry {
        name: "staleness_threshold_ms",
        expression: "staleness_cycle_count × sample_period_ms (or with_staleness_threshold override)",
        tier: ConstantTier::Tier2Derivable,
        evidence: "pending: Phase FPD-TelemetryAutocorrelation",
        env_override: None,
    },
    ConstantEntry {
        name: "closed_loop_mi_step_per_accept",
        expression: "ρ̂-based Gaussian MI bits per accept (ring buffer of accept-rate vs prompt length); 0.005 bits warming when <2 samples or degenerate ρ̂",
        tier: ConstantTier::Tier2Derivable,
        evidence: "UMST.Formal.RhoEstimator::rho_based_mi_formula",
        env_override: None,
    },
    ConstantEntry {
        name: "delta_mi_single_turn_cap_bits",
        expression: "10.0 bits default",
        tier: ConstantTier::Tier3Policy,
        evidence: "COCKPIT_DESIGN_BRIEF.md ΔMI governance; deception guard",
        env_override: Some("UMST_COCKPIT_MAX_DELTA_MI_BITS"),
    },
    ConstantEntry {
        name: "ranker_weight_bounds",
        expression: "Wasteful [0.5,0.9], Frugal [1.0,1.2], LandauerFloorBound [0.8,1.0] defaults",
        tier: ConstantTier::Tier3Policy,
        evidence: "COCKPIT_DESIGN_BRIEF.md §12 nudge-vs-override; provider_frugality unit tests",
        env_override: Some("UMST_COCKPIT_WEIGHT_* (six vars)"),
    },
    ConstantEntry {
        name: "dignity_scalar_range",
        expression: "[0.0, 10.0] / D_MAX in umst-math::dignity",
        tier: ConstantTier::Tier3Policy,
        evidence: "UMST.Formal.Dignity structural bound + design brief",
        env_override: None,
    },
    ConstantEntry {
        name: "audit_rotation_keep_count",
        expression: "3 default generations path, path.1, … (clamp 1–32)",
        tier: ConstantTier::Tier3Policy,
        evidence: "COCKPIT_DESIGN_BRIEF.md §12 retention policy",
        env_override: Some("UMST_COCKPIT_AUDIT_ROTATIONS"),
    },
    ConstantEntry {
        name: "cockpit_audit_schema_version",
        expression: "1 JSONL envelope",
        tier: ConstantTier::Tier3Policy,
        evidence: "Forward-compat audit event schema; audit_persist.rs",
        env_override: None,
    },
    ConstantEntry {
        name: "cockpit_snapshot_schema_version",
        expression: "4",
        tier: ConstantTier::Tier3Policy,
        evidence: "Phase M-simd — `kernel_dispatch` field; COCKPIT_DESIGN_BRIEF",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_ffi_abi_version",
        expression: "8",
        tier: ConstantTier::Tier3Policy,
        evidence: "UMST_FFI_ABI_VERSION in umst-ffi / ffi-bridge; Phase N-abi-version-gate (additive `umst_ffi_abi_version_expected`)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_ffi_abi_version_min_compatible",
        expression: "7",
        tier: ConstantTier::Tier3Policy,
        evidence: "Phase N-abi-version-gate — egoffimprov §24; `UMST_FFI_ABI_VERSION_MIN_COMPATIBLE` / `assertAbiCompatible`",
        env_override: None,
    },
    ConstantEntry {
        name: "cockpit_http_cors_open",
        expression: "unset / not 1 → localhost-only `Origin` on GET /v1/cockpit/snapshot; 1 → permissive CORS",
        tier: ConstantTier::Tier3Policy,
        evidence: "Phase N6-TUI-cockpit-panels — egoffimprov §24a; egoff/src/api.rs",
        env_override: Some("UMST_COCKPIT_HTTP_CORS_OPEN"),
    },
    ConstantEntry {
        name: "umst_discovery_refresh_secs",
        expression: "default 3600 s; interval between per-provider `models` list HTTP polls in CockpitHub::start",
        tier: ConstantTier::Tier3Policy,
        evidence: "Phase B-extend — model_discovery + cockpit hub; egoffimprov §24a; COCKPIT_DESIGN_BRIEF",
        env_override: Some("UMST_DISCOVERY_REFRESH_SECS"),
    },
    ConstantEntry {
        name: "umst_tool_timeout_secs",
        expression: "default 30 s; per-operator-tool wall-clock budget (egoff operator_toolpalette, shell spawn timeout, reqwest, glob/grep walk)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Phase C — zeroclaw tool palette; egoffimprov §24a; egoff/COCKPIT_DESIGN_BRIEF",
        env_override: Some("UMST_TOOL_TIMEOUT_SECS"),
    },
    ConstantEntry {
        name: "audit_max_bytes_cap",
        expression: "10 MiB default on-disk JSONL cap",
        tier: ConstantTier::Tier4Infra,
        evidence: "Typical rotation sizing; COCKPIT_DESIGN_BRIEF §12",
        env_override: Some("UMST_COCKPIT_AUDIT_MAX_BYTES"),
    },
    ConstantEntry {
        name: "eta_rolling_window_capacity",
        expression: "MEDIAN_WINDOW = 32 compile-time in frugality.rs",
        tier: ConstantTier::Tier4Infra,
        evidence: "Ring-buffer sizing for cockpit η history (no env in code path 2026-04-21)",
        env_override: None,
    },
    ConstantEntry {
        name: "embedding_http_timeout_seconds",
        expression: "30 s design default for embedding HTTP adapters",
        tier: ConstantTier::Tier4Infra,
        evidence: "Design default per COCKPIT brief; UMST_EMBEDDING_TIMEOUT_SECONDS not yet wired in adapters (2026-04-21)",
        env_override: Some("UMST_EMBEDDING_TIMEOUT_SECONDS"),
    },
    ConstantEntry {
        name: "umst_math_simd_feature",
        expression: "default off (`cargo build -p umst-math --features simd`)",
        tier: ConstantTier::Tier4Infra,
        evidence: "Phase M-simd — portable_simd kernels; egoffimprov §24",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_haskell_toolchain_reference",
        expression: "GHC 9.10.3 + cabal ≥ 3.12.1.0 (pinned in repo root egoff-haskell-toolchain.txt)",
        tier: ConstantTier::Tier4Infra,
        evidence: "egoff-haskell-toolchain.txt; scripts/run-ffi-tests.sh native Haskell gate",
        env_override: Some("UMST_NATIVE_GHC"),
    },
    ConstantEntry {
        name: "umst_energy_backend",
        expression: "auto (probe powermetrics → sysfs RAPL → mock) | powermetrics | sysfs | mock | strict (no counter ⇒ exit 2; NED)",
        tier: ConstantTier::Tier3Policy,
        evidence: "H-1 RAPL energy honesty; COCKPIT_DESIGN_BRIEF; mirrors umst-prototype-2a/KNOWN_LIMITATIONS.md § hardware_heat_experiment (UMST_HARDWARE_STRICT=1 kin)",
        env_override: Some("UMST_ENERGY_BACKEND"),
    },
    ConstantEntry {
        name: "umst_epistemic_proxy_estimator",
        expression: "donsker_varadhan | info_nce (default donsker_varadhan)",
        tier: ConstantTier::Tier3Policy,
        evidence: "H-2 epistemic proxy in egoff; shape from umst-prototype-2a epistemic_proxy_selector; COCKPIT_DESIGN_BRIEF + §24a",
        env_override: Some("UMST_EPISTEMIC_PROXY_ESTIMATOR"),
    },
    ConstantEntry {
        name: "umst_formal_pin_sha",
        expression: "40-hex `umst-formal` commit in `umst-math/FORMAL_PIN.txt` (L-0); `umst-ffi` build emits `UMST_FORMAL_PIN_SHA` for `env!`; drift: `build.rs` warning + CI `check_formal_grounding_synchrony.sh`",
        tier: ConstantTier::Tier4Infra,
        evidence: "L-0 formal-grounding synchrony; `.github/workflows/formal-grounding.yml`",
        env_override: None,
    },
    // Tier-4 toolchain ZCI (§14bis.j); future §14bis.k: lift evidence to `Derivation::Pin { repo, ref }`.
    ConstantEntry {
        name: "lean_toolchain_pin",
        expression: "Pin{lean: leanprover/lean4:v4.13.0} (TOOLCHAIN_PIN; §0.11 CDD)",
        tier: ConstantTier::Tier4Infra,
        evidence: "§14bis.j; `umst-math/TOOLCHAIN_PIN.txt`",
        env_override: None,
    },
    ConstantEntry {
        name: "coq_version_pin",
        expression: "Pin{coq: 8.20.0} (TOOLCHAIN_PIN; §0.11 CDD)",
        tier: ConstantTier::Tier4Infra,
        evidence: "§14bis.j; `umst-math/TOOLCHAIN_PIN.txt`",
        env_override: None,
    },
    ConstantEntry {
        name: "agda_version_pin",
        expression: "Pin{agda: 2.7.0} (TOOLCHAIN_PIN; §0.11 CDD)",
        tier: ConstantTier::Tier4Infra,
        evidence: "§14bis.j; `umst-math/TOOLCHAIN_PIN.txt`",
        env_override: None,
    },
    ConstantEntry {
        name: "ghc_version_pin",
        expression: "Pin{ghc: 9.10.1} (TOOLCHAIN_PIN; §0.11 CDD)",
        tier: ConstantTier::Tier4Infra,
        evidence: "§14bis.j; `umst-math/TOOLCHAIN_PIN.txt`",
        env_override: None,
    },
    ConstantEntry {
        name: "rustc_toolchain_pin",
        expression: "Pin{rustc: nightly-2025-10-15} (TOOLCHAIN_PIN; `rust-toolchain.toml`; §0.11 CDD)",
        tier: ConstantTier::Tier4Infra,
        evidence: "§14bis.j; `umst-math/TOOLCHAIN_PIN.txt`",
        env_override: None,
    },
    ConstantEntry {
        name: "python_version_pin",
        expression: "Pin{python: 3.13.1} (TOOLCHAIN_PIN; §0.11 CDD)",
        tier: ConstantTier::Tier4Infra,
        evidence: "§14bis.j; `umst-math/TOOLCHAIN_PIN.txt`",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_wide_gate_strict",
        expression: "if true, scripts/verify-egoff-wide.sh fails on soft cells (G4, G6) without --baseline-mode; policy flag is CLI-only",
        tier: ConstantTier::Tier3Policy,
        evidence: "§14bis.l W-1; `scripts/verify-egoff-wide.sh`; not env-driven (parametric: --strict default)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_discovery_lru_capacity",
        expression: "16 (model discovery LRU; §14bis.e TUI-5; UMST_DISCOVERY_LRU_CAPACITY; operator capacity bound)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-5; B-extend cache; REGISTRY-witnessed)",
        env_override: Some("UMST_DISCOVERY_LRU_CAPACITY"),
    },
    ConstantEntry {
        name: "umst_llm_chain_mode",
        expression: "merge (static | registry | dynamic; tier-fold chain source; §14bis.o-O-0/O-4)",
        tier: ConstantTier::Tier3Policy,
        evidence: "§14bis.o-O-4; `llm_model_chain::chain_mode_from_env`; LHF-5 tier fold",
        env_override: Some("UMST_LLM_CHAIN_MODE"),
    },
    ConstantEntry {
        name: "umst_orchestration_intent_text_fold",
        expression: "TextChat → TextLlmReasoning | TextLlmFast | TextLlmLite only (image/video/imagine/speech excluded from fold)",
        tier: ConstantTier::Tier3Policy,
        evidence: "§14bis.o-O-4; `orchestration_intent::subgraph_for_intent`; `resolve_model_chain_for_intent`",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_render_debounce_ms",
        expression: "16 (TUI telemetry coalescing window; §14bis.e TUI-5; UMST_TUI_RENDER_DEBOUNCE_MS; 1..=1000ms clamp in egoff tui runtime)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-5; coalesces idle redraws; keystroke fast path stays immediate)",
        env_override: Some("UMST_TUI_RENDER_DEBOUNCE_MS"),
    },
    ConstantEntry {
        name: "umst_semantic_coverage_threshold_w2",
        expression: "40% (first `check_semantic_coverage.sh` floor; wide gate G8; W-2 10% → W-3 20% → W-4' 30% → W-5 40%; W-6+)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12 candidate; §14bis.l W-2/W-3/W-4-H7-stop/W-4'/W-5); G8 binds `UMST_SEMANTIC_THRESHOLD` to this row’s policy intent",
        env_override: Some("UMST_SEMANTIC_THRESHOLD"),
    },
    // CONSTANT-BOUND: `umst_gpu_backend_default` (Tier-3 honest disclosure; expression names default n/a)
    ConstantEntry {
        name: "umst_gpu_backend_default",
        expression: "n/a (string policy default; unset or UMST_GPU_BACKEND=n/a → CockpitSnapshot.gpu_backend None; §0.5 NED)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6a + §14bis.f H-6a; no fabricated GPU energy reading)",
        env_override: Some("UMST_GPU_BACKEND"),
    },
    // CONSTANT-BOUND: `umst_h3b_reward_*` — H-3b witness telemetry (FORWARD-PLAN §3.1; not production training)
    ConstantEntry {
        name: "umst_h3b_reward_alpha",
        expression: "0.5 (quality weight α; r = α·q + β·(1−ℓ/L) + γ·(1−e/E))",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-H-3b Path B; `h3b_witness_reward_scalar`; fixture-quality inputs in tests)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_h3b_reward_beta",
        expression: "0.3 (latency slack weight β)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-H-3b Path B witness reward bridge)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_h3b_reward_gamma",
        expression: "0.2 (energy slack weight γ)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-H-3b Path B witness reward bridge)",
        env_override: None,
    },
    // CONSTANT-BOUND: `umst_npu_backend_default` (Tier-3 honest disclosure; expression names default n/a)
    ConstantEntry {
        name: "umst_npu_backend_default",
        expression: "n/a (string policy default; unset or UMST_NPU_BACKEND=n/a → CockpitSnapshot.npu_backend None; §0.5 NED)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6a + §14bis.f H-6a; no fabricated NPU energy reading)",
        env_override: Some("UMST_NPU_BACKEND"),
    },
    // CONSTANT-BOUND: `umst_closed_loop_rcc_accept_tick` (Tier-3 RCC policy per accept; egoffplan §0.4 CGD)
    ConstantEntry {
        name: "umst_closed_loop_rcc_accept_tick",
        expression: "0.001 per accepted proposal (RCC += tick, cap 1.0)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (egoffplan §0.4 CGD; `closed_loop::record_proposal_with_prompt` accept path)",
        env_override: None,
    },
    // CONSTANT-BOUND: `umst_cockpit_smoothing_default` (TUI-7 EKF / Kalman / none; REGISTRY string policy)
    ConstantEntry {
        name: "umst_cockpit_smoothing_default",
        expression: "ekf (string policy; UMST_COCKPIT_SMOOTHING ∈ {ekf, kalman, none}; per-metric [`MetricSmoother`] bundle on CockpitHub::sample_now)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-7; `umst-math::smoothing` vendor umst-prototype-2a; :explain raw+smoothed+variance)",
        env_override: Some("UMST_COCKPIT_SMOOTHING"),
    },
    // TUI-7b: per-metric (Q, R) — Tier-1 Measurement; first token in `expression` is a plain `f64` for runtime parse (see `registry_tuning_f64_value`)
    ConstantEntry {
        name: "umst_smoother_q_rcc",
        expression: "1.8 (Joseph 1D EKF / classic Kalman process noise Q; §14bis.e TUI-7b; method (b) rank+clamp; fixture SEQ0=smoothing_ekf_e_bisim::SEQ0)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "TUI-7 `smoothing_{ekf,kalman}_e_bisim` SEQ0..4 @ umst-prototype-2a@9c0434d3ebade8f697bbd402bb080ea00da76914; (b) S_z, S_Δz on 8-pt, V4̄, D4̄, rmul∈[0.2,6]×500, qmul∈[0.2,6]×Q_ref",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_smoother_r_rcc",
        expression: "3180.0 (measurement noise R for rcc lane; method (b); SEQ0; see companion Q row evidence)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "TUI-7 `smoothing_{ekf,kalman}_e_bisim` SEQ0; umst-prototype-2a@9c0434d3; method (b) as `umst_smoother_q_rcc`",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_smoother_q_mi",
        expression: "1.6 (process Q for cumulative-MI lane; method (b); fixture SEQ1)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "TUI-7 ε-bisim `SEQ1` (sparse toggles); umst-prototype-2a@9c0434d3; method (b) rank+clamp to V4̄, D4̄",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_smoother_r_mi",
        expression: "3120.0 (measurement R; SEQ1; see companion Q evidence)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "TUI-7 `SEQ1` row; 9c0434d3; (b) same scheme as rcc",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_smoother_q_eta_cog",
        expression: "1.4 (process Q for η_cog; method (b); fixture SEQ2)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "TUI-7 `SEQ2` (ramp); umst-prototype-2a@9c0434d3; (b) S_Δz floor=0.05 for near-linear D",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_smoother_r_eta_cog",
        expression: "3240.0 (measurement R; SEQ2)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "TUI-7 `SEQ2`; 9c0434d3; (b) S_z / V4̄ clamped",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_smoother_q_dignity",
        expression: "1.2 (process Q for dignity; method (b); fixture SEQ3)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "TUI-7 `SEQ3` (dignity ramp); 9c0434d3; (b) ranks + clamp",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_smoother_r_dignity",
        expression: "3060.0 (measurement R; SEQ3)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "TUI-7 `SEQ3`; 9c0434d3; (b) as above",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_smoother_q_landauer_slack",
        expression: "2.0 (process Q for Landauer slack; method (b); fixture SEQ4)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "TUI-7 `SEQ4` (wide dynamic range); 9c0434d3; (b) S_Δz rank uses max(D,1e-2) floor; landauer in D4̄ (cockpit mean)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_smoother_r_landauer_slack",
        expression: "3300.0 (measurement R; SEQ4)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "TUI-7 `SEQ4`; 9c0434d3; (b) R from S_z / V4̄ clamp; excludes landauer from V4̄ to avoid scale blow-up",
        env_override: None,
    },
    // CONSTANT-BOUND: TUI-6b sRGB (dark theme) + light pair — one stem per M0.4 color slot; leading `#RRGGBB` parse in `egoff::theme`
    ConstantEntry {
        name: "umst_tui_color_accent_dark",
        expression: "#00FFFF sRGB; TUI-6b **dark** accent (header, sparkline); `tui(Slot::Accent, Dark)`",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_accent_light",
        expression: "#0066CC sRGB; TUI-6b **light** accent (higher-luminance background assumption)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_body_dark",
        expression: "#FFFFFF sRGB; TUI-6b **dark** body text (response, metric label)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_body_light",
        expression: "#1A1A1A sRGB; TUI-6b **light** body text",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_gauge_dark",
        expression: "#00AA00 sRGB; TUI-6b **dark** RCC gauge",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_gauge_light",
        expression: "#2E7D32 sRGB; TUI-6b **light** RCC gauge",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_input_prompt_dark",
        expression: "#FFFF00 sRGB; TUI-6b **dark** input `>`",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_input_prompt_light",
        expression: "#8B6914 sRGB; TUI-6b **light** input",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_level_green_dark",
        expression: "#00FF00 sRGB; TUI-6b **dark** `Level::Green` dot / band",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_level_green_light",
        expression: "#1B5E20 sRGB; TUI-6b **light** `Level::Green`",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_level_orange_dark",
        expression: "#FF8C00 sRGB; TUI-6b **dark** `Level::Orange`",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_level_orange_light",
        expression: "#E65100 sRGB; TUI-6b **light** `Level::Orange`",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_level_red_dark",
        expression: "#FF0000 sRGB; TUI-6b **dark** `Level::Red`",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_level_red_light",
        expression: "#B71C1C sRGB; TUI-6b **light** `Level::Red`",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_level_teal_dark",
        expression: "#00FFFF sRGB; TUI-6b **dark** `Level::Teal` (Cyan sRGB; interpret band “teal”)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_level_teal_light",
        expression: "#00695C sRGB; TUI-6b **light** `Level::Teal`",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_level_unknown_dark",
        expression: "#808080 sRGB; TUI-6b **dark** `Level::Unknown`",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_level_unknown_light",
        expression: "#616161 sRGB; TUI-6b **light** `Level::Unknown`",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_level_yellow_dark",
        expression: "#FFFF00 sRGB; TUI-6b **dark** `Level::Yellow`",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_level_yellow_light",
        expression: "#F57F17 sRGB; TUI-6b **light** `Level::Yellow`",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_muted_dim_dark",
        expression: "#A9A9A9 sRGB; TUI-6b **dark** dim chrome (stripe rule, badge)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_muted_dim_light",
        expression: "#78909C sRGB; TUI-6b **light** dim chrome",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_status_muted_dark",
        expression: "#808080 sRGB; TUI-6b **dark** left status (muted)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_tui_color_status_muted_light",
        expression: "#455A64 sRGB; TUI-6b **light** left status",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.e TUI-6b; UMST_TUI_THEME; COCKPIT_DESIGN_BRIEF Theme+keybindings)",
        env_override: None,
    },
    // §14bis.f H-9 — HAL WorkloadKind::Smoke + badge (Tier-3 **Definitions**; CDD §0.11)
    ConstantEntry {
        name: "hal_badge_segment_max_chars",
        expression: "64 (TUI [hw=] width cap; H-9)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; §14bis.f-H-9; egoff::hal::badge::render_hal_badge)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_intel_cpu_smoke_buf_size_bytes",
        expression: "1024 (WorkloadKind::Smoke host buffer; H-9)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (H-9; `IntelCpu` allocate / smoke)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_intel_cpu_smoke_iterations",
        expression: "1 (T3; one smoke round-trip per H-9 slice)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (H-9)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_permission_probe_timeout_ms",
        expression: "50 (H-9 probe policy window; not hard wall-clock in H-9 — reserved)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (H-9; future polkit/udev timing; placeholder)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_supported_precisions_intel_cpu_count",
        expression: "4 (f32, f64, i32, i64 — H-9 `ComputePrecision` surface)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (H-9; `IntelCpu::supported_precisions`)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_supported_precisions_intel_igpu_count",
        expression: "3 (f32, f16, i32; H-9 i915 lane)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (H-9; `IntelIgpu`)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_supported_precisions_intel_npu_count",
        expression: "2 (f16, int8; H-9 NPU lane)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (H-9; `IntelNpu`)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_workload_smoke_byte_size",
        expression: "1024 (must match SMOKE buffer; REGISTRY mirror; H-9)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (H-9; B-2 extends WorkloadKind)",
        env_override: None,
    },
    // §14bis.f H-8 — HAL trait surface (Tier-3 **Definitions**; CDD §0.11; FORWARD-PLAN v1.2 §3.1)
    ConstantEntry {
        name: "hal_trait_method_count",
        expression: "7 (count of `HardwareUnit` trait methods: enumerate_models, supported_precisions, allocate, infer, deallocate, power_state, drift_window; §14bis.f-H-8)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; FORWARD-PLAN v1.2 Q5/G5; H-8 trait surface; egoff/egoffimprov §24a; umst-math::hal::traits::HardwareUnit)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_unit_presence_variant_count",
        expression: "4 (Present | AbsentByArch | AbsentByConfig | AbsentByFault(Reason); UnitPresence ADT; FORWARD-PLAN §0.1 Q5)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; NED §0.5; umst-math::hal::presence::UnitPresence)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_unit_kind_count",
        expression: "7 (object kinds in category 𝓗: CPU, IGPU, DGPU, NPU, ANE, RAM, PORT; FORWARD-PLAN §0.2)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; umst-math::hal::kinds::UnitKind)",
        env_override: None,
    },
    ConstantEntry {
        name: "hal_canonical_fallback_chain_max_len",
        expression: "8 (B-2.5 `ArchitectureProfile` chain length cap; §14.2; inventory schema placeholder H-8)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (HSAD §0.12; FORWARD-PLAN v1.2 §14.2 B-2.5; H-8 profile.rs)",
        env_override: None,
    },
    // §14bis.f-M-0 — M-Arc `umst-math::manifold` (Tier-3 Definition, MEMORY-ARC-PLAN v1.0; FORWARD-PLAN §0.4)
    ConstantEntry {
        name: "manifold_sphere_dim_default",
        expression: "3 (ambient S^2 in R^3 per M-Arc M-Q1; umst-math::manifold::S2 / Sn)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (MEMORY-ARC-PLAN §0; M-0 sphere.rs; CDD)",
        env_override: None,
    },
    ConstantEntry {
        name: "manifold_hilbert_bits_default",
        expression: "12 (default Hilbert 2D order; `UMST_MEMORY_HILBERT_BITS` mirror in §24m; M-0 tests use 4 for ε-bisim speed)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (MEMORY-ARC-PLAN §6; umst-math::manifold::hilbert)",
        env_override: None,
    },
    ConstantEntry {
        name: "manifold_resolution_floor",
        expression: "8 (minimum bits per axis for refuse-to-degrade; MEMORY-ARC-PLAN §6)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (M-Arc; umst-math::manifold::ResolutionLevel; REGISTRY M-0)",
        env_override: None,
    },
    ConstantEntry {
        name: "manifold_resolution_ceiling",
        expression: "12 (max bits per axis; host storage policy)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (MEMORY-ARC-PLAN §6; CDD M-0)",
        env_override: None,
    },
    ConstantEntry {
        name: "manifold_octree_max_depth",
        expression: "16 (octree `OctreeNode.depth` cap; I4; tests use smaller chains)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (M-0 octree.rs; CDD)",
        env_override: None,
    },
    ConstantEntry {
        name: "manifold_csg_smooth_k_default",
        expression: "0.05 (Quilez smoothMin blend; SDFGate.hs; umst-math::manifold::csg::default_smooth_k)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (Haskell SDFGate.smoothUnionSDF; M-0 csg)",
        env_override: None,
    },
    ConstantEntry {
        name: "manifold_canonicalize_eps",
        expression: "1e-9 (affine + float residual; ε-bisim; `MANIFOLD_CANONICALIZE_EPS`)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (GMD-2; I3; umst-math::manifold::MANIFOLD_CANONICALIZE_EPS)",
        env_override: None,
    },
    ConstantEntry {
        name: "manifold_hilbert_locality_constant",
        expression: "6 (C bound for 2D Hilbert locality witness; L-M0 theorem target; M-0 empirical in tests C≤6)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (MEMORY-ARC-PLAN §3.2 L-M0; empirical in `manifold` Hilbert-locality test module)",
        env_override: None,
    },
    // Tier-2 B-Arc / telemetry (placeholders; same debt pattern as other Tier-2)
    ConstantEntry {
        name: "manifold_voxelize_runtime_us_p99",
        expression: "pending: B-Arc p99 of canonicalize_voxelize wall time (us)",
        tier: ConstantTier::Tier2Derivable,
        evidence: "pending: Phase FPD-M-Arc-VoxelP99 (M-B calibration)",
        env_override: None,
    },
    ConstantEntry {
        name: "manifold_canonicalize_runtime_us_p99",
        expression: "pending: B-Arc p99 of canonicalize + FNV (us)",
        tier: ConstantTier::Tier2Derivable,
        evidence: "pending: Phase FPD-M-Arc-CanonicalizeP99",
        env_override: None,
    },
    ConstantEntry {
        name: "manifold_octree_density_typical",
        expression: "pending: B-Arc typical non-empty leaf count / m³ for cockpit badge",
        tier: ConstantTier::Tier2Derivable,
        evidence: "pending: Phase FPD-M-Arc-OctreeDensity",
        env_override: None,
    },
    ConstantEntry {
        name: "manifold_hilbert_index_range_typical",
        expression: "pending: B-Arc index span on reference traces for sled key layout (M-5)",
        tier: ConstantTier::Tier2Derivable,
        evidence: "pending: Phase FPD-M-Arc-HilbertSpan",
        env_override: None,
    },
    // §14bis.f-M-1 — `egoff::memory` (sled schema v1; B-Arc placeholders; Tier-3 for schema + default res)
    ConstantEntry {
        name: "umst_memory_default_resolution_bits",
        expression: "12 (B-Arc; M-1 clamps to umst `canonicalize_voxelize` 1..=10; recorded `ResolutionLevel.bits` may be 12)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (M-1 MEMORY-ARC; GMD-3; `umst-math::manifold` resolution ceiling 12 policy vs 10-bit voxels M-0)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_inspect_runtime_us_p99",
        expression: "pending: B-Arc p99 of `:memory inspect` wall time (us)",
        tier: ConstantTier::Tier2Derivable,
        evidence: "pending: Phase FPD-M-Arc-M1-InspectP99",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_load_runtime_us_p99",
        expression: "pending: B-Arc p99 of memory `load` (us)",
        tier: ConstantTier::Tier2Derivable,
        evidence: "pending: Phase FPD-M-Arc-M1-LoadP99",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_local_tier_size_typical",
        expression: "pending: B-Arc typical local-tier row count for cockpit (count)",
        tier: ConstantTier::Tier2Derivable,
        evidence: "pending: Phase FPD-M-Arc-M1-LocalSize",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_schema_version",
        expression: "1 (bincode v1; see `egoff::memory::schema`)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (M-1 sled `MemoryV1` wire; migration path: bump + multi-decode in M-2+)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_store_runtime_us_p99",
        expression: "pending: B-Arc p99 of memory `store` (us)",
        tier: ConstantTier::Tier2Derivable,
        evidence: "pending: Phase FPD-M-Arc-M1-StoreP99",
        env_override: None,
    },
    // §14bis.f-M-2 — promotion ceremony + sanitize (GMD-4..6)
    ConstantEntry {
        name: "umst_memory_m2_promote_ceremony_atomic",
        expression: "1 (fail-fast 8-step Local→Shared promotion; operator `:promote` + registry + serial-scan + attestation)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-2; THEOREM-BOUND ceremony; `egoff::memory::promote`)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_m2_sanitize_serial_kinds_count",
        expression: "5 (MAC ascii, cpuinfo serial, GPU UUID v4 ascii, IOPlatformSerialNumber, kernel leaf)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-2 GMD-6; `egoff::memory::sanitize::SerialKind`)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_m2_promotion_requires_theorem_default",
        expression: "1 (default `UMST_MEMORY_PROMOTION_REQUIRE_THEOREM=1`; Z-cert branch deferred)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-2; CONSTANT-BOUND default; `egoff::memory::promotion_require_theorem_enabled`)",
        env_override: Some("UMST_MEMORY_PROMOTION_REQUIRE_THEOREM"),
    },
    ConstantEntry {
        name: "umst_memory_m2_serial_scrub_placeholder_len",
        expression: "17 (`<EGOFF-SCRUBBED>` byte length; preview scrub only)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-2; `egoff::memory::sanitize::SCRUB_PLACEHOLDER`)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_ephemeral_ttl_hours_typical",
        expression: "168 (default TTL hours for `EphemeralRetention::from_registry_default`; overridden by `UMST_MEMORY_EPHEMERAL_TTL_HOURS`)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-3 ephemeral retention witness; MEMORY-ARC)",
        env_override: Some("UMST_MEMORY_EPHEMERAL_TTL_HOURS"),
    },
    ConstantEntry {
        name: "umst_memory_m3_palette_federated_inspect_min_rows",
        expression:
            "0 (offline GREEN stub may yield zero federation rows for `:fed inspect` single-instance palettes)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-3 federation inspector dispatch; no libp2p peers in this slice)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_merge_safe_attestation_wire_version",
        expression: "1 (bincode discriminator for persisted merge-safe federation witness structs)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-3 GMD-8; `MergeSafeAttestation` bincode shim)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_schema_version_v2",
        expression:
            "2 (`MemoryV2` sled wire discriminator; GREEN promotion persists `bincode`(v2) by default; decode accepts v1 + v2)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-3 rename-fed; MEMORY-ARC schema migration posture)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_tier_repr_byte_device",
        expression:
            "`0` (`repr(u8)`; preserves legacy v1 wire byte for Rename-fed Device tier preimage)",
        tier: ConstantTier::Tier3Policy,
        evidence:
            "Definition (§14bis.f-M-3 MemoryTier ABI; MEMORY-ARC local→device rename-fed witness)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_tier_repr_byte_ephemeral",
        expression: "`2` (`repr(u8)`; ephemeral tier preimage byte for sandboxed graduation targets)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-3 MemoryTier ABI; MEMORY-ARC §10(h) graduation)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_tier_repr_byte_federated",
        expression:
            "`1` (`repr(u8)`; preserves legacy v1 preimage byte Shared→rename-fed Federated tier)",
        tier: ConstantTier::Tier3Policy,
        evidence:
            "Definition (§14bis.f-M-3 MemoryTier ABI; MEMORY-ARC promotion federation merge witnesses)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_retention_alpha_default",
        expression:
            "`0.60` (default α in `retain = α·MI + β·pareto`; β = 1 − α)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-3-retention; `memory::env::retention_alpha_or_default`)",
        env_override: Some("UMST_MEMORY_RETENTION_ALPHA"),
    },
    ConstantEntry {
        name: "umst_memory_retention_evict_default",
        expression: "`0` (opt-in; `1` enables post-store eviction + `:memory budget` would_evict)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-3-retention; `UMST_MEMORY_RETENTION_EVICT`)",
        env_override: Some("UMST_MEMORY_RETENTION_EVICT"),
    },
    ConstantEntry {
        name: "umst_memory_retention_degrade_first_default",
        expression: "`1` (prefer resolution degrade before drop when both apply)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-3-retention; `UMST_MEMORY_RETENTION_DEGRADE_FIRST`)",
        env_override: Some("UMST_MEMORY_RETENTION_DEGRADE_FIRST"),
    },
    ConstantEntry {
        name: "umst_memory_retention_mi_estimate_p99_us",
        expression: "pending: B-Arc p99 wall for `mi_estimate` (µs); GREEN bound < 500",
        tier: ConstantTier::Tier2Derivable,
        evidence: "pending: Phase FPD-M-3-retention-MiP99",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_retention_pareto_compute_p99_us",
        expression: "pending: B-Arc p99 wall for `pareto_dominance` / corpus scan (µs)",
        tier: ConstantTier::Tier2Derivable,
        evidence: "pending: Phase FPD-M-3-retention-ParetoP99",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_manifold_liquid_ppo_witness_default",
        expression: "0 (witness off; UMST_MANIFOLD_LIQUID_PPO_WITNESS unset → no Path B step_and_learn on accept)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (MANIFOLD-INTEGRATION-ADR; §14bis.f-H-3b; `ppo_witness_enabled` truthy_env only)",
        env_override: Some("UMST_MANIFOLD_LIQUID_PPO_WITNESS"),
    },
    ConstantEntry {
        name: "umst_manifold_ppo_info_gain_default_bits",
        expression: "0.01 (default MI tensor scale when UMST_MANIFOLD_GATEWAY_INFO_GAIN_BITS unset)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-I-4; `ppo_info_gain_bits`; proposal-length fallback when unset)",
        env_override: Some("UMST_MANIFOLD_GATEWAY_INFO_GAIN_BITS"),
    },
    ConstantEntry {
        name: "umst_manifold_emergence_lambda",
        expression: "0.1 (EmergenceMonitor λ; `UMST_MANIFOLD_EMERGENCE_LAMBDA` when unset)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-I-5 / §14bis.f-M-SDF-emergence; `emergence_lambda`)",
        env_override: Some("UMST_MANIFOLD_EMERGENCE_LAMBDA"),
    },
    ConstantEntry {
        name: "umst_msdf_emergence_max_voxels",
        expression: "512 (default 3³ lattice; cap enforced in `sdf_grid_for_emergence_sdf`)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-SDF-emergence; `max_emergence_voxels`)",
        env_override: Some("UMST_MSDF_EMERGENCE_MAX_VOXELS"),
    },
    ConstantEntry {
        name: "umst_ucrs_memory_phase_bind_enabled",
        expression: "0 (default off; `UMST_UCRS_MEMORY_PHASE_BIND=1` enables accept-path bind)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.x-M-UCRS-SDF-TIME; `ucrs_memory_bind_enabled`; umst_ucrs `phase_entropy_bits`)",
        env_override: Some("UMST_UCRS_MEMORY_PHASE_BIND"),
    },
    ConstantEntry {
        name: "umst_msdf_layer_stack_max_depth",
        expression: "4 (ring cap when `UMST_MSDF_LAYER_STACK=1` + emergence grid on)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.x-M-UCRS-SDF-TIME; `msdf_layer_stack_max_depth`)",
        env_override: Some("UMST_MSDF_LAYER_STACK_MAX_DEPTH"),
    },
    ConstantEntry {
        name: "umst_memory_observed_wall_ms_source",
        expression: "monotonic_clock (Tier-1 wall_ms on accept; `UcrsObservedAt::observed_wall_ms`)",
        tier: ConstantTier::Tier1Measurement,
        evidence: "Definition (§14bis.x-M-UCRS-SDF-TIME; `observed_wall_ms`)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_hilbert_bits",
        expression: "8 (M-0 Hilbert order cap; policy target 12 in MEMORY-ARC M-5; `UMST_MEMORY_HILBERT_BITS`)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-5; `memory_hilbert_bits`; umst_math::manifold::hilbert)",
        env_override: Some("UMST_MEMORY_HILBERT_BITS"),
    },
    ConstantEntry {
        name: "umst_msdf_hilbert_persist_enabled",
        expression: "0 (requires UCRS bind + MSDF grid + layer stack env)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-5; `msdf_hilbert_persist_enabled`)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_memory_cockpit_badge_format",
        expression: "\"[mem=E:N D:M F:K]\" (§14bis.f-M-6; `memory_tier_badge_edf`)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-6; `memory::badge`)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_manifold_introspect_enabled",
        expression: "0 (`UMST_MANIFOLD_INTROSPECT=1` adds verbose :manifold lines only)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-6; `manifold_introspect_verbose_enabled`)",
        env_override: Some("UMST_MANIFOLD_INTROSPECT"),
    },
    ConstantEntry {
        name: "umst_mcert_strict_paired_default",
        expression: "0 (`egoff mcert --paired` / `:mcert --paired` opt-in heavy scripts)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-7; `run_mcert_paired`)",
        env_override: Some("UMST_MCERT_STRICT_PAIRED"),
    },
    ConstantEntry {
        name: "umst_action_shape_canonicalize_kind",
        expression: "blake3 preimage over FNV-8 + voxel f64 block + axis bits (§14bis.f-M-4)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-4; `egoff::credit::action_sdf_canonicalize`)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_action_shape_quotient_default_enabled",
        expression: "1 (merge credits by intrinsic geometry key; `0` disables)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-4; `UMST_ACTION_SHAPE_QUOTIENT`)",
        env_override: Some("UMST_ACTION_SHAPE_QUOTIENT"),
    },
    ConstantEntry {
        name: "umst_action_shape_palette_max_entries_default",
        expression: "100",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-M-4 `:action-shapes`; palette truncation)",
        env_override: Some("UMST_ACTION_SHAPE_PALETTE_MAX_ENTRIES"),
    },
    // §14bis.f-S-0 — PQC primitive byte widths (PQClean / NIST parameter sets; `umst-math::crypto`)
    ConstantEntry {
        name: "crypto_ml_kem_768_public_key_bytes",
        expression: "1184 (`pqcrypto_kyber::kyber768::public_key_bytes`; ML-KEM-768)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-S-0; FIPS 203 ML-KEM-768; `Crypto/KEM.lean` L-S0 stub)",
        env_override: None,
    },
    ConstantEntry {
        name: "crypto_ml_kem_768_secret_key_bytes",
        expression: "2400 (`kyber768::secret_key_bytes`)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-S-0; ML-KEM-768 SK wire)",
        env_override: None,
    },
    ConstantEntry {
        name: "crypto_ml_kem_768_ciphertext_bytes",
        expression: "1088 (`kyber768::ciphertext_bytes`)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-S-0; ML-KEM-768 ciphertext)",
        env_override: None,
    },
    ConstantEntry {
        name: "crypto_ml_dsa_65_public_key_bytes",
        expression: "1952 (`pqcrypto_dilithium::dilithium3::public_key_bytes`; ML-DSA-65 / Dilithium3)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-S-0; FIPS 204 class mapping; `Crypto/Sig.lean` L-S1 stub)",
        env_override: None,
    },
    ConstantEntry {
        name: "crypto_ml_dsa_65_secret_key_bytes",
        expression: "4032 (`dilithium3::secret_key_bytes`)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-S-0; ML-DSA-65 SK wire)",
        env_override: None,
    },
    ConstantEntry {
        name: "crypto_slh_dsa_128s_public_key_bytes",
        expression: "32 (`pqcrypto_sphincsplus::sphincssha2128ssimple`; SLH-DSA SHA2-128s)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-S-0; SPHINCS+ SHA2-128s-simple PK seed size)",
        env_override: None,
    },
    ConstantEntry {
        name: "crypto_sha3_256_digest_bytes",
        expression: "32 (SHA3-256 digest width)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.f-S-0; FIPS 202 Keccak via `sha3` crate; `Crypto/Hash.lean` L-S2 stub)",
        env_override: None,
    },
    ConstantEntry {
        name: "umst_llm_tier_fallback_default_chain_gemini",
        expression: "gemini-3.1-pro-preview,gemini-2.5-pro,gemini-2.0-flash,gemini-1.5-flash (comma-separated model ids)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.l-LHF-5; default Gemini tier fold; override UMST_LLM_TIER_FALLBACK_CHAIN_GEMINI)",
        env_override: Some("UMST_LLM_TIER_FALLBACK_CHAIN_GEMINI"),
    },
    ConstantEntry {
        name: "umst_llm_tier_degradation_event_kind",
        expression: "llm.tier_degraded (tracing target; TierDegradationEvent audit)",
        tier: ConstantTier::Tier3Policy,
        evidence: "Definition (§14bis.l-LHF-5; cockpit-honest tier-degradation witness)",
        env_override: None,
    },
];

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
    v.sort_by(|a, b| a.tier.cmp(&b.tier).then_with(|| a.name.cmp(b.name)));
    v
}

/// Parse the markdown table in `egoffimprov.md` §24a: first column of each data row (after the header row).
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

#[cfg(test)]
mod tests {
    use super::{registry_sorted_by_tier, ConstantTier, REGISTRY};

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
    fn tier2_evidence_is_pending_fpd_or_lean() {
        for e in REGISTRY {
            if e.tier != ConstantTier::Tier2Derivable {
                continue;
            }
            let ev = e.evidence.trim();
            assert!(
                ev.starts_with("pending: Phase FPD-") || ev.starts_with("UMST.Formal"),
                "Tier2 {} evidence must be FPD-pending or Lean-prefixed: {:?}",
                e.name,
                ev
            );
        }
    }

    #[test]
    fn tier0_evidence_lean_prefixed() {
        for e in REGISTRY {
            if e.tier != ConstantTier::Tier0Physical {
                continue;
            }
            assert!(
                e.evidence.trim().starts_with("UMST.Formal"),
                "Tier0 {} evidence must start UMST.Formal: {:?}",
                e.name,
                e.evidence
            );
        }
    }

    #[test]
    fn registry_sorted_by_tier_is_sorted_and_complete() {
        assert_eq!(REGISTRY.len(), 162);
        let sorted = registry_sorted_by_tier();
        assert_eq!(sorted.len(), REGISTRY.len());
        for w in sorted.windows(2) {
            assert!(w[0].tier <= w[1].tier);
            if w[0].tier == w[1].tier {
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

    /// §24a first column (`egoffimprov.md`) must list the same machine ids as [`REGISTRY`] `name`s (set equality).
    #[test]
    fn registry_machine_ids_mirror_egoffimprov_section_24a() {
        use std::collections::HashSet;
        use std::path::PathBuf;

        let md = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../egoff/egoffimprov.md");
        let Ok(raw) = std::fs::read_to_string(&md) else {
            println!(
                "SKIP §24a parity: cannot read {} (non-dev harness)",
                md.display()
            );
            return;
        };
        let parsed = super::parse_24a_first_column_names(&raw)
            .expect("egoffimprov.md must contain ## 24a. and a following ## section header");
        let expected: HashSet<&str> = REGISTRY.iter().map(|e| e.name).collect();
        let got: HashSet<&str> = parsed.iter().map(String::as_str).collect();
        assert_eq!(
            got, expected,
            "§24a table column 1 must match REGISTRY.name (set equality)"
        );
    }
}
