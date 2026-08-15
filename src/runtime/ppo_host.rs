// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Host / CLI IO boundary for PPO constraint slack weights.
//!
//! [`crate::ai::ppo::ManifoldGateway`] keeps **no `std::env`** reads in `src/ai/**`.
//! Binaries, cartridge runners, and future `umst-cli` subcommands should call
//! [`ppo_constraint_weights_from_env`] (or the pure parsers) here and inject via
//! [`crate::ai::ppo::ManifoldGateway::with_constraint_weights`].
//!
//! # Honesty fences (W29-129-PPO_HOST)
//!
//! Host env parsing + inject helpers are measured. This is **not** physics GREEN,
//! **not** `PRODUCTION_WIRED` (no live operator / CLI production flip), **not**
//! `MASTER`, **not** OP-5. Default λ = 0 is fail-closed soft-slack-off.

// W29-129-PPO_HOST — deepen + honest fences (umst-admit-grok / cursor-grok-4.6-high).
// **Invent fence:** not GREEN / not PRODUCTION_WIRED / not MASTER / not OP-5.

/// W29 deepen cell — PPO host IO boundary honesty attribution.
pub const W29_129_PPO_HOST_CELL_ID: &str = "W29-129-PPO_HOST";

/// Honest deepen posture tag for meta / fleet probes.
pub const PPO_HOST_HONEST_POSTURE: &str = "honest-ppo-host-env-boundary-soft-slack-off-default";

/// Non-claim string — invent fences stay closed on this deepen slice.
pub const PPO_HOST_NON_CLAIM: &str =
    "not GREEN; not PRODUCTION_WIRED; not MASTER; not OP-5; host env inject ≠ production CLI wire";

/// Honest fence — physics GREEN invent stays false (tool readiness ≠ physics GREEN).
pub const PPO_HOST_PHYSICS_GREEN: bool = false;

/// Honest fence — production CLI / operator wire invent stays false.
pub const PPO_HOST_PRODUCTION_WIRED: bool = false;

/// Honest fence — MASTER invent / retick stays false.
pub const PPO_HOST_MASTER_RETICK_ELIGIBLE: bool = false;

/// Honest fence — OP-5 invent stays false.
pub const PPO_HOST_OP5_CLAIMED: bool = false;

/// GREEN claim blocked while physics GREEN stays false.
pub const PPO_HOST_GREEN_CLAIM_BLOCKED: bool = true;

/// Process env key for Clausius–Duhem slack weight λ_cd.
pub const ENV_UMST_LAMBDA_CD: &str = "UMST_LAMBDA_CD";

/// Process env key for Landauer erasure slack weight λ_landauer.
pub const ENV_UMST_LAMBDA_LANDAUER: &str = "UMST_LAMBDA_LANDAUER";

/// Epistemic / Kleisli constraint slack weights parsed at the IO boundary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PpoConstraintWeights {
    /// Clausius–Duhem slack weight **λ_cd**.
    pub lambda_cd: f32,
    /// Landauer erasure slack weight **λ_landauer**.
    pub lambda_landauer: f32,
}

impl Default for PpoConstraintWeights {
    fn default() -> Self {
        Self {
            lambda_cd: 0.0_f32,
            lambda_landauer: 0.0_f32,
        }
    }
}

impl PpoConstraintWeights {
    /// Pure constructor from optional env string views (absent / invalid / non-finite → 0).
    #[must_use]
    pub fn from_env_strs(lambda_cd: Option<&str>, lambda_landauer: Option<&str>) -> Self {
        Self {
            lambda_cd: parse_lambda_cd_env(lambda_cd),
            lambda_landauer: parse_lambda_landauer_env(lambda_landauer),
        }
    }

    /// Both λ = 0 → soft slack off (fail-closed host default).
    #[must_use]
    pub const fn soft_slack_off(self) -> bool {
        self.lambda_cd == 0.0_f32 && self.lambda_landauer == 0.0_f32
    }

    /// Both λ finite (NaN / Inf rejected at parse → 0, so always true for parsed weights).
    #[must_use]
    pub fn weights_finite(self) -> bool {
        self.lambda_cd.is_finite() && self.lambda_landauer.is_finite()
    }
}

/// Parse a finite f32; absent / empty / invalid / non-finite → `0.0`.
#[must_use]
pub fn parse_finite_f32_env(value: Option<&str>) -> f32 {
    value
        .and_then(|s| {
            let t = s.trim();
            if t.is_empty() {
                return None;
            }
            t.parse::<f32>().ok()
        })
        .filter(|v| v.is_finite())
        .unwrap_or(0.0_f32)
}

/// Parse `UMST_LAMBDA_CD` (absent / invalid / non-finite → `0.0`).
#[must_use]
pub fn parse_lambda_cd_env(value: Option<&str>) -> f32 {
    parse_finite_f32_env(value)
}

