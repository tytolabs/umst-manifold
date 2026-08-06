// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
//
// AGAP-2033-MANIFOLD-SEM — semantic lane residuals on WEB-005 constitutive cartridge.
// W29-133 deepen — honest fences; fallible row extract; no invent GREEN / PRODUCTION_WIRED /
// MASTER / OP-5. HCOM-008 DEC composition remains stub-open.
//
// Bridges HCOM-006 semantic lanes (carrier indices 57..64) with the WEB-004 64D tensor:
// index 56 is web-only (UCRS head); 57..63 align with [`SemanticLaneBundleV1`].

use umst_gate::{ConjunctVerdict, GateRejectReason};

use crate::ai::semantic_evolution_bridge::{mi_deficit_from_bits, CHAIR_I_REQUIRED_BITS};
use crate::core::semantic_lane_schema::{
    consistency_defect_from_dec_stub, try_consistency_defect_from_dec_stub, SemanticLaneBundleV1,
    SemanticLaneSchemaError, DEC_GRAPH_CONSISTENCY_HOOK_V1_STUB, LANE_MI_VALUE,
    LANE_RELATION_GRAPH, LANE_TOPOLOGY_SIGNATURE, SEMANTIC_LANE_BASE, UMST_CARRIER_LANE_COUNT,
};

use super::slice_layout;

/// W29 deepen cell — semantic residual honest fence bundle.
pub const W29_SEMANTIC_RESIDUAL_DEEPEN_CELL: &str = "W29-133-SEMANTIC_RESIDUAL";

/// Hook revision for semantic residual conjuncts (audit trail).
pub const SEMANTIC_RESIDUAL_HOOK_V1: &str = "web-005-semantic-residual-v1";

/// Default tolerance on DEC/graph consistency defect scalar.
pub const DEFAULT_SEMANTIC_DEFECT_TOLERANCE: f64 = 1e-6;

/// Honest posture tag — WEB-005 residual bridge; HCOM-008 DEC remains stub.
pub const SEMANTIC_RESIDUAL_POSTURE_TAG: &str =
    "honest-web-005-semantic-residual-stub-dec-not-production";

/// Residual surface landed (extract + conjuncts + overlap invariant) — not fleet claim.
pub const SEMANTIC_RESIDUAL_SURFACE_LANDED: bool = true;

/// DEC path still routes through HCOM-006 stub (HCOM-008 open).
pub const SEMANTIC_RESIDUAL_DEC_HOOK_STUB: bool = true;

/// Honest refusal — residual bridge is staging surface, not production-wired.
pub const SEMANTIC_RESIDUAL_PRODUCTION_WIRED: bool = false;

/// Honest refusal — residual conjuncts ≠ physics GREEN / oracle certification.
pub const SEMANTIC_RESIDUAL_PHYSICS_GREEN: bool = false;

/// Honest refusal — no MASTER / fleet-complete posture at residual seam.
pub const SEMANTIC_RESIDUAL_MASTER: bool = false;

/// Honest refusal — production flip not authorized while DEC is stub.
pub const SEMANTIC_RESIDUAL_FLIP_AUTHORIZED: bool = false;

/// Honest refusal — no OP-5 / fleet-master claim at residual seam.
pub const SEMANTIC_RESIDUAL_OP5_CLAIMED: bool = false;

/// Fence facet inventory size (wired + deferred).
pub const SEMANTIC_RESIDUAL_FENCE_FACET_COUNT: usize = 8;

/// Fence facets measured/wired today (surface + overlap + stub DEC + open HCOM-008).
pub const SEMANTIC_RESIDUAL_FENCE_WIRED_COUNT: usize = 4;

/// Deferred / refused fence facets (facet_count − wired_count).
pub const SEMANTIC_RESIDUAL_FENCE_REFUSED_COUNT: usize =
    SEMANTIC_RESIDUAL_FENCE_FACET_COUNT - SEMANTIC_RESIDUAL_FENCE_WIRED_COUNT;

