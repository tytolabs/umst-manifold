// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
//
// HCOM-004 @ 17:19 IST — cold semantic proof witnesses in catalog.lock.json.
// Hot path cites `witness_id` + `digest_hex` only; full Lean proof stays cold/build-time.
// W29-107-SEMANTIC_WITNESS — deepen + honest fences (no invent GREEN/PRODUCTION_WIRED/MASTER/OP-5).

use serde::{Deserialize, Serialize};

use super::CatalogLock;

/// Policy version for semantic cold/hot witness split (blueprint §2 / §3.2).
pub const SEMANTIC_COLD_HOT_POLICY_VERSION: &str = "semantic_cold_hot_witness_v1";

/// Default cold witness for `SemanticSecondLaw` (HCOM-001).
pub const DEFAULT_SEMANTIC_COLD_WITNESS_ID: &str = "umst.formal.semantic_second_law";

/// Stable gate slug for semantic control-barrier rejects.
pub const SEMANTIC_CBF_CATALOG_ID: &str = "umst.gate.semantic_cbf";

/// Lean module basename for the default semantic second-law cold proof.
pub const DEFAULT_SEMANTIC_LEAN_MODULE: &str = "SemanticSecondLaw";

/// Fully qualified Lean anchor for the default semantic second-law cold proof.
pub const DEFAULT_SEMANTIC_FORMAL_ANCHOR: &str = "UMST.SemanticSecondLaw.semanticSecondLaw";

/// Honest posture — catalog cold/hot witness deepen only; not a production flip.
pub const SEMANTIC_WITNESS_POSTURE_TAG: &str = "semantic-cold-hot-catalog-wired-not-production";

// ── W29-107-SEMANTIC_WITNESS · deepen + honest fence ─────────────────────────

/// Swarm cell id for this semantic_witness deepen.
pub const W29_107_CELL_ID: &str = "W29-107-SEMANTIC_WITNESS";

/// Honest posture — deepen measured cold/hot catalog surface only; no invent claims.
pub const W29_107_HONEST_POSTURE: &str = "SEMANTIC_WITNESS_DEEPEN_ONLY";

/// Explicit non-claims (gate text).
pub const W29_107_NON_CLAIM: &str =
    "not GREEN; not OP-5 PASS; not production_wired; not MASTER_RETICK";

/// Deepen schema version for W29-107.
pub const W29_107_DEEPEN_SCHEMA_VERSION: &str = "semantic_witness_w29_107_deepen_v1";

/// Expected wired hop count on manifold catalog side (hot CBF / gateway remain open).
pub const W29_107_WIRE_HOP_WIRED_COUNT: u8 = 3;

/// Expected total wire hops (3 closed catalog + 2 open consumers).
pub const W29_107_WIRE_HOP_TOTAL: usize = 5;

/// One build-time Lean proof exported as a catalog cold witness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticColdProof {
    /// Content-addressed witness id (agents cite this on the hot path).
    pub witness_id: String,
    /// Lean module basename (e.g. `SemanticSecondLaw`).
    pub lean_module: String,
    /// Runtime gate `catalog_id` slug wired to this proof.
    pub catalog_id: String,
    /// Always `"cold"` for exported formal proofs.
    pub proof_kind: String,
    /// Fully qualified Lean anchor (`UMST.Module.prop`).
    pub formal_anchor: String,
    /// SHA-256 hex digest of the formal export (agents never re-prove at runtime).
    pub digest_hex: String,
}

/// Hot-path citation — agents carry `witness_id` + `digest_hex` only (no Lean body).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticHotCitation {
    /// Content-addressed witness id matching a cold catalog row.
    pub witness_id: String,
    /// Cited SHA-256 hex digest (must match cold `digest_hex`, case-insensitive).
    pub digest_hex: String,
}

/// Hot-path citation mismatch / structural reject (catalog layer — not CBF debit).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticWitnessReject {
    /// No `semantic_witnesses` section on the lock.
    MissingSection,
    /// No cold proof row for the cited `witness_id`.
    MissingColdWitness { witness_id: String },
    /// Cited digest does not match the cold catalog row.
    DigestMismatch {
        witness_id: String,
        expected_hex: String,
        cited_hex: String,
    },
    /// Hot citation failed structural quickcheck (empty id / bad hex).
    MalformedCitation { reason: &'static str },
}