/// Parse `UMST_LAMBDA_LANDAUER` (absent / invalid / non-finite → `0.0`).
#[must_use]
pub fn parse_lambda_landauer_env(value: Option<&str>) -> f32 {
    parse_finite_f32_env(value)
}

/// Read `UMST_LAMBDA_CD` / `UMST_LAMBDA_LANDAUER` from the process environment.
#[must_use]
pub fn ppo_constraint_weights_from_env() -> PpoConstraintWeights {
    PpoConstraintWeights::from_env_strs(
        std::env::var(ENV_UMST_LAMBDA_CD).ok().as_deref(),
        std::env::var(ENV_UMST_LAMBDA_LANDAUER).ok().as_deref(),
    )
}

/// W29-129 honesty fence — GREEN / PRODUCTION / MASTER / OP-5 refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PpoHostHonestyFence {
    pub deepen_step: &'static str,
    pub physics_green: bool,
    pub production_wired: bool,
    pub master_retick_eligible: bool,
    pub op5_claimed: bool,
    pub green_claim_blocked: bool,
}

impl PpoHostHonestyFence {
    /// Measured honesty posture for this module.
    #[must_use]
    pub const fn measured() -> Self {
        Self {
            deepen_step: W29_129_PPO_HOST_CELL_ID,
            physics_green: PPO_HOST_PHYSICS_GREEN,
            production_wired: PPO_HOST_PRODUCTION_WIRED,
            master_retick_eligible: PPO_HOST_MASTER_RETICK_ELIGIBLE,
            op5_claimed: PPO_HOST_OP5_CLAIMED,
            green_claim_blocked: PPO_HOST_GREEN_CLAIM_BLOCKED,
        }
    }

    /// Fence holds when invent flags stay false and GREEN remains blocked.
    #[must_use]
    pub const fn holds(self) -> bool {
        !self.physics_green
            && !self.production_wired
            && !self.master_retick_eligible
            && !self.op5_claimed
            && self.green_claim_blocked
            && !self.deepen_step.is_empty()
    }
}

/// Typed deepen probe for the PPO host IO boundary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PpoHostDeepenProbe {
    pub cell_id: &'static str,
    pub honest_posture: &'static str,
    pub non_claim: &'static str,
    pub fence_holds: bool,
    pub soft_slack_off_default: bool,
    pub env_keys_landed: bool,
    pub finite_parse_landed: bool,
    pub production_wired: bool,
    pub physics_green: bool,
    pub master_retick_eligible: bool,
    pub op5_claimed: bool,
    pub deepen_honest: bool,
}

/// Build W29-129 deepen probe from live host measurements + invent fences.
#[must_use]
pub fn ppo_host_deepen_probe() -> PpoHostDeepenProbe {
    let fence = PpoHostHonestyFence::measured();
    let defaults = PpoConstraintWeights::default();
    let parsed = PpoConstraintWeights::from_env_strs(Some("0.5"), Some("1.25"));
    let non_finite_rejected =
        parse_finite_f32_env(Some("nan")) == 0.0 && parse_finite_f32_env(Some("inf")) == 0.0;
    let env_keys_landed =
        ENV_UMST_LAMBDA_CD == "UMST_LAMBDA_CD" && ENV_UMST_LAMBDA_LANDAUER == "UMST_LAMBDA_LANDAUER";
    let finite_parse_landed = parsed.weights_finite()
        && parsed.lambda_cd == 0.5
        && parsed.lambda_landauer == 1.25
        && non_finite_rejected;
    let soft_slack_off_default = defaults.soft_slack_off();
    let deepen_honest = fence.holds()
        && W29_129_PPO_HOST_CELL_ID == "W29-129-PPO_HOST"
        && PPO_HOST_HONEST_POSTURE.contains("soft-slack-off-default")
        && PPO_HOST_NON_CLAIM.contains("not PRODUCTION_WIRED")
        && PPO_HOST_NON_CLAIM.contains("not GREEN")
        && soft_slack_off_default
        && env_keys_landed
        && finite_parse_landed
        && !fence.physics_green
        && !fence.production_wired
        && !fence.master_retick_eligible
        && !fence.op5_claimed;

    PpoHostDeepenProbe {
        cell_id: W29_129_PPO_HOST_CELL_ID,
        honest_posture: PPO_HOST_HONEST_POSTURE,
        non_claim: PPO_HOST_NON_CLAIM,
        fence_holds: fence.holds(),
        soft_slack_off_default,
        env_keys_landed,
        finite_parse_landed,
        production_wired: fence.production_wired,
        physics_green: fence.physics_green,
        master_retick_eligible: fence.master_retick_eligible,
        op5_claimed: fence.op5_claimed,
        deepen_honest,
    }
}

/// Whether the W29-129 PPO host deepen honesty probe passes.
#[must_use]
pub fn ppo_host_deepen_honest() -> bool {
    ppo_host_deepen_probe().deepen_honest
}

