# Cursor todo semantics — Option B1 (MaOS closeout)

This document fixes **Option B1**: a single vocabulary for **Cursor todo `status`** vs **verification matrix “Completion %”** vs **Solver-Status honest gates**, so agents do not mark a todo **completed** when the matrix row is still **partial** or a **rollup gate** is **false**.

## Layers (do not conflate)

1. **Cursor todo `completed`** — implementation or doc task **as written in the todo text** (e.g. “add script”, “fix clippy lint”). Completing the todo does **not** upgrade a matrix row unless the todo explicitly says so.
2. **Matrix Completion %** ([`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md) + rubric in [`Solver-Status.md`](Solver-Status.md)) — **physics / scale / acceptance** truth. It moves only when the **Exact acceptance criterion** for that row is met (or the criterion is formally revised with review).
3. **Milestone rollup booleans** (e.g. **`gates_track_b8_all_pass`**) — **hard gates** for Ring‑1 checklist items. If **false**, related todos (**`m1-b8`**, **`int-striatus`**, **`m1-l`**) stay **pending** even when tests pass with skip semantics.

## Rules for agents

| Situation | Cursor todo | Matrix % | Notes |
| --- | --- | --- | --- |
| CI green, matrix acceptance **not** met | **pending** or **in_progress** with “blocked on acceptance X” | **unchanged** | Example: row **#2** ratio-band test passes but **§R2.1** within-5% gate is still **ignored / open**. |
| **`gates_track_b8_all_pass` false** | **`m1-b8`**, **`int-striatus`**, **`m1-l`** → **pending** | **unchanged** | Do **not** mark complete until committed sidecar flips **true** (or acceptance text is explicitly renegotiated). |
| Doc-only “verification log” entry | Can **complete** if the todo was “append log” | **unchanged** | Logging records **honest** status; it does not inflate %. |
| FP backlog row | **`fp_*`** todos track engineering debt | Independent of matrix unless the FP item **is** the matrix blocker | **`fp_004`** remains closed under **`gap-ci-physics-allowlist`** per merge doc. |

## `completed_subtitle` / timeline hygiene

Parent timeline subtitles should use **past-tense** for what was **actually done** (e.g. “Recorded B8 gate failure”), not “Closed matrix row #1” when **#1** is still at **25%**.

## Cross-references

- [`CURSOR_TODO_RECOMMENDATIONS_MAOS_CLOSEOUT.md`](CURSOR_TODO_RECOMMENDATIONS_MAOS_CLOSEOUT.md) — per-id recommendations.
- [`MAOS_CLOSEOUT_VERIFICATION_LOG.md`](MAOS_CLOSEOUT_VERIFICATION_LOG.md) — append-only command receipts.
- [`CURSOR_TODO_MERGE_FP_GAPS.md`](CURSOR_TODO_MERGE_FP_GAPS.md) — **`fp_*`** vs **`gap-*`** dispositions.