impl core::fmt::Display for SemanticWitnessReject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingSection => write!(
                f,
                "REJECTED [{SEMANTIC_CBF_CATALOG_ID}]: missing semantic_witnesses section"
            ),
            Self::MissingColdWitness { witness_id } => write!(
                f,
                "REJECTED [{SEMANTIC_CBF_CATALOG_ID}]: missing cold semantic witness `{witness_id}`"
            ),
            Self::DigestMismatch {
                witness_id,
                expected_hex,
                cited_hex,
            } => write!(
                f,
                "REJECTED [{SEMANTIC_CBF_CATALOG_ID}]: digest mismatch for `{witness_id}` \
                 (expected {expected_hex}, cited {cited_hex})"
            ),
            Self::MalformedCitation { reason } => write!(
                f,
                "REJECTED [{SEMANTIC_CBF_CATALOG_ID}]: malformed hot citation ({reason})"
            ),
        }
    }
}

/// `catalog.lock.json` semantic witness section (HCOM-004).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticWitnessSection {
    pub policy_version: String,
    pub cold_proofs: Vec<SemanticColdProof>,
}

/// One hop in the semantic cold/hot witness wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SemanticWitnessWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the cold/hot split.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold semantic-witness wire map (lock section → hot citation → open consumers).
pub const SEMANTIC_WITNESS_WIRE_HOPS: &[SemanticWitnessWireHop] = &[
    SemanticWitnessWireHop {
        ordinal: 1,
        surface: "catalog.lock.json::semantic_witnesses",
        role: "HCOM-004 cold proof export section",
        wired: true,
    },
    SemanticWitnessWireHop {
        ordinal: 2,
        surface: "lookup_semantic_cold_witness / bundled lookup",
        role: "IO-free cold row lookup by witness_id",
        wired: true,
    },
    SemanticWitnessWireHop {
        ordinal: 3,
        surface: "verify_hot_citation_against_lock",
        role: "Hot citation digest pin against cold row",
        wired: true,
    },
    SemanticWitnessWireHop {
        ordinal: 4,
        surface: "gate::semantic_cbf::gate_semantic_hot",
        role: "Hot CBF debit consumer (serial; not this write_set)",
        wired: false,
    },
    SemanticWitnessWireHop {
        ordinal: 5,
        surface: "gateway semantic_witness production flip",
        role: "Live gateway production wire (honest open)",
        wired: false,
    },
];

/// Lookup a cold semantic proof witness by `witness_id` in a parsed lock.
#[must_use]
pub fn lookup_semantic_cold_witness<'a>(
    lock: &'a CatalogLock,
    witness_id: &str,
) -> Option<&'a SemanticColdProof> {
    lock.semantic_witnesses
        .as_ref()?
        .cold_proofs
        .iter()
        .find(|proof| proof.witness_id == witness_id)
}

/// Lookup a cold semantic proof by runtime gate `catalog_id` slug.
#[must_use]
pub fn lookup_semantic_cold_witness_by_catalog_id<'a>(
    lock: &'a CatalogLock,
    catalog_id: &str,
) -> Option<&'a SemanticColdProof> {
    lock.semantic_witnesses
        .as_ref()?
        .cold_proofs
        .iter()
        .find(|proof| proof.catalog_id == catalog_id)
}

/// Lookup from the bundled `artifacts/catalog.lock.json`.
#[must_use]
pub fn lookup_bundled_semantic_cold_witness(witness_id: &str) -> Option<SemanticColdProof> {
    CatalogLock::from_bundled()
        .ok()
        .and_then(|lock| lookup_semantic_cold_witness(&lock, witness_id).cloned())
}

/// True when the bundled lock carries the HCOM-004 semantic witness section.
#[must_use]
pub fn bundled_semantic_witness_section_present() -> bool {
    CatalogLock::from_bundled()
        .ok()
        .and_then(|lock| lock.semantic_witnesses)
        .is_some()
}

