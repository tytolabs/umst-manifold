# Cursor todo merge — FP gaps

Source: [`FP_GAP_BACKLOG.md`](FP_GAP_BACKLOG.md) § *Suggested Cursor todos* (scan 2026-05-11).

**Already closed (do not re-file under these ids):** `gap-ci-physics-allowlist`, `gap-track14`.

- **`gap-ci-physics-allowlist`:** overlaps backlog row **FP-004** (gradient-escape allowlist audit). That row is **omitted** below; keep using the allowlist process for new `.into_scalar()` / `.into_data()` sites.
- **`gap-track14`:** implicit Newton PNP / dispatch / verification memo scope is closed separately; none of the remaining FP-### lines are one-line duplicates of that todo. Band LU (**`fp_001`**) now has **entry-point** parity vs dense-expand (**`full_sg_newton_band_lu_matches_dense_expand_n17_fixture`**); a true in-place **`O(dim·bw²)`** factorisation remains backlog per [`FP_GAP_BACKLOG.md`](FP_GAP_BACKLOG.md).

| id | content (one line) | verify | depends_on |
|----|--------------------|--------|------------|
| `fp_001` | **Partial.** **`solve_newton_correction_full_sg_row_band_via_band_lu`** matches dense-expand on the **N=17** fixture (**`full_sg_newton_band_lu_matches_dense_expand_n17_fixture`**) by **forwarding** to the same Gaussian path. **Still open:** in-place pivoting band LU without `(3N)²` dense scratch / silent fill truncation. | `cargo test -p umst-manifold --lib full_sg_newton_ --features solver-experimental` → **3** tests[^full-sg-newton]; optional ignored **timing** harness[^ignored-lu]: `cargo test -p umst-manifold --lib full_sg_chain_n256_band_lu_vs_dense_expand_wall_clock_and_residual_parity --features solver-experimental --release -- --ignored --nocapture` | — |
| `fp_002` | Row–column invariant: apply `J` to probe `x` via band vs dense. | Same **`full_sg_newton_`** lib filter as `fp_001` until a dedicated row-apply test lands; Picard path smoke: `cargo test -p umst-manifold --test pnp_debye_layer picard_ --features solver-experimental`[^picard] | `fp_001` (fixture shared) |
| `fp_003` | Remove `eprintln!` from any restored LU Jacobian linearisation test (historical name `full_sg_newton_band_lu_satisfies_jacobian_linear_system_multi_n` is **not** in-tree). | After a LU test lands, match its `cargo test` name filter; until then: same ignored harness as `fp_001`[^ignored-lu] | `fp_001` |
| `fp_005` | Batch-fix manifold rustdoc debt: baseline `cargo doc`, then **`--document-private-items` + `solver-experimental`** (both **0** `rustdoc` warnings on 2026-05-11 refresh — keep as hardening / regression guard). | `RUSTDOCFLAGS='-D warnings' cargo doc -p umst-manifold --no-deps` then `--document-private-items --features solver-experimental` | — |
| `fp_006` | Fix cfg-sensitive / bracket false links in `mechanics.rs` / `topology.rs` (split docs or `doc(alias)`). | Same as `fp_005` second command | `fp_005` |
| `fp_007` | Guard / harden `electrochemistry.rs` private-helper rustdoc (qualified paths vs plain text; **0** `rustdoc` warnings on 2026-05-11 refresh — keep aligned with FP-005 slice). | Same as `fp_005` second command | `fp_005` |
| `fp_008` | Re-audit cartridge rustdoc if regressions appear (2026-05-11: `cargo doc -p umst-concrete-cartridge --no-deps` clean under `-D warnings`). | `cd umst-concrete-cartridge && RUSTDOCFLAGS='-D warnings' cargo doc -p umst-concrete-cartridge --no-deps` | — |
| `fp_009` | Replace `StatisticalBridge::upscale_potentials` `panic!` on bad dims with `Result` or `debug_assert` + fallible API per project convention. | `cargo test -p umst-manifold statistical_mechanics` | — |
| `fp_010` | Review electrochemistry `.expect(` chain in tensor host Newton; return `Option`/`Result` to callers where feasible. | `cargo clippy -p umst-manifold --features solver-experimental -- -D warnings`; optional Picard integration slice: `cargo test -p umst-manifold --test pnp_debye_layer picard_ --features solver-experimental`[^picard] | — |
| `fp_011` | Cartridge bundled-calibration load path: failures should surface as **`Result`** with preserved context (re-audit if `panic!` reappears in `implementation.rs` or siblings). | `cargo test -p umst-concrete-cartridge` | — |
| `fp_012` | Add `scripts/check_solver_status.py` to cartridge **or** document “run from manifold sibling” in cartridge README (pick one; avoid duplicate drift). | Single source of truth + one CI snippet | — |
| `fp_013` | Add CI matrix entry: `cargo test -p umst-manifold --features solver-experimental --no-fail-fast` (or ensure lib green first) + physics gradient script. | GitLab/GitHub CI green | `fp_001` |
| `fp_014` | Mechanics analytic `#[ignore]`: add reason + env to run (mirror rheology pattern). | `cargo test -p umst-manifold --test mechanics_analytic -- --ignored` | — |
| `fp_015` | Optional nightly: `--release` ignored electrochemistry **N=256** band LU vs dense-expand harness (stdout thresholds logged; **not** a parity gate[^ignored-lu]). | Manual / scheduled log artefact | `fp_001` |

**Merged row count:** **14** (source had 15; **`fp_004`** omitted as closed under `gap-ci-physics-allowlist`).

[^full-sg-newton]: Filter `full_sg_newton_` matches **`full_sg_newton_band_expand_dense_matches_dense_column_fd_reference`**, **`full_sg_newton_dense_expand_matches_direct_gaussian_multi_n`**, and **`full_sg_newton_band_lu_matches_dense_expand_n17_fixture`** (`newton_chain_tests` under `electrochemistry-mvp`, pulled in via `solver-experimental`).

[^ignored-lu]: **`#[ignore]`** `full_sg_chain_n256_band_lu_vs_dense_expand_wall_clock_and_residual_parity` prints assembly/solve wall-clock at large **N**. The **`via_band_lu`** entry point **forwards** to dense-expand — **`max|δ_lu−δ_de|`** should be ~**0**; use for timing triage, not as a distinct LU kernel signal.

[^picard]: Filter `picard_` on `pnp_debye_layer` matches `picard_convergence_smoke`, `picard_coupling_iters_finite_smoke`, and `picard_coupling_linf_tol_never_triggers_matches_full_iters`.
