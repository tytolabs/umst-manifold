// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Cockpit η_cog → Q1-hex solve budget (pure functor; JSON parsed at vault/cartridge IO only).
//!
//! Low η_cog tightens PCG iteration cap and enables warm-start / op-cache.

use super::adjoint_q1_hex::Q1HexSolveOptions;
use super::time_orchestration::MechanicsInnerLoopConfig;
use serde::Deserialize;

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

/// Minimal egoff [`CockpitSnapshot`] schema v4 fields (IO boundary only).
#[derive(Debug, Deserialize)]
struct EgoffCockpitJson {
    eta_cog: Option<f64>,
    eta_cog_raw: Option<f64>,
    dignity_value: Option<f64>,
    dignity_value_raw: Option<f64>,
    /// Optional throughput hint; egoff v4 has no canonical field — default 0.
    #[serde(default)]
    tokens_per_sec: Option<f64>,
}

/// Map egoff cockpit JSON (schema v4) → pure [`CockpitSnapshot`].
pub fn cockpit_from_egoff_json(json: &str) -> Result<CockpitSnapshot, CockpitParseError> {
    let raw: EgoffCockpitJson =
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
    fn low_eta_cog_reduces_pcg_max_iter() {
        let snap = CockpitSnapshot::new(0.1, 100.0, 1.0);
        let opts = q1hex_opts_from_cockpit(&snap);
        let cap = opts.pcg_max_iter.expect("pcg_max_iter set");
        assert!(
            cap <= DEFAULT_PCG_MAX_ITER,
            "low η_cog cap {cap} should be ≤ default {DEFAULT_PCG_MAX_ITER}"
        );
        assert!(opts.pcg_warm_start);
        assert!(opts.use_operator_cache);
    }

    #[test]
    fn high_eta_cog_aggressive_budget() {
        let snap = CockpitSnapshot::new(0.9, 500.0, 2.0);
        let opts = q1hex_opts_from_cockpit(&snap);
        assert_eq!(opts.pcg_max_iter, Some(HIGH_ETA_PCG_MAX_ITER));
    }

    #[test]
    fn mechanics_config_inherits_cap() {
        let snap = CockpitSnapshot::new(0.05, 200.0, 1.0);
        let base = MechanicsInnerLoopConfig::default();
        let cg = mechanics_config_from_cockpit(&snap, &base);
        assert!(cg.max_cg_iterations <= DEFAULT_PCG_MAX_ITER);
    }

    #[test]
    fn cockpit_from_egoff_json_maps_v4_fields() {
        let json = r#"{
            "schema_version": 4,
            "eta_cog": 0.42,
            "dignity_value": 7.5,
            "tokens_per_sec": 120.0
        }"#;
        let snap = cockpit_from_egoff_json(json).expect("parse");
        assert!((snap.eta_cog - 0.42).abs() < 1e-9);
        assert!((snap.dignity - 7.5).abs() < 1e-9);
        assert!((snap.tokens_per_sec - 120.0).abs() < 1e-9);
    }

    #[test]
    fn cockpit_from_egoff_json_falls_back_to_raw_fields() {
        let json = r#"{"eta_cog_raw": 0.15, "dignity_value_raw": 3.0}"#;
        let snap = cockpit_from_egoff_json(json).expect("parse");
        assert!((snap.eta_cog - 0.15).abs() < 1e-9);
        assert!((snap.dignity - 3.0).abs() < 1e-9);
        assert_eq!(snap.tokens_per_sec, 0.0);
    }
}
