// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Cockpit η_cog → Q1-hex solve budget (pure functor; JSON parsed at vault/cartridge IO only).
//!
//! A bare dimensionless η does not set a PCG iteration count. Warm-start and the operator
//! cache stay on. Joules come only from an explicit allowance.
//!
//! **Track C migration:** [`map_cockpit_solve_budget`] maps explicit joule allowances to
//! [`CockpitEnergyBudget`]. A bare dimensionless η_cog yields [`UnmeasuredBudget`] — joules are
//! not invented from η. [`q1hex_opts_from_cockpit_budget_lane`] never emits a literal
//! `pcg_max_iter` cap (termination is energy-bounded upstream).
//!
//! # Honest boundary (W29-070)
//!
//! Pure η_cog → warm-start / op-cache mapping for Q1-hex. JSON is parsed only
//! at the vault/cartridge IO boundary — no `std::fs` in physics core. Does **not** certify
//! Striatus wall-clock wins, fleet TO wiring, or embodied cockpit loop closure.
//! Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER` / OP-5.

use super::adjoint_q1_hex::Q1HexSolveOptions;
use super::time_orchestration::MechanicsInnerLoopConfig;
use core::num::NonZeroUsize;
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
        && probe
            .honest_fence
            .contains("solve_budget_mapping_landed=true")
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

/// Cockpit budget input at the physics functor boundary (no JSON).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CockpitBudgetInput {
    /// Dimensionless η_cog only — remaining energy is not measured on this path.
    DimensionlessEta { eta_cog: f64 },
    /// Operator- or meter-supplied remaining solve energy (joules) at a known temperature.
    RemainingJoules { joules: f64, temperature_k: f64 },
}

/// Energy budget could not be measured from η alone (honest absence — not an error).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnmeasuredBudget {
    /// The dimensionless η that was offered without a joule meter.
    pub eta_cog: f64,
}

/// Remaining solve energy when the caller supplied joules explicitly.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CockpitEnergyBudget {
    remaining_joules: f64,
    temperature_k: f64,
}

impl CockpitEnergyBudget {
    /// Build a budget from finite, strictly positive joules and temperature (kelvin).
    pub fn try_new(joules: f64, temperature_k: f64) -> Result<Self, CockpitBudgetRefuse> {
        if !joules.is_finite() || joules <= 0.0 {
            return Err(CockpitBudgetRefuse::NonPositiveJoules);
        }
        if !temperature_k.is_finite() || temperature_k <= 0.0 {
            return Err(CockpitBudgetRefuse::NonPositiveTemperature);
        }
        Ok(Self {
            remaining_joules: joules,
            temperature_k,
        })
    }

    /// Joules still available for the solve combinator.
    #[must_use]
    pub fn remaining_joules(self) -> f64 {
        self.remaining_joules
    }

    /// Absolute temperature used with the Landauer floor (kelvin).
    #[must_use]
    pub fn temperature_k(self) -> f64 {
        self.temperature_k
    }
}

/// Why an explicit joule budget was refused (η-only paths use [`UnmeasuredBudget`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CockpitBudgetRefuse {
    NonPositiveJoules,
    NonPositiveTemperature,
    NonFiniteEta,
}

/// Result of mapping [`CockpitBudgetInput`] — measured joules, honest unmeasured η, or refuse.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CockpitSolveBudgetOutcome {
    Measured(CockpitEnergyBudget),
    Unmeasured(UnmeasuredBudget),
    Refused(CockpitBudgetRefuse),
}

/// Map cockpit budget input to a typed energy outcome (no η→joule surrogate).
#[must_use]
pub fn map_cockpit_solve_budget(input: CockpitBudgetInput) -> CockpitSolveBudgetOutcome {
    match input {
        CockpitBudgetInput::DimensionlessEta { eta_cog } => {
            if !eta_cog.is_finite() {
                return CockpitSolveBudgetOutcome::Refused(CockpitBudgetRefuse::NonFiniteEta);
            }
            CockpitSolveBudgetOutcome::Unmeasured(UnmeasuredBudget { eta_cog })
        }
        CockpitBudgetInput::RemainingJoules {
            joules,
            temperature_k,
        } => match CockpitEnergyBudget::try_new(joules, temperature_k) {
            Ok(budget) => CockpitSolveBudgetOutcome::Measured(budget),
            Err(refuse) => CockpitSolveBudgetOutcome::Refused(refuse),
        },
    }
}

