# Cursor todo recommendations — MaOS closeout

**Sources:** [`CURSOR_TODO_SEMANTICS.md`](CURSOR_TODO_SEMANTICS.md) (**Option B1** — **engineering-slice** completion vs **matrix-exact / row-%** closure; read this before interpreting “recommend complete” below), [`MAOS_CLOSEOUT_VERIFICATION_LOG.md`](MAOS_CLOSEOUT_VERIFICATION_LOG.md) (append-only ladder; **present** in this workspace), [`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md) (matrix **#1** / **#6** and scoring rubric), [`Solver-Status.md`](Solver-Status.md) (milestone matrix, B6 appendix, `gates_track_b8_all_pass`, **int-striatus** checklist), [`CURSOR_TODO_MERGE_FP_GAPS.md`](CURSOR_TODO_MERGE_FP_GAPS.md) (**gap-*** disposition), [`MAOS_PLAN_ALIGNMENT_REPORT.md`](MAOS_PLAN_ALIGNMENT_REPORT.md) (todo ↔ matrix tensions), [`MAOS_MATRIX_AUDIT_SNAPSHOT.md`](MAOS_MATRIX_AUDIT_SNAPSHOT.md) (alignment-audit lanes **#5 / #9 / #10** + Gate B).

**Semantics:** “**Recommend complete**” here means the **named Cursor id** may reflect **completed** for an **engineering slice** or a **closed gap id** without implying the parent **Verification completion matrix** row jumped to **100%**. Gated milestones (**`m1-b8`**, **`int-striatus`**, **`m6-dec`**, etc.) follow **Solver-Status** + matrix **Exact acceptance** — not Cursor tick inflation. See [`CURSOR_TODO_SEMANTICS.md`](CURSOR_TODO_SEMANTICS.md).

**Hard rule (this file):** do **not** recommend marking **`m1-b8`** or **`int-striatus`** as complete until **`gates_track_b8_all_pass`** is **`true`** in committed **`striatus_shell_v0.4.print_ready.json`** (sibling **`umst-concrete-cartridge/`**). Evidence for that rule is quoted under **`m1-b8`** below.

---

## Milestone / integration todos

### `m1-b6` — **Keep pending**

**Evidence:** full **`shell_topology_rib_pattern_full_v04`** run (**200** outers, **`--release`**, **`UMST_SHELL_RIB_PATTERN=1`**) **FAIL** at greyness (**0.510002** vs **&lt; 0.15**); **`xy_var ≈ 3.37×10⁻⁸`**; Adam skips on NaN at outers 5 and 9 — [`Solver-Status.md`](Solver-Status.md) appendix *m1-b6 honest rerun (2026-05-11)*.

**Note:** matrix [**#1**](VERIFICATION_COMPLETION_MATRIX.md) still lists **25%** and acceptance including **`mean(4ρ(1−ρ)) < 0.15`** at **40×40×4** / **200** iters — not met.

---

### `m1-b8` — **Keep pending** (blocked on **`gates_track_b8_all_pass`**)

**Evidence:** committed **`striatus_shell_v0.4.print_ready.json`** has **`gates_track_b8_all_pass`: false** because **`gate_topo_complexity_b7`** and **`gate_density_xy_variance_b8`** are **false** (genus **0**, χ **2**; planar density variance **≪ 0.1**); **`UMST_REQUIRE_B8=1 pytest …`** **fails** until regeneration yields rollup **true** — [`Solver-Status.md`](Solver-Status.md) *Matrix row 1 — milestone gates (`m1-l` / `m1-b8`)*.

**Do not** recommend completion until **`gates_track_b8_all_pass`** is **true** (rollup = **`gate_topo_complexity_b7` ∧ `gate_volume_fraction_mesh_b7` ∧ `gate_density_xy_variance_b8`** per same file *`gates_track_b8_all_pass` semantics*).

---

### `m1-l` — **Keep pending**

**Evidence:** Track L artefacts exist and **`test_striatus_stl_feasibility`** / VF band can pass, but **shell topology / rib texture** acceptance (**`gate_topo_complexity_b7`**, **`gate_density_xy_variance_b8`**) is **not** met; **`gates_track_b8_all_pass`** stays **false** — [`Solver-Status.md`](Solver-Status.md) *`m1-l` (Track L committed artefacts)*.

Matrix [**#1**](VERIFICATION_COMPLETION_MATRIX.md) **Exact acceptance** still requires B8 print-ready (**genus ≥ 1**, **variance ≥ 0.1**, VF band) and Track L committed assets under stated thresholds — not fully satisfied while topology gates fail.

---

### `m6-dec` — **Keep pending**

**Evidence:** photonics / DEC row is **50%**; **open:** incidence from **assembled** meshes, **metric / Hodge** weights, **volumetric 3D** complexes, **sparse** Krylov, **complex ε** + **PML** on patch path, BCs beyond gauge **pin** — [`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md) table row **#6** *Blocker* / *Exact acceptance*.