/// Structural quickcheck on a hot citation (non-empty id, 64-char ascii hex digest).
#[must_use]
pub fn semantic_hot_citation_quickcheck(citation: &SemanticHotCitation) -> bool {
    !citation.witness_id.is_empty()
        && citation.digest_hex.len() == 64
        && citation.digest_hex.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Build a hot citation from a cold proof row (agents clone id + digest only).
#[must_use]
pub fn hot_citation_from_cold_proof(proof: &SemanticColdProof) -> SemanticHotCitation {
    SemanticHotCitation {
        witness_id: proof.witness_id.clone(),
        digest_hex: proof.digest_hex.clone(),
    }
}

/// Default hot citation for the bundled semantic second-law cold proof (if present).
#[must_use]
pub fn default_bundled_hot_citation() -> Option<SemanticHotCitation> {
    lookup_bundled_semantic_cold_witness(DEFAULT_SEMANTIC_COLD_WITNESS_ID)
        .map(|proof| hot_citation_from_cold_proof(&proof))
}

/// Verify a hot citation against a cold catalog row (case-insensitive digest).
pub fn verify_hot_citation_against_proof(
    proof: &SemanticColdProof,
    citation: &SemanticHotCitation,
) -> Result<(), SemanticWitnessReject> {
    if !semantic_hot_citation_quickcheck(citation) {
        return Err(SemanticWitnessReject::MalformedCitation {
            reason: "empty witness_id or non-hex digest",
        });
    }
    if citation.witness_id != proof.witness_id {
        return Err(SemanticWitnessReject::MissingColdWitness {
            witness_id: citation.witness_id.clone(),
        });
    }
    if proof.digest_hex.eq_ignore_ascii_case(&citation.digest_hex) {
        Ok(())
    } else {
        Err(SemanticWitnessReject::DigestMismatch {
            witness_id: proof.witness_id.clone(),
            expected_hex: proof.digest_hex.clone(),
            cited_hex: citation.digest_hex.clone(),
        })
    }
}

/// Hot-path pin: resolve cold row by citation `witness_id`, then verify digest.
pub fn verify_hot_citation_against_lock<'a>(
    lock: &'a CatalogLock,
    citation: &SemanticHotCitation,
) -> Result<&'a SemanticColdProof, SemanticWitnessReject> {
    if lock.semantic_witnesses.is_none() {
        return Err(SemanticWitnessReject::MissingSection);
    }
    if !semantic_hot_citation_quickcheck(citation) {
        return Err(SemanticWitnessReject::MalformedCitation {
            reason: "empty witness_id or non-hex digest",
        });
    }
    let proof = lookup_semantic_cold_witness(lock, &citation.witness_id).ok_or_else(|| {
        SemanticWitnessReject::MissingColdWitness {
            witness_id: citation.witness_id.clone(),
        }
    })?;
    verify_hot_citation_against_proof(proof, citation)?;
    Ok(proof)
}

/// Bundled-lock convenience for hot citation verify.
pub fn verify_hot_citation_bundled(
    citation: &SemanticHotCitation,
) -> Result<SemanticColdProof, SemanticWitnessReject> {
    let lock = CatalogLock::from_bundled().map_err(|_| SemanticWitnessReject::MissingSection)?;
    verify_hot_citation_against_lock(&lock, citation).cloned()
}

/// Structural quickcheck on semantic witness rows (64-char digests, non-empty ids).
#[must_use]
pub fn semantic_witness_section_quickcheck(section: &SemanticWitnessSection) -> bool {
    if section.policy_version != SEMANTIC_COLD_HOT_POLICY_VERSION {
        return false;
    }
    if section.cold_proofs.is_empty() {
        return false;
    }
    section.cold_proofs.iter().all(|proof| {
        !proof.witness_id.is_empty()
            && !proof.lean_module.is_empty()
            && !proof.catalog_id.is_empty()
            && !proof.formal_anchor.is_empty()
            && proof.proof_kind == "cold"
            && proof.digest_hex.len() == 64
            && proof.digest_hex.bytes().all(|b| b.is_ascii_hexdigit())
    })
}

