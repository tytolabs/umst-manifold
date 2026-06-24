// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Kleisli gate pipeline composition sketch (`math-kleisli-gate-pipeline`).
//!
//! **propose → penalize (constraint_loss host mirror) → witness (GateCartridge)**
//!
//! Pure `f64` carriers and [`umst_manifold::gate::kleisli::Admissible`] composition —
//! Burn `constraint_loss` is the autodiff mirror of `penalize` (see `docs/KLEISLI_GATE_PIPELINE.md`).

use umst_manifold::gate::{
    kleisli_compose_pair, Admissible, KleisliArrow, ThermodynamicStateSnapshot,
    TransitionScalars, TRANSITION_TOLERANCE,
};
use umst_manifold::runtime::gate::{
    explain_cd_transition_host, AdmissibilityToken, CdTransitionCartridge, ConstraintExplanation,
    GateCartridge, TransitionEvidence,
};

/// Agent / policy intent stub (Warm parse target).
#[derive(Debug, Clone, Copy)]
struct TransitionIntent {
    pub old: ThermodynamicStateSnapshot,
    pub binder_liquid_ratio: f64,
    pub reaction_extent: f64,
    pub temperature_k: f64,
    pub dt: f64,
}

/// Propose-stage carrier: old/new thermodynamic snapshots + step width.
#[derive(Debug, Clone, Copy)]
struct TransitionPair {
    pub old: ThermodynamicStateSnapshot,
    pub new: ThermodynamicStateSnapshot,
    pub dt: f64,
}

/// Penalize-stage carrier: transition pair + [`ConstraintExplanation`] (constraint_loss semantics).
#[derive(Debug, Clone, Copy)]
struct PenalizedTransition {
    pub pair: TransitionPair,
    pub explanation: ConstraintExplanation,
}

fn propose(intent: TransitionIntent) -> Admissible<TransitionPair> {
    let scalars = TransitionScalars {
        binder_liquid_ratio: intent.binder_liquid_ratio,
        reaction_extent: intent.reaction_extent,
        temperature_k: intent.temperature_k,
        s_intrinsic_mpa: None,
    };
    let new = scalars.thermodynamic_snapshot();
    Admissible::pure(TransitionPair {
        old: intent.old,
        new,
        dt: intent.dt,
    })
}

fn penalize(pair: TransitionPair) -> Admissible<PenalizedTransition> {
    let explanation = explain_cd_transition_host(
        &pair.old,
        &pair.new,
        pair.dt,
        TRANSITION_TOLERANCE,
    );
    let admissible = explanation.admissibility == AdmissibilityToken::Admissible;
    Admissible {
        value: PenalizedTransition { pair, explanation },
        result: umst_manifold::gate::KleisliAdmissibilityResult {
            admissible,
            dissipation: explanation.violation,
            violation: if admissible {
                None
            } else {
                Some("constraint_loss_violation".into())
            },
        },
    }
}

fn witness(pen: PenalizedTransition) -> Admissible<TransitionEvidence> {
    let evidence = CdTransitionCartridge.transition_evidence(
        &pen.pair.old,
        &pen.pair.new,
        pen.pair.dt,
    );
    let admissible = evidence.admissibility == AdmissibilityToken::Admissible;
    Admissible {
        value: evidence,
        result: umst_manifold::gate::KleisliAdmissibilityResult {
            admissible,
            dissipation: pen.explanation.violation,
            violation: if admissible {
                None
            } else {
                Some("witness_inadmissible".into())
            },
        },
    }
}

fn penalize_then_witness(pair: TransitionPair) -> Admissible<TransitionEvidence> {
    penalize(pair).bind(witness)
}

fn pipeline() -> KleisliArrow<TransitionIntent, TransitionEvidence> {
    kleisli_compose_pair(propose, penalize_then_witness, "propose_penalize_witness")
}

#[test]
fn kleisli_gate_pipeline_admissible_composition() {
    let scalars = TransitionScalars {
        binder_liquid_ratio: 0.5,
        reaction_extent: 0.42,
        temperature_k: 293.15,
        s_intrinsic_mpa: Some(12.7),
    };
    let old = scalars.thermodynamic_snapshot();
    let intent = TransitionIntent {
        old,
        binder_liquid_ratio: scalars.binder_liquid_ratio,
        reaction_extent: scalars.reaction_extent,
        temperature_k: scalars.temperature_k,
        dt: 1.0,
    };

    let out = pipeline().run(intent);
    assert!(out.result.admissible, "reflexive propose → zero slack");
    assert_eq!(out.value.admissibility, AdmissibilityToken::Admissible);
    assert!(out.value.catalog_id.contains("cd_transition"));
}

#[test]
fn kleisli_gate_pipeline_inadmissible_short_circuits_penalize() {
    let old = ThermodynamicStateSnapshot {
        density: 2200.0,
        temperature: 300.0,
        free_energy: -2.0e5,
        entropy: 0.2,
        reaction_extent: 0.5,
        strength: 20.0,
    };
    let pair = TransitionPair {
        old,
        new: ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        },
        dt: 1.0,
    };

    let penalized = penalize(pair);
    assert!(!penalized.result.admissible);
    assert!(
        penalized.value.explanation.violation > 0.0,
        "ψ spike must incur positive constraint_loss slack"
    );

    let out = penalize_then_witness(pair);
    assert!(!out.result.admissible);
    assert_eq!(out.value.admissibility, AdmissibilityToken::Inadmissible);
}

#[test]
fn kleisli_gate_pipeline_penalize_witness_agree_on_token() {
    let old = ThermodynamicStateSnapshot {
        density: 2400.0,
        temperature: 293.15,
        free_energy: -1.35e5,
        entropy: 0.05,
        reaction_extent: 0.42,
        strength: 12.7,
    };
    let pair = TransitionPair {
        old,
        new: old,
        dt: 1.0,
    };

    let pen = penalize(pair);
    let wit = witness(pen.value);
    assert_eq!(
        pen.value.explanation.admissibility,
        wit.value.admissibility,
        "penalize explanation and GateCartridge witness must agree"
    );
}
