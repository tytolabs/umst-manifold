# Proof / verification status (solver index)

Authoritative solver–lane–verification mapping and long-form notes live in **[`Solver-Status.md`](Solver-Status.md)** (same directory). This table is a **short index** (Track J3): one row per solver in that document’s main table. **OPEN ROADMAP ITEM** sections and narrative limits are not duplicated here.

**Columns (Track J):** `lane`, `benchmark_test` (path(s) to `tests/**/*.rs` exercised on CI for the claim), `verification_status` ∈ {`mechanised`, `analytic-benchmark`, `literature`, `none`}. **Reject** a row with `lane = stable` and empty `benchmark_test`. Formal citations for solver rustdoc accumulate in **[`References.bib`](References.bib)** (Track J4).

CI lint for the full table: `python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set` (from `umst-manifold/`; validates [`Solver-Status.md`](Solver-Status.md)).

| solver | lane | benchmark_test | verification_status | notes |
| ------ | ---- | ---------------- | --------------------- | ----- |
| `solvers::topology_solver` | stable | `tests/topology_continuation.rs`, `tests/topology_filter.rs` | mechanised | `topology-density-evolution`; default `cargo test` path for stable lane. |
| `mechanics::VectorMechanicsSolver`, `adjoint::AdjointCompliance` | research | `tests/verification/mechanics_analytic.rs`, `tests/verification/adjoint_compliance_analytic.rs` | analytic-benchmark | Bar/plate + adjoint; Kirchhoff 5 % gate open — see full table. |
| `solvers::fracture_field` (`PhaseFieldFractureSolver`) | research | `tests/verification/fracture_gamma_convergence.rs`, `tests/verification/staggered_fracture_mechanics_chain.rs`, `tests/verification/staggered_ud_loop_milestone.rs`, `tests/verification/thmc_drying_shrinkage.rs` | analytic-benchmark | AT2 + partial Γ-type harnesses; THMC kinematic parity. |
| `solvers::acoustics` | research | `tests/verification/acoustics_plane_wave.rs` | analytic-benchmark | Newmark vs dense ref; return-map default **n = 100**; **n = 128** phase slip documented. |
| `solvers::electrochemistry` | research | `tests/verification/pnp_debye_layer.rs` | analytic-benchmark | Picard + opt-in MVP-chain implicit Newton; λ_D exponential-fit gates `#[ignore]`. |
| `solvers::photonics` (`PhotonicsSolver`, `PhotonicsHelmholtzSolver`) | research | `tests/verification/photonics_fresnel.rs`, `tests/verification/photonics_curl_curl_stub_default_build.rs` | analytic-benchmark | TE Helmholtz / chain curl–curl; 2D/3D DEC vector solve open. |
| `solvers::rheology_flow` (`BinghamFlowSolver`) | research | `tests/verification/rheology_poiseuille.rs` | analytic-benchmark | Analytic / Chorin smoke; developed-channel surrogate Poisson open (`#[ignore]`). |
| `solvers::thmc` (`ThmcSolver`, …) | research | `tests/verification/thmc_drying_shrinkage.rs` | analytic-benchmark | Drying/shrinkage; implicit \(T,\alpha\) block; monolithic stacked Newton open. |
| `solvers::statistical_mechanics` | research | `tests/verification/statmech_vinet_eos.rs`, `tests/verification/statmech_lj_bridge_contract.rs`, `tests/verification/statmech_lj_johnson_eos_reference.rs` | analytic-benchmark | Vinet + LJ bridge scaling + Johnson reference gap (placeholder honesty). |
