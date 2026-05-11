# Proof / verification status (stub)

Authoritative solver–lane–verification mapping and long-form notes live in **[`Solver-Status.md`](Solver-Status.md)** (same directory). This table is a short index only. **`References.bib` is not in this repository yet** (Track J4 honesty).

| solver | lane | verification_test | status | notes |
| ------ | ---- | ----------------- | ------ | ----- |
| `solvers::topology_solver` | stable | `tests/topology_continuation.rs`, `tests/topology_filter.rs` | default `cargo test` | Conservative stable-lane entry; `topology-density-evolution`. |
| `mechanics::VectorMechanicsSolver`, `adjoint::AdjointCompliance` | research | `tests/verification/mechanics_analytic.rs`, `tests/verification/adjoint_compliance_analytic.rs` | `--features solver-experimental` | Bar/plate + adjoint vs FD; details in full table. |
| `solvers::fracture_field` (`PhaseFieldFractureSolver`) | research | `tests/verification/fracture_gamma_convergence.rs`, `tests/verification/staggered_fracture_mechanics_chain.rs`, `tests/verification/staggered_ud_loop_milestone.rs` | research lane | AT2 inner loop + staggered harnesses; Γ-limit deferred. |
| `solvers::acoustics` | research | `tests/verification/acoustics_plane_wave.rs` | research lane | Newmark / plane-wave checks; `acoustics-newmark`. |
| `solvers::electrochemistry` | research | `tests/verification/pnp_debye_layer.rs` | research lane | Picard + opt-in MVP-chain implicit Newton; see deferral memo. |
| `solvers::thmc` | research | `tests/verification/thmc_drying_shrinkage.rs` | research lane | Drying/shrinkage; implicit \(T,\alpha\) block; SI bar strain parity vs fracture helper. |

CI lint for **`Solver-Status.md`**: `python3 scripts/check_solver_status.py` (optional `--check-paths`).
