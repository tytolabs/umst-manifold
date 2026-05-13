# FP gap verification report

**Generated:** 2026-05-11  
**Workspace:** `/Users/santhoshshyamsundar/Desktop/MaOS-Workspace`  
**Source backlog:** `umst-manifold/docs/FP_GAP_BACKLOG.md`

Commands were run from `umst-manifold/` or `umst-concrete-cartridge/` as appropriate for each crate.

---

## Summary table

| ID | Status | Command output summary | Next action |
|----|--------|-------------------------|-------------|
| FP-001 | partial | `cargo test -p umst-manifold --lib full_sg_newton_band_lu_ --features solver-experimental` → **0 tests** matched (name filter stale vs current tree). **`cargo test -p umst-manifold --lib --features solver-experimental`** → **77 passed**, 1 ignored, exit **0**. | Update backlog verify filter to current tests (e.g. `full_sg_newton_band_expand_dense_matches_dense_column_fd_reference`, `full_sg_newton_dense_expand_matches_direct_gaussian_multi_n`) or document rename/removal of old `full_sg_newton_band_lu_*` suite. |
| FP-002 | partial | Same as FP-001 (backlog cites same command). | Same as FP-001; add targeted band-vs-dense randomized apply tests when re-scoping FP-002. |
| FP-003 | partial | `cargo test -p umst-manifold --lib full_sg_newton_band_lu_satisfies --features solver-experimental` → **0 tests** matched; no `full_sg_newton_band_lu_satisfies*` symbol in tree (grep). Full `--lib` slice still green (see FP-001). | Rename verify command after any restored/split test; remove `eprintln!` only once that test exists again. |
| FP-004 | pass | `bash umst-manifold/scripts/check_physics_no_gradient_break.sh` → exit **0**, “OK: physics gradient escape check passed”. | Keep allowlist/docs in sync when touching gradient escape hotspots. |
| FP-005 | pass | `RUSTDOCFLAGS='-D warnings' cargo doc -p umst-manifold --no-deps` → exit **0**, doc generated. | Maintain on default feature set in CI/pre-push. |
| FP-006 | fail | `RUSTDOCFLAGS='-D warnings' cargo doc -p umst-manifold --no-deps --features solver-experimental` → exit **101**. Representative errors: unresolved `[0,1]` in `ai/topology.rs`; unresolved `ThmcSolver::step` in `fracture_field.rs`; `rustdoc::private-intra-doc-links` for `hydration_arrhenius_rate`, `VectorMechanicsSolver::projected_bar_equilibrium_residual`, `Self::one_damped_newton_step_qs_r_u_inner`; duplicate `[0,1]` in `thmc.rs`. | Escape bracket pseudo-links; fix or qualify doc links; use `doc(cfg)`, plain text, or `--document-private-items` policy consistently. |
| FP-007 | pass | Same family as FP-005: default manifold rustdoc with `-D warnings` succeeds (electrochemistry module docs compile under default features for this run). | Re-run after edits under `solver-experimental` docs path if module docs are cfg-expanded later. |
| FP-008 | pass | `RUSTDOCFLAGS='-D warnings' cargo doc -p umst-concrete-cartridge --no-deps` (from `umst-concrete-cartridge/`) → exit **0**. | Keep cartridge docs aligned with redundant-link cleanups noted in backlog. |
| FP-009 | pass | `cargo test -p umst-manifold statistical_mechanics` → **6 passed** (lib filtered), exit **0**. | Optional follow-up: replace remaining `panic!` paths called out in backlog if still present outside these tests. |
| FP-010 | pass | `cargo clippy -p umst-manifold --features solver-experimental -- -D warnings` → exit **0**. | Continue using same command in CI lint job. |
| FP-011 | fail | `cargo test -p umst-concrete-cartridge` → **`proof_status_markdown_matches_committed_snapshot` FAILED** — docs/PROOF-STATUS.md drift vs committed snapshot (test suggests regeneration via ignored refresh test). Other crates/tests may pass; overall exit **101**. | Run `cargo test -p umst-concrete-cartridge --test proof_status_doc proof_status_refresh_markdown_on_disk -- --ignored --nocapture` and commit refreshed markdown **or** sync snapshot expectations. Separate backlog item (panic in `implementation.rs`) not exercised by failing test name above. |
| FP-012 | pass | `python3 umst-manifold/scripts/check_solver_status.py` → OK (9 rows). `python3 umst-concrete-cartridge/scripts/check_solver_status.py` → OK (stable lane + extras). Cartridge now has script (backlog said “not present” at scan date). | Prefer single CI snippet calling one canonical path if avoiding drift remains a goal. |
| FP-013 | partial | `.github/workflows/rust.yml`: PR jobs include default `cargo test`, `solver-stable` on PR, `cargo check` research union on PR; **`solver-experimental` tests run only on `main` / `workflow_dispatch` (release)**. No `check_physics_no_gradient_break.sh` or `solver-experimental` **debug** matrix entry on PR. | Add PR-safe checks if solver-experimental regressions must not wait for `main`; optionally add gradient script to CI. |
| FP-014 | partial | Backlog command `cargo test -p umst-manifold --test mechanics_analytic -- --ignored` → **0 tests** (integration test crate needs feature `topology-density-evolution` via `solver-stable`). **`cargo test -p umst-manifold --features solver-stable --test mechanics_analytic -- --ignored`** → **1 failed**: `plate_r21_kirchhoff_ssss_centre_w_within_5pct_brick_path_gate` — rel_err ≈ **0.9999** vs ≤5% (exit **101**, ~37s). Matches documented “ignored until §R2.1 BC harness” intent. | Document required `--features solver-stable` in backlog verify line; optional env/runbook for heavy ignored case. |
| FP-015 | partial | No automated nightly harness run in this session (manual / scheduled artefact per backlog). | Wire optional scheduled job + `--release` ignored electrochemistry parity when FP-001 bandwidth LU diagnostics are stable. |

