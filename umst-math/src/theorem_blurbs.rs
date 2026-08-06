//! Operator-readable one-line summaries for [`theorem_registry::THEOREM_REGISTRY`] rows.
//!
//! §14bis.h L-3 — `:explain` enrichment; NED §0.5 plain-English honesty (no invented proofs).

use crate::theorem_registry::THEOREM_REGISTRY;

/// L-3 coverage gate: ≥95% of registry rows must have a non-empty blurb.
pub const L3_COVERAGE_TARGET: f64 = 0.95;

/// `(theorem_id, one_line_blurb)` — keyed by full `UMST.Formal*::theorem` hint.
pub const THEOREM_BLURBS: &[(&str, &str)] = &[
    (
        "UMST.FormalDoubleSlit.QuantumClassicalBridge::complementarity_fringe_path",
        "Fringe visibility trades off with which-path information (complementarity).",
    ),
    (
        "UMST.FormalDoubleSlit.LandauerBound::principle_of_maximal_information_collapse",
        "Erasing one bit costs at least k_B T ln 2 joules at temperature T.",
    ),
    (
        "UMST.FormalDoubleSlit.KleinInequality::spectralRelativeEntropynonneg",
        "Spectral relative entropy is nonnegative on valid density operators.",
    ),
    (
        "UMST.FormalDoubleSlit.QuantumMutualInfo::I(A:B)_formula",
        "Quantum mutual information is S(ρ_A)+S(ρ_B)−S(ρ_AB) on bipartite states.",
    ),
    (
        "UMST.FormalDoubleSlit.TensorPartialTrace::tensorDensity",
        "Tensor product of marginals forms a valid joint density when factors commute.",
    ),
    (
        "UMST.FormalDoubleSlit.VonNeumannEntropy::vN_entropy_nonneg",
        "Von Neumann entropy S(ρ) is never negative for physical states.",
    ),
    (
        "UMST.FormalDoubleSlit.DataProcessingInequality::vonNeumannEntropy_nondecreasing_unital_CPTP_n",
        "Unital CPTP maps cannot decrease von Neumann entropy.",
    ),
    (
        "UMST.FormalDoubleSlit.ErasureChannel::idealResetErasure_saturates",
        "Ideal reset erasure saturates the Landauer cost per erased bit.",
    ),
    (
        "UMST.FormalDoubleSlit.EpistemicSensing::QuantumProbe",
        "Quantum probe model bounds extractable path information.",
    ),
    (
        "UMST.Formal.LandauerLaw::landauerBound",
        "Landauer floor: dissipation is at least k_B T ln 2 per erased bit.",
    ),
    (
        "UMST.Formal.InfoTheory::product_joint_mass",
        "Independent factors multiply into a valid joint probability mass.",
    ),
    (
        "UMST.FormalDoubleSlit.LindbladDynamics::dephasingSolution_tendsto_diagonal",
        "Pure dephasing drives off-diagonal coherences to zero.",
    ),
    (
        "UMST.FormalDoubleSlit.WhichPathMeasurementUpdate::measurementUpdateWhichPath",
        "Which-path measurement update collapses path superpositions.",
    ),
    (
        "UMST.FormalDoubleSlit.SchrodingerDynamics::unitary_channel",
        "Unitary evolution preserves trace and positivity of ρ.",
    ),
    (
        "UMST.FormalDoubleSlit.PMICVisibility::path_entropy_visibility",
        "Path entropy upper-bounds interference visibility.",
    ),
    (
        "UMST.FormalDoubleSlit.ExamplesQubit::qubit_zero",
        "Computational |0⟩ is a valid one-qubit density witness.",
    ),
    (
        "UMST.FormalDoubleSlit.EpistemicMI::epistemicMIBits_nonneg",
        "Epistemic mutual information in bits is nonnegative.",
    ),
    (
        "UMST.FormalDoubleSlit.GeneralResidualCoherence::residual_coherence_capacity",
        "Residual coherence capacity RCC lies in [0,1] for admissible states.",
    ),
    (
        "UMST.FormalDoubleSlit.InformationCostIdentity::residualCoherence_eq_one_minus_epistemic_bits",
        "RCC equals one minus epistemic MI bits under the bridge identity.",
    ),
    (
        "UMST.Formal.Gate::gate_check",
        "Thermo gate rejects transitions that violate admissibility margins.",
    ),
    (
        "UMST.FormalDoubleSlit.EpistemicMI::epistemicMIBits_le_one",
        "Epistemic MI cannot exceed one bit on the standard probe scale.",
    ),
    (
        "UMST.FormalDoubleSlit.EpistemicProxySelector::MI_weighted_rank",
        "MI-weighted ranking orders epistemic proxy candidates.",
    ),
    (
        "UMST.FormalDoubleSlit.EpistemicPolicy::runtime_specialisation_hook",
        "Runtime policy may specialise probes without breaking DPI bounds.",
    ),
    (
        "UMST.Formal.GraphProperties::hypergraph_incidence",
        "Hypergraph incidence matrix encodes edge–vertex membership.",
    ),
    (
        "UMST.Formal.GraphProperties::finite_edge_union",
        "Finite unions of hyperedges remain well-formed in the ledger.",
    ),
    (
        "UMST.Formal.Naturality::functor_compose_vertex",
        "Vertex relabeling commutes with functor composition on states.",
    ),
    (
        "UMST.Formal.Naturality::relabeling_coherent",
        "Relabeling preserves thermodynamic morphism coherence.",
    ),
    (
        "UMST.Formal.MonoidalState::tensor_product_monoid",
        "Tensor product of monoidal states is associative up to witness.",
    ),
    (
        "UMST.Formal.CreditGreedy::credit_greedy_optimal",
        "Greedy filter-then-sum is optimal for admissible credit mass.",
    ),
    (
        "UMST.Formal.Dignity::dignity_monotone_under_mi_gain",
        "Honest MI gain cannot decrease the dignity scalar on [0,10].",
    ),
    (
        "UMST.Formal.EtaCog::eta_cog_nonneg",
        "Cognitive efficiency η_cog is nonnegative when inputs are admissible.",
    ),
    (
        "UMST.Formal.RhoEstimator::rho_based_mi_formula",
        "Gaussian ρ̂ yields a conservative MI estimate per accept.",
    ),
    (
        "UMST.Formal.MedianConvergence::median_convergence_sample_size",
        "Median warmup sample size concentrates under admissible windows.",
    ),
    (
        "UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible",
        "Warmup threshold max(⌈√W⌉,3) is admissible for rolling windows.",
    ),
    (
        "UMST.Formal.OrderStatisticsBand::order_statistic_concentration",
        "Order statistics concentrate around quantile targets.",
    ),
    (
        "UMST.Formal.OrderStatisticsBand::p25_p75_admissibility",
        "Empirical P25/P75 bands are admissible frugality cutoffs.",
    ),
];