[`Solver-Status.md`](Solver-Status.md) **DEFERRAL — Photonics** (and **Plan-gap / photonics backlog** cross-ref to [`GAP_AUDIT.md`](../GAP_AUDIT.md) phase **7**) aligns: small-patch milestones exist; production **2D/3D** DEC vector curl–curl acceptance remains open.

---

### `int-striatus` — **Keep pending** (blocked on **`gates_track_b8_all_pass`** for Ring‑1 honesty)

**Evidence:** checklist item 3 — while committed **`gates_track_b8_all_pass`** is **`false`**, **`test_print_ready_track_b8_topology_gates`** **skips**; closing B8 is **not** a doc edit — re-run **`optimize_shell_3d`** / **`_run_shell_demo.sh`** at **40×40×4**, **200** outers, re-export, then confirm pytest — [`Solver-Status.md`](Solver-Status.md) *int-striatus — todo close criteria (honest)*.

**Do not** recommend marking **int-striatus** complete until **`gates_track_b8_all_pass`** is **true** (same gating as **`m1-b8`**).

---

## `gap-*` Cursor todos

### `gap-ci-physics-allowlist` — **Recommend complete** (id closed)

**Evidence:** *Already closed* per [`CURSOR_TODO_MERGE_FP_GAPS.md`](CURSOR_TODO_MERGE_FP_GAPS.md); allowlist covers audited files and **`bash umst-manifold/scripts/check_physics_no_gradient_break.sh`** is expected **0** on a clean tree — [`FP_CATEGORICAL_BURN.md`](FP_CATEGORICAL_BURN.md) *Update 2026-05-11 (`gap-ci-physics-allowlist`)*.

---

### `gap-track14` — **Recommend complete** (id closed)

**Evidence:** *Already closed* per [`CURSOR_TODO_MERGE_FP_GAPS.md`](CURSOR_TODO_MERGE_FP_GAPS.md) — implicit Newton PNP / dispatch / verification memo scope closed separately; **band LU parity (`fp_001`)** and follow-ons remain **open** under **`fp_*`** backlog, not under this **`gap-`** id.

---

### `gap-fp-inner-loop-syncs` — **Keep pending** (engineering follow-on)

**Evidence:** canonical stance is documented (outer **`iterate_until` vs `repeat_controlled`** does not remove per-iteration `.into_scalar()` in CG); **optional future work** (batched reductions, fused ops, deferred stopping) is a **separate** design + review — [`FP_FIXED_POINT_CANONICAL.md`](FP_FIXED_POINT_CANONICAL.md) *Branch note (`gap-fp-inner-loop-syncs`)*.

---

## Summary counts

| Recommendation | Count | Items |
| --- | ---: | --- |
| **Recommend complete** | **2** | `gap-ci-physics-allowlist`, `gap-track14` |
| **Keep pending** | **6** | `m1-b6`, `m1-b8`, `m1-l`, `m6-dec`, `int-striatus`, `gap-fp-inner-loop-syncs` |

**Totals:** **recommend-complete = 2**, **keep-pending = 6** (for the listed milestone/integration items plus all **`gap-*`** named in-repo).

**Matrix-100 note:** These counts do **not** claim any matrix row is **100%** except where [`Solver-Status.md`](Solver-Status.md) already states so for that lane. **`gap-*`** “complete” rows are **process / allowlist** closures, not **#1–#10** acceptance completion.