/// Operator-visible honesty string — does **not** authorize production flip, physics GREEN,
/// MASTER, or OP-5.
pub const SEMANTIC_RESIDUAL_HONEST_FENCE: &str =
    "residual_surface_landed=true|web_semantic_overlap=true|dec_hook_stub=true|hcom008_open=true|production_wired=false|physics_green=false|master=false|flip_authorized=false|op5_claimed=false";

const _: () = assert!(!SEMANTIC_RESIDUAL_PRODUCTION_WIRED);
const _: () = assert!(!SEMANTIC_RESIDUAL_PHYSICS_GREEN);
const _: () = assert!(!SEMANTIC_RESIDUAL_MASTER);
const _: () = assert!(!SEMANTIC_RESIDUAL_FLIP_AUTHORIZED);
const _: () = assert!(!SEMANTIC_RESIDUAL_OP5_CLAIMED);
const _: () = assert!(SEMANTIC_RESIDUAL_SURFACE_LANDED);
const _: () = assert!(SEMANTIC_RESIDUAL_DEC_HOOK_STUB);
const _: () = assert!(
    SEMANTIC_RESIDUAL_FENCE_REFUSED_COUNT
        == SEMANTIC_RESIDUAL_FENCE_FACET_COUNT - SEMANTIC_RESIDUAL_FENCE_WIRED_COUNT
);

/// One facet of the semantic-residual production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticResidualFenceFacet {
    pub facet: &'static str,
    pub wired: bool,
    pub owning_slice: &'static str,
}

/// Semantic-residual production fence facet inventory (honest posture SSOT).
pub const SEMANTIC_RESIDUAL_FENCE_FACETS: &[SemanticResidualFenceFacet] = &[
    SemanticResidualFenceFacet {
        facet: "residual_surface_landed",
        wired: true,
        owning_slice: W29_SEMANTIC_RESIDUAL_DEEPEN_CELL,
    },
    SemanticResidualFenceFacet {
        facet: "web_semantic_overlap",
        wired: true,
        owning_slice: W29_SEMANTIC_RESIDUAL_DEEPEN_CELL,
    },
    SemanticResidualFenceFacet {
        facet: "dec_hook_stub",
        wired: true,
        owning_slice: W29_SEMANTIC_RESIDUAL_DEEPEN_CELL,
    },
    SemanticResidualFenceFacet {
        facet: "hcom008_composition_open",
        wired: true,
        owning_slice: "hcom-008-deferred",
    },
    SemanticResidualFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
    SemanticResidualFenceFacet {
        facet: "physics_green",
        wired: false,
        owning_slice: "deferred-physics-oracle",
    },
    SemanticResidualFenceFacet {
        facet: "master",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
    SemanticResidualFenceFacet {
        facet: "op5_claimed",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
];

/// Typed probe for semantic residual posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticResidualPostureProbe {
    pub deepen_cell: &'static str,
    pub posture_tag: &'static str,
    pub hook_revision: &'static str,
    pub dec_hook_revision: &'static str,
    pub residual_surface_landed: bool,
    pub web_semantic_overlap_ok: bool,
    pub dec_hook_is_stub: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub fence_refused_count: usize,
    pub production_wired: bool,
    pub physics_green: bool,
    pub master: bool,
    pub flip_authorized: bool,
    pub op5_claimed: bool,
    pub honest_fence: &'static str,
}

/// Semantic residual legs extracted from a 64D carrier / web tensor row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WebSemanticResidual {
    /// HCOM-006 v1 semantic lane bundle (indices 57..64).
    pub bundle: SemanticLaneBundleV1,
    /// DEC/graph consistency defect (HCOM-006 stub → HCOM-008 deepens).
    pub dec_defect: f64,
    /// MI deficit against chair fixture `i_required` bits.
    pub mi_deficit: f64,
}