/// Dimensionless η from a cockpit snapshot — energy remains [`UnmeasuredBudget`].
#[must_use]
pub fn map_cockpit_solve_budget_from_snapshot(snap: &CockpitSnapshot) -> CockpitSolveBudgetOutcome {
    map_cockpit_solve_budget(CockpitBudgetInput::DimensionlessEta {
        eta_cog: snap.eta_cog,
    })
}

/// Warm-start / op-cache knobs from η without emitting a compiled PCG iteration cap.
#[must_use]
pub fn q1hex_opts_from_cockpit_budget_lane(
    _snap: &CockpitSnapshot,
    _outcome: &CockpitSolveBudgetOutcome,
) -> Q1HexSolveOptions {
    Q1HexSolveOptions {
        pcg_warm_start: true,
        use_operator_cache: true,
        pcg_max_iter: None,
        ..Default::default()
    }
}

/// Overlay migrated cockpit budget lane onto base options (no literal `pcg_max_iter`).
#[must_use]
pub fn apply_cockpit_budget_lane(
    mut base: Q1HexSolveOptions,
    snap: &CockpitSnapshot,
    outcome: &CockpitSolveBudgetOutcome,
) -> Q1HexSolveOptions {
    let cockpit = q1hex_opts_from_cockpit_budget_lane(snap, outcome);
    base.pcg_max_iter = cockpit.pcg_max_iter;
    base.pcg_warm_start = cockpit.pcg_warm_start;
    base.use_operator_cache = cockpit.use_operator_cache;
    base
}

/// Map cockpit efficiency to Q1-hex solve knobs.
///
/// Dimensionless η does not choose an iteration count. [`pcg_max_iter`](Q1HexSolveOptions::pcg_max_iter)
/// stays unset; energy is [`UnmeasuredBudget`] until the caller supplies joules.
#[must_use]
pub fn q1hex_opts_from_cockpit(snap: &CockpitSnapshot) -> Q1HexSolveOptions {
    let outcome = map_cockpit_solve_budget_from_snapshot(snap);
    q1hex_opts_from_cockpit_budget_lane(snap, &outcome)
}

/// Overlay cockpit-derived PCG caps onto env/base options (precond_kind unchanged).
///
/// η-only snapshots carry no iteration cap, so an existing `base.pcg_max_iter` is left in place.
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

