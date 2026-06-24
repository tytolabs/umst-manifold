# Kleisli gate pipeline — propose → penalize → witness

**Status:** Witness sketch (`math-kleisli-gate-pipeline`, Wave 5 slot 3).  
**Scope:** Pure composition + docs only — no hot-path wiring changes.

## Purpose

Fracture 2 requires a **dual gate path**: a differentiable **soft** penalty for Burn
training and a **hard** `f64` witness for commit. The Kleisli gate pipeline is the
categorical composition that chains those stages without dropping gradients at the
rejection boundary and without committing except through witness.

| Stage | Tier | Carrier | Key symbol |
|-------|------|---------|------------|
| **Propose** | Hot | `TransitionScalars` → `(old, new)` snapshots | [`TransitionFilter`](../src/gate/transition_proposal.rs), [`evaluate_transition_pure_with_params`](../src/gate/transition_proposal.rs) |
| **Penalize** | Hot | Burn `[B]` tensors → ReLU slack | [`ai::constraint_loss`](../src/ai/constraint_loss.rs) (`clausius_duhem_violation`, `ConstraintExplanation`) |
| **Witness** | Cold edge | `ThermodynamicStateSnapshot` pair → evidence | [`GateCartridge`](../src/runtime/gate/cartridge.rs) (`transition_evidence` → [`TransitionEvidence`](../src/runtime/gate/evidence.rs)) |

See also [`RUNTIME_TOPOLOGY.md`](RUNTIME_TOPOLOGY.md) boundary rule 3 (dual gate path) and
[`GateUnificationSpec.md`](GateUnificationSpec.md) for `catalog_id` stability.

## Kleisli semantics

The admissibility monad `M(A) = (A, AdmissibilityResult)` lives in
[`gate::kleisli`](../src/gate/kleisli.rs). Each stage is a Kleisli arrow `A → M(B)`;
sequential composition short-circuits on the first inadmissible carrier:

```text
witness ∘ penalize ∘ propose  :  Intent  →  M(Evidence)
         (f ● g)(x) = f(x) >>= g
```

| Arrow | Input | Output | Admissibility source |
|-------|-------|--------|----------------------|
| `propose` | agent / policy intent | `(s_old, s_new, Δt)` | mass + snapshot well-formedness |
| `penalize` | transition pair | `ConstraintExplanation` | `relu(−D_int)` slack ≈ 0 (hot) or host `transition_outcome` (cold mirror) |
| `witness` | penalized pair | `TransitionEvidence` | `GateCartridge::transition_evidence` (commit token) |

**Gradient rule:** `penalize` never branches on `violation > ε` inside the Burn graph;
`witness` runs only on detached host scalars at the Cold edge. Training backprops through
`constraint_loss`; commit reads `TransitionEvidence` only.

## Host / hot mirror contract

`explain_cd_transition_host` and `explain_clausius_duhem_violation` share the same
scalar Clausius–Duhem surrogate (`ρ`, `ψ̇`, `D_int = −ρ ψ̇`). Parity is enforced in
`ai::constraint_loss` unit tests and `CdTransitionCartridge` cartridge tests.

| Path | Function | Autodiff |
|------|----------|----------|
| Hot (Burn) | `clausius_duhem_violation` | yes |
| Hot explain | `explain_clausius_duhem_violation` | detached |
| Cold witness | `explain_cd_transition_host` → `GateCartridge` | no |

`catalog_id` for CD transitions: `umst.gate.cd_transition` (see
[`runtime::catalog::traceability`](../src/runtime/catalog/traceability.rs)).

## Composition sketch (stub types)

The integration witness [`tests/kleisli_gate_pipeline_sketch.rs`](../tests/kleisli_gate_pipeline_sketch.rs)
composes three pure arrows over host `f64` carriers — no Burn device in the test harness:

```text
TransitionIntent
    │ propose   (TransitionScalars → snapshot pair)
    ▼
TransitionPair
    │ penalize  (constraint_loss host mirror)
    ▼
PenalizedTransition { explanation: ConstraintExplanation }
    │ witness   (CdTransitionCartridge)
    ▼
TransitionEvidence
```

Kleisli short-circuit: if `penalize` marks inadmissible, `witness` still runs in the
sketch (value threading) but the composed `AdmissibilityResult` reflects the first failure.

## Tests

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cargo test kleisli_gate_pipeline
```

- `tests/kleisli_gate_pipeline_sketch.rs` — pure `kleisli_compose_pair` chain, admissible + inadmissible fixtures.
- Existing parity: `tests/gate_kleisli.rs`, `src/ai/constraint_loss.rs` (module tests), `src/runtime/gate/cartridge.rs` (module tests).

## Related

- [`RELEASE_WITNESS_LADDER.md`](RELEASE_WITNESS_LADDER.md) — R4 Kleisli short-circuit
- [`p4-constraint-loss-spike`](../CHANGELOG.md) — `clausius_duhem_violation` landing
- `outputs/.plans/umst-master-reengineering.md` — Fracture 2 backlog (`p4-ppo-wire`, `p5-transition-evidence`)
