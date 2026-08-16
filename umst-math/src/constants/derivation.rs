// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Constant-Derivation Discipline (CDD) — §0.11 shapes for `REGISTRY` rows.
//!
//! Every [`super::registry::ConstantEntry`] carries a [`Derivation`] describing how egoff
//! re-verifies the constant at runtime (`:verify` palette; lands K-6). Legacy rows default
//! to [`Derivation::Pending`] until K-2..K-7 backfill (forbidden after K-7 GREEN).

/// Schema version bumped when `Derivation` or `ConstantEntry` CDD fields change (K-1 = 1).
pub const DERIVATION_SCHEMA_VERSION: u32 = 1;

/// How a registry constant is re-derived at runtime (egoffplan §0.11).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Derivation {
    /// Re-run the cited theorem via Z-2 LeanExecutor; compare to `expected_value`.
    Theorem {
        /// Lean theorem identifier (e.g. `UMST.Formal.Real.log_two_pos`).
        theorem_id: &'static str,
        /// Expected numeric value after proof extraction.
        expected_value: f64,
    },
    /// Re-measure on the current host; compare against JSONL receipt.
    Measurement {
        /// Path to measurement receipt (e.g. `.umst-ci/measurement-receipts/*.jsonl`).
        receipt_path: &'static str,
        /// Methodology anchor (design brief / script cite).
        methodology_anchor: &'static str,
    },
    /// Authority document fetch + SHA-256 compare.
    Definition {
        /// Authority URL (CODATA, RFC, ISO).
        authority_url: &'static str,
        /// Expected SHA-256 of the authority payload.
        expected_sha256: &'static str,
    },
    /// Upstream repo ref pin (toolchain, formal SHA).
    Pin {
        /// Repository slug or URL stem.
        repo: &'static str,
        /// Ref name (`main`, tag, or pin file line).
        ref_name: &'static str,
    },
    /// Awaiting K-2..K-7 backfill — forbidden after K-7 GREEN.
    Pending,
}

impl Derivation {
    /// Stable operator label for TUI `:registry` / receipts.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Theorem { .. } => "Theorem",
            Self::Measurement { .. } => "Measurement",
            Self::Definition { .. } => "Definition",
            Self::Pin { .. } => "Pin",
            Self::Pending => "Pending",
        }
    }

    /// True when the row still awaits CDD backfill.
    #[must_use]
    pub const fn is_pending(self) -> bool {
        matches!(self, Self::Pending)
    }
}

/// K-1 landed witness — compile-time schema present on every `REGISTRY` row.
#[must_use]
pub const fn k1_schema_landed() -> bool {
    DERIVATION_SCHEMA_VERSION == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derivation_schema_version_is_one() {
        assert_eq!(DERIVATION_SCHEMA_VERSION, 1);
        assert!(k1_schema_landed());
    }

    #[test]
    fn derivation_labels_match_cdd_taxonomy() {
        assert_eq!(Derivation::Pending.label(), "Pending");
        assert_eq!(
            Derivation::Theorem {
                theorem_id: "UMST.Formal.Real.log_two_pos",
                expected_value: std::f64::consts::LN_2,
            }
            .label(),
            "Theorem"
        );
    }
}