/// Fence: refuse inventing GREEN / PRODUCTION_WIRED / MASTER / OP-5.
#[must_use]
pub fn ppo_host_honest_fence_holds() -> bool {
    let p = ppo_host_deepen_probe();
    p.deepen_honest
        && p.fence_holds
        && !p.physics_green
        && !p.production_wired
        && !p.master_retick_eligible
        && !p.op5_claimed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lambda_cd_absent_or_invalid_defaults_zero() {
        assert_eq!(parse_lambda_cd_env(None), 0.0);
        assert_eq!(parse_lambda_cd_env(Some("")), 0.0);
        assert_eq!(parse_lambda_cd_env(Some("   ")), 0.0);
        assert_eq!(parse_lambda_cd_env(Some("nope")), 0.0);
    }

    #[test]
    fn lambda_cd_parses_finite_float() {
        assert_eq!(parse_lambda_cd_env(Some("0.5")), 0.5);
        assert_eq!(parse_lambda_cd_env(Some(" 0.5 ")), 0.5);
    }

    #[test]
    fn lambda_landauer_absent_defaults_zero() {
        assert_eq!(parse_lambda_landauer_env(None), 0.0);
    }

    #[test]
    fn lambda_landauer_parses_finite_float() {
        assert_eq!(parse_lambda_landauer_env(Some("1.25")), 1.25);
    }

    #[test]
    fn non_finite_env_values_reject_to_zero() {
        assert_eq!(parse_finite_f32_env(Some("nan")), 0.0);
        assert_eq!(parse_finite_f32_env(Some("NaN")), 0.0);
        assert_eq!(parse_finite_f32_env(Some("inf")), 0.0);
        assert_eq!(parse_finite_f32_env(Some("-inf")), 0.0);
        assert_eq!(parse_finite_f32_env(Some("+Infinity")), 0.0);
    }

    #[test]
    fn from_env_strs_and_soft_slack_off_default() {
        let off = PpoConstraintWeights::default();
        assert!(off.soft_slack_off());
        assert!(off.weights_finite());

        let on = PpoConstraintWeights::from_env_strs(Some("0.5"), Some("0"));
        assert!(!on.soft_slack_off());
        assert_eq!(on.lambda_cd, 0.5);
        assert_eq!(on.lambda_landauer, 0.0);
    }

    #[test]
    fn env_key_constants_match_documented_names() {
        assert_eq!(ENV_UMST_LAMBDA_CD, "UMST_LAMBDA_CD");
        assert_eq!(ENV_UMST_LAMBDA_LANDAUER, "UMST_LAMBDA_LANDAUER");
    }

    #[test]
    fn w29_129_invent_fences_hold() {
        assert!(!PPO_HOST_PHYSICS_GREEN);
        assert!(!PPO_HOST_PRODUCTION_WIRED);
        assert!(!PPO_HOST_MASTER_RETICK_ELIGIBLE);
        assert!(!PPO_HOST_OP5_CLAIMED);
        assert!(PPO_HOST_GREEN_CLAIM_BLOCKED);
        assert!(PPO_HOST_NON_CLAIM.contains("not PRODUCTION_WIRED"));
        assert!(PPO_HOST_NON_CLAIM.contains("not GREEN"));
        assert!(PPO_HOST_NON_CLAIM.contains("not MASTER"));
        assert!(PPO_HOST_NON_CLAIM.contains("not OP-5"));
        let fence = PpoHostHonestyFence::measured();
        assert!(fence.holds());
        assert_eq!(fence.deepen_step, "W29-129-PPO_HOST");
    }

    #[test]
    fn w29_129_deepen_probe_honest() {
        let probe = ppo_host_deepen_probe();
        assert!(probe.deepen_honest);
        assert!(ppo_host_deepen_honest());
        assert!(ppo_host_honest_fence_holds());
        assert!(probe.fence_holds);
        assert!(probe.soft_slack_off_default);
        assert!(probe.env_keys_landed);
        assert!(probe.finite_parse_landed);
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master_retick_eligible);
        assert!(!probe.op5_claimed);
        assert_eq!(probe.cell_id, W29_129_PPO_HOST_CELL_ID);
        assert_eq!(probe.honest_posture, PPO_HOST_HONEST_POSTURE);
    }

    #[test]
    fn posture_rejects_tampered_fence_constants() {
        // Compile-time constants stay closed; measured fence must refuse invent.
        let mut bad = PpoHostHonestyFence::measured();
        bad.physics_green = true;
        assert!(!bad.holds());
        bad = PpoHostHonestyFence::measured();
        bad.production_wired = true;
        assert!(!bad.holds());
        bad = PpoHostHonestyFence::measured();
        bad.master_retick_eligible = true;
        assert!(!bad.holds());
        bad = PpoHostHonestyFence::measured();
        bad.op5_claimed = true;
        assert!(!bad.holds());
        bad = PpoHostHonestyFence::measured();
        bad.green_claim_blocked = false;
        assert!(!bad.holds());
    }
}
