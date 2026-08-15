//! Compile-time registry of (theorem hint, DOI) pairs for cross-checks and telemetry.
//!
//! Canonical Zenodo family for quantum bridge rows: **10.5281/zenodo.19159660** (`umst-formal-double-slit`).
//!
//! **N3-FPD-c:** every `module::theorem` hint uses **double-colon** (`::`) — no slash (`/`) form.

/// `(module path hint, Zenodo DOI)` — human-facing; Lean names are canonical in source docs.
/// THEOREM-BOUND: `UMST.FormalDoubleSlit.QuantumClassicalBridge::complementarity_fringe_path` (§14bis.l W-3 G8)
pub const THEOREM_REGISTRY: &[(&str, &str)] = &[
    ("UMST.FormalDoubleSlit.QuantumClassicalBridge::complementarity_fringe_path", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.LandauerBound::principle_of_maximal_information_collapse", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.KleinInequality::spectralRelativeEntropynonneg", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.QuantumMutualInfo::I(A:B)_formula", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.TensorPartialTrace::tensorDensity", "10.5281/zenodo.19159660"),
    // Parity extension (Appendix J.5a) — mirrors `/// Proof:` citations across `umst-math` modules.
    ("UMST.FormalDoubleSlit.VonNeumannEntropy::vN_entropy_nonneg", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.DataProcessingInequality::vonNeumannEntropy_nondecreasing_unital_CPTP_n", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.ErasureChannel::idealResetErasure_saturates", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.EpistemicSensing::QuantumProbe", "10.5281/zenodo.19159660"),
    ("UMST.Formal.LandauerLaw::landauerBound", "10.5281/zenodo.19159660"),
    ("UMST.Formal.InfoTheory::product_joint_mass", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.LindbladDynamics::dephasingSolution_tendsto_diagonal", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.WhichPathMeasurementUpdate::measurementUpdateWhichPath", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.SchrodingerDynamics::unitary_channel", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.PMICVisibility::path_entropy_visibility", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.ExamplesQubit::qubit_zero", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.EpistemicMI::epistemicMIBits_nonneg", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.GeneralResidualCoherence::residual_coherence_capacity", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.InformationCostIdentity::residualCoherence_eq_one_minus_epistemic_bits", "10.5281/zenodo.19159660"),
    // Phase K1 — epistemic proxy selector (bind-only)
    ("UMST.Formal.Gate::gate_check", "10.5281/zenodo.19159660"),
    ("UMST.Formal.Gate::transitionTolerance", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.EpistemicMI::epistemicMIBits_le_one", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.EpistemicProxySelector::MI_weighted_rank", "10.5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.EpistemicPolicy::runtime_specialisation_hook", "10.5281/zenodo.19159660"),
    // Phase K2 — hypergraph + monoidal functor (bind-only)
    ("UMST.Formal.GraphProperties::hypergraph_incidence", "10.5281/zenodo.19159660"),
    ("UMST.Formal.GraphProperties::finite_edge_union", "10.5281/zenodo.19159660"),
    ("UMST.Formal.Naturality::functor_compose_vertex", "10.5281/zenodo.19159660"),
    ("UMST.Formal.Naturality::relabeling_coherent", "10.5281/zenodo.19159660"),
    ("UMST.Formal.MonoidalState::tensor_product_monoid", "10.5281/zenodo.19159660"),
    // Phase M4 — cockpit consumer: `cockpit credit module` (`record_contribution` / influence mass)
    ("UMST.Formal.CreditGreedy::credit_greedy_optimal", "10.5281/zenodo.19159660"),
    ("UMST.Formal.Dignity::dignity_monotone_under_mi_gain", "10.5281/zenodo.19159660"),
    ("UMST.Formal.EtaCog::eta_cog_nonneg", "10.5281/zenodo.19159660"),
    ("UMST.Formal.RhoEstimator::rho_based_mi_formula", "10.5281/zenodo.19159660"),
    ("UMST.Formal.MedianConvergence::median_convergence_sample_size", "10.5281/zenodo.19159660"),
    ("UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible", "10.5281/zenodo.19159660"),
    ("UMST.Formal.OrderStatisticsBand::order_statistic_concentration", "10.5281/zenodo.19159660"),
    ("UMST.Formal.OrderStatisticsBand::p25_p75_admissibility", "10.5281/zenodo.19159660"),
];

/// Maps `THEOREM_REGISTRY` hints to `constants::registry::REGISTRY` `name`s (§14bis.h L-6).
///
/// Honest partial map — grows as Tier-1/2 rows gain explicit `::theorem` evidence.
pub const THEOREM_DERIVES_CONSTANT: &[(&str, &str)] = &[
    (
        "UMST.Formal.RhoEstimator::rho_based_mi_formula",
        "closed_loop_mi_step_per_accept",
    ),
    (
        "UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible",
        "warmup_sample_threshold",
    ),
    (
        "UMST.Formal.OrderStatisticsBand::p25_p75_admissibility",
        "frugality_band_p25_percentile",
    ),
    (
        "UMST.Formal.OrderStatisticsBand::p25_p75_admissibility",
        "frugality_band_p75_percentile",
    ),
    (
        "UMST.Formal.Gate::transitionTolerance",
        "transition_tolerance",
    ),
];

/// Honest L-6 crosswalk coverage snapshot (operator / `:lcert` probe).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrosswalkStats {
    /// Rows in [`THEOREM_DERIVES_CONSTANT`].
    pub map_rows: usize,
    /// Distinct constant `name`s covered by the map.
    pub mapped_constants: usize,
    /// Tier-0..2 registry rows whose evidence cites `UMST.Formal…::theorem`.
    pub derived_constant_rows: usize,
    /// Derived rows with an explicit map entry.
    pub covered_derived_rows: usize,
}