/// Mirror cockpit knobs into mechanics inner-loop config.
///
/// An η-only snapshot does not replace `base.max_cg_iterations`.
#[must_use]
pub fn mechanics_config_from_cockpit(
    snap: &CockpitSnapshot,
    base: &MechanicsInnerLoopConfig,
) -> MechanicsInnerLoopConfig {
    let opts = q1hex_opts_from_cockpit(snap);
    let max_it = opts
        .pcg_max_iter
        .map(|cap| NonZeroUsize::new(cap).unwrap_or(NonZeroUsize::MIN))
        .or(base.max_cg_iterations);
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
        assert!(SOLVE_BUDGET_HONEST_FENCE.contains("vault_cockpit_wired=false"));
        assert!(SOLVE_BUDGET_HONEST_FENCE.contains("physics_green=false"));
    }

    #[test]
    fn low_eta_cog_does_not_invent_a_pcg_cap() {
        let snap = CockpitSnapshot::new(0.1, 100.0, 1.0);
        let opts = q1hex_opts_from_cockpit(&snap);
        assert!(opts.pcg_max_iter.is_none());
        assert!(matches!(
            map_cockpit_solve_budget_from_snapshot(&snap),
            CockpitSolveBudgetOutcome::Unmeasured(_)
        ));
        assert!(opts.pcg_warm_start);
        assert!(opts.use_operator_cache);
    }

    #[test]
    fn mid_eta_cog_does_not_invent_a_pcg_cap() {
        let snap = CockpitSnapshot::new(0.5, 200.0, 1.0);
        let opts = q1hex_opts_from_cockpit(&snap);
        assert!(opts.pcg_max_iter.is_none());
        assert!(opts.pcg_warm_start);
        assert!(opts.use_operator_cache);
    }

    #[test]
    fn high_eta_cog_does_not_invent_a_pcg_cap() {
        let snap = CockpitSnapshot::new(0.9, 500.0, 2.0);
        let opts = q1hex_opts_from_cockpit(&snap);
        assert!(opts.pcg_max_iter.is_none());
        assert!(opts.pcg_warm_start);
        assert!(opts.use_operator_cache);
    }

    #[test]
    fn low_throughput_does_not_invent_a_pcg_cap() {
        let snap = CockpitSnapshot::new(0.5, 25.0, 1.0);
        let opts = q1hex_opts_from_cockpit(&snap);
        assert!(opts.pcg_max_iter.is_none());
    }

    #[test]
    fn apply_cockpit_budget_preserves_caller_cap() {
        let base = Q1HexSolveOptions {
            pcg_warm_start: false,
            use_operator_cache: false,
            pcg_max_iter: Some(999),
            precond_kind: None,
            pcg_seed_displacement: None,
        };
        let snap = CockpitSnapshot::new(0.1, 100.0, 1.0);
        let out = apply_cockpit_budget(base, &snap);
        assert_eq!(out.pcg_max_iter, Some(999));
        assert!(out.pcg_warm_start);
        assert!(out.use_operator_cache);
        assert!(out.precond_kind.is_none());
    }

    #[test]
    fn mechanics_config_keeps_caller_iteration_bound() {
        let snap = CockpitSnapshot::new(0.05, 200.0, 1.0);
        let base = MechanicsInnerLoopConfig::default();
        let cg = mechanics_config_from_cockpit(&snap, &base);
        assert_eq!(cg.max_cg_iterations, base.max_cg_iterations);
    }

    #[test]
    fn mechanics_config_keeps_explicit_single_iteration_bound() {
        let snap = CockpitSnapshot::new(0.05, 200.0, 1.0);
        let base = MechanicsInnerLoopConfig {
            max_cg_iterations: Some(NonZeroUsize::MIN),
            ..MechanicsInnerLoopConfig::default()
        };
        let cg = mechanics_config_from_cockpit(&snap, &base);
        assert_eq!(cg.max_cg_iterations, Some(NonZeroUsize::MIN));
        assert_eq!(cg.iteration_budget(40), 1);
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
        let err = cockpit_from_external_json(json)
            .expect_err("missing η_cog / η_cog_raw must yield CockpitParseError::MissingEtaCog");
        assert_eq!(err, CockpitParseError::MissingEtaCog);
    }

    #[test]
    fn cockpit_from_external_json_rejects_non_finite_eta() {
        let json = r#"{"eta_cog": null}"#;
        let err = cockpit_from_external_json(json).expect_err("null η_cog must fail MissingEtaCog");
        assert_eq!(err, CockpitParseError::MissingEtaCog);

        let json_nan = r#"{"eta_cog": "nan"}"#;
        assert!(matches!(
            cockpit_from_external_json(json_nan),
            Err(CockpitParseError::Json(_))
        ));
    }

    #[test]
    fn bare_eta_refuses_invented_joules_and_emits_no_pcg_cap() {
        let outcome =
            map_cockpit_solve_budget(CockpitBudgetInput::DimensionlessEta { eta_cog: 0.42 });
        match outcome {
            CockpitSolveBudgetOutcome::Unmeasured(u) => {
                assert!((u.eta_cog - 0.42).abs() < 1e-12);
            }
            other => panic!("bare η must not invent joules: {other:?}"),
        }

        let snap = CockpitSnapshot::new(0.42, 120.0, 1.0);
        assert!(matches!(
            map_cockpit_solve_budget_from_snapshot(&snap),
            CockpitSolveBudgetOutcome::Unmeasured(_)
        ));

        let opts = q1hex_opts_from_cockpit_budget_lane(&snap, &outcome);
        assert!(opts.pcg_max_iter.is_none());
        let overlaid = apply_cockpit_budget_lane(
            Q1HexSolveOptions {
                pcg_max_iter: Some(999),
                ..Default::default()
            },
            &snap,
            &outcome,
        );
        assert!(overlaid.pcg_max_iter.is_none());
    }

    #[test]
    fn explicit_positive_joule_budget_is_measured() {
        let outcome = map_cockpit_solve_budget(CockpitBudgetInput::RemainingJoules {
            joules: 3.5e-9,
            temperature_k: 293.15,
        });
        let budget = match outcome {
            CockpitSolveBudgetOutcome::Measured(b) => b,
            other => panic!("positive joules must map to Measured: {other:?}"),
        };
        assert!((budget.remaining_joules() - 3.5e-9).abs() < 1e-18);
        assert!((budget.temperature_k() - 293.15).abs() < 1e-9);

        let snap = CockpitSnapshot::new(0.9, 500.0, 2.0);
        let opts = q1hex_opts_from_cockpit_budget_lane(&snap, &outcome);
        assert!(opts.pcg_max_iter.is_none());
    }

    #[test]
    fn non_positive_joules_refused_not_unmeasured() {
        let outcome = map_cockpit_solve_budget(CockpitBudgetInput::RemainingJoules {
            joules: 0.0,
            temperature_k: 293.15,
        });
        assert_eq!(
            outcome,
            CockpitSolveBudgetOutcome::Refused(CockpitBudgetRefuse::NonPositiveJoules)
        );
    }
}