---

## Roll-up counts

| Outcome | Count | IDs |
|---------|-------|-----|
| **pass** | **7** | FP-004, FP-005, FP-007, FP-008, FP-009, FP-010, FP-012 |
| **fail** | **2** | FP-006, FP-011 |
| **partial** | **6** | FP-001, FP-002, FP-003, FP-013, FP-014, FP-015 |

---

## Top blockers (by impact)

1. **Rustdoc under `solver-experimental` (FP-006)** — `cargo doc -p umst-manifold --no-deps --features solver-experimental` fails `-D warnings`; blocks doc-as-lint for the full experimental graph until intra-doc links and bracket escapes are fixed.
2. **Cartridge proof-status snapshot (FP-011)** — default `cargo test -p umst-concrete-cartridge` fails `proof_status_markdown_matches_committed_snapshot` until markdown is regenerated or expectations updated.
3. **Stale FP-001–FP-003 verify filters** — backlog test name filters match **zero** tests; parity signal is currently “full `--lib` green” only until filters/docs are updated.

---

## Evidence notes

- Full manifold lib verification run recorded: `cargo test -p umst-manifold --lib --features solver-experimental` → **77 passed**, **0 failed**, **1 ignored**, exit **0**.
- Do not treat FP-001 as “closed”: cited filters did not execute any test by name.

---

## Workspace FP hotspot sweep — 2026-05-12

**Task:** `fp-hardening-workspace-sweep`  
**Sweep command (counts):** `grep -R --include='*.rs'` with fixed-string literals `into_scalar(`, `into_data(`, `panic!(`, `.unwrap()`, `.expect(` across the listed trees; **`unwrap!` macro:** `0` matches in both trees (same session).

**Blocked-on-hygiene note:** Per [`MAOS_V04_SWARM_TODO_2026_05_12.md`](../../MAOS_V04_SWARM_TODO_2026_05_12.md), this sweep prefers a clean working tree after **`branch-hygiene-*`**; **`umst-manifold`** / **`umst-concrete-cartridge`** were **still dirty** at capture time (~50 / ~7 tracked edits plus untracked docs). Numbers below reflect **HEAD + working tree** file contents in this workspace checkout, not necessarily committed-on-`origin` rows alone.

### Count roll-up (`into_scalar` / `into_data` / `panic!` / `unwrap` / `expect`)

| Scope | Files (`*.rs`) | `into_scalar(` | `into_data(` | `panic!(` | `.unwrap()` | `.expect(` |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `umst-manifold/src/` | *(tree)* | 43 | 151 | 3 | 9 | 42 |
| `umst-concrete-cartridge/crates/` *(all crates)* | *(tree)* | 12 | 39 | 7 | 22 | 37 |

**Canonical framing:** Prefer classifying residual calls against allowlists / compositor patterns in [`FP_FIXED_POINT_CANONICAL.md`](FP_FIXED_POINT_CANONICAL.md) and [`FP_CATEGORICAL_BURN.md`](FP_CATEGORICAL_BURN.md) — **`into_*`** at host boundaries / exporters / tests is routine; gratuitous **`into_*`** inside inner Krylov/Newton iterations remains the tightening target. Prefer **`Result`** / fallible adapters (e.g. [`gmres_f32_try`](../src/physics/solvers/krylov_host.rs)) over new **`panic!`** / unchecked **`unwrap()`** on solver orchestration surfaces.

### Manifold hotspots (combined pattern density, top paths)

Highest combined hit density under `src/` (same five substrings summed per file):

| Hits (approx.) | Path |
| ---: | --- |
| 64 | `src/physics/solvers/electrochemistry.rs` |
| 27 | `src/physics/solvers/fracture_field.rs` |
| 26 | `src/physics/solvers/photonics.rs` |
| 20 | `src/physics/solvers/thmc_residual.rs` |
| 19 | `src/physics/solvers/statistical_mechanics.rs` |

### Clippy checkpoint (solver-stable acceptance)

`(cd umst-manifold && cargo clippy --features solver-stable -- -D warnings)` → **exit 0** (2026-05-12, same session as table above).
