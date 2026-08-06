// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
//
// HCOM-004 @ 17:19 IST — semantic control-barrier extends thermodynamic CBF.
// Cold proofs live in `catalog.lock.json`; hot path performs digest lookup only.

use crate::ai::cbf::ThermodynamicCBF;
use crate::core::error_boundary::CbfReject;
use crate::runtime::catalog::{
    lookup_semantic_cold_witness, CatalogLock, SemanticColdProof,
    DEFAULT_SEMANTIC_COLD_WITNESS_ID, SEMANTIC_CBF_CATALOG_ID,
};

pub use crate::runtime::catalog::semantic_witness::{
    bundled_semantic_witness_section_present, semantic_witness_section_quickcheck,
    SEMANTIC_COLD_HOT_POLICY_VERSION,
};

/// Hot-path rejection when semantic CBF cannot cite a cold catalog witness.
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticCbfReject {
    /// No matching cold proof in `catalog.lock.json` `semantic_witnesses` section.
    MissingColdWitness {
        witness_id: String,
        catalog_id: &'static str,
    },
    /// Witness digest mismatch (tamper or stale hot citation).
    DigestMismatch {
        witness_id: String,
        expected_hex: String,
        cited_hex: String,
    },
    /// Thermodynamic envelope rejected after witness lookup succeeded.
    Thermodynamic(CbfReject),
    /// Semantic admissibility margin negative (net dissipation below tolerance).
    SemanticInadmissible { net_dissipation: f64, tolerance: f64 },
}

impl core::fmt::Display for SemanticCbfReject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingColdWitness {
                witness_id,
                catalog_id,
            } => write!(
                f,
                "REJECTED [{catalog_id}]: missing cold semantic witness `{witness_id}`"
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
            Self::Thermodynamic(inner) => write!(f, "{inner}"),
            Self::SemanticInadmissible {
                net_dissipation,
                tolerance,
            } => write!(
                f,
                "REJECTED [{SEMANTIC_CBF_CATALOG_ID}]: semantic net dissipation \
                 {net_dissipation} < -{tolerance}"
            ),
        }
    }
}

impl SemanticCbfReject {
    /// Stable gate slug for telemetry (see `GateUnificationSpec.md`).
    #[must_use]
    pub const fn catalog_id(&self) -> &'static str {
        SEMANTIC_CBF_CATALOG_ID
    }
}

/// Semantic control-barrier: thermodynamic CBF + cold-proof witness lookup.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticCBF {
    /// Underlying thermodynamic envelope (Landauer + Clausius–Duhem).
    pub thermodynamic: ThermodynamicCBF,
    /// Hot-path citation of the cold catalog witness id.
    pub cold_witness_id: String,
}

impl SemanticCBF {
    /// Seed from thermodynamic parameters and a cold witness id (default: semantic second law).
    #[must_use]
    pub fn new(temperature_k: f64, initial_credit_joules: f64, cold_witness_id: &str) -> Self {
        Self {
            thermodynamic: ThermodynamicCBF::new(temperature_k, initial_credit_joules),
            cold_witness_id: cold_witness_id.to_string(),
        }
    }

    /// Default fixture: room-temperature bath with chair-scale credit budget.
    #[must_use]
    pub fn chair_fixture() -> Self {
        Self::new(300.0, 1.0e-6, DEFAULT_SEMANTIC_COLD_WITNESS_ID)
    }
}

/// Hot-path witness lookup — returns the cold proof row or [`SemanticCbfReject::MissingColdWitness`].
pub fn hot_gate_lookup_cold_witness<'a>(
    lock: &'a CatalogLock,
    witness_id: &str,
) -> Result<&'a SemanticColdProof, SemanticCbfReject> {
    lookup_semantic_cold_witness(lock, witness_id).ok_or_else(|| SemanticCbfReject::MissingColdWitness {
        witness_id: witness_id.to_string(),
        catalog_id: SEMANTIC_CBF_CATALOG_ID,
    })
}

/// Verify cited digest matches catalog (optional hot-path pin).
pub fn verify_cold_witness_digest(
    proof: &SemanticColdProof,
    cited_digest_hex: &str,
) -> Result<(), SemanticCbfReject> {
    if proof.digest_hex.eq_ignore_ascii_case(cited_digest_hex) {
        Ok(())
    } else {
        Err(SemanticCbfReject::DigestMismatch {
            witness_id: proof.witness_id.clone(),
            expected_hex: proof.digest_hex.clone(),
            cited_hex: cited_digest_hex.to_string(),
        })
    }
}