/// Compute partial crosswalk coverage (never claims full GREEN).
#[must_use]
pub fn crosswalk_stats() -> CrosswalkStats {
    use crate::constants::registry::{ConstantTier, REGISTRY};
    use std::collections::HashSet;

    let mapped: HashSet<&str> = THEOREM_DERIVES_CONSTANT.iter().map(|(_, c)| *c).collect();
    let derived: Vec<&str> = REGISTRY
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
        .map(|r| r.name)
        .collect();
    let covered = derived.iter().filter(|n| mapped.contains(**n)).count();
    CrosswalkStats {
        map_rows: THEOREM_DERIVES_CONSTANT.len(),
        mapped_constants: mapped.len(),
        derived_constant_rows: derived.len(),
        covered_derived_rows: covered,
    }
}

/// Lookup constant `name` derived by a theorem hint (L-6 crosswalk helper).
#[must_use]
pub fn constant_for_theorem(theorem_hint: &str) -> Option<&'static str> {
    THEOREM_DERIVES_CONSTANT
        .iter()
        .find(|(t, _)| *t == theorem_hint)
        .map(|(_, c)| *c)
}

/// All theorem hints that derive a given constant `name`.
#[must_use]
pub fn theorems_for_constant(constant_name: &str) -> Vec<&'static str> {
    THEOREM_DERIVES_CONSTANT
        .iter()
        .filter(|(_, c)| *c == constant_name)
        .map(|(t, _)| *t)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{crosswalk_stats, THEOREM_DERIVES_CONSTANT, THEOREM_REGISTRY};

    #[test]
    fn crosswalk_stats_honest_partial() {
        let s = crosswalk_stats();
        assert_eq!(s.map_rows, THEOREM_DERIVES_CONSTANT.len());
        assert!(s.map_rows >= 5, "L-6 deepen expects ≥5 explicit map rows");
        assert!(s.covered_derived_rows >= 4);
        assert!(
            s.covered_derived_rows < s.derived_constant_rows,
            "L-6 stays partial until full :: derived census covered"
        );
    }

    #[test]
    fn registry_parity_minimum_rows() {
        assert!(
            THEOREM_REGISTRY.len() >= 36,
            "expected ≥36 registry rows (Phase FPD-OrderStatisticsBand+), got {}",
            THEOREM_REGISTRY.len()
        );
    }

    #[test]
    fn registry_hints_use_double_colon_form() {
        for (hint, _) in THEOREM_REGISTRY {
            assert!(
                hint.contains("::"),
                "registry hint must use `::` form, got {hint}"
            );
            assert!(
                !hint.contains('/'),
                "registry hint must not contain `/` path separator, got {hint}"
            );
        }
    }
}