impl WebSemanticResidual {
    /// Neutral fixture — unset semantic lanes, zero defects.
    #[must_use]
    pub const fn neutral() -> Self {
        Self {
            bundle: SemanticLaneBundleV1 {
                concept_id: 0.0,
                relation_graph: 0.0,
                context_vector: 0.0,
                timestamp: 0.0,
                speaker_id: 0.0,
                mi_value: 0.0,
                topology_signature: 0.0,
            },
            dec_defect: 0.0,
            mi_deficit: 0.0,
        }
    }

    /// Whether DEC defect is within tolerance.
    #[must_use]
    pub fn dec_defect_within(self, tolerance: f64) -> bool {
        self.dec_defect <= tolerance
    }

    /// Whether MI witness improved or held (non-increasing deficit).
    #[must_use]
    pub fn mi_monotone_vs(self, prior: Self) -> bool {
        self.mi_deficit <= prior.mi_deficit + f64::EPSILON
    }

    /// Strict MI improvement vs prior (deficit decreased by more than epsilon).
    #[must_use]
    pub fn mi_improved_vs(self, prior: Self) -> bool {
        self.mi_deficit + f64::EPSILON < prior.mi_deficit
    }

    /// Whether the residual is the neutral unset fixture (zero lanes, zero DEC).
    #[must_use]
    pub fn is_unset_neutral(self) -> bool {
        self.dec_defect == 0.0
            && self.bundle.concept_id == 0.0
            && self.bundle.relation_graph == 0.0
            && self.bundle.context_vector == 0.0
            && self.bundle.timestamp == 0.0
            && self.bundle.speaker_id == 0.0
            && self.bundle.mi_value == 0.0
            && self.bundle.topology_signature == 0.0
    }
}

/// Transition witness for semantic residual conjuncts (old → new tensor row).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WebSemanticTransitionWitness {
    pub old: WebSemanticResidual,
    pub new: WebSemanticResidual,
}

impl WebSemanticTransitionWitness {
    /// MI deficit delta (new − old); negative ⇒ improvement.
    #[must_use]
    pub fn mi_deficit_delta(self) -> f64 {
        self.new.mi_deficit - self.old.mi_deficit
    }

    /// DEC defect delta (new − old).
    #[must_use]
    pub fn dec_defect_delta(self) -> f64 {
        self.new.dec_defect - self.old.dec_defect
    }
}

/// Outcome of semantic residual conjunct evaluation (orthogonal to web + Core).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WebSemanticGateOutcome {
    pub verdict: ConjunctVerdict,
    pub dec_defect_ok: bool,
    pub mi_monotone_ok: bool,
}

impl WebSemanticGateOutcome {
    #[must_use]
    pub fn is_accepted(self) -> bool {
        self.verdict.is_accepted()
    }

    /// Reject reason when not accepted; `None` on accept.
    #[must_use]
    pub fn reject_reason(self) -> Option<GateRejectReason> {
        match self.verdict {
            ConjunctVerdict::Accepted => None,
            ConjunctVerdict::Rejected(reason) => Some(reason),
        }
    }
}

/// Honest production wiring — **false** until HCOM-008 + fleet measured eval.
#[must_use]
pub const fn semantic_residual_production_wired() -> bool {
    SEMANTIC_RESIDUAL_PRODUCTION_WIRED
}

/// Honest physics GREEN claim — **false** (residual ≠ oracle).
#[must_use]
pub const fn semantic_residual_physics_green() -> bool {
    SEMANTIC_RESIDUAL_PHYSICS_GREEN
}

/// Honest master-tier claim — **false** until orchestrator pin.
#[must_use]
pub const fn semantic_residual_master() -> bool {
    SEMANTIC_RESIDUAL_MASTER
}

/// Honest flip authorization — **false** while DEC hook is stub.
#[must_use]
pub const fn semantic_residual_flip_authorized() -> bool {
    SEMANTIC_RESIDUAL_FLIP_AUTHORIZED
}

