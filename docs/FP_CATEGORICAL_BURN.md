# FP categorical burn — physics host-sync audit

**Epic:** `fp-sort-io-monad-audit` · **Scope:** `src/physics/` tensor escape tiers and solver hotspots.

This note is the SSOT for `into_data` / `into_scalar` allowlist rationale in [`scripts/physics_gradient_escape_allowlist.txt`](../scripts/physics_gradient_escape_allowlist.txt). Code links here from `thmc.rs`, `thmc_residual.rs`, and related solver modules.

## E4 — physics `src/physics/` tier tags

| Tier | Meaning | Policy |
|------|---------|--------|
| **ConvergenceRequired** | Inner Krylov/CG stopping scalars | Keep; do not blanket-remove |
| **HostBridge** | Staging to host GMRES/PCG/hex solvers | Document; collapse only with device-native parity |
| **Diagnostic** | Witness / telemetry scalars | Narrow use |
| **TestOnly** | `#[cfg(test)]` asserts | Allowed |

## Hotspots (operator-split THMC vs CG helper)

- **`thmc.rs`:** Inner CG + L2 telemetry `.into_scalar()` for convergence; operator-split THMC path — see module rustdoc. Re-audit if production path regains per-row host materialization.
- **`thmc_residual.rs`:** ‖R‖² stacks + residual stitching for host GMRES; AD/stopping semantics in module docs.

## §A–C — electrochemistry PNP

- **`electrochemistry.rs`:** CG inner-loop scalars, Newton reductions, 1-D host staging for PNP sub-problems — intentional solver math + structural CPU bridge.

## §F — statistical mechanics

- **`statistical_mechanics.rs`:** HostBridge virial/EOS materialization; TestOnly parity asserts.

## exec-solver-purge (deferred)

- **`extruded_plate.rs` / `adjoint_q1_hex.rs`:** HostBridge `into_data` feeds host Q1-hex PCG. Collapsing without device-native operator + parity is a large numerics project — do not drive from allowlist purge alone.

## Maintenance

1. New `into_data` in production physics → justify tier here + add to allowlist with `REVIEWED` line.
2. Cross-ref [`Category-of-Material-Updates.md`](Category-of-Material-Updates.md) for orchestration composition (fold over plan intents).