/// Hot semantic gate: witness lookup → semantic margin → thermodynamic CBF debit.
///
/// `net_dissipation` is the semantic admissibility margin (negative ⇒ reject).
/// `bits_resolved` feeds the Landauer branch of the underlying thermodynamic CBF.
pub fn gate_semantic_hot(
    cbf: &mut SemanticCBF,
    lock: &CatalogLock,
    net_dissipation: f64,
    bits_resolved: f64,
    tolerance: f64,
    cited_digest_hex: Option<&str>,
) -> Result<f64, SemanticCbfReject> {
    let proof = hot_gate_lookup_cold_witness(lock, &cbf.cold_witness_id)?;
    if let Some(cited) = cited_digest_hex {
        verify_cold_witness_digest(proof, cited)?;
    }
    if net_dissipation < -tolerance {
        return Err(SemanticCbfReject::SemanticInadmissible {
            net_dissipation,
            tolerance,
        });
    }
    let entropy_joules = net_dissipation.max(0.0);
    cbf.thermodynamic
        .verify_and_deduct_update(entropy_joules, bits_resolved)
        .map_err(SemanticCbfReject::Thermodynamic)
}

/// Bundled-lock convenience for tests and gateway stubs.
pub fn gate_semantic_hot_bundled(
    cbf: &mut SemanticCBF,
    net_dissipation: f64,
    bits_resolved: f64,
    tolerance: f64,
) -> Result<f64, SemanticCbfReject> {
    let lock = CatalogLock::from_bundled().map_err(|_| SemanticCbfReject::MissingColdWitness {
        witness_id: cbf.cold_witness_id.clone(),
        catalog_id: SEMANTIC_CBF_CATALOG_ID,
    })?;
    gate_semantic_hot(cbf, &lock, net_dissipation, bits_resolved, tolerance, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::error_boundary::CbfReject;
    use approx::assert_relative_eq;

    const TOLERANCE: f64 = 1e-6;
    const TEMP_K: f64 = 300.0;
    const CREDIT_J: f64 = 1.0e-6;

    fn bundled_lock() -> CatalogLock {
        CatalogLock::from_bundled().expect("bundled lock parses")
    }

    fn default_proof(lock: &CatalogLock) -> &SemanticColdProof {
        hot_gate_lookup_cold_witness(lock, DEFAULT_SEMANTIC_COLD_WITNESS_ID)
            .expect("default semantic second law witness")
    }

    #[test]
    fn hot_gate_rejects_without_witness() {
        let mut cbf = SemanticCBF::new(TEMP_K, 1.0, "umst.formal.nonexistent_proof");
        let lock = bundled_lock();
        let err = gate_semantic_hot(&mut cbf, &lock, 0.0, 0.0, TOLERANCE, None)
            .expect_err("must reject when cold witness absent");
        assert!(matches!(err, SemanticCbfReject::MissingColdWitness { .. }));
        assert_eq!(err.catalog_id(), SEMANTIC_CBF_CATALOG_ID);
    }

    #[test]
    fn hot_gate_accepts_with_default_witness() {
        let mut cbf = SemanticCBF::chair_fixture();
        let outcome = gate_semantic_hot_bundled(&mut cbf, 0.0, 0.0, TOLERANCE)
            .expect("consistent semantic margin with catalog witness");
        assert!(outcome >= 0.0);
    }

    #[test]
    fn hot_gate_rejects_inadmissible_margin() {
        let mut cbf = SemanticCBF::chair_fixture();
        let err = gate_semantic_hot_bundled(&mut cbf, -2.0, 0.0, TOLERANCE)
            .expect_err("negative net dissipation must reject");
        assert!(matches!(err, SemanticCbfReject::SemanticInadmissible { .. }));
    }

    #[test]
    fn digest_mismatch_rejects() {
        let lock = bundled_lock();
        let proof = default_proof(&lock);
        let err = verify_cold_witness_digest(proof, "0".repeat(64).as_str())
            .expect_err("wrong digest");
        assert!(matches!(err, SemanticCbfReject::DigestMismatch { .. }));
    }

    #[test]
    fn semantic_cbf_new_and_chair_fixture_fields() {
        let custom = SemanticCBF::new(TEMP_K, CREDIT_J, DEFAULT_SEMANTIC_COLD_WITNESS_ID);
        assert_relative_eq!(custom.thermodynamic.temperature_k, TEMP_K, epsilon = 1.0e-30);
        assert_relative_eq!(
            custom.thermodynamic.available_credit_joules,
            CREDIT_J,
            epsilon = 1.0e-30
        );
        assert_eq!(custom.cold_witness_id, DEFAULT_SEMANTIC_COLD_WITNESS_ID);

        let chair = SemanticCBF::chair_fixture();
        assert_relative_eq!(chair.thermodynamic.temperature_k, TEMP_K, epsilon = 1.0e-30);
        assert_relative_eq!(
            chair.thermodynamic.available_credit_joules,
            1.0e-6,
            epsilon = 1.0e-30
        );
        assert_eq!(chair.cold_witness_id, DEFAULT_SEMANTIC_COLD_WITNESS_ID);
    }

    #[test]
    fn hot_gate_lookup_default_witness_from_bundled_lock() {
        let lock = bundled_lock();
        let proof = default_proof(&lock);
        assert_eq!(proof.witness_id, DEFAULT_SEMANTIC_COLD_WITNESS_ID);
        assert_eq!(proof.catalog_id, SEMANTIC_CBF_CATALOG_ID);
        assert_eq!(proof.proof_kind, "cold");
        assert_eq!(proof.digest_hex.len(), 64);
    }

    #[test]
    fn verify_cold_witness_digest_accepts_case_insensitive_match() {
        let lock = bundled_lock();
        let proof = default_proof(&lock);
        let upper = proof.digest_hex.to_ascii_uppercase();
        verify_cold_witness_digest(proof, &upper).expect("digest pin must be case-insensitive");
    }

    #[test]
    fn gate_semantic_hot_accepts_with_matching_digest_pin() {
        let lock = bundled_lock();
        let proof = default_proof(&lock);
        let mut cbf = SemanticCBF::chair_fixture();
        gate_semantic_hot(
            &mut cbf,
            &lock,
            0.0,
            0.0,
            TOLERANCE,
            Some(&proof.digest_hex),
        )
        .expect("matching digest pin must admit");
    }

    #[test]
    fn gate_semantic_hot_rejects_digest_before_semantic_margin() {
        let lock = bundled_lock();
        let mut cbf = SemanticCBF::chair_fixture();
        let err = gate_semantic_hot(
            &mut cbf,
            &lock,
            -2.0,
            0.0,
            TOLERANCE,
            Some("f".repeat(64).as_str()),
        )
        .expect_err("digest mismatch must short-circuit before semantic margin");
        assert!(matches!(err, SemanticCbfReject::DigestMismatch { .. }));
    }

    #[test]
    fn semantic_margin_boundary_at_negative_tolerance_is_admissible() {
        let mut cbf = SemanticCBF::chair_fixture();
        gate_semantic_hot_bundled(&mut cbf, -TOLERANCE, 0.0, TOLERANCE)
            .expect("net dissipation exactly at -tolerance must admit");
    }

    #[test]
    fn semantic_margin_just_below_tolerance_rejects() {
        let mut cbf = SemanticCBF::chair_fixture();
        let err = gate_semantic_hot_bundled(&mut cbf, -(TOLERANCE + 1.0e-12), 0.0, TOLERANCE)
            .expect_err("net dissipation below -tolerance must reject");
        match err {
            SemanticCbfReject::SemanticInadmissible {
                net_dissipation,
                tolerance,
            } => {
                assert!(net_dissipation < -tolerance);
            }
            other => panic!("expected SemanticInadmissible, got {other:?}"),
        }
    }

    #[test]
    fn positive_net_dissipation_feeds_thermodynamic_entropy_branch() {
        let mut cbf = SemanticCBF::new(TEMP_K, CREDIT_J, DEFAULT_SEMANTIC_COLD_WITNESS_ID);
        let dissipation = 1.0e-12;
        let bits = 0.0;
        let erasure = cbf.thermodynamic.calculate_landauer_cost(bits);
        let cost = gate_semantic_hot_bundled(&mut cbf, dissipation, bits, TOLERANCE)
            .expect("positive semantic margin with zero bits must admit");
        assert_relative_eq!(cost, erasure, epsilon = 1.0e-30, max_relative = 1.0e-9);
        assert_relative_eq!(
            cbf.thermodynamic.available_credit_joules,
            CREDIT_J - erasure,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
    }

    #[test]
    fn negative_margin_within_tolerance_clamps_entropy_to_zero() {
        let mut cbf = SemanticCBF::new(TEMP_K, CREDIT_J, DEFAULT_SEMANTIC_COLD_WITNESS_ID);
        let bits = 0.0;
        let credit_before = cbf.thermodynamic.available_credit_joules;
        gate_semantic_hot_bundled(&mut cbf, -TOLERANCE * 0.5, bits, TOLERANCE)
            .expect("sub-tolerance negative margin clamps to zero entropy");
        assert_relative_eq!(
            cbf.thermodynamic.available_credit_joules,
            credit_before,
            epsilon = 1.0e-30
        );
    }

    #[test]
    fn gate_semantic_hot_propagates_insufficient_global_energy_credit() {
        let lock = bundled_lock();
        let mut cbf = SemanticCBF::new(TEMP_K, 0.0, DEFAULT_SEMANTIC_COLD_WITNESS_ID);
        let err = gate_semantic_hot(&mut cbf, &lock, 0.0, 1.0, TOLERANCE, None)
            .expect_err("zero credit must reject positive bit resolution");
        match err {
            SemanticCbfReject::Thermodynamic(CbfReject::InsufficientGlobalEnergyCredit { .. }) => {}
            other => panic!("expected thermodynamic insufficient credit, got {other:?}"),
        }
    }

    #[test]
    fn gate_semantic_hot_propagates_clausius_duhem_violation() {
        let lock = bundled_lock();
        let mut cbf = SemanticCBF::new(TEMP_K, CREDIT_J, DEFAULT_SEMANTIC_COLD_WITNESS_ID);
        let bits = 1.0;
        let err = gate_semantic_hot(&mut cbf, &lock, 0.0, bits, TOLERANCE, None)
            .expect_err("zero entropy with positive bits must violate CD");
        match err {
            SemanticCbfReject::Thermodynamic(CbfReject::ClausiusDuhemViolation { .. }) => {}
            other => panic!("expected thermodynamic CD violation, got {other:?}"),
        }
    }

    #[test]
    fn gate_semantic_hot_bundled_sequential_debits_accumulate() {
        let mut cbf = SemanticCBF::chair_fixture();
        let bits = 0.25;
        let erasure = cbf.thermodynamic.calculate_landauer_cost(bits);
        gate_semantic_hot_bundled(&mut cbf, erasure, bits, TOLERANCE)
            .expect("first admissible step on CD boundary");
        let credit_after_first = cbf.thermodynamic.available_credit_joules;
        gate_semantic_hot_bundled(&mut cbf, erasure, bits, TOLERANCE)
            .expect("second admissible step on CD boundary");
        assert_relative_eq!(
            credit_after_first,
            1.0e-6 - erasure,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
        assert_relative_eq!(
            cbf.thermodynamic.available_credit_joules,
            1.0e-6 - 2.0 * erasure,
            epsilon = 1.0e-30,
            max_relative = 1.0e-9
        );
    }

    #[test]
    fn semantic_cbf_reject_display_formats_all_variants() {
        let missing = SemanticCbfReject::MissingColdWitness {
            witness_id: "umst.formal.missing".to_string(),
            catalog_id: SEMANTIC_CBF_CATALOG_ID,
        };
        assert!(missing.to_string().contains("missing cold semantic witness"));
        assert!(missing.to_string().contains("umst.formal.missing"));

        let digest = SemanticCbfReject::DigestMismatch {
            witness_id: DEFAULT_SEMANTIC_COLD_WITNESS_ID.to_string(),
            expected_hex: "aa".repeat(64),
            cited_hex: "bb".repeat(64),
        };
        assert!(digest.to_string().contains("digest mismatch"));
        assert!(digest.to_string().contains(SEMANTIC_CBF_CATALOG_ID));

        let thermo = SemanticCbfReject::Thermodynamic(CbfReject::ClausiusDuhemViolation {
            generalized_entropy: -0.01,
        });
        assert!(thermo.to_string().contains("Clausius-Duhem"));

        let inadmissible = SemanticCbfReject::SemanticInadmissible {
            net_dissipation: -1.0,
            tolerance: TOLERANCE,
        };
        assert!(inadmissible.to_string().contains("semantic net dissipation"));
    }

    #[test]
    fn semantic_cbf_reject_catalog_id_is_stable() {
        let variants = [
            SemanticCbfReject::MissingColdWitness {
                witness_id: "x".to_string(),
                catalog_id: SEMANTIC_CBF_CATALOG_ID,
            },
            SemanticCbfReject::DigestMismatch {
                witness_id: "x".to_string(),
                expected_hex: "aa".repeat(64),
                cited_hex: "bb".repeat(64),
            },
            SemanticCbfReject::Thermodynamic(CbfReject::LegacyDetail {
                detail: "legacy".to_string(),
            }),
            SemanticCbfReject::SemanticInadmissible {
                net_dissipation: -1.0,
                tolerance: TOLERANCE,
            },
        ];
        for variant in variants {
            assert_eq!(variant.catalog_id(), SEMANTIC_CBF_CATALOG_ID);
        }
    }

    #[test]
    fn bundled_semantic_witness_exports_match_catalog_section() {
        assert!(bundled_semantic_witness_section_present());
        let lock = bundled_lock();
        let section = lock
            .semantic_witnesses
            .as_ref()
            .expect("semantic_witnesses section");
        assert!(semantic_witness_section_quickcheck(section));
        assert_eq!(
            section.policy_version,
            SEMANTIC_COLD_HOT_POLICY_VERSION
        );
    }

    #[test]
    fn w8e14_semantic_cbf_tolerance_is_positive_finite() {
        assert!(TOLERANCE > 0.0);
        assert!(TOLERANCE.is_finite());
    }
}
