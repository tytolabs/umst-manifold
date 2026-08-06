// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Signed thermodynamic admissibility margin (R2 SSOT).
//!
//! `margin = D_int = −ρ·ψ̇` (signed Clausius–Duhem dissipation surrogate).
//! `violation = relu(−margin)` preserves backward-compatible slack semantics.
//!
//! # Honesty fences
//!
//! Host/cold-edge scalar SSOT only. This module does **not** claim:
//! - swarm/physics `GREEN`
//! - `PRODUCTION_WIRED` / live operator wire
//! - `MASTER` retick eligibility
//! - `OP-5` clearance
//!
//! Non-finite margins are refused as inadmissible so NaN cannot soft-pass via
//! [`f32::max`] (IEEE: `max(NaN, 0) → 0`).

use super::evidence::{admissibility_from_violation, AdmissibilityToken};

/// Hard witness floor matching [`super::evidence::admissibility_from_violation`].
pub const ADMISSIBILITY_MARGIN_EPS: f32 = 1e-4;

/// Honest fence — margin SSOT is not a production wire.
pub const ADMISSIBILITY_MARGIN_PRODUCTION_WIRED: bool = false;
/// Honest fence — GREEN claims stay blocked at this surface.
pub const ADMISSIBILITY_MARGIN_GREEN_CLAIM_BLOCKED: bool = true;
/// Honest fence — MASTER retick not claimed from margin deepen.
pub const ADMISSIBILITY_MARGIN_MASTER_RETICK_ELIGIBLE: bool = false;
/// Honest fence — OP-5 not cleared from margin deepen.
pub const ADMISSIBILITY_MARGIN_OP5_CLEARED: bool = false;

const _: () = assert!(!ADMISSIBILITY_MARGIN_PRODUCTION_WIRED);
const _: () = assert!(ADMISSIBILITY_MARGIN_GREEN_CLAIM_BLOCKED);
const _: () = assert!(!ADMISSIBILITY_MARGIN_MASTER_RETICK_ELIGIBLE);
const _: () = assert!(!ADMISSIBILITY_MARGIN_OP5_CLEARED);

/// Signed thermodynamic headroom. Positive = dissipative / admissible headroom.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AdmissibilityMargin(pub f32);

impl AdmissibilityMargin {
    #[must_use]
    pub fn value(self) -> f32 {
        self.0
    }

    /// Finite Clausius–Duhem scalar (NaN/±∞ refused at hard gate).
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    /// Exterior penalty slack: `relu(−margin)`.
    ///
    /// Non-finite margins yield `+∞` so they never soft-pass the ε floor.
    #[must_use]
    pub fn violation(self) -> f32 {
        if !self.0.is_finite() {
            return f32::INFINITY;
        }
        (-self.0).max(0.0)
    }

    #[must_use]
    pub fn token(self) -> AdmissibilityToken {
        admissibility_from_margin(self)
    }

    /// Hard witness: finite and `margin ≥ −ε`.
    #[must_use]
    pub fn is_hard_admissible(self) -> bool {
        self.0.is_finite() && self.0 >= -ADMISSIBILITY_MARGIN_EPS
    }
}

/// Build margin from signed dissipation `D_int` (host `transition_outcome.dissipation`).
#[must_use]
pub fn admissibility_margin_from_dissipation(d_int: f32) -> AdmissibilityMargin {
    AdmissibilityMargin(d_int)
}

/// Hard token from signed margin (ε = 1e−4). Non-finite → [`AdmissibilityToken::Inadmissible`].
#[must_use]
pub fn admissibility_from_margin(margin: AdmissibilityMargin) -> AdmissibilityToken {
    if !margin.is_finite() {
        return AdmissibilityToken::Inadmissible;
    }
    admissibility_from_violation(margin.violation())
}

/// Measured honesty posture for this write_set (never invents GREEN/PRODUCTION_WIRED).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmissibilityMarginHonestyProbe {
    pub production_wired: bool,
    pub green_claim_blocked: bool,
    pub master_retick_eligible: bool,
    pub op5_cleared: bool,
    pub eps_finite: bool,
    pub deepen_honest: bool,
}

