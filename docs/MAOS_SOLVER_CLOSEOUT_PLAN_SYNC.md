# MaOS solver close-out plan ↔ verification matrix sync

**Purpose:** Map v0.4 **solver / verification close-out** work to the canonical **numbered verification rows** when no dedicated Cursor plan file exists for that scope.

## Source of truth (no matching `.cursor/plans` close-out file)

Under **`.cursor/plans/`** there is **no** plan whose name or overview targets **MaOS umst-manifold solver verification**, **100% completion rows**, or **solver close-out** as the primary subject. Existing plans there are **constitution-system** (core constitution / admissibility architecture) and **MaOS Unity** (epistemic concrete sensing UI / contracts); neither carries todos aligned to [`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md) rows **1–10**.

**Authoritative pair for solver verification close-out:**

| Document | Role |
| --- | --- |
| [`docs/VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md) | Per-row **%**, goals, shipped hooks, **exact acceptance**, effort, blockers, next PR slice |
| [`docs/Solver-Status.md`](Solver-Status.md) | Lane tables, **Verification scope (remaining)** checklist **#1–#10**, CI commands, honest runbooks |

**Optional index:** [`docs/VERIFICATION_SCOPE_INDEX.md`](VERIFICATION_SCOPE_INDEX.md) (lane → memo → tests).

## Original close-out plan file

**Does a dedicated “MaOS solver / 100% / close-out” plan exist in `.cursor/plans/`?** **No.**

---

## Sync table

**Legend — Status (met / partial / open):**

- **met:** Matrix **Exact acceptance criterion** satisfied for that row on default CI (or committed ship artefacts as stated), consistent with **100%** bin in [`Solver-Status.md`](Solver-Status.md) **Completion scoring (v0.4)**.
- **partial:** Substantial shipped hooks or **50% / 75%** style progress; acceptance paragraph **not** fully satisfied.
- **open:** **25%** bin or acceptance still far; primarily hooks, smokes, or doc-honesty unless row explicitly closed.

**Plan todo id:** No Cursor plan file defines these rows — use **—** until a plan is authored.

**Cursor-equivalent id:** Suggested stable id if this scope is added to a Cursor plan’s YAML `todos` (not currently present).

| Plan todo id | Matrix row | Cursor-equivalent id | Status | Evidence path / test |
| --- | --- | --- | --- | --- |
| — | **#1** Topology / shell (Tracks B + L) | `solver-closeout-v04-01` | partial | Sibling `umst-concrete-cartridge/`: `tests/shell_topology_rib_pattern.rs` (`shell_topology_rib_pattern_full_v04`, `shell_topology_rib_pattern_quick`); manifold `src/physics/adjoint.rs`, `mechanics.rs`, `topology_filter.rs`; `notebooks/tests/test_print_ready.py`. Acceptance: matrix row **#1** + [`Solver-Status.md`](Solver-Status.md) Track B6 / m1-b8 / B8 rollup. |
| — | **#2** Mechanics (plates) — Kirchhoff / Q1 hex | `solver-closeout-v04-02` | open | `tests/verification/mechanics_analytic.rs` (`plate_centre_deflection_kirchhoff_ratio_q1_hex_locked_band`); `#[ignore]` `plate_r21_kirchhoff_ssss_centre_w_within_5pct_brick_path_gate`; `src/physics/q1_hex_elasticity.rs`. Acceptance: **within-5%** centre deflection vs Kirchhoff SSSS per matrix **#2** (not met by ratio band alone). |
| — | **#3** Fracture — Γ / ψ⁺ / stagger | `solver-closeout-v04-03` | partial | `tests/verification/fracture_gamma_convergence.rs`, `staggered_fracture_mechanics_chain.rs`, `staggered_ud_loop_milestone.rs`, `thmc_drying_shrinkage.rs`; memo `docs/research/v0.4_track12_staggered_fracture_mechanics.md` §7. |
| — | **#4** Acoustics — return map | `solver-closeout-v04-04` | met | `tests/verification/acoustics_plane_wave.rs`: `plane_wave_return_map_n64_l2_within_two_percent`, `plane_wave_return_map_n100_l2_within_two_percent`, `plane_wave_return_map_n128_l2_within_two_percent`; `src/physics/solvers/acoustics.rs` rustdoc. |
| — | **#5** Electrochemistry — PNP / Debye / Newton | `solver-closeout-v04-05` | partial | `tests/verification/pnp_debye_layer.rs` (`debye_screening_256_cells_phi_*`, dispatch smokes, linearized SG Newton); `src/physics/solvers/electrochemistry.rs`; memo `docs/research/v0.4_track14_implicit_newton_pnp.md`. Open tail: large-**N** inner solve / general-graph Newton per matrix. |
| — | **#6** Photonics — DEC curl–curl / Fresnel | `solver-closeout-v04-06` | **partial — not row #6 closure** | `tests/verification/photonics_fresnel.rs` (**`solve_maxwell_curl_curl`**, **`PhotonicsDecFacesPatch`** patch solves + chain TE checks), `photonics_curl_curl_stub_default_build.rs`, `tests/dec_identities.rs` (DEC **`d₁`** identities — **not** the photonics solve path); memo `docs/research/v0.4_track15_dec_curl_curl_photonics.md`. See [**§ Matrix #6 / m6-dec — honest status**](#matrix-6--m6-dec--honest-status) below. |
| — | **#7** Rheology — Chorin / channel | `solver-closeout-v04-07` | partial | `tests/verification/rheology_poiseuille.rs` (`chorin_*` smokes, `chorin_surrogate_poisson_amplification_regression_guard`); `src/physics/solvers/rheology_flow.rs`, `laplacian.rs`. Long-run **L²** steady acceptance still open per matrix. |
| — | **#8** THMC — monolithic Newton | `solver-closeout-v04-08` | partial | `tests/verification/thmc_drying_shrinkage.rs` (`thmc_step_monolithic_newton_*`, `thmc_step_monolithic_implicit_lowers_*`, …); `src/physics/solvers/thmc.rs`, `thmc_residual.rs`; memos track 13. Scale / AD-safe early-exit / within-step **u↔d** open per matrix + [`Solver-Status.md`](Solver-Status.md#solver-lane-thmc). |
| — | **#9** Statistical mechanics — virial / `upscale_potentials` | `solver-closeout-v04-09` | open | `tests/verification/statmech_vinet_eos.rs`, `statmech_lj_bridge_contract.rs`, `statmech_lj_johnson_eos_reference.rs`; memo `docs/research/v0.4_track16_virial_lj_bridge.md`. |
| — | **#10** Mechanics — transient vector / contact | `solver-closeout-v04-10` | open | Quasi-static: `mechanics_analytic.rs`, `adjoint_compliance_analytic.rs`; scalar bar wave: `acoustics_plane_wave.rs` (**not** vector 3D solid transient); `docs/research/contact_mechanics_scope.md`. |

