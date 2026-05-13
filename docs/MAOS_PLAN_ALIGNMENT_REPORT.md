# MaOS solver close-out — plan ↔ repo alignment report

**Generated:** 2026-05-11. **Canonical plan YAML + body:** [`MAOS_SOLVER_100_PERCENT_CLOSEOUT_PLAN.md`](MAOS_SOLVER_100_PERCENT_CLOSEOUT_PLAN.md). **Repo scoring:** [`Solver-Status.md`](Solver-Status.md) (**Completion scoring (v0.4)** + **Verification scope (remaining)** table @ **#1–#10**), [`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md).

**Legend — Plan status:** Reflects **Cursor session bookkeeping** from the parent execution transcript (TodoWrite “completed”), **not** automatic proof that matrix **Exact acceptance** rows moved to **100%**. Where the transcript did not record a completion tick, status is **open / not recorded**.

**Cursor-equivalent:** Stable lane ids matching [`MAOS_SOLVER_CLOSEOUT_PLAN_SYNC.md`](MAOS_SOLVER_CLOSEOUT_PLAN_SYNC.md) matrix rows (extended with `solver-closeout-ci` / `solver-closeout-integration` for non-matrix todos).

**Phase B — Cursor todo semantics (Option B1):** [`CURSOR_TODO_SEMANTICS.md`](CURSOR_TODO_SEMANTICS.md) (**engineering-slice** todos vs **matrix / acceptance** todos; no false **100%** equivalence). **Alignment audit checklist (`maos-gate-b-backward-chain`, `maos-matrix9-exact-audit`, `maos-matrix10-combined-audit`, `maos-m5-matrix-narrative-sync`):** [`MAOS_MATRIX_AUDIT_SNAPSHOT.md`](MAOS_MATRIX_AUDIT_SNAPSHOT.md).

---

## Per–todo-id alignment

| Todo id | Plan status (Cursor transcript) | Cursor-equivalent | Matrix row / % (`Solver-Status` checklist + main table) | Repo reality (evidence) |
| --- | --- | --- | --- | --- |
| **x-cut-ci** | completed | `solver-closeout-ci` | Cross-cutting (CI mirrors [`Solver-Status.md`](Solver-Status.md) **CI** bullet) | Green transcript claim on **`x-cut-ci`**; ongoing obligation: full `solver-experimental` + `check_solver_status.py` after **Solver-Status** edits. |
| **x-stash** | completed | `solver-closeout-stash` | — | Hygiene only; does not advance matrix %. |
| **m1-b6** | **open** (B6 greyness miss documented) | `solver-closeout-v04-01` | **#1** — **25%** | [`Solver-Status.md`](Solver-Status.md): full **`shell_topology_rib_pattern_full_v04`** **FAIL** greyness **≈0.51** vs **< 0.15**; appendix *m1-b6 honest rerun*. |
| **m1-b8** | **open** | `solver-closeout-v04-01` | **#1** — **25%** | **`gates_track_b8_all_pass`: false** in committed **`striatus_shell_v0.4.print_ready.json`** — [`Solver-Status.md`](Solver-Status.md) *Matrix row 1 — milestone gates*. |
| **m1-l** | **open** | `solver-closeout-v04-01` | **#1** — **25%** | Track L artefacts exist; **genus / variance** acceptance **not** met (`gate_topo_complexity_b7`, `gate_density_xy_variance_b8` false) — same subsection. |
| **m1-doc** | completed (transcript) | `solver-closeout-v04-01` | **#1** — **25%** | **Tension:** todo ticked **completed** while matrix **#1** remains **25%** and checklist bullets still list full B6/B8/L proof — docs sync ≠ acceptance closure. |
| **m2-sri** | completed (transcript) | `solver-closeout-v04-02` | **#2** — **25%**; mechanics row **min(#2,#10)** | **Tension:** Cursor **completed** vs **[`Solver-Status.md`](Solver-Status.md)** mechanics row: §R2.1 **within-5%** Kirchhoff **not** default-CI; ratio-band harness explicitly **not** §R2.1 closure. |
| **m2-doc** | completed (transcript) | `solver-closeout-v04-02` | **#2** — **25%** | Same as **m2-sri**: documentation ticks do not raise matrix **#2** % without acceptance tests. |
| **m3-stagger-stop** | completed (transcript) | `solver-closeout-v04-03` | **#3** — **50%** | Outer stopping harness landed per transcript; **Solver-Status** fracture lane still **50%** — §7 **Γ / ψ⁺ / u↔d** backlog remains. |
| **m3-memo7** | completed (transcript) | `solver-closeout-v04-03` | **#3** — **50%** | **Tension:** todo **completed** vs matrix **#3** exact acceptance (memo §7 **full** bullet list: driven **ψ⁺**, broader **(l₀,h)**, within-step THMC stagger, etc.) — still **partial** in [`Solver-Status.md`](Solver-Status.md) + matrix row. |
| **m4-maint** | completed (transcript) | `solver-closeout-v04-04` | **#4** — **100%** | Acoustics row **met**; maintenance-only todo aligns with repo (**[`Solver-Status.md`](Solver-Status.md)** row **`solvers::acoustics`** **100%**). |
| **m5-scale** | completed (transcript) | `solver-closeout-v04-05` | **#5** — **75%** | **Tension (flagged):** Cursor **completed** while **[`Solver-Status.md`](Solver-Status.md)** electrochemistry row + item **#5** still document **O((3N)³)** dense-expand inner ceiling and **band LU not wired into `try_solve`**; [`MAOS_CLOSEOUT_VERIFICATION_LOG.md`](MAOS_CLOSEOUT_VERIFICATION_LOG.md) Row **5** states **m5-scale** dense ceiling **remains**. |
| **m5-graph** | completed (transcript) | `solver-closeout-v04-05` | **#5** — **75%** | Experimental non-chain Poisson / graph hooks exist; matrix **#5** **blocker** column still lists general-graph implicit Newton and variable-ε gaps — **75%** honest. |
| **m6-dec** | **open** | `solver-closeout-v04-06` | **#6** — **50%** | [`Solver-Status.md`](Solver-Status.md) photonics: **dual Hodge**, assembled incidence, **sparse** large-**N**, **complex ε + PML** on patch path — still **open**; aligns with [`MAOS_SOLVER_CLOSEOUT_PLAN_SYNC.md`](MAOS_SOLVER_CLOSEOUT_PLAN_SYNC.md) § Matrix #6 / **m6-dec**. |
| **m6-fresnel** | completed (transcript) | `solver-closeout-v04-06` | **#6** — **50%** | Fresnel / tensor-**ε** slices improved per transcript; **row #6** still **50%** until DEC production bullets close. |
| **m7-longrun** | completed (transcript) | `solver-closeout-v04-07` | **#7** — **50%** | **Tension:** todo **completed** vs matrix **#7** / [`Solver-Status.md`](Solver-Status.md): long-run steady **L²** Poiseuille acceptance **still open**; short-horizon **65×17** smokes are **not** that gate. |
| **m7-mac** | completed (transcript) | `solver-closeout-v04-07` | **#7** — **50%** | Transcript: opt-in MAC/open-**x** hooks merged — **partial** milestone; matrix **#7** remains **50%** until MAC/long-run acceptance satisfied. |
| **m8-jfnk** | completed (transcript) | `solver-closeout-v04-08` | **#8** — **75%** | JFNK slice landed per transcript; [`Solver-Status.md`](Solver-Status.md) THMC lane: **large-N**, AD-safe ‖R‖ exit, **u↔d** stagger, adaptive **dt** — **still open** at scale. |
| **m8-scale-ad** | completed (transcript) | `solver-closeout-v04-08` | **#8** — **75%** | **Tension:** todo **completed** vs transcript’s own caveat (**adaptive dt** / **u↔d** “matrix-scale work”); [`Solver-Status.md`](Solver-Status.md) lists host-read ‖R‖ norms and within-step stagger as remaining. |
| **m9-upscale** | completed (transcript) | `solver-closeout-v04-09` | **#9** — **25%** | **Tension (flagged):** Cursor **completed** vs **[`Solver-Status.md`](Solver-Status.md)** statistical-mechanics lane + matrix **#9**: **virial / MD reference K**, **`γ_gc`** placeholder, AD-safe **`K`** — **still open** ([`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md) row **#9** MD-reference bullets). |
| **m10-transient** | completed (transcript) | `solver-closeout-v04-10` | **#10** — **25%** | Doc slice A/B may be refined; **no** default-CI **vector** transient mechanics stack per [`Solver-Status.md`](Solver-Status.md) item **#10** + matrix row. |
| **m10-contact** | completed (transcript) | `solver-closeout-v04-10` | **#10** — **25%** | **Tension (flagged):** Cursor **completed** vs **[`Solver-Status.md`](Solver-Status.md)** item **#10** — **contact / friction** explicitly **none** in shipped stack; matrix **Next PR slice B** defers contact engineering. |
| **int-striatus** | **partial / open** | `solver-closeout-integration` | **#1** coupling + manifold lanes | [`MAOS_SOLVER_100_PERCENT_CLOSEOUT_TICKS.md`](MAOS_SOLVER_100_PERCENT_CLOSEOUT_TICKS.md): **no** until **`gates_track_b8_all_pass`**; transcript: stub/central-FD progress, **not** full AD chain. |

---

## Tensions summary (examples requested)

| Topic | Plan / Cursor signal | Repo signal |
| --- | --- | --- |
| **m5-scale** | Marked **completed** in Cursor | Matrix **#5** **75%**; [`Solver-Status.md`](Solver-Status.md) item **#5** + electrochemistry row still describe **dense (3N)² / O((3N)³)** full-SG path and **unwired** band LU — see also [`MAOS_CLOSEOUT_VERIFICATION_LOG.md`](MAOS_CLOSEOUT_VERIFICATION_LOG.md). |
| **m10-contact** | Marked **completed** | Matrix **#10** / [`Solver-Status.md`](Solver-Status.md): contact **deferred**; no verification stack — **25%** row. |
| **m9-upscale** | Marked **completed** | Matrix **#9** / [`Solver-Status.md`](Solver-Status.md): Johnson **[B,4]** shipped as **host** bridge; **MD / virial reference K**, **`γ_gc`**, AD-safe **`K`** remain **open** — **25%** row. |

Additional high-signal tensions: **m3-memo7** completed vs **#3** still **50%**; **m7-longrun** completed vs **#7** long-run **L²** acceptance **open**; **m1-doc** completed vs **#1** still **25%**.

---

## Solver-Status mirror — manifold vs `umst-concrete-cartridge` (matrix **#1**, **#9**, **#10**)

Read-only compare of **[`umst-manifold/docs/Solver-Status.md`](Solver-Status.md)** vs **[`umst-concrete-cartridge/docs/Solver-Status.md`](../../umst-concrete-cartridge/docs/Solver-Status.md)** (cartridge declares manifold authoritative; refresh is expected to lag). **Completion (%)** figures were **not** edited here — any future **%** change belongs with **`check_solver_status.py`** after touching **`Solver-Status.md`**.

| Matrix row | Honest mismatch / drift |
| --- | --- |
| **#1** (Topology / shell) | **Verification scope — item 1:** Manifold adds **Quick vs full greyness** and **Quick vs demo roof** callouts (CI quick harness vs full B6 volume mean, **`optimize_shell_3d`** pressure/ramp vs B6 harness semantics). The cartridge mirror **omits** those paragraphs; a cartridge-only reader misses the explicit quick/full distinction unless they follow manifold. **int-striatus — checklist item 1:** Manifold gives one **`cargo test -p umst-concrete-cartridge --features solver-experimental`**; cartridge stages **default package** `cargo test`, then **`--features solver-experimental`**, then **pytest** — same intent (coupled gates), **not** the same one-liner. |
| **#9** (Statistical mechanics) | **Verification scope — item 9** text aligns. **Solver lanes — Statistical mechanics:** Manifold documents shipped **`[B,4]`** Johnson **`K_T`** via Burn column slices, scalar bridge integrations, and “**`[B,4]`** Johnson **`K`** is **always compiled**” (see manifold lane § **Closed** / **Path sketch** / **Feature gate sketch**). Cartridge still frames **`upscale_potentials` (today)** as **`[B,2]`**-only in the path-sketch table and treats **`[B,4]`** as **future API** in **Tensor channel design** / **Feature gate sketch** — **documentation drift** on matrix **#9** mechanics relative to manifold, not a claim that code differs. |
| **#10** (Transient / contact) | **Verification scope — item 10** core prose matches (quasi-static bar network vs **scalar** **`acoustics-newmark`**; **no** contact — memo link). Manifold includes a full **Matrix PR slices — item #10** subsection (slices **A** / **B** bullets). Cartridge **compresses** that into one **PR slices A/B (#10)** sentence pointing at manifold — **no contradiction**, but slice-level wording lives only on manifold; audit **#10** PR slices from manifold **`Solver-Status`**. |

---

## Recommended next engineering steps (ordered)

1. **Reconcile Cursor todo hygiene with evidence gates** — Reset or downgrade any **completed** todo whose scope is contradicted by [`Solver-Status.md`](Solver-Status.md) + [`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md) for the same matrix row (especially **m5-scale**, **m9-upscale**, **m10-contact**, **m7-longrun**, **m3-memo7**). **VERIFY:** human audit table vs this report; no Solver-Status edit required for the audit alone.

2. **Electrochemistry scale truth path (#5)** — Wire **band LU** with proven **δ** parity **or** ship matrix-free Newton correction; update **item #5** text only when [`try_solve`](../src/physics/solvers/electrochemistry.rs) path matches claimed complexity. **VERIFY:** `cargo test --features electrochemistry-pnp,solver-experimental --test pnp_debye_layer` + `python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set` **after** any [`Solver-Status.md`](Solver-Status.md) edit.

3. **Track L / B8 / B6 (#1)** — Single operator queue: regenerate **`final.npy` → export → pytest** until **`gates_track_b8_all_pass`: true**; schedule **`shell_topology_rib_pattern_full_v04`** proof run per P0 runbook. **VERIFY:** `UMST_REQUIRE_B8=1 pytest notebooks/tests/test_print_ready.py` (cartridge); optional `shell_topology_rib_pattern_full_v04` with **`UMST_SHELL_RIB_PATTERN=1`**.

4. **§R2.1 mechanics (#2)** — Implement BC-aligned plate test meeting matrix **within-X%** on default CI **or** explicitly lower matrix ambition with composer approval (document-only **not** preferred for **100%** claims). **VERIFY:** `cargo test --features mechanics-voigt-cauchy,topology-density-evolution,solver-experimental --test mechanics_analytic`.

5. **Photonics DEC (#6)** — Close **m6-dec**: metric-weighted operators + assembled incidence + sparse inner solves per [`Solver-Status.md`](Solver-Status.md) photonics DEFERRAL list. **VERIFY:** `cargo test --features photonics-fdfd,solver-experimental --test photonics_fresnel --test dec_identities`.

6. **Statistical mechanics bridge (#9)** — Deliver matrix **#9** bullets: MD or independent reference **K**, documented protocol, **`γ_gc`** path or deferral **in matrix**. **VERIFY:** `cargo test --features solver-experimental --test statmech_lj_bridge_contract --test statmech_lj_johnson_eos_reference` (+ optional `--features statistical-mechanics-johnson-reference`).

7. **Transient / contact (#10)** — Either land **default-CI** vector transient harness **or** keep docs/matrix aligned on deferral; **do not** mark **m10-contact** complete until tests exist. **VERIFY:** `cargo test --features solver-experimental --test mechanics_analytic` + matrix row **#10** review.

8. **int-striatus** — Full-chain finite **`backward()`** THMC → mechanics → fracture at small **N**, plus **`gates_track_b8_all_pass`** for Ring‑1 closure per [`CURSOR_TODO_RECOMMENDATIONS_MAOS_CLOSEOUT.md`](CURSOR_TODO_RECOMMENDATIONS_MAOS_CLOSEOUT.md). **VERIFY:** cartridge `verify_striatus_coupled_gates.sh` + manifold coupled tests cited in [`Solver-Status.md`](Solver-Status.md) *int-striatus* checklist.

---

## Top 3 misalignments (executive)

1. **m5-scale marked done vs matrix #5 / Solver-Status** — Production narrative still documents **dense cubic inner cost** and **unwired** band LU on the full-SG chain; **75%** bin unchanged.

2. **m10-contact (and m10-transient) marked done vs matrix #10** — **Contact** remains **out of scope** in shipped stack; **transient vector** mechanics **not** on default CI — row **25%**.

3. **m9-upscale marked done vs matrix #9** — **Johnson / [B,4]** helpers ≠ **virial / MD reference K** acceptance; **`γ_gc`** still placeholder — row **25%**.
