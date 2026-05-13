# CI gap notes (MaOS workspace)

This file records **differences** between GitHub Actions **`.github/workflows/rust-solvers.yml`** and the **local Phase A ladder** described in `docs/MAOS_CLOSEOUT_VERIFICATION_LOG.md` (physics script → `cargo test --features solver-experimental` in both crates → clippy with the same feature → `RUSTDOCFLAGS=-D warnings cargo doc` with **`--document-private-items`** on manifold → `check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set`).

## Resolved / aligned (2026-05-11)

| Item | Before | After |
| --- | --- | --- |
| Manifold tests | `cargo test` + `solver-tests` only | Added **`cargo test --features solver-experimental`**, **`cargo clippy --features solver-experimental`**, **`check_solver_status.py`** (full memo/path/statmech flags), **`cargo doc`** with **`RUSTDOCFLAGS=-D warnings`** and **`--document-private-items`**. |
| Cartridge tests | `cargo test` default only | Added **`cargo test -p umst-concrete-cartridge --features solver-experimental`**, matching Striatus / THMC integration lanes. |
| Cartridge docs | Not built in CI | Added **`cargo doc -p umst-concrete-cartridge --no-deps`** with **`-D warnings`**. |

## Intentional exclusions (cost / environment)

- **`--release`** ignored electrochemistry **N=256** LU harness and long Chorin / B6 harnesses remain **manual / scheduled** — not in default Actions minutes budget. **Runbook (fp_015 / band LU vs dense-expand parity print):** `cargo test -p umst-manifold --features electrochemistry-pnp,solver-experimental full_sg_chain_n256_band_lu_vs_dense_expand_wall_clock_and_residual_parity --release -- --ignored --nocapture` (full rustdoc on the test in `physics/solvers/electrochemistry.rs`).
- **`pytest`** / **`uv run`** Track L print-ready gates are **cartridge-local**; use `umst-concrete-cartridge/scripts/verify_striatus_coupled_gates.sh` or notebook tests on a runner with Python deps when closing **B8**.

## Striatus script vs B8 rollup (`closeout-int-striatus`)

**Contributor / CI contract (not a closure claim):**

- **`bash umst-concrete-cartridge/scripts/verify_striatus_coupled_gates.sh`** (from the MaOS workspace root; the script **`cd`s** into the cartridge) is **expected** to exit **0** while committed **`gates_track_b8_all_pass`** in **`striatus_shell_v0.4.print_ready.json`** is still **`false`**. Default **`pytest notebooks/tests/test_print_ready.py`** **skips** **`test_print_ready_track_b8_topology_gates`** in that state — a green script run **does not** mean B8 rollup is satisfied.
- With **`UMST_REQUIRE_B8=1`**, the same pytest target **must fail** until Track L regeneration commits a sidecar with **`gates_track_b8_all_pass`: `true`**. Treat that failure as **Ring‑1 honesty** until the checklist is met; do not “fix” it by relaxing the test or editing JSON by hand.
- **Checklist:** **[`Solver-Status.md` → *int-striatus — todo close criteria (honest)*](Solver-Status.md#int-striatus--todo-close-criteria-honest)** (cartridge-side mirror: [`../../umst-concrete-cartridge/docs/Solver-Status.md`](../../umst-concrete-cartridge/docs/Solver-Status.md#int-striatus--todo-close-criteria-honest)). Per that section, **do not** mark **`int-striatus`** / **`closeout-int-striatus`** complete from automation green alone while the committed rollup stays **`false`**.

### Cartridge ↔ manifold contract (step 4 of `verify_striatus_coupled_gates.sh`)

| Piece | Behaviour |
| --- | --- |
| **Entrypoint** | `bash umst-concrete-cartridge/scripts/verify_striatus_coupled_gates.sh` from the **MaOS workspace** root (or any cwd); the shell script resolves **`ROOT`** and **`cd "${ROOT}"`** into **`umst-concrete-cartridge/`**. |
| **pytest argv** | **`"${PY}" -m pytest "${ROOT}/notebooks/tests/test_print_ready.py"`** — paths are absolute under cartridge **`ROOT`** after **`cd`**. |
| **Status checker** | Cartridge **`scripts/check_solver_status.py`** is a thin shim: if **`../umst-manifold/scripts/check_solver_status.py`** exists, it invokes manifold’s script with **`--status-md`** pointing at **this cartridge’s** **`docs/Solver-Status.md`** and **`--root`** the sibling **`umst-manifold/`** checkout. If the sibling is absent, the shim prints to stderr and exits **0** (local-only friendly). |
| **Strict B8 for “real” green** | **`UMST_REQUIRE_B8=1`** (exported in the environment for the script run) forces pytest to **fail** while **`gates_track_b8_all_pass`** is **`false`** — use only when validating honest closure after a regen, not as a default dev loop. |

**Optional bridge smoke (manifold, not in the shell script):** `cd umst-manifold && cargo test -p umst-manifold --features solver-experimental --test mechanics_analytic` — listed in **[`MULTI_AGENT_GAP_CLOSURE_PLAN.md`](MULTI_AGENT_GAP_CLOSURE_PLAN.md)** § *Bridge lane — int-striatus* alongside PPO + adjoint smokes.

## Honest residual gaps

- **`gates_track_b8_all_pass`** remains **false** in committed Striatus JSON — Ring‑1 todos (**`m1-b8`**, **`int-striatus`**, **`m1-l`**) stay **pending** regardless of CI green (see `docs/CURSOR_TODO_RECOMMENDATIONS_MAOS_CLOSEOUT.md`).
- **Band LU vs dense-expand** parity is **not** CI-asserted at **N=17** (experimental LU can diverge from production dense-expand); diagnostics remain the ignored **N=256** harness in `electrochemistry.rs`.
