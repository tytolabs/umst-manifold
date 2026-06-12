# Solve contract (integration-contracts D1)

**Module:** `src/solve_report.rs`  
**Audit:** P0 #1, findings **#3**, **#5**, **#12**, **#19** (solver-quality audit synthesis)

## Law

`SolveReport::converged()` is true iff:

1. `rel_tol > 0`, and  
2. `rel_residual` is finite, and  
3. `rel_residual <= rel_tol`.

The relative residual is \(\|P(f-Ku)\|_2 / \|Pf\|_2\) (or lane-equivalent) at solver exit — same scale as
[`VectorMechanicsSolver::packed_bar_network_equilibrium`](../../src/physics/mechanics.rs) PCG exit and
[`HexPcgReport::rel_residual`](../../src/physics/q1_hex_elasticity.rs) (binding true residual on the Q1 path).

## Objects and morphisms

| Object | Role |
| --- | --- |
| [`SolveReport`](../../src/solve_report.rs) | Immutable witness at solver boundary |
| [`PrecisionLane`](../../src/solve_report.rs) | Numeric path tag (f32 bar, f64 adjoint, Q1 hex, dense, Krylov) |
| [`ReportedSolve`](../../src/solve_report.rs) | Trait morphism from lane telemetry → `SolveReport` |
| [`SolverEntryPoint`](../../src/solve_report.rs) | Static inventory row for adoption ladder |

## Adoption ladder

| Stage | Action | Wave |
| --- | --- | --- |
| 0 | `SOLVER_ENTRY_POINTS` inventory + `tests/solve_contract_entry_points.rs` | **integration-contracts** (this PR) |
| 1 | Wire `SolveReport` at `solve_equilibrium_with_pcg_report` call sites | Wave 3 bar→Q1 |
| 2 | THMC stacked-\(R\) exit + post-step diagnostics | Wave 1 |
| 3 | Execute `#[ignore]` envelopes; ledger pass/fail | Wave 2 |

## CI end-state (target)

- **Blocking:** `cargo test` unit tests in `solve_report.rs` + entry-point inventory test.
- **Non-blocking (optional):** compare exported JSON fixtures against golden `SolveReport` rows once solvers adopt the contract.

## Deliberately not done (this wave)

- No edits to `q1_hex_elasticity.rs`, `topology.rs`, cartridge, or shell harness.
- No consumer ports (`thmc`, `fracture_field`, `adjoint`) — inventory only.
- No THMC `tol` wiring to stacked-\(R\) exit (Wave 1).
