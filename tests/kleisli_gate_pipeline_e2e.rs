// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Kleisli gate pipeline end-to-end (`math-kleisli-e2e-test`, Wave 7 slot 4).
//!
//! **propose** ([`evaluate_transition_pure_with_params`]) → **penalize**
//! ([`umst_manifold::ai::constraint_loss::explain_clausius_duhem_violation`]) → **witness**
//! ([`CdTransitionCartridge::transition_evidence`]).
//!
//! Cold-path modules only at the witness edge; penalize uses the Burn host mirror with detached
//! scalars (see `docs/KLEISLI_GATE_PIPELINE.md`).

use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::ai::constraint_loss::{
    explain_clausius_duhem_violation, AdmissibilityToken, ConstraintExplanation,
};
use umst_manifold::core::material_transition::SubstrateMaterialParams;
use umst_manifold::gate::{
    evaluate_transition_pure_with_params, kleisli_compose_pair, AdmissibilityVerdict, Admissible,
    KleisliAdmissibilityResult, KleisliArrow, ThermodynamicStateSnapshot, TransitionScalars,
    TRANSITION_TOLERANCE,
};
use umst_manifold::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;
use umst_manifold::runtime::gate::{CdTransitionCartridge, GateCartridge, TransitionEvidence};

type B = NdArray<f32>;

/// Agent / policy intent: old/new scalar proposals + step width.
#[derive(Debug, Clone, Copy)]
struct TransitionIntent {
    pub old: TransitionScalars,
    pub new: TransitionScalars,
    pub dt: f64,
}

/// Propose-stage carrier: thermodynamic snapshot pair + step width.
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

fn scalar_tensor(dev: &NdArrayDevice, values: &[f32]) -> Tensor<B, 1> {
    let batch = values.len();
    Tensor::<B, 1>::from_data(Data::new(values.to_vec(), Shape::new([batch])), dev)
}

fn propose(intent: TransitionIntent) -> Admissible<TransitionPair> {
    let params = SubstrateMaterialParams;
    let old = intent.old.thermodynamic_snapshot_with_params(&params);
    let new = intent.new.thermodynamic_snapshot_with_params(&params);
    let outcome = evaluate_transition_pure_with_params(
        &intent.old,
        &intent.new,
        intent.dt,
        &params,
        TRANSITION_TOLERANCE,
    );
    let mass_ok = outcome.is_mass_conserved();
    Admissible {
        value: TransitionPair {
            old,
            new,
            dt: intent.dt,
        },
        result: KleisliAdmissibilityResult::from_verdict(
            if mass_ok {
                AdmissibilityVerdict::Accepted
            } else {
                AdmissibilityVerdict::MassViolation
            },
            outcome.dissipation as f32,
            if mass_ok {
                None
            } else {
                Some("propose_mass_violation".into())
            },
        ),
    }
}

fn penalize(pair: TransitionPair) -> Admissible<PenalizedTransition> {
    let dev = NdArrayDevice::default();
    let explanation = explain_clausius_duhem_violation(
        scalar_tensor(&dev, &[pair.old.density as f32]),
        scalar_tensor(&dev, &[pair.new.density as f32]),
        scalar_tensor(&dev, &[pair.old.free_energy as f32]),
        scalar_tensor(&dev, &[pair.new.free_energy as f32]),
        scalar_tensor(&dev, &[pair.dt as f32]),
    );
    let admissible = explanation.admissibility == AdmissibilityToken::Admissible;
    Admissible {
        value: PenalizedTransition { pair, explanation },
        result: KleisliAdmissibilityResult::from_verdict(
            if admissible {
                AdmissibilityVerdict::Accepted
            } else {
                AdmissibilityVerdict::Unknown
            },
            explanation.violation,
            if admissible {
                None
            } else {
                Some("constraint_loss_violation".into())
            },
        ),
    }
}

fn witness(pen: PenalizedTransition) -> Admissible<TransitionEvidence> {
    let evidence =
        CdTransitionCartridge.transition_evidence(&pen.pair.old, &pen.pair.new, pen.pair.dt);
    let admissible = evidence.admissibility == AdmissibilityToken::Admissible;
    Admissible {
        value: evidence,
        result: KleisliAdmissibilityResult::from_verdict(
            if admissible {
                AdmissibilityVerdict::Accepted
            } else {
                AdmissibilityVerdict::Unknown
            },
            pen.explanation.violation,
            if admissible {
                None
            } else {
                Some("witness_inadmissible".into())
            },
        ),
    }
}

fn penalize_then_witness(pair: TransitionPair) -> Admissible<TransitionEvidence> {
    penalize(pair).bind(witness)
}

fn pipeline() -> KleisliArrow<TransitionIntent, TransitionEvidence> {
    kleisli_compose_pair(
        propose,
        penalize_then_witness,
        "propose_penalize_witness_e2e",
    )
}

#[test]
fn kleisli_gate_pipeline_e2e_admissible_identity_transition() {
    let scalars = TransitionScalars {
        binder_liquid_ratio: 0.5,
        reaction_extent: 0.42,
        temperature_k: 293.15,
        s_intrinsic_mpa: Some(12.7),
    };
    let intent = TransitionIntent {
        old: scalars,
        new: scalars,
        dt: 1.0,
    };

    let out = pipeline().run(intent);
    assert!(
        out.result.is_admissible(),
        "identity transition must admit through full pipeline"
    );
    assert_eq!(out.value.admissibility, AdmissibilityToken::Admissible);
    assert_eq!(out.value.catalog_id, CD_TRANSITION_CATALOG_ID);
}

#[test]
fn kleisli_gate_pipeline_e2e_inadmissible_psi_spike_rejects_at_penalize() {
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
    assert!(!penalized.result.is_admissible());
    assert!(
        penalized.value.explanation.violation > 0.0,
        "ψ spike must incur positive constraint_loss slack"
    );

    let out = penalize_then_witness(pair);
    assert!(!out.result.is_admissible());
    assert_eq!(out.value.admissibility, AdmissibilityToken::Inadmissible);
    assert_eq!(out.value.catalog_id, CD_TRANSITION_CATALOG_ID);
}

#[test]
fn kleisli_gate_pipeline_e2e_penalize_witness_agree_on_token() {
    let params = SubstrateMaterialParams;
    let scalars = TransitionScalars {
        binder_liquid_ratio: 0.5,
        reaction_extent: 0.42,
        temperature_k: 293.15,
        s_intrinsic_mpa: Some(12.7),
    };
    let snap = scalars.thermodynamic_snapshot_with_params(&params);
    let pair = TransitionPair {
        old: snap,
        new: snap,
        dt: 1.0,
    };

    let pen = penalize(pair);
    let wit = witness(pen.value);
    assert_eq!(
        pen.value.explanation.admissibility, wit.value.admissibility,
        "constraint_loss host mirror and GateCartridge witness must agree"
    );
}