/// Honest OP-5 claim — **false** (residual surface ≠ OP-5 fleet pin).
#[must_use]
pub const fn semantic_residual_op5_claimed() -> bool {
    SEMANTIC_RESIDUAL_OP5_CLAIMED
}

/// Count wired fence facets from the inventory table.
#[must_use]
pub fn semantic_residual_fence_wired_count() -> usize {
    SEMANTIC_RESIDUAL_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Measured honest-posture snapshot for semantic residual.
#[must_use]
pub fn semantic_residual_posture_probe() -> SemanticResidualPostureProbe {
    SemanticResidualPostureProbe {
        deepen_cell: W29_SEMANTIC_RESIDUAL_DEEPEN_CELL,
        posture_tag: SEMANTIC_RESIDUAL_POSTURE_TAG,
        hook_revision: SEMANTIC_RESIDUAL_HOOK_V1,
        dec_hook_revision: DEC_GRAPH_CONSISTENCY_HOOK_V1_STUB,
        residual_surface_landed: SEMANTIC_RESIDUAL_SURFACE_LANDED,
        web_semantic_overlap_ok: web_semantic_lane_overlap_valid(),
        dec_hook_is_stub: SEMANTIC_RESIDUAL_DEC_HOOK_STUB
            && DEC_GRAPH_CONSISTENCY_HOOK_V1_STUB.contains("stub"),
        fence_facet_count: SEMANTIC_RESIDUAL_FENCE_FACET_COUNT,
        fence_wired_count: SEMANTIC_RESIDUAL_FENCE_WIRED_COUNT,
        fence_refused_count: SEMANTIC_RESIDUAL_FENCE_REFUSED_COUNT,
        production_wired: semantic_residual_production_wired(),
        physics_green: semantic_residual_physics_green(),
        master: semantic_residual_master(),
        flip_authorized: semantic_residual_flip_authorized(),
        op5_claimed: semantic_residual_op5_claimed(),
        honest_fence: SEMANTIC_RESIDUAL_HONEST_FENCE,
    }
}

/// Honesty gate — refuse fake production / GREEN / MASTER / OP-5 claims.
#[must_use]
pub fn semantic_residual_posture_honest(probe: &SemanticResidualPostureProbe) -> bool {
    probe.deepen_cell == W29_SEMANTIC_RESIDUAL_DEEPEN_CELL
        && probe.posture_tag == SEMANTIC_RESIDUAL_POSTURE_TAG
        && probe.hook_revision == SEMANTIC_RESIDUAL_HOOK_V1
        && probe.dec_hook_revision == DEC_GRAPH_CONSISTENCY_HOOK_V1_STUB
        && probe.residual_surface_landed
        && probe.web_semantic_overlap_ok
        && probe.dec_hook_is_stub
        && probe.fence_facet_count == SEMANTIC_RESIDUAL_FENCE_FACET_COUNT
        && probe.fence_wired_count == SEMANTIC_RESIDUAL_FENCE_WIRED_COUNT
        && probe.fence_refused_count == SEMANTIC_RESIDUAL_FENCE_REFUSED_COUNT
        && semantic_residual_fence_wired_count() == SEMANTIC_RESIDUAL_FENCE_WIRED_COUNT
        && !probe.production_wired
        && !probe.physics_green
        && !probe.master
        && !probe.flip_authorized
        && !probe.op5_claimed
        && probe.honest_fence.contains("residual_surface_landed=true")
        && probe.honest_fence.contains("dec_hook_stub=true")
        && probe.honest_fence.contains("hcom008_open=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("op5_claimed=false")
}

/// Fail closed on invented GREEN / PRODUCTION_WIRED / MASTER / OP-5.
pub fn validate_semantic_residual_honesty() -> Result<(), &'static str> {
    let probe = semantic_residual_posture_probe();
    if probe.physics_green {
        return Err("SEMANTIC_RESIDUAL_PHYSICS_GREEN must stay false until fleet physics closes");
    }
    if probe.production_wired {
        return Err("SEMANTIC_RESIDUAL_PRODUCTION_WIRED must stay false until HCOM-008 closes");
    }
    if probe.master {
        return Err("SEMANTIC_RESIDUAL_MASTER must stay false until orchestrator pin");
    }
    if probe.flip_authorized {
        return Err("SEMANTIC_RESIDUAL_FLIP_AUTHORIZED must stay false while DEC is stub");
    }
    if probe.op5_claimed {
        return Err("SEMANTIC_RESIDUAL_OP5_CLAIMED must stay false — not claimed by residual alone");
    }
    if !semantic_residual_posture_honest(&probe) {
        return Err("semantic residual posture fence inconsistent");
    }
    Ok(())
}

/// Extract semantic residual from a 64D web / carrier row.
///
/// Requires `row.len() >= UMST_CARRIER_LANE_COUNT`; shorter rows yield neutral residual.
/// Prefer [`try_residual_from_row`] at honesty-gated call sites.
#[must_use]
pub fn residual_from_row(row: &[f64]) -> WebSemanticResidual {
    try_residual_from_row(row).unwrap_or_else(|_| WebSemanticResidual::neutral())
}

/// Fallible residual extract — refuses short carrier rows (honest width gate).
pub fn try_residual_from_row(row: &[f64]) -> Result<WebSemanticResidual, SemanticLaneSchemaError> {
    let bundle = SemanticLaneBundleV1::try_read_from_row(row)?;
    let dec_defect = try_consistency_defect_from_dec_stub(row)?;
    let mi_deficit = mi_deficit_from_bits(CHAIR_I_REQUIRED_BITS, bundle.mi_value);
    Ok(WebSemanticResidual {
        bundle,
        dec_defect,
        mi_deficit,
    })
}

/// Extract semantic residual from a fixed 64D web tensor.
#[must_use]
pub fn residual_from_web_tensor(tensor: &[f64; slice_layout::DIM]) -> WebSemanticResidual {
    residual_from_row(tensor)
}

/// Build a semantic transition witness from old/new 64D tensors.
#[must_use]
pub fn semantic_transition_witness_from_tensors(
    old: &[f64; slice_layout::DIM],
    new: &[f64; slice_layout::DIM],
) -> WebSemanticTransitionWitness {
    WebSemanticTransitionWitness {
        old: residual_from_web_tensor(old),
        new: residual_from_web_tensor(new),
    }
}

/// Fallible transition witness from arbitrary-length rows.
pub fn try_semantic_transition_witness_from_rows(
    old: &[f64],
    new: &[f64],
) -> Result<WebSemanticTransitionWitness, SemanticLaneSchemaError> {
    Ok(WebSemanticTransitionWitness {
        old: try_residual_from_row(old)?,
        new: try_residual_from_row(new)?,
    })
}

/// Evaluate semantic residual conjuncts on a transition witness.
///
/// Conjuncts: DEC defect ≤ tolerance ∧ MI deficit non-regression.
#[must_use]
pub fn evaluate_semantic_conjuncts(
    witness: &WebSemanticTransitionWitness,
    defect_tolerance: f64,
) -> WebSemanticGateOutcome {
    let dec_defect_ok = witness.new.dec_defect_within(defect_tolerance);
    let mi_monotone_ok = witness.new.mi_monotone_vs(witness.old);

    let verdict = if !dec_defect_ok {
        ConjunctVerdict::Rejected(GateRejectReason::MalformedInput)
    } else if !mi_monotone_ok {
        ConjunctVerdict::Rejected(GateRejectReason::NegativeDissipation)
    } else {
        ConjunctVerdict::Accepted
    };

    WebSemanticGateOutcome {
        verdict,
        dec_defect_ok,
        mi_monotone_ok,
    }
}

/// Documented overlap: web BEHAVIOR_UCRS[0] (index 56) is UCRS head; semantic band starts at 57.
#[must_use]
pub const fn web_semantic_lane_overlap_valid() -> bool {
    slice_layout::BEHAVIOR_UCRS.start + 1 == SEMANTIC_LANE_BASE
}

const _: () = assert!(web_semantic_lane_overlap_valid());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_invariant_links_web_behavior_to_semantic_base() {
        assert!(web_semantic_lane_overlap_valid());
        assert_eq!(slice_layout::BEHAVIOR_UCRS.start, 56);
        assert_eq!(SEMANTIC_LANE_BASE, 57);
    }

    #[test]
    fn residual_neutral_on_zero_tensor() {
        let tensor = [0.0_f64; slice_layout::DIM];
        let r = residual_from_web_tensor(&tensor);
        assert_eq!(r.dec_defect, 0.0);
        assert_eq!(r.mi_deficit, CHAIR_I_REQUIRED_BITS);
        assert!(r.is_unset_neutral() || r.mi_deficit == CHAIR_I_REQUIRED_BITS);
        // mi_value=0 ⇒ full chair deficit; lanes otherwise unset.
        assert_eq!(r.bundle.mi_value, 0.0);
        assert_eq!(r.bundle.relation_graph, 0.0);
    }

    #[test]
    fn short_row_silent_path_yields_neutral_fallible_refuses() {
        let short = [0.0_f64; 32];
        let silent = residual_from_row(&short);
        assert!(silent.is_unset_neutral());
        assert_eq!(
            try_residual_from_row(&short),
            Err(SemanticLaneSchemaError::RowWidthMismatch {
                expected: UMST_CARRIER_LANE_COUNT,
                found: 32,
            })
        );
        assert_eq!(
            try_semantic_transition_witness_from_rows(&short, &short),
            Err(SemanticLaneSchemaError::RowWidthMismatch {
                expected: UMST_CARRIER_LANE_COUNT,
                found: 32,
            })
        );
    }

    #[test]
    fn dec_defect_flags_relation_without_topology() {
        let mut tensor = [0.0_f64; slice_layout::DIM];
        tensor[LANE_RELATION_GRAPH] = 0.5;
        let r = residual_from_web_tensor(&tensor);
        assert!((r.dec_defect - 0.5).abs() < f64::EPSILON);
        let witness = WebSemanticTransitionWitness {
            old: WebSemanticResidual::neutral(),
            new: r,
        };
        let outcome = evaluate_semantic_conjuncts(&witness, DEFAULT_SEMANTIC_DEFECT_TOLERANCE);
        assert!(!outcome.is_accepted());
        assert!(!outcome.dec_defect_ok);
        assert_eq!(
            outcome.reject_reason(),
            Some(GateRejectReason::MalformedInput)
        );
    }

    #[test]
    fn mi_regression_rejects_semantic_conjunct() {
        let mut old_tensor = [0.0_f64; slice_layout::DIM];
        let mut new_tensor = [0.0_f64; slice_layout::DIM];
        old_tensor[LANE_MI_VALUE] = 5.0;
        new_tensor[LANE_MI_VALUE] = 2.0;
        let witness = semantic_transition_witness_from_tensors(&old_tensor, &new_tensor);
        assert!(witness.mi_deficit_delta() > 0.0);
        let outcome = evaluate_semantic_conjuncts(&witness, DEFAULT_SEMANTIC_DEFECT_TOLERANCE);
        assert!(!outcome.is_accepted());
        assert!(!outcome.mi_monotone_ok);
        assert_eq!(
            outcome.reject_reason(),
            Some(GateRejectReason::NegativeDissipation)
        );
    }

    #[test]
    fn mi_improvement_accepts_when_dec_clean() {
        let mut old_tensor = [0.0_f64; slice_layout::DIM];
        let mut new_tensor = [0.0_f64; slice_layout::DIM];
        old_tensor[LANE_MI_VALUE] = 2.0;
        new_tensor[LANE_MI_VALUE] = CHAIR_I_REQUIRED_BITS;
        new_tensor[LANE_TOPOLOGY_SIGNATURE] = 1.0;
        new_tensor[LANE_RELATION_GRAPH] = 0.2;
        let witness = semantic_transition_witness_from_tensors(&old_tensor, &new_tensor);
        assert!(witness.new.mi_improved_vs(witness.old));
        assert!(witness.mi_deficit_delta() < 0.0);
        let outcome = evaluate_semantic_conjuncts(&witness, DEFAULT_SEMANTIC_DEFECT_TOLERANCE);
        assert!(outcome.is_accepted());
        assert!(outcome.reject_reason().is_none());
    }

    #[test]
    fn consistent_bundle_accepts_semantic_conjunct() {
        let mut tensor = [0.0_f64; slice_layout::DIM];
        tensor[LANE_MI_VALUE] = CHAIR_I_REQUIRED_BITS;
        tensor[LANE_TOPOLOGY_SIGNATURE] = 1.0;
        tensor[LANE_RELATION_GRAPH] = 0.3;
        let r = try_residual_from_row(&tensor).expect("full-width residual");
        assert!((r.mi_deficit).abs() < f64::EPSILON);
        assert!((r.dec_defect).abs() < f64::EPSILON);
        let witness = WebSemanticTransitionWitness { old: r, new: r };
        assert_eq!(witness.mi_deficit_delta(), 0.0);
        assert_eq!(witness.dec_defect_delta(), 0.0);
        let outcome = evaluate_semantic_conjuncts(&witness, DEFAULT_SEMANTIC_DEFECT_TOLERANCE);
        assert!(outcome.is_accepted());
    }

    #[test]
    fn semantic_residual_posture_honest_not_green() {
        validate_semantic_residual_honesty().expect("honest fence");
        let probe = semantic_residual_posture_probe();
        assert!(semantic_residual_posture_honest(&probe));
        assert_eq!(probe.deepen_cell, W29_SEMANTIC_RESIDUAL_DEEPEN_CELL);
        assert!(probe.posture_tag.contains("honest"));
        assert!(probe.dec_hook_is_stub);
        assert!(!probe.production_wired);
        assert!(!probe.physics_green);
        assert!(!probe.master);
        assert!(!probe.flip_authorized);
        assert!(!probe.op5_claimed);
        assert!(probe.honest_fence.contains("production_wired=false"));
        assert!(probe.honest_fence.contains("physics_green=false"));
        assert!(probe.honest_fence.contains("master=false"));
        assert!(probe.honest_fence.contains("op5_claimed=false"));
        assert_eq!(
            semantic_residual_fence_wired_count(),
            SEMANTIC_RESIDUAL_FENCE_WIRED_COUNT
        );
        assert_eq!(
            SEMANTIC_RESIDUAL_FENCE_FACETS.len(),
            SEMANTIC_RESIDUAL_FENCE_FACET_COUNT
        );
        // Refuse overclaim strings that invent GREEN/PRODUCTION/MASTER/OP-5.
        assert!(!probe.honest_fence.contains("production_wired=true"));
        assert!(!probe.honest_fence.contains("physics_green=true"));
        assert!(!probe.honest_fence.contains("master=true"));
        assert!(!probe.honest_fence.contains("op5_claimed=true"));
    }

    #[test]
    fn refuse_overclaim_probe_mutation_fails_honesty() {
        let mut bad = semantic_residual_posture_probe();
        bad.production_wired = true;
        assert!(!semantic_residual_posture_honest(&bad));
        bad = semantic_residual_posture_probe();
        bad.physics_green = true;
        assert!(!semantic_residual_posture_honest(&bad));
        bad = semantic_residual_posture_probe();
        bad.master = true;
        assert!(!semantic_residual_posture_honest(&bad));
        bad = semantic_residual_posture_probe();
        bad.op5_claimed = true;
        assert!(!semantic_residual_posture_honest(&bad));
    }
}
