SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
# Kleisli gate pipeline — propose → penalize → witness

**Status:** Penalize hot-bind landed (`kleisli-ppo-hot-bind`, Wave 9 slot 4).  
**Scope:** Burn `step_and_learn` can subtract `constraint_loss` slack; full `kleisli_compose_pair` remains test-only.

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

## Production hot path (partial)

| Stage | Burn PPO equivalent | Wired? |
|-------|---------------------|--------|
| **propose** | `AdjointNeuralODE::forward` | yes (always) |
| **penalize (hard)** | `ThermodynamicCBF::verify_tensor_update` | yes (CBF reject) |
| **penalize (soft)** | `constraint_loss_penalty` → `scaled_clausius_duhem_violation` | **yes** with `kleisli-ppo-hot-bind` or `epistemic-ppo` when `lambda_cd ≠ 0` |
| **witness** | `CdTransitionCartridge::transition_evidence` | **no** — cold/test edge only |
| **compose** | `kleisli_compose_pair` | **no** — zero `src/` call sites |

### Feature flag: `kleisli-ppo-hot-bind`

Enable on `umst-manifold` to route [`BurnLiquidPPOAgent::step_and_learn`](../src/ai/liquid_ppo.rs)
through `step_and_learn_kleisli_penalize` instead of `step_and_learn_stub`. Set
[`ManifoldGateway::lambda_cd`](../src/ai/ppo.rs) to a non-zero value to activate the soft
Clausius–Duhem slack subtraction post-CBF.

```bash
cargo test kleisli_ppo_hot_bind --features kleisli-ppo-hot-bind --quiet
```

**Default build:** feature off; `lambda_cd = 0` — identical to legacy stub (no penalty graph).

**Blocked (documented):** Full hot bind of `kleisli_compose_pair(propose, penalize, witness)` is
not attempted here. Propose on the Burn path remains ODE/tensor-based, not
`evaluate_transition_pure_with_params`; witness remains host-only. Wiring those stages through
`Admissible::bind` on the autodiff graph is deferred to `p5-transition-evidence`.

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
cargo test kleisli constraint_loss liquid_ppo --quiet
cargo test kleisli_ppo_hot_bind --features kleisli-ppo-hot-bind --quiet
```

- `tests/kleisli_gate_pipeline_sketch.rs` — pure `kleisli_compose_pair` chain, admissible + inadmissible fixtures.
- Existing parity: `tests/gate_kleisli.rs`, `src/ai/constraint_loss.rs` (module tests), `src/runtime/gate/cartridge.rs` (module tests).

## Related

- [`RELEASE_WITNESS_LADDER.md`](RELEASE_WITNESS_LADDER.md) — R4 Kleisli short-circuit
- [`p4-constraint-loss-spike`](../CHANGELOG.md) — `clausius_duhem_violation` landing
- `outputs/.plans/umst-master-reengineering.md` — Fracture 2 backlog (`p4-ppo-wire`, `p5-transition-evidence`)
