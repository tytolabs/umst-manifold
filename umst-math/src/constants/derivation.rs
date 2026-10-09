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

    /// The `docs/….md#anchor` an `Absent` reason cites, as (document path, anchor); `None` for any other
    /// derivation or a reason without one. A typed absence names where its measurement is planned.
    #[must_use]
    pub fn absent_doc_anchor(self) -> Option<(&'static str, &'static str)> {
        let Self::Absent { reason } = self else {
            return None;
        };
        reason
            .split(|c: char| c.is_whitespace() || c == ';' || c == ',')
            .find_map(|tok| {
                let (doc, anchor) = tok.split_once('#')?;
                let is_md = std::path::Path::new(doc)
                    .extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("md"));
                (is_md && !anchor.is_empty()).then_some((doc, anchor))
            })
    }

    /// True when a field the derivation's kind requires is empty: such a row is classified in name only.
    #[must_use]
    pub fn payload_is_empty(self) -> bool {
        match self {
            Self::Theorem { decl, .. } => decl.module.is_empty() || decl.name.is_empty(),
            Self::Measurement {
                receipt_path,
                methodology_anchor,
            } => receipt_path.trim().is_empty() || methodology_anchor.trim().is_empty(),
            Self::Definition {
                authority_url,
                expected_sha256,
            } => authority_url.trim().is_empty() || expected_sha256.trim().is_empty(),
            Self::Pin { repo, ref_name } => repo.trim().is_empty() || ref_name.trim().is_empty(),
            Self::Policy { rationale } => rationale.trim().is_empty(),
            Self::Absent { reason } => reason.trim().is_empty(),
        }
    }
}

/// K-1 landed witness — compile-time schema present on every `REGISTRY` row.
#[must_use]
pub const fn k1_schema_landed() -> bool {
    DERIVATION_SCHEMA_VERSION >= 1
}

/// Count of registry rows per derivation kind: the K-7 ratchet reads it (`Absent` rows are the constants not yet
/// derived; a row with an empty payload is classified in name only).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DerivationCensus {
    /// Rows counted.
    pub rows: usize,
    /// `Theorem` rows.
    pub theorem: usize,
    /// `Measurement` rows.
    pub measurement: usize,
    /// `Definition` rows.
    pub definition: usize,
    /// `Pin` rows.
    pub pin: usize,
    /// `Policy` rows.
    pub policy: usize,
    /// `Absent` rows.
    pub absent: usize,
    /// Rows whose required payload field is empty ([`Derivation::payload_is_empty`]).
    pub empty_payload: usize,
}

impl DerivationCensus {
    /// Census of `derivations`.
    #[must_use]
    pub fn of<I: IntoIterator<Item = Derivation>>(derivations: I) -> Self {
        let mut c = Self::default();
        for d in derivations {
            c.rows += 1;
            match d {
                Derivation::Theorem { .. } => c.theorem += 1,
                Derivation::Measurement { .. } => c.measurement += 1,
                Derivation::Definition { .. } => c.definition += 1,
                Derivation::Pin { .. } => c.pin += 1,
                Derivation::Policy { .. } => c.policy += 1,
                Derivation::Absent { .. } => c.absent += 1,
            }
            if d.payload_is_empty() {
                c.empty_payload += 1;
            }
        }
        c
    }

    /// Census of the live `REGISTRY`.
    #[must_use]
    pub fn of_registry() -> Self {
        Self::of(super::registry::REGISTRY.iter().map(|e| e.derivation))
    }

    /// One-line JSON object with every count.
    #[must_use]
    pub fn to_json(&self) -> String {
        format!(
            "{{\"rows\":{},\"theorem\":{},\"measurement\":{},\"definition\":{},\"pin\":{},\"policy\":{},\"absent\":{},\"empty_payload\":{}}}",
            self.rows, self.theorem, self.measurement, self.definition, self.pin, self.policy, self.absent, self.empty_payload
        )
    }
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
    fn census_counts_each_kind_and_empty_payloads() {
        let c = DerivationCensus::of([
            Derivation::Absent { reason: "unmeasured; docs/x.md#plan" },
            Derivation::Absent { reason: " " },
            Derivation::Policy { rationale: "chosen" },
            Derivation::Pin { repo: "umst-formal", ref_name: "main" },
        ]);
        assert_eq!((c.rows, c.absent, c.policy, c.pin, c.theorem, c.empty_payload), (4, 2, 1, 1, 0, 1));
        assert_eq!(
            c.to_json(),
            r#"{"rows":4,"theorem":0,"measurement":0,"definition":0,"pin":1,"policy":1,"absent":2,"empty_payload":1}"#
        );
        let live = DerivationCensus::of_registry();
        assert_eq!(live.rows, super::super::registry::REGISTRY.len());
        assert_eq!(
            live.theorem + live.measurement + live.definition + live.pin + live.policy + live.absent,
            live.rows
        );
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