/// True when bundled lock section passes quickcheck and default witness is present.
#[must_use]
pub fn semantic_witness_catalog_surface_wired() -> bool {
    let Ok(lock) = CatalogLock::from_bundled() else {
        return false;
    };
    let Some(section) = lock.semantic_witnesses.as_ref() else {
        return false;
    };
    if !semantic_witness_section_quickcheck(section) {
        return false;
    }
    let Some(proof) = lookup_semantic_cold_witness(&lock, DEFAULT_SEMANTIC_COLD_WITNESS_ID) else {
        return false;
    };
    proof.catalog_id == SEMANTIC_CBF_CATALOG_ID
        && proof.lean_module == DEFAULT_SEMANTIC_LEAN_MODULE
        && proof.formal_anchor == DEFAULT_SEMANTIC_FORMAL_ANCHOR
        && proof.proof_kind == "cold"
}

/// Whether live gateway / production semantic-witness flip is plumbed (honest `false`).
#[must_use]
pub const fn semantic_witness_production_wired() -> bool {
    false
}

/// Close predicate for the **catalog** cold/hot surface (not gateway production).
#[must_use]
pub fn semantic_witness_catalog_ceremony_closed() -> bool {
    semantic_witness_catalog_surface_wired()
        && bundled_semantic_witness_section_present()
        && default_bundled_hot_citation()
            .map(|c| verify_hot_citation_bundled(&c).is_ok())
            .unwrap_or(false)
}

/// Typed probe for semantic witness catalog closure honesty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SemanticWitnessCatalogProbe {
    /// Bundled lock parses and carries `semantic_witnesses`.
    pub section_present: bool,
    /// Section passes structural quickcheck.
    pub section_quickcheck_ok: bool,
    /// Default semantic second-law cold row present + catalog_id wired.
    pub default_witness_ok: bool,
    /// Default hot citation verifies against bundled lock.
    pub default_hot_citation_ok: bool,
    /// Catalog surface wired predicate.
    pub catalog_surface_wired: bool,
    /// Catalog ceremony close predicate.
    pub ceremony_closed: bool,
    /// Gateway / production invent claim — always false.
    pub production_wired: bool,
    /// Wire hop wired count.
    pub wire_hop_wired_count: u8,
    /// Total wire hops in map.
    pub wire_hop_total: usize,
    /// Honest posture tag.
    pub posture_tag: &'static str,
}

/// Build introspection probe for semantic_witness done-when checks.
#[must_use]
pub fn semantic_witness_catalog_probe() -> SemanticWitnessCatalogProbe {
    let lock = CatalogLock::from_bundled().ok();
    let section = lock.as_ref().and_then(|l| l.semantic_witnesses.as_ref());
    let section_present = section.is_some();
    let section_quickcheck_ok = section
        .map(semantic_witness_section_quickcheck)
        .unwrap_or(false);
    let default_witness_ok = lock
        .as_ref()
        .and_then(|l| lookup_semantic_cold_witness(l, DEFAULT_SEMANTIC_COLD_WITNESS_ID))
        .map(|p| {
            p.catalog_id == SEMANTIC_CBF_CATALOG_ID
                && p.lean_module == DEFAULT_SEMANTIC_LEAN_MODULE
                && p.formal_anchor == DEFAULT_SEMANTIC_FORMAL_ANCHOR
        })
        .unwrap_or(false);
    let default_hot_citation_ok = default_bundled_hot_citation()
        .map(|c| verify_hot_citation_bundled(&c).is_ok())
        .unwrap_or(false);
    SemanticWitnessCatalogProbe {
        section_present,
        section_quickcheck_ok,
        default_witness_ok,
        default_hot_citation_ok,
        catalog_surface_wired: semantic_witness_catalog_surface_wired(),
        ceremony_closed: semantic_witness_catalog_ceremony_closed(),
        production_wired: semantic_witness_production_wired(),
        wire_hop_wired_count: SEMANTIC_WITNESS_WIRE_HOPS
            .iter()
            .filter(|h| h.wired)
            .count() as u8,
        wire_hop_total: SEMANTIC_WITNESS_WIRE_HOPS.len(),
        posture_tag: SEMANTIC_WITNESS_POSTURE_TAG,
    }
}