/// Snapshot honesty fences + ε finiteness.
#[must_use]
pub fn admissibility_margin_honesty_probe() -> AdmissibilityMarginHonestyProbe {
    let production_wired = ADMISSIBILITY_MARGIN_PRODUCTION_WIRED;
    let green_claim_blocked = ADMISSIBILITY_MARGIN_GREEN_CLAIM_BLOCKED;
    let master_retick_eligible = ADMISSIBILITY_MARGIN_MASTER_RETICK_ELIGIBLE;
    let op5_cleared = ADMISSIBILITY_MARGIN_OP5_CLEARED;
    let eps_finite = ADMISSIBILITY_MARGIN_EPS.is_finite() && ADMISSIBILITY_MARGIN_EPS > 0.0;
    let deepen_honest = !production_wired
        && green_claim_blocked
        && !master_retick_eligible
        && !op5_cleared
        && eps_finite;
    AdmissibilityMarginHonestyProbe {
        production_wired,
        green_claim_blocked,
        master_retick_eligible,
        op5_cleared,
        eps_finite,
        deepen_honest,
    }
}

/// Fail-closed honesty check for margin deepen.
#[must_use]
pub fn validate_admissibility_margin_honesty() -> Result<(), &'static str> {
    let p = admissibility_margin_honesty_probe();
    if p.production_wired {
        return Err("admissibility_margin production_wired must stay honest false");
    }
    if !p.green_claim_blocked {
        return Err("admissibility_margin GREEN claims must stay blocked");
    }
    if p.master_retick_eligible {
        return Err("admissibility_margin must not claim MASTER retick");
    }
    if p.op5_cleared {
        return Err("admissibility_margin must not claim OP-5 cleared");
    }
    if !p.eps_finite {
        return Err("ADMISSIBILITY_MARGIN_EPS must be finite and positive");
    }
    if !p.deepen_honest {
        return Err("admissibility_margin deepen_honest failed");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn violation_is_relu_neg_margin() {
        for m in [-2.0_f32, -1e-4, 0.0, 1.0, 100.0] {
            let margin = AdmissibilityMargin(m);
            let v = margin.violation();
            let expected = (-m).max(0.0);
            assert!(
                (v - expected).abs() < 1e-7,
                "margin {m}: violation {v} != relu(-m) {expected}"
            );
        }
    }

    #[test]
    fn token_threshold_at_eps() {
        assert_eq!(
            admissibility_from_margin(AdmissibilityMargin(-2e-4)),
            AdmissibilityToken::Inadmissible
        );
        assert_eq!(
            admissibility_from_margin(AdmissibilityMargin(-5e-5)),
            AdmissibilityToken::Admissible
        );
        assert_eq!(
            admissibility_from_margin(AdmissibilityMargin(1.0)),
            AdmissibilityToken::Admissible
        );
    }

    #[test]
    fn hard_admissible_tracks_token_at_eps_boundary() {
        let just_in = AdmissibilityMargin(-ADMISSIBILITY_MARGIN_EPS);
        let just_out = AdmissibilityMargin(-ADMISSIBILITY_MARGIN_EPS - 1e-6);
        assert!(just_in.is_hard_admissible());
        assert_eq!(just_in.token(), AdmissibilityToken::Admissible);
        assert!(!just_out.is_hard_admissible());
        assert_eq!(just_out.token(), AdmissibilityToken::Inadmissible);
    }

    #[test]
    fn from_dissipation_is_identity_wrapper() {
        for d in [-3.0_f32, -1e-4, 0.0, 2.5] {
            let m = admissibility_margin_from_dissipation(d);
            assert_eq!(m.value(), d);
            assert_eq!(m, AdmissibilityMargin(d));
        }
    }

    #[test]
    fn non_finite_margin_refused() {
        for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let m = AdmissibilityMargin(bad);
            assert!(!m.is_finite());
            assert!(!m.is_hard_admissible());
            assert!(m.violation().is_infinite() && m.violation() > 0.0);
            assert_eq!(
                admissibility_from_margin(m),
                AdmissibilityToken::Inadmissible,
                "non-finite {bad} must be Inadmissible"
            );
        }
    }

    #[test]
    fn honesty_fences_block_green_production_master_op5() {
        assert!(!ADMISSIBILITY_MARGIN_PRODUCTION_WIRED);
        assert!(ADMISSIBILITY_MARGIN_GREEN_CLAIM_BLOCKED);
        assert!(!ADMISSIBILITY_MARGIN_MASTER_RETICK_ELIGIBLE);
        assert!(!ADMISSIBILITY_MARGIN_OP5_CLEARED);
        let probe = admissibility_margin_honesty_probe();
        assert!(!probe.production_wired);
        assert!(probe.green_claim_blocked);
        assert!(!probe.master_retick_eligible);
        assert!(!probe.op5_cleared);
        assert!(probe.eps_finite);
        assert!(probe.deepen_honest);
        assert!(validate_admissibility_margin_honesty().is_ok());
    }
}
