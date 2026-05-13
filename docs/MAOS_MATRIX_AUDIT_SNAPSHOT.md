# MaOS matrix alignment — honest audit snapshot (docs)

**Purpose:** Checklist for multi-agent **alignment audit** todo ids. Each row ties to [`Solver-Status.md`](Solver-Status.md) bullets / main table — **not** to Cursor “completed” ticks. **Do not** treat any row here as **100%** unless the cited Solver-Status matrix **Completion** and **Exact acceptance** text already say so.

**Hub:** [`MAOS_PLAN_ALIGNMENT_REPORT.md`](MAOS_PLAN_ALIGNMENT_REPORT.md). **Semantics:** [`CURSOR_TODO_SEMANTICS.md`](CURSOR_TODO_SEMANTICS.md) (Option **B1** — engineering-slice ≠ matrix-100). **Matrix rubric:** [`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md).

---

## `maos-gate-b-backward-chain` — Grand integration Gate B

**Intent:** Finite **`backward()`** through **THMC → mechanics → fracture** at small **N**, plus integration tests and Solver-Status honesty.

| Check | Source of truth (2026-05-11 docs) | Honest status |
| --- | --- | --- |
| Full autodiff backward through **`ThmcSolver::step`** into fracture | [`Solver-Status.md`](Solver-Status.md) *Coupled lanes* / THMC–fracture narrative: **“Finite backward through full `ThmcSolver::step` into fracture: **not shipped**”** | **Open** — doc explicitly **not shipped** |
| Forward / stub coupling vs full chain | Same file: forward strain → fracture smokes vs **full** backward chain | **Partial** — smokes ≠ Gate B closure |
| **`gates_track_b8_all_pass`** for Ring‑1 | [`Solver-Status.md`](Solver-Status.md) *int-striatus* checklist; [`MAOS_PLAN_ALIGNMENT_REPORT.md`](MAOS_PLAN_ALIGNMENT_REPORT.md) **int-striatus** row | **Open** while rollup **false** |
| **`burn_liquid_ppo_step_finite_backward_chain_smoke`** (no renamed successor in-tree as of 2026-05-11) | [`src/ai/liquid_ppo.rs`](../src/ai/liquid_ppo.rs) unit test: **Neural ODE** forward → **`ManifoldGateway::evaluate_topology_step`** → **`AdjointNeuralODE::backward_adjoint`** → **AdamW** on `policy_weights`, with **`PpoChainStubCartridge`** (zeros-out physics — **not** `ThmcSolver`, bars, or AT2) | **Partial** — proves **PPO ↔ gateway ↔ adjoint surrogate** wiring and finite weight updates; **does not** close Grand Gate B vs Solver-Status “full step” wording |

**Integration story (honest):** The shipped smoke is a **differentiation / control-flow slice** on a **stub** thermodynamic body. THMC–mechanics–fracture **forward** coupling is exercised elsewhere (e.g. [`Solver-Status.md`](Solver-Status.md) *Forward* bullets in the coupled-lanes section); **backward through the real `ThmcSolver::step` stack** remains the open Gate B item per Solver-Status. Treat this test as **Striatus / AI lane** regression, not proof of full-physics autodiff depth.

**VERIFY (executable, minimal — matches test rustdoc “default features”):**

```text
cd umst-manifold && cargo test --lib burn_liquid_ppo_step_finite_backward_chain_smoke
```

**VERIFY (same test, aligned with int-striatus ladder in [`MAOS_CLOSEOUT_VERIFICATION_LOG.md`](MAOS_CLOSEOUT_VERIFICATION_LOG.md)):**

```text
cd umst-manifold && cargo test --features solver-experimental --lib burn_liquid_ppo_step_finite_backward_chain_smoke
```

**After any edit to [`Solver-Status.md`](Solver-Status.md):** run `check_solver_status.py` from the repo’s documented close-out flow — **do not** bump matrix **%** or acceptance prose from this checklist alone.

---

## `maos-matrix9-exact-audit` — Matrix **#9** (statistical mechanics)

| Check | Solver-Status / matrix signal | Honest status |
| --- | --- | --- |
| Row **#9** **Completion (%)** | Main table: **`solvers::statistical_mechanics`** → **25%** ([#9](VERIFICATION_COMPLETION_MATRIX.md)) | **25%** until MD / virial / **γ_gc** bullets close |
| Johnson **[B,4]** vs reference **K** | Shipped tests/helpers ≠ full **MD reference K** / virial acceptance — see [`MAOS_PLAN_ALIGNMENT_REPORT.md`](MAOS_PLAN_ALIGNMENT_REPORT.md) **m9-upscale** row | **Gap documented** — bridge slice ≠ row closure |
| **`γ_gc`**, AD-safe **`K`** | Listed open in alignment report + Solver-Status statmech row narrative | **Open** |

**Verifier:** `statmech_lj_*` tests + matrix row **#9** read side-by-side with [`Solver-Status.md`](Solver-Status.md) statmech bullets.

---

## `maos-matrix10-combined-audit` — Matrix **#10** + mechanics **`min(#2,#10)`**

| Check | Solver-Status / matrix signal | Honest status |
| --- | --- | --- |
| **#10** vector transient on mechanics graph | Item **#10** / mechanics row: quasi-static **`VectorMechanicsSolver`**; **no** default-CI **vector** second-order transient elastodynamics stack on the mechanics graph for general 3D solid DOFs; **`acoustics-newmark`** = **scalar** 1-D bar wave | **Not 100%** — scope explicitly partial / scalar lane |
| **Contact / friction** | Solver-Status **#10**: **no** contact constraints in shipped stack; deferral cross-ref | **Open / deferred** — not **100%** |
| **#2** §R2.1 vs **#10** coupling | Main table maps mechanics completion to **`min(#2,#10)`**; thin-plate §R2.1 gate **not** met by ratio-band harness alone (Solver-Status **Completion scoring** line) | **Combined row stays &lt;100%** until both lanes honest |

### `min(#2,#10)` vs `mechanics_analytic` (overlap / agent-7)

**Matrix mechanics %** follows **`min(#2,#10)`**: the combined row is only as strong as the **weaker** of the two lanes. **`mechanics_analytic`** is a **partial** verifier slice (analytic / harness coverage cited in Solver-Status and the matrix) — it **does not** substitute for full **#2** §R2.1 acceptance or full **#10** transient + solid-DOF scope. **§R2.1** remains **explicitly partial / deferred** in docs (ratio-band and related harness ≠ thin-plate gate closure). **Contact and friction** stay **out of shipped mechanics** (deferral in **#10** narrative), so any “mechanics green” story that leans on `mechanics_analytic` alone is **misleading**; honest language is **partial mechanics** until **both** numbered lanes and contact scope match their matrix acceptance text.

**Verifier:** `mechanics_analytic` + matrix rows **#2** and **#10** + Solver-Status **#10** PR slices section.

---

## `maos-m5-matrix-narrative-sync` — Matrix **#5** (electrochemistry)

| Check | Solver-Status / alignment signal | Honest status |
| --- | --- | --- |
| Row **#5** **%** | **`solvers::electrochemistry`** **75%** in main table; alignment report **m5-scale** tension | **75%** — do not narrate as **100%** |
| Full-SG inner cost | **`try_solve_pnp_backward_euler_newton_chain`**: **dense expand** to **(3N)×(3N)** + **host Gaussian elimination** (**O((3N)³)**); **pivot-safe band LU** unwired — same breakdown as `electrochemistry.rs` rustdoc | Doc–code aligned on 2026-05-11 pass; **band LU** wiring still **open** in code |
| Band LU vs dense-expand narrative | Solver-Status + rustdoc name **`full_sg_chain_n256_band_lu_vs_dense_expand_*`** (non-asserting parity); CI **`full_sg_newton_band_expand_dense_*`** / **`full_sg_newton_dense_expand_matches_direct_gaussian_multi_n`** | **Synced** — parity at large **N** remains research |

**Verifier:** `pnp_debye_layer` + `electrochemistry.rs` rustdoc + matrix **#5** *Blocker* / *Exact acceptance* columns.

---

## Snapshot maintenance

Update this file when Solver-Status matrix **%** or acceptance paragraphs change materially (append date stamp in commit message). Optional: paste one-line **evidence** URLs or commit hashes when a check flips from open → closed.

---

## Appendix — closeout lanes (`m1-b6`, `m1-b8`, `m1-l`, `m6-dec`, `int-striatus`) — 2026-05-11

**Purpose:** Single-place cross-reference for alignment-audit parents; **documentation only** — no implied Cursor todo completion.

| Lane | Honest signal (2026-05-11) | Evidence |
| --- | --- | --- |
| **`m1-b8` / `m1-l`** | **`gates_track_b8_all_pass` = `false`** in committed **`striatus_shell_v0.4.print_ready.json`** | JSON under `umst-concrete-cartridge/notebooks/_artifacts/`; [`MAOS_CLOSEOUT_VERIFICATION_LOG.md`](MAOS_CLOSEOUT_VERIFICATION_LOG.md) §**m1-b8** / **m1-l** |
| **`m1-b6`** | Full B6 harness **not** passing documented greyness gate; quick CI ≠ full acceptance | Cartridge + manifold [`Solver-Status.md`](Solver-Status.md) Track B6 / appendix |
| **`int-striatus`** | Coupled script can be **green** while B8 rollup **false**; **`UMST_REQUIRE_B8=1`** fails pytest until sidecar regenerates | `umst-concrete-cartridge/scripts/verify_striatus_coupled_gates.sh`; same JSON |
| **`m6-dec`** | Matrix **#6** / photonics row stays **50%**: **shipped** = small-patch DEC + named tests; **open** = dual Hodge, sparse **N**, 3D volumetric assembly, complex ε+PML patch path, tighter Fresnel — per [`Solver-Status.md`](Solver-Status.md) photonics row + **Still open** bullets | No **%** change from this doc pass |

### `closeout-m1-b8` — Re-read `striatus_shell_v0.4.print_ready.json` (evidence)

**Committed sidecar:** `umst-concrete-cartridge/notebooks/_artifacts/striatus_shell_v0.4.print_ready.json` (paths relative to workspace root with both repos checked out as siblings).

**Rollup:** **`gates_track_b8_all_pass`** = **`false`**. Per [`Solver-Status.md`](Solver-Status.md) and exporter semantics, **do not** claim **m1-b8** / Ring‑1 B8 closure until committed JSON shows **`true`** (no doc-only “completion”).

**Subgates** (rollup = all three **true**; failing rows are the blockers):

| Gate field | Committed value | Role |
| --- | --- | --- |
| `gate_topo_complexity_b7` | **`false`** | **Failing** — topology / complexity (e.g. genus) not met |
| `gate_volume_fraction_mesh_b7` | **`true`** | **Passing** — nodal **mean(ρ)** in **[0.10, 0.25]** |
| `gate_density_xy_variance_b8` | **`false`** | **Failing** — planar density variance gate not met |
| `gates_track_b8_all_pass` | **`false`** | Rollup (**∧** of the three gates above) |

**Spot-check numerics (same file, re-read 2026-05-11):** `mesh_genus_closed_orientable_largest` = **0**; `mesh_euler_characteristic_largest` = **2**; `density_xy_plane_variance` ≈ **1.57739×10⁻⁸**; `nodal_volume_fraction` ≈ **0.152728**.

**Pytest — `UMST_REQUIRE_B8=1`:** [`umst-concrete-cartridge/notebooks/tests/test_print_ready.py`](../../umst-concrete-cartridge/notebooks/tests/test_print_ready.py) — **`test_print_ready_track_b8_topology_gates`**: when **`gates_track_b8_all_pass`** is **`false`**, default runs **skip** with the regeneration message; if **`UMST_REQUIRE_B8=1`**, the test **fails** via **`pytest.fail`** (same message — fail instead of skip). After a real Track L regen + **`export_print_ready.py`**, the rollup **`true`** path runs the explicit asserts on the three gates.

```text
cd umst-concrete-cartridge && UMST_REQUIRE_B8=1 python3 -m pytest notebooks/tests/test_print_ready.py -v
```

If **`python3 -m pytest`** is unavailable on PATH, use the same env the close-out log records:

```text
cd umst-concrete-cartridge && UMST_REQUIRE_B8=1 uv run --with pytest --with trimesh pytest notebooks/tests/test_print_ready.py -v
```

**Budgets:** `bash notebooks/check_shell_artifact_budgets.sh` from **`umst-concrete-cartridge/`** — re-run **pass** (GIF/STL under README C9 caps) on 2026-05-11 verification pass.
