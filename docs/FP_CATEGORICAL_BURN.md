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

## H0 — SolverRegion (shipped 2026-06-28)

- **`solver_region.rs`:** `PcgWorkspace` + `SolverRegion` reuse `u`/`diag`/`scratch_ku` and optional `HexStructuredOperatorCache` across outer TO iterations.
- **`adjoint_q1_hex.rs`:** `forward_loss_with_diagnostics(..., solve_options, region)` seeds PCG from `region.warm_u` when `pcg_warm_start` is set; writes back `equilibrium_displacement` and `AdjointForwardPhaseTiming`.
- **Parity:** `tests/verification/solver_region_parity.rs` — `|Δc| < 1e-4`, warm `pcg_iters ≤ cold`.
- **Vault/cartridge:** `VaultSolveContext` and B6 `q1_compliance_with_region` wire warm-start + op-cache without changing PCG math.

## H1 — DeviceSheet (shipped 2026-06-28)

- **`device_sheet.rs`:** `DeviceSheet` host slab for ρ, f, mask; `sync_from_tensors` at IO boundary.
- **`adjoint_q1_hex.rs`:** `forward_loss_with_diagnostics(..., sheet)` avoids per-call `Vec` alloc on `(ρ, f, mask)`.
- **Parity:** `device_sheet_parity` in `solver_region_parity` — `|Δc| < 1e-4`.

## H2 — SIMD fused Krylov reductions (shipped 2026-06-28)

- **`pcg_reduction.rs`:** fused dot/norm helpers with f64 accumulators + 8-wide chunks for LLVM autovec.
- **`q1_hex_elasticity.rs`:** PCG inner loops (`rz`, `pap`, masked `‖r‖`, `‖Pf‖`) call helpers; matvec/precond paths unchanged.
- **Parity:** `solver_region_parity`, `q1_hex_pcg_warm_start_ab`, `q1_hex_perf_levers_ab` **ok**.

## H3 — Const-generic grid witnesses (shipped 2026-06-28)

- **`grid_witness.rs`:** `GridWitness` + catalog grids (`symmetry_quick`, `demo`, `striatus_witness`).
- Test: `grid_witness_catalog` — dimensions match `ExtrudedPlateMechanics`.

## H4 — DEC fused Laplacian (shipped 2026-06-28)

- **`laplacian.rs`:** `scalar_laplacian_fused` spike; lib test `|Δ| < 1e-5` vs `scalar_laplacian`.
- Two-scatter DEC convention preserved.

## H5 — CheckpointPolicy (shipped 2026-06-28)

- **`umst-research/checkpoint_policy.rs`:** trait + vault/cartridge adapters.
- Vault `DeviationTracker` → `DeviationCheckpoint`; cartridge B6 peak via `CompliancePeakCheckpoint`.

## H6 — Cockpit budget (shipped 2026-06-28, manifold stub)

- **`solve_budget.rs`:** `CockpitSnapshot` → `Q1HexSolveOptions` (`pcg_max_iter`, warm-start, op-cache).
- Vault `apply_cockpit_budget`; egoff wire deferred (repo absent).

## Maintenance

1. New `into_data` in production physics → justify tier here + add to allowlist with `REVIEWED` line.
2. Cross-ref [`Category-of-Material-Updates.md`](Category-of-Material-Updates.md) for orchestration composition (fold over plan intents).
