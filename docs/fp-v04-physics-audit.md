# FP v0.4 — `src/physics/` globals and hot-path notes

Audit scope: **umst-manifold** `src/physics/` (Burn lazy tensors + solver inner loops).

## Global / side-effect surfaces

- **`rheology_flow::chorin_poisson_richardson_fallback_enabled`** (`--features rheology-bingham`): reads `UMST_RHEOLOGY_POISSON_RICHARDSON_FALLBACK` once via `std::sync::OnceLock<bool>`. Environment side effect; thread-safe, idempotent.
- **Solver modules** otherwise avoid process-wide mutable state; configuration flows through struct fields and feature flags.

## `into_scalar()` on hot paths

Synchronizing the device to read one `f32` is expensive inside tight loops (CG / Newton / rheology). Known clusters:

| Area | Role |
|------|------|
| `physics/solvers/electrochemistry.rs` | PNP/CG residuals and convergence checks |
| `physics/solvers/rheology_flow.rs` | Pressure Poisson CG, Bingham continuation |
| `physics/solvers/thmc.rs` | Inner linear solves (CG scalars) |
| `physics/solvers/thmc_residual.rs` | Block residual energy norms |
| `physics/mechanics.rs` | Iterative solver diagnostics |
| `physics/solvers/fracture_field.rs` | Weighted reductions |

**Note:** `thmc.rs` documents paths that intentionally keep norms on-device where possible; residual assembly still uses scalar reductions for stopping criteria.

## Downstream cartridge

`umst-concrete-cartridge` topology merge uses `into_scalar()` for headline merges and `tensor_l1` populated checks — orchestration-level, not inner solver iteration counts.
