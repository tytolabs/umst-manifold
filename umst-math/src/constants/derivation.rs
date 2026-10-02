// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Constant-Derivation Discipline (CDD) — §0.11 shapes for `REGISTRY` rows.
//!
//! Every [`super::registry::ConstantEntry`] carries a [`Derivation`] describing how egoff
//! re-verifies the constant at runtime (`:verify` palette; lands K-6). A row is classified when it is
//! written: the type has no unclassified state.

/// Schema version bumped when `Derivation` or `ConstantEntry` CDD fields change (K-1 = 1).
pub const DERIVATION_SCHEMA_VERSION: u32 = 2;

use super::registry::ConstantTier;

/// A declaration of the formal Lean catalog: its module (as `catalog.json` names it) and its name. A typed pair
/// in place of a free string, so a reference names a module and a declaration that tests resolve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeanDecl {
    /// Catalog module id, e.g. `Concrete.Gate`.
    pub module: &'static str,
    /// Declaration name in that module, e.g. `δMass_val`.
    pub name: &'static str,
}

/// How a registry constant is re-derived at runtime (egoffplan §0.11).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Derivation {
    /// A Lean declaration whose statement fixes the value; Z-2 LeanExecutor re-runs it and compares.
    Theorem {
        /// The declaration, resolved against the pinned catalog (`artifacts/upstream_catalog.json`) by test.
        decl: LeanDecl,
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
    /// A documented model or configuration choice: no theorem fixes the value and nothing measures it (the formal
    /// constants table's `policy` status). It may change by decision; it is never counted as derived.
    Policy {
        /// The reason for the value and what bounds it; the value itself is the registry row's.
        rationale: &'static str,
    },
    /// A runtime figure not yet measured: the registry carries no invented value until a measurement lands.
    Absent {
        /// What is unmeasured and where the measurement is planned.
        reason: &'static str,
    },
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
            Self::Policy { .. } => "Policy",
            Self::Absent { .. } => "Absent",
        }
    }

    /// The tier a derivation places its row in: the tier is this function of how the value is known. A
    /// definition counts as physical when its authority is an external standard.
    #[must_use]
    pub fn tier(self) -> ConstantTier {
        match self {
            Self::Theorem { .. } => ConstantTier::Tier2Derivable,
            Self::Definition { authority_url, .. } if authority_url.starts_with("https://") => {
                ConstantTier::Tier0Physical
            }
            Self::Definition { .. } | Self::Policy { .. } => ConstantTier::Tier3Policy,
            Self::Measurement { .. } | Self::Absent { .. } => ConstantTier::Tier1Measurement,
            Self::Pin { .. } => ConstantTier::Tier4Infra,
        }
    }

}

/// K-1 landed witness — compile-time schema present on every `REGISTRY` row.
#[must_use]
pub const fn k1_schema_landed() -> bool {
    DERIVATION_SCHEMA_VERSION >= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derivation_schema_carries_policy() {
        assert_eq!(Derivation::Policy { rationale: "r" }.label(), "Policy");
        assert_eq!(Derivation::Policy { rationale: "r" }.tier(), ConstantTier::Tier3Policy);
        assert!(k1_schema_landed());
    }

    #[test]
    fn derivation_labels_match_cdd_taxonomy() {
        assert_eq!(
            Derivation::Theorem {
                decl: LeanDecl { module: "LandauerLaw", name: "uniformBinaryEntropy" },
                expected_value: std::f64::consts::LN_2,
            }
            .label(),
            "Theorem"
        );
    }
}
