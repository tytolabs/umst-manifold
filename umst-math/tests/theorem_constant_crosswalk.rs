// SPDX-FileCopyrightText: 2018-2026 Studio TYTO
// SPDX-License-Identifier: LicenseRef-MIT

//! §14bis.h L-6 — theorem ↔ constant crosswalk adversarial witnesses (honest partial).

use umst_math::constants::registry::{ConstantTier, REGISTRY};
use umst_math::theorem_registry::{THEOREM_DERIVES_CONSTANT, THEOREM_REGISTRY};

fn registry_constant_names() -> std::collections::HashSet<&'static str> {
    REGISTRY.iter().map(|r| r.name).collect()
}

fn registry_theorem_hints() -> std::collections::HashSet<&'static str> {
    THEOREM_REGISTRY.iter().map(|(h, _)| *h).collect()
}

/// Tier-1/2 rows whose evidence cites `UMST.Formal…::theorem` (honest forward set).
fn theorem_derived_constant_rows() -> Vec<(&'static str, &'static str)> {
    REGISTRY
        .iter()
        .filter(|r| {
            matches!(
                r.tier,
                ConstantTier::Tier0Physical
                    | ConstantTier::Tier1Measurement
                    | ConstantTier::Tier2Derivable
            ) && r.evidence.contains("UMST.Formal")
                && r.evidence.contains("::")
        })
        .map(|r| (r.name, r.evidence))
        .collect()
}

#[test]
fn crosswalk_map_theorems_in_registry() {
    let hints = registry_theorem_hints();
    for (theorem, constant) in THEOREM_DERIVES_CONSTANT {
        assert!(
            hints.contains(theorem),
            "THEOREM_DERIVES_CONSTANT theorem not in THEOREM_REGISTRY: {theorem}"
        );
        let names = registry_constant_names();
        assert!(
            names.contains(constant),
            "THEOREM_DERIVES_CONSTANT constant not in REGISTRY: {constant}"
        );
    }
}

#[test]
fn crosswalk_partial_covers_known_rows() {
    let mapped: std::collections::HashSet<_> =
        THEOREM_DERIVES_CONSTANT.iter().map(|(_, c)| *c).collect();
    let derived = theorem_derived_constant_rows();
    assert!(
        !derived.is_empty(),
        "expected ≥1 Tier-1/2 row with UMST.Formal:: evidence"
    );
    let covered = derived
        .iter()
        .filter(|(name, _)| mapped.contains(name))
        .count();
    assert!(
        covered >= 3,
        "L-6 honest partial: expected ≥3 derived rows covered, got {covered}/{derived_len}",
        derived_len = derived.len()
    );
}

#[test]
fn crosswalk_not_claiming_full_green() {
    use umst_math::theorem_registry::crosswalk_stats;
    let stats = crosswalk_stats();
    assert!(
        stats.map_rows >= 5,
        "L-6 deepen expects ≥5 explicit THEOREM_DERIVES_CONSTANT rows"
    );
    assert!(
        stats.covered_derived_rows < stats.derived_constant_rows,
        "L-6 must stay partial until full crosswalk lands ({} / {})",
        stats.covered_derived_rows,
        stats.derived_constant_rows
    );
}

#[test]
fn crosswalk_gate_transition_tolerance_mapped() {
    use umst_math::theorem_registry::constant_for_theorem;
    assert_eq!(
        constant_for_theorem("UMST.Formal.Gate::transitionTolerance"),
        Some("transition_tolerance")
    );
}