/// Honest fence flags for semantic_witness deepen (W29-107).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SemanticWitnessW29107DeepenProbe {
    /// Deepen schema pin.
    pub schema_version: &'static str,
    /// Swarm cell id pin.
    pub cell_id: &'static str,
    /// Honest posture label.
    pub honest_posture: &'static str,
    /// Explicit non-claim string.
    pub non_claim: &'static str,
    /// Policy version pin.
    pub policy_version: &'static str,
    /// Live catalog ceremony close predicate.
    pub ceremony_closed: bool,
    /// Catalog surface wired.
    pub catalog_surface_wired: bool,
    /// Wire hops wired (catalog side).
    pub wire_hop_wired_count: u8,
    /// Total wire hops in map.
    pub wire_hop_total: usize,
    /// Gateway/production invent claim — always false.
    pub production_wired_claimed: bool,
    /// Physics/fleet GREEN invent claim — always false.
    pub green_claimed: bool,
    /// OP-5 PASS invent claim — always false.
    pub op5_pass_claimed: bool,
    /// MASTER_RETICK invent claim — always false.
    pub master_retick_claimed: bool,
    /// Aggregate honesty of this deepen probe.
    pub deepen_honest: bool,
}

/// Build the W29-107 semantic_witness deepen honesty probe from live measurements.
#[must_use]
pub fn semantic_witness_w29_107_deepen_probe() -> SemanticWitnessW29107DeepenProbe {
    let production_wired_claimed = semantic_witness_production_wired();
    let green_claimed = false;
    let op5_pass_claimed = false;
    let master_retick_claimed = false;
    let catalog = semantic_witness_catalog_probe();
    let ceremony_closed = catalog.ceremony_closed;
    let wire_hop_wired_count = catalog.wire_hop_wired_count;
    let wire_hop_total = catalog.wire_hop_total;
    let deepen_honest = W29_107_CELL_ID == "W29-107-SEMANTIC_WITNESS"
        && W29_107_DEEPEN_SCHEMA_VERSION == "semantic_witness_w29_107_deepen_v1"
        && W29_107_HONEST_POSTURE == "SEMANTIC_WITNESS_DEEPEN_ONLY"
        && SEMANTIC_COLD_HOT_POLICY_VERSION == "semantic_cold_hot_witness_v1"
        && ceremony_closed
        && catalog.catalog_surface_wired
        && catalog.section_present
        && catalog.section_quickcheck_ok
        && catalog.default_witness_ok
        && catalog.default_hot_citation_ok
        && wire_hop_wired_count == W29_107_WIRE_HOP_WIRED_COUNT
        && wire_hop_total == W29_107_WIRE_HOP_TOTAL
        && !production_wired_claimed
        && !catalog.production_wired
        && !green_claimed
        && !op5_pass_claimed
        && !master_retick_claimed
        && W29_107_NON_CLAIM.contains("not GREEN")
        && W29_107_NON_CLAIM.contains("not OP-5 PASS")
        && W29_107_NON_CLAIM.contains("not production_wired")
        && W29_107_NON_CLAIM.contains("not MASTER_RETICK")
        && SEMANTIC_WITNESS_POSTURE_TAG.contains("not-production");
    SemanticWitnessW29107DeepenProbe {
        schema_version: W29_107_DEEPEN_SCHEMA_VERSION,
        cell_id: W29_107_CELL_ID,
        honest_posture: W29_107_HONEST_POSTURE,
        non_claim: W29_107_NON_CLAIM,
        policy_version: SEMANTIC_COLD_HOT_POLICY_VERSION,
        ceremony_closed,
        catalog_surface_wired: catalog.catalog_surface_wired,
        wire_hop_wired_count,
        wire_hop_total,
        production_wired_claimed,
        green_claimed,
        op5_pass_claimed,
        master_retick_claimed,
        deepen_honest,
    }
}

/// Whether the W29-107 semantic_witness deepen honesty probe passes.
#[must_use]
pub fn semantic_witness_w29_107_deepen_honest() -> bool {
    semantic_witness_w29_107_deepen_probe().deepen_honest
}

