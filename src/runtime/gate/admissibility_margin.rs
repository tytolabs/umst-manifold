// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Signed thermodynamic admissibility margin (R2 SSOT).
//!
//! `margin = D_int = −ρ·ψ̇` (signed Clausius–Duhem dissipation surrogate).
//! `violation = relu(−margin)` preserves backward-compatible slack semantics.

use super::evidence::{admissibility_from_violation, AdmissibilityToken};

/// Hard witness floor matching [`super::evidence::admissibility_from_violation`].
pub const ADMISSIBILITY_MARGIN_EPS: f32 = 1e-4;

/// Signed thermodynamic headroom. Positive = dissipative / admissible headroom.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AdmissibilityMargin(pub f32);

impl AdmissibilityMargin {
    #[must_use]
    pub fn value(self) -> f32 {
        self.0
    }

    /// Exterior penalty slack: `relu(−margin)`.
    #[must_use]
    pub fn violation(self) -> f32 {
        (-self.0).max(0.0)
    }

    #[must_use]
    pub fn token(self) -> AdmissibilityToken {
        admissibility_from_violation(self.violation())
    }

    #[must_use]
    pub fn is_hard_admissible(self) -> bool {
        self.0 >= -ADMISSIBILITY_MARGIN_EPS
    }
}

/// Build margin from signed dissipation `D_int` (host `transition_outcome.dissipation`).
#[must_use]
pub fn admissibility_margin_from_dissipation(d_int: f32) -> AdmissibilityMargin {
    AdmissibilityMargin(d_int)
}

/// Hard token from signed margin (ε = 1e−4).
#[must_use]
pub fn admissibility_from_margin(margin: AdmissibilityMargin) -> AdmissibilityToken {
    admissibility_from_violation(margin.violation())
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
}