---

### Matrix #6 / m6-dec — honest status

Green integration tests for **`PhotonicsSolver::solve_maxwell_curl_curl`**, optional **`PhotonicsDecFacesPatch`**, and DEC topology helpers in **`tests/dec_identities.rs`** are **necessary evidence** but **do not** mean [`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md) row **#6** is closed. Per that row’s **Exact acceptance criterion** and [**Solver lanes — Photonics**](Solver-Status.md#solver-lane-photonics), matrix **#6** stays **partial** (**50%** bin in [`Solver-Status.md`](Solver-Status.md)) until at least: **dual Hodge** / metric-weighted DEC (beyond today’s **unweighted** **`d₁ᵀd₁`** small-patch path), **assembled** incidence and metrics from manifold/mesh state (not only test-authored **`faces_b2` COO**), **sparse** inner solves at **large N**, and **complex ε** plus **PML** on the patch solve path — alongside the listed Fresnel / BC follow-ups.

**Do not** tick todo **m6-dec** or narrative row **#6** as **100%** until CI-backed tests satisfy those acceptance bullets; treating passing **`photonics_fresnel`** / **`dec_identities`** alone as “DEC production complete” would **overstate** shipped photonics.

**Latest verification command (local):** `cargo test --features solver-experimental --test photonics_fresnel --test dec_identities` — logged **2026-05-11** (all tests passed in that run).

---

## Path integrity check

After edits to [`Solver-Status.md`](Solver-Status.md) (not required for this sync doc alone), from repo root **`umst-manifold/`**:

```bash
python3 scripts/check_solver_status.py --check-paths
```

Full CI-style solver-status gate (see [`Solver-Status.md`](Solver-Status.md) **CI** bullet):  
`python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set`

---

## Close-out tick policy (2026)

The original **MaOS solver / 100% / close-out** Cursor plan file is **absent** from **`.cursor/plans/`** (see [**Source of truth (no matching `.cursor/plans` close-out file)**](#source-of-truth-no-matching-cursorplans-close-out-file) and [**Original close-out plan file**](#original-close-out-plan-file) above). There is therefore **no** YAML `todos` block in-repo that authoritatively owns matrix rows **#1–#10**; tracking uses **[`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md)** + **[`Solver-Status.md`](Solver-Status.md)** only.

**Do not** mark the following Cursor todos **complete** until both gates are satisfied: **`VERIFICATION_COMPLETION_MATRIX`** row **Exact acceptance criterion** text (and **%** bins where applicable) **and** **`Solver-Status`** milestone / lane honesty (including **`gates_track_b8_all_pass`** where cited) — i.e. **evidence-backed closure**, not narrative or doc-only ticks.

| Todo id | Gate (summary) |
| --- | --- |
| **`m1-b6`** | Matrix **#1** / B6 full harness acceptance (e.g. documented **`shell_topology_rib_pattern_full_v04`** gates); **`Solver-Status`** P0 / appendix alignment. |
| **`m1-b8`**, **`m1-l`**, **`int-striatus`** | **`gates_track_b8_all_pass`: true** in committed **`striatus_shell_v0.4.print_ready.json`** (sibling **`umst-concrete-cartridge/`**) where those milestones require it; matrix **#1** / Track L bullets; **`Solver-Status`** checklist. |
| **`m6-dec`** | Matrix **#6** full acceptance (not small-patch / identity tests alone); **`Solver-Status`** photonics / DEFERRAL bullets. |

Per-todo evidence, commands, and **keep pending** recommendations: **[`docs/CURSOR_TODO_RECOMMENDATIONS_MAOS_CLOSEOUT.md`](CURSOR_TODO_RECOMMENDATIONS_MAOS_CLOSEOUT.md)**. Append-only verification runs: **[`docs/MAOS_CLOSEOUT_VERIFICATION_LOG.md`](MAOS_CLOSEOUT_VERIFICATION_LOG.md)** (when present).