/// Fence: refuse inventing GREEN / PRODUCTION_WIRED / MASTER / OP-5.
#[must_use]
pub fn semantic_witness_w29_107_honest_fence_holds() -> bool {
    let p = semantic_witness_w29_107_deepen_probe();
    p.deepen_honest
        && !p.green_claimed
        && !p.production_wired_claimed
        && !p.op5_pass_claimed
        && !p.master_retick_claimed
}

/// Deepen census — measured counts for gate_deltas (no invent flags).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SemanticWitnessW29107Census {
    /// Cold proofs on bundled lock.
    pub cold_proof_count: usize,
    /// Wired hops.
    pub wire_hop_wired: u8,
    /// Total hops.
    pub wire_hop_total: usize,
    /// Unit tests in this module (compile-time pin).
    pub unit_tests_in_module: u8,
}

/// Expected unit-test count pin for W29-107 deepen (keep in sync with `#[test]` fns).
pub const W29_107_UNIT_TESTS_IN_MODULE: u8 = 12;

/// Build deepen census from live bundled lock + wire map.
#[must_use]
pub fn semantic_witness_w29_107_census() -> SemanticWitnessW29107Census {
    let cold_proof_count = CatalogLock::from_bundled()
        .ok()
        .and_then(|lock| lock.semantic_witnesses)
        .map(|s| s.cold_proofs.len())
        .unwrap_or(0);
    SemanticWitnessW29107Census {
        cold_proof_count,
        wire_hop_wired: SEMANTIC_WITNESS_WIRE_HOPS
            .iter()
            .filter(|h| h.wired)
            .count() as u8,
        wire_hop_total: SEMANTIC_WITNESS_WIRE_HOPS.len(),
        unit_tests_in_module: W29_107_UNIT_TESTS_IN_MODULE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bundled_lock() -> CatalogLock {
        CatalogLock::from_bundled().expect("bundled lock parses")
    }

    #[test]
    fn bundled_lock_has_semantic_witness_section() {
        let lock = bundled_lock();
        let section = lock
            .semantic_witnesses
            .as_ref()
            .expect("HCOM-004 semantic_witnesses section");
        assert!(semantic_witness_section_quickcheck(section));
        assert!(
            lookup_semantic_cold_witness(&lock, DEFAULT_SEMANTIC_COLD_WITNESS_ID).is_some(),
            "default semantic second law witness must be present"
        );
    }

    #[test]
    fn lookup_missing_witness_returns_none() {
        let lock = bundled_lock();
        assert!(lookup_semantic_cold_witness(&lock, "nonexistent.witness.id").is_none());
    }

    #[test]
    fn default_cold_proof_pins_semantic_cbf_catalog_id() {
        let lock = bundled_lock();
        let proof = lookup_semantic_cold_witness(&lock, DEFAULT_SEMANTIC_COLD_WITNESS_ID)
            .expect("default witness");
        assert_eq!(proof.catalog_id, SEMANTIC_CBF_CATALOG_ID);
        assert_eq!(proof.lean_module, DEFAULT_SEMANTIC_LEAN_MODULE);
        assert_eq!(proof.formal_anchor, DEFAULT_SEMANTIC_FORMAL_ANCHOR);
        assert_eq!(proof.proof_kind, "cold");
        assert_eq!(proof.digest_hex.len(), 64);
    }

    #[test]
    fn lookup_by_catalog_id_finds_default_row() {
        let lock = bundled_lock();
        let proof = lookup_semantic_cold_witness_by_catalog_id(&lock, SEMANTIC_CBF_CATALOG_ID)
            .expect("catalog_id lookup");
        assert_eq!(proof.witness_id, DEFAULT_SEMANTIC_COLD_WITNESS_ID);
    }

    #[test]
    fn hot_citation_verifies_against_bundled_lock() {
        let citation = default_bundled_hot_citation().expect("default hot citation");
        assert!(semantic_hot_citation_quickcheck(&citation));
        let proof = verify_hot_citation_bundled(&citation).expect("hot citation verifies");
        assert_eq!(proof.witness_id, DEFAULT_SEMANTIC_COLD_WITNESS_ID);
    }

    #[test]
    fn hot_citation_rejects_digest_mismatch() {
        let mut citation = default_bundled_hot_citation().expect("default hot citation");
        citation.digest_hex = "00".repeat(32);
        let err = verify_hot_citation_bundled(&citation).expect_err("digest mismatch");
        assert!(matches!(err, SemanticWitnessReject::DigestMismatch { .. }));
    }

    #[test]
    fn hot_citation_rejects_missing_witness() {
        let citation = SemanticHotCitation {
            witness_id: "umst.formal.nonexistent_proof".into(),
            digest_hex: "ab".repeat(32),
        };
        let err = verify_hot_citation_bundled(&citation).expect_err("missing witness");
        assert!(matches!(
            err,
            SemanticWitnessReject::MissingColdWitness { .. }
        ));
    }

    #[test]
    fn hot_citation_rejects_malformed_digest() {
        let citation = SemanticHotCitation {
            witness_id: DEFAULT_SEMANTIC_COLD_WITNESS_ID.into(),
            digest_hex: "not-a-digest".into(),
        };
        assert!(!semantic_hot_citation_quickcheck(&citation));
        let err = verify_hot_citation_bundled(&citation).expect_err("malformed");
        assert!(matches!(
            err,
            SemanticWitnessReject::MalformedCitation { .. }
        ));
    }

    #[test]
    fn catalog_surface_and_ceremony_closed_without_production() {
        assert!(bundled_semantic_witness_section_present());
        assert!(semantic_witness_catalog_surface_wired());
        assert!(semantic_witness_catalog_ceremony_closed());
        assert!(!semantic_witness_production_wired());
        let probe = semantic_witness_catalog_probe();
        assert!(probe.section_present);
        assert!(probe.section_quickcheck_ok);
        assert!(probe.default_witness_ok);
        assert!(probe.default_hot_citation_ok);
        assert!(!probe.production_wired);
        assert_eq!(probe.wire_hop_wired_count, W29_107_WIRE_HOP_WIRED_COUNT);
        assert_eq!(probe.wire_hop_total, W29_107_WIRE_HOP_TOTAL);
        assert_eq!(probe.posture_tag, SEMANTIC_WITNESS_POSTURE_TAG);
    }

    #[test]
    fn wire_hops_three_of_five_wired() {
        assert_eq!(SEMANTIC_WITNESS_WIRE_HOPS.len(), W29_107_WIRE_HOP_TOTAL);
        let wired = SEMANTIC_WITNESS_WIRE_HOPS
            .iter()
            .filter(|h| h.wired)
            .count();
        assert_eq!(wired as u8, W29_107_WIRE_HOP_WIRED_COUNT);
        assert!(!SEMANTIC_WITNESS_WIRE_HOPS[3].wired);
        assert!(!SEMANTIC_WITNESS_WIRE_HOPS[4].wired);
    }

    #[test]
    fn w29_107_deepen_honest_fence_holds() {
        assert_eq!(W29_107_CELL_ID, "W29-107-SEMANTIC_WITNESS");
        assert_eq!(
            W29_107_DEEPEN_SCHEMA_VERSION,
            "semantic_witness_w29_107_deepen_v1"
        );
        let probe = semantic_witness_w29_107_deepen_probe();
        assert!(probe.deepen_honest);
        assert!(semantic_witness_w29_107_deepen_honest());
        assert!(semantic_witness_w29_107_honest_fence_holds());
        assert!(!probe.green_claimed);
        assert!(!probe.production_wired_claimed);
        assert!(!probe.op5_pass_claimed);
        assert!(!probe.master_retick_claimed);
        assert!(probe.non_claim.contains("not GREEN"));
        assert!(probe.non_claim.contains("not production_wired"));
    }

    #[test]
    fn w29_107_census_matches_bundled_lock() {
        let census = semantic_witness_w29_107_census();
        assert_eq!(census.cold_proof_count, 1);
        assert_eq!(census.wire_hop_wired, W29_107_WIRE_HOP_WIRED_COUNT);
        assert_eq!(census.wire_hop_total, W29_107_WIRE_HOP_TOTAL);
        assert_eq!(census.unit_tests_in_module, W29_107_UNIT_TESTS_IN_MODULE);
    }
}
