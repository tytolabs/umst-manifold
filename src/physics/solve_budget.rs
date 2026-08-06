// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Cockpit η_cog → Q1-hex solve budget (pure functor; JSON parsed at vault/cartridge IO only).
//!
//! Low η_cog tightens PCG iteration cap and enables warm-start / op-cache.
//!
//! # Honest boundary (W29-070)
//!
//! Pure η_cog → PCG-cap / warm-start / op-cache mapping for Q1-hex. JSON is parsed only
//! at the vault/cartridge IO boundary — no `std::fs` in physics core. Does **not** certify
//! Striatus wall-clock wins, fleet TO wiring, or embodied cockpit loop closure.
//! Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER` / OP-5.

use super::adjoint_q1_hex::Q1HexSolveOptions;
use super::time_orchestration::MechanicsInnerLoopConfig;
use serde::Deserialize;

/// W29 deepen cell — cockpit solve-budget honest fence bundle.
pub const W29_SOLVE_BUDGET_DEEPEN_CELL: &str = "W29-070-SOLVE_BUDGET";

/// Honest posture tag — η_cog→PCG budget functor landed; fleet production wiring refused.
pub const SOLVE_BUDGET_POSTURE_TAG: &str = "honest-cockpit-solve-budget-research-lane";

/// Honest physics posture — unit mapping contracts pass; does not certify fleet physics GREEN.
pub const SOLVE_BUDGET_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by the pure budget functor alone (vault cockpit wire deferred).
pub const SOLVE_BUDGET_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const SOLVE_BUDGET_MASTER: bool = false;

/// Whether η_cog → [`Q1HexSolveOptions`] mapping contracts are landed in this module.
pub const SOLVE_BUDGET_MAPPING_LANDED: bool = true;

/// Whether external cockpit JSON → [`CockpitSnapshot`] IO-boundary parse is landed.
pub const SOLVE_BUDGET_JSON_IO_LANDED: bool = true;

/// Whether vault/cartridge embodied cockpit loop is closed (honestly open — deferred).
pub const SOLVE_BUDGET_VAULT_COCKPIT_WIRED: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const SOLVE_BUDGET_HONEST_FENCE: &str =
    "solve_budget_mapping_landed=true json_io_boundary_landed=true mechanics_mirror_landed=true vault_cockpit_wired=false striatus_wallclock_certified=false production_wired=false master_composition_wired=false physics_green=false";

const _: () = assert!(!SOLVE_BUDGET_PHYSICS_GREEN);
const _: () = assert!(!SOLVE_BUDGET_PRODUCTION_WIRED);
const _: () = assert!(!SOLVE_BUDGET_MASTER);
const _: () = assert!(!SOLVE_BUDGET_VAULT_COCKPIT_WIRED);
const _: () = assert!(SOLVE_BUDGET_MAPPING_LANDED);
const _: () = assert!(SOLVE_BUDGET_JSON_IO_LANDED);

/// Typed probe for cockpit solve-budget posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolveBudgetPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub mapping_landed: bool,
    pub json_io_landed: bool,
    pub vault_cockpit_wired: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for the cockpit solve-budget functor.