/// Plain-English blurb for a registry theorem hint, when present.
#[must_use]
pub fn blurb_for(theorem_id: &str) -> Option<&'static str> {
    THEOREM_BLURBS
        .iter()
        .find(|(id, _)| *id == theorem_id)
        .map(|(_, b)| *b)
}

/// Fraction of [`THEOREM_REGISTRY`] rows with a non-empty blurb entry.
#[must_use]
pub fn coverage_fraction() -> f64 {
    if THEOREM_REGISTRY.is_empty() {
        return 0.0;
    }
    let covered = THEOREM_REGISTRY
        .iter()
        .filter(|(id, _)| blurb_for(id).is_some())
        .count();
    covered as f64 / THEOREM_REGISTRY.len() as f64
}

/// L-3 gate: ≥95% registry rows have blurbs.
#[must_use]
pub fn l3_coverage_gate_passes() -> bool {
    coverage_fraction() + f64::EPSILON >= L3_COVERAGE_TARGET
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l3_blurb_coverage_meets_target() {
        assert!(
            l3_coverage_gate_passes(),
            "blurbs {:.1}% < {:.0}% target",
            coverage_fraction() * 100.0,
            L3_COVERAGE_TARGET * 100.0
        );
    }

    #[test]
    fn every_blurb_row_is_in_registry() {
        for (id, blurb) in THEOREM_BLURBS {
            assert!(
                THEOREM_REGISTRY.iter().any(|(h, _)| h == id),
                "blurb key not in THEOREM_REGISTRY: {id}"
            );
            assert!(!blurb.is_empty(), "empty blurb for {id}");
        }
    }
}
