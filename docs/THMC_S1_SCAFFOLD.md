# THMC stacked-R S1 scaffold (feature-flagged)

**Status:** Prep only — no production wire until USER B6 geometry sign-off.

## Scope

| Item | Status |
|------|--------|
| Post-step `wire_gate_evidence_post_step` | **Shipped** (W10) |
| Configurable `gate_intrinsic_strength_mpa` on `ThmcSolver` | **Shipped** (W6) |
| `ConcreteGateCartridge` in cartridge (`tier2c-handshake`) | **Shipped** |
| Stacked-R monolith ≤64 DOF | **Partial** — research tests only |
| Production-scale JFNK / adaptive `dt` | **Deferred** |

## Oracle fixture

Run (research feature lane):

```bash
cargo test --features thmc-coupled,solver-research \
  --test thmc_gate_evidence_wire \
  --test verification::thmc_monolithic_newton_chain
```

## USER gates

- B6 200-outer (`UMST_SHELL_RIB_FULL_ITERS=200`) — **HALT**
- Ignore suite one-shot — see [`SOLVER_NEVER_RUN_LEDGER.md`](SOLVER_NEVER_RUN_LEDGER.md)