#[must_use]
pub fn solve_budget_honest_posture_bundle() -> SolveBudgetPostureProbe {
    SolveBudgetPostureProbe {
        physics_green: SOLVE_BUDGET_PHYSICS_GREEN,
        production_wired: SOLVE_BUDGET_PRODUCTION_WIRED,
        master: SOLVE_BUDGET_MASTER,
        mapping_landed: SOLVE_BUDGET_MAPPING_LANDED,
        json_io_landed: SOLVE_BUDGET_JSON_IO_LANDED,
        vault_cockpit_wired: SOLVE_BUDGET_VAULT_COCKPIT_WIRED,
        honest_fence: SOLVE_BUDGET_HONEST_FENCE,
        posture_tag: SOLVE_BUDGET_POSTURE_TAG,
        deepen_cell: W29_SOLVE_BUDGET_DEEPEN_CELL,
    }
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER claims on the budget surface.
#[must_use]
pub fn solve_budget_posture_honest(probe: &SolveBudgetPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.vault_cockpit_wired
        && probe.mapping_landed
        && probe.json_io_landed
        && probe.deepen_cell == W29_SOLVE_BUDGET_DEEPEN_CELL
        && probe.honest_fence.contains("solve_budget_mapping_landed=true")
        && probe.honest_fence.contains("vault_cockpit_wired=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// Compile-time / runtime refuse path for invented GREEN / production pins.
pub fn solve_budget_refuse_invented_pins() -> Result<(), &'static str> {
    if SOLVE_BUDGET_PHYSICS_GREEN {
        return Err("SOLVE_BUDGET_PHYSICS_GREEN must stay false — budget functor ≠ fleet physics");
    }
    if SOLVE_BUDGET_PRODUCTION_WIRED {
        return Err(
            "SOLVE_BUDGET_PRODUCTION_WIRED must stay false until vault cockpit loop closes",
        );
    }
    if SOLVE_BUDGET_MASTER {
        return Err("SOLVE_BUDGET_MASTER must stay false — not an OP-5 composition pin");
    }
    if SOLVE_BUDGET_VAULT_COCKPIT_WIRED {
        return Err("SOLVE_BUDGET_VAULT_COCKPIT_WIRED must stay false — embodied wire deferred");
    }
    Ok(())
}

/// Default PCG iteration cap from [`MechanicsInnerLoopConfig::default`].
pub const DEFAULT_PCG_MAX_ITER: usize = 200;

/// Reduced cap when cockpit efficiency is low.
pub const LOW_ETA_PCG_MAX_ITER: usize = 80;

/// Aggressive cap when η_cog is high (vault-scale budget).
pub const HIGH_ETA_PCG_MAX_ITER: usize = 800;

/// η_cog threshold below which the budget tightens.
pub const ETA_COG_LOW_THRESHOLD: f64 = 0.25;

/// η_cog threshold above which the budget is aggressive.
pub const ETA_COG_HIGH_THRESHOLD: f64 = 0.75;

/// Snapshot of cockpit telemetry at the IO boundary (precomputed η_cog).
#[derive(Clone, Debug)]
pub struct CockpitSnapshot {
    pub eta_cog: f64,
    pub tokens_per_sec: f64,
    pub dignity: f64,
}

impl CockpitSnapshot {
    #[must_use]
    pub fn new(eta_cog: f64, tokens_per_sec: f64, dignity: f64) -> Self {
        Self {
            eta_cog,
            tokens_per_sec,
            dignity,
        }
    }
}

/// Parse error at the cockpit IO boundary (no `std::fs` in physics core).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CockpitParseError {
    Json(String),
    MissingEtaCog,
}

/// Minimal external cockpit-telemetry [`CockpitSnapshot`] schema v4 fields (IO boundary only).
#[derive(Debug, Deserialize)]
struct RawCockpitJson {
    eta_cog: Option<f64>,
    eta_cog_raw: Option<f64>,
    dignity_value: Option<f64>,
    dignity_value_raw: Option<f64>,
    /// Optional throughput hint; schema v4 has no canonical field — default 0.
    #[serde(default)]
    tokens_per_sec: Option<f64>,
}

/// Map external cockpit-telemetry JSON (schema v4) → pure [`CockpitSnapshot`].
pub fn cockpit_from_external_json(json: &str) -> Result<CockpitSnapshot, CockpitParseError> {
    let raw: RawCockpitJson =
        serde_json::from_str(json).map_err(|e| CockpitParseError::Json(e.to_string()))?;
    let eta_cog = raw
        .eta_cog
        .or(raw.eta_cog_raw)
        .filter(|v| v.is_finite())
        .ok_or(CockpitParseError::MissingEtaCog)?;
    let dignity = raw
        .dignity_value
        .or(raw.dignity_value_raw)
        .filter(|v| v.is_finite())
        .unwrap_or(0.0);
    let tokens_per_sec = raw.tokens_per_sec.filter(|v| v.is_finite()).unwrap_or(0.0);
    Ok(CockpitSnapshot::new(eta_cog, tokens_per_sec, dignity))
}

/// Map cockpit efficiency to Q1-hex solve knobs.
#[must_use]
pub fn q1hex_opts_from_cockpit(snap: &CockpitSnapshot) -> Q1HexSolveOptions {
    let mut opts = Q1HexSolveOptions {
        pcg_warm_start: true,
        use_operator_cache: true,
        ..Default::default()
    };

    if snap.eta_cog < ETA_COG_LOW_THRESHOLD {
        opts.pcg_max_iter = Some(LOW_ETA_PCG_MAX_ITER);
    } else if snap.eta_cog >= ETA_COG_HIGH_THRESHOLD {
        opts.pcg_max_iter = Some(HIGH_ETA_PCG_MAX_ITER);
        opts.pcg_warm_start = true;
        opts.use_operator_cache = true;
    } else {
        opts.pcg_max_iter = Some(DEFAULT_PCG_MAX_ITER);
    }

    // Secondary throttle: very low token throughput also tightens cap.
    if snap.tokens_per_sec > 0.0 && snap.tokens_per_sec < 50.0 {
        let cap = opts.pcg_max_iter.unwrap_or(DEFAULT_PCG_MAX_ITER);
        opts.pcg_max_iter = Some(cap.min(LOW_ETA_PCG_MAX_ITER));
    }

    opts
}

/// Overlay cockpit-derived PCG caps onto env/base options (precond_kind unchanged).
#[must_use]
pub fn apply_cockpit_budget(
    mut base: Q1HexSolveOptions,
    snap: &CockpitSnapshot,
) -> Q1HexSolveOptions {
    let cockpit = q1hex_opts_from_cockpit(snap);
    if let Some(cap) = cockpit.pcg_max_iter {
        base.pcg_max_iter = Some(cap);
    }
    base.pcg_warm_start = cockpit.pcg_warm_start;
    base.use_operator_cache = cockpit.use_operator_cache;
    base
}

/// Mirror PCG cap into mechanics inner-loop config (vault / cartridge harness).
#[must_use]
pub fn mechanics_config_from_cockpit(
    snap: &CockpitSnapshot,
    base: &MechanicsInnerLoopConfig,
) -> MechanicsInnerLoopConfig {
    let opts = q1hex_opts_from_cockpit(snap);
    let max_it = opts.pcg_max_iter.unwrap_or(base.max_cg_iterations).max(1);
    MechanicsInnerLoopConfig {
        max_cg_iterations: max_it,
        ..base.clone()
    }
}

#[cfg(feature = "math-constants")]
/// Compute η_cog from dignity + claim at the cockpit boundary (delegates to `umst-math`).
#[must_use]
pub fn cockpit_eta_from_claim(dignity: f64, claim: &umst_math::eta_cog::EtaCogClaim) -> f64 {
    umst_math::eta_cog::eta_cog(dignity, claim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_budget_honest_posture_refuses_green_and_production() {
        let probe = solve_budget_honest_posture_bundle();
        assert!(solve_budget_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.vault_cockpit_wired);
        assert!(probe.mapping_landed);
        assert!(probe.json_io_landed);
        assert_eq!(probe.deepen_cell, W29_SOLVE_BUDGET_DEEPEN_CELL);
        assert!(solve_budget_refuse_invented_pins().is_ok());
        assert!(!SOLVE_BUDGET_PHYSICS_GREEN);
        assert!(!SOLVE_BUDGET_PRODUCTION_WIRED);
        assert!(!SOLVE_BUDGET_MASTER);
        assert!(!SOLVE_BUDGET_VAULT_COCKPIT_WIRED);
        assert!(SOLVE_BUDGET_HONEST_FENCE.contains("vault_cockpit_wired=false"));
        assert!(SOLVE_BUDGET_HONEST_FENCE.contains("physics_green=false"));
    }

    #[test]
    fn low_eta_cog_reduces_pcg_max_iter() {
        let snap = CockpitSnapshot::new(0.1, 100.0, 1.0);
        let opts = q1hex_opts_from_cockpit(&snap);
        let cap = opts.pcg_max_iter.expect(
            "q1hex_opts_from_cockpit on low η_cog snapshot must set pcg_max_iter cap (FP §6 Track G solve budget)",
        );
        assert_eq!(cap, LOW_ETA_PCG_MAX_ITER);
        assert!(
            cap <= DEFAULT_PCG_MAX_ITER,
            "low η_cog cap {cap} should be ≤ default {DEFAULT_PCG_MAX_ITER}"
        );
        assert!(opts.pcg_warm_start);
        assert!(opts.use_operator_cache);
    }

    #[test]
    fn mid_eta_cog_uses_default_pcg_cap() {
        let snap = CockpitSnapshot::new(0.5, 200.0, 1.0);
        let opts = q1hex_opts_from_cockpit(&snap);
        assert_eq!(opts.pcg_max_iter, Some(DEFAULT_PCG_MAX_ITER));
        assert!(opts.pcg_warm_start);
        assert!(opts.use_operator_cache);
    }

    #[test]
    fn high_eta_cog_aggressive_budget() {
        let snap = CockpitSnapshot::new(0.9, 500.0, 2.0);
        let opts = q1hex_opts_from_cockpit(&snap);
        assert_eq!(opts.pcg_max_iter, Some(HIGH_ETA_PCG_MAX_ITER));
        assert!(opts.pcg_warm_start);
        assert!(opts.use_operator_cache);
    }

    #[test]
    fn low_throughput_tightens_cap() {
        // Mid η_cog would be DEFAULT, but tokens_per_sec < 50 clamps to LOW.
        let snap = CockpitSnapshot::new(0.5, 25.0, 1.0);
        let opts = q1hex_opts_from_cockpit(&snap);
        assert_eq!(opts.pcg_max_iter, Some(LOW_ETA_PCG_MAX_ITER));
    }

    #[test]
    fn apply_cockpit_budget_overlays_cap_preserves_precond() {
        let base = Q1HexSolveOptions {
            pcg_warm_start: false,
            use_operator_cache: false,
            pcg_max_iter: Some(999),
            precond_kind: None,
            pcg_seed_displacement: None,
        };
        let snap = CockpitSnapshot::new(0.1, 100.0, 1.0);
        let out = apply_cockpit_budget(base, &snap);
        assert_eq!(out.pcg_max_iter, Some(LOW_ETA_PCG_MAX_ITER));
        assert!(out.pcg_warm_start);
        assert!(out.use_operator_cache);
        assert!(out.precond_kind.is_none());
    }

    #[test]
    fn mechanics_config_inherits_cap() {
        let snap = CockpitSnapshot::new(0.05, 200.0, 1.0);
        let base = MechanicsInnerLoopConfig::default();
        let cg = mechanics_config_from_cockpit(&snap, &base);
        assert_eq!(cg.max_cg_iterations, LOW_ETA_PCG_MAX_ITER);
        assert!(cg.max_cg_iterations <= DEFAULT_PCG_MAX_ITER);
    }

    #[test]
    fn cockpit_from_external_json_maps_v4_fields() {
        let json = r#"{
            "schema_version": 4,
            "eta_cog": 0.42,
            "dignity_value": 7.5,
            "tokens_per_sec": 120.0
        }"#;
        let snap = cockpit_from_external_json(json).expect(
            "cockpit_from_external_json on v4 schema fields (η_cog, dignity, tokens_per_sec) (FP §6 Track G solve budget)",
        );
        assert!((snap.eta_cog - 0.42).abs() < 1e-9);
        assert!((snap.dignity - 7.5).abs() < 1e-9);
        assert!((snap.tokens_per_sec - 120.0).abs() < 1e-9);
    }

    #[test]
    fn cockpit_from_external_json_falls_back_to_raw_fields() {
        let json = r#"{"eta_cog_raw": 0.15, "dignity_value_raw": 3.0}"#;
        let snap = cockpit_from_external_json(json).expect(
            "cockpit_from_external_json raw-field fallback (η_cog_raw, dignity_value_raw) (FP §6 Track G solve budget)",
        );
        assert!((snap.eta_cog - 0.15).abs() < 1e-9);
        assert!((snap.dignity - 3.0).abs() < 1e-9);
        assert_eq!(snap.tokens_per_sec, 0.0);
    }

    #[test]
    fn cockpit_from_external_json_missing_eta_cog_errors() {
        let json = r#"{"dignity_value": 1.0, "tokens_per_sec": 10.0}"#;
        let err = cockpit_from_external_json(json).expect_err(
            "missing η_cog / η_cog_raw must yield CockpitParseError::MissingEtaCog",
        );
        assert_eq!(err, CockpitParseError::MissingEtaCog);
    }

    #[test]
    fn cockpit_from_external_json_rejects_non_finite_eta() {
        let json = r#"{"eta_cog": null}"#;
        let err = cockpit_from_external_json(json)
            .expect_err("null η_cog must fail MissingEtaCog");
        assert_eq!(err, CockpitParseError::MissingEtaCog);

        let json_nan = r#"{"eta_cog": "nan"}"#;
        assert!(matches!(
            cockpit_from_external_json(json_nan),
            Err(CockpitParseError::Json(_))
        ));
    }
}
