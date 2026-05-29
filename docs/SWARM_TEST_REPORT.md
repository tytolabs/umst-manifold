# Swarm cargo test report

**Run:** 2026-05-21 · **Verified:** 2026-05-21T21:18:37Z (UTC) · **Workspace:** `MaOS-Workspace`

**Status SSOT:** [`AGENT_STATUS.md`](AGENT_STATUS.md) · **Handoffs:** [`PARALLEL_HANDOFFS.md`](PARALLEL_HANDOFFS.md) · **Witness ladder:** [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md)

---

## Process narrative

### Swarm waves

| Wave | Activity | Agents / scripts | Outcome |
|------|----------|------------------|---------|
| **Cargo sweep** | Full `cargo test` on manifold + cartridges | S11, coordinator | All three crates **PASS** (table below) |
| **Gate parity** | `gate_dual_run_parity`, Kleisli, CBF, HTTP | W4, W7, W10 | 8/8 dual-run; see [`END_CONDITION_REPORT.md`](END_CONDITION_REPORT.md) |
| **Stack verify** | `scripts/verify_umst_stack.sh` | S8, W2, W10 | Exit **0** @ 2026-05-21T20:50:20Z |
| **Read-only audit** | Six docs on disk | S1–S11 swarm | No code changes; evidence in [`TODO_COMPLETION.md`](TODO_COMPLETION.md) |

Tests prove **witness alignment**, not god-grade closure: R0 digest pin is CI; R1/R3 parity is 8/8; R4 Kleisli lacks hot-path `GateEvaluator`; R5 git manifest bridge still **PARTIAL** (W8).

### Witness ladder (why tests are ordered this way)

[`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md) requires **lazy** composition: CD → Landauer → constitutive → probe. The verification recipe below runs **catalog first (R0)**, then **host gates (R1/R3)**, then **CBF/formal (R2/R5)**, then **dual-run (P7 parity functor)** — matching stack script order, not reversing failure priority on a single step.

### Completion % (swarm scope)

| Metric | Value |
|--------|-------|
| Crates swept (manifold + 2 cartridges) | **3/3 PASS** |
| Plan infra todos | **86%** strict / **93%** local ([`UMST_PROGRESS_REPORT.md`](UMST_PROGRESS_REPORT.md)) |
| Migration P0–P7 | **~88%** — **P7** partial (tests ✅, production adversarial open) |
| God-grade checklist | **60%** |

### P0–P7 vs test evidence

| Phase | Test / script evidence | Swarm verdict |
|-------|------------------------|---------------|
| P0 | Docs only (`GateUnificationSpec.md`) | ✅ |
| P1 | `verify_umst_stack.sh` + drift workflow | ✅ |
| P2 | `catalog_all_ids_registered` (4 passed) | ✅ |
| P3 | `gate_kleisli` (4 passed) | ✅ |
| P4 | `gate_cbf_parity`, `cbf`, `formal_witness` | ✅ |
| P5 | `embodied_orchestrator` (6 passed) | ✅ |
| P6 | `gate_server_http` (1 passed) | ✅ |
| P7 | `gate_dual_run_parity` 8/8; adversarial optional | ⚠️ |

---

## Verification recipe (copy-paste)

### Tier 1 — Swarm cargo sweep (this report)

```bash
cd umst-manifold && cargo test -p umst-manifold
cd umst-concrete-cartridge && cargo test -p umst-concrete-cartridge --features manifest-bridge
cd umst-supercap-cartridge && cargo test -p umst-supercap-cartridge
```

### Tier 2 — Gate + catalog (P2–P7)

```bash
cd umst-manifold
cargo test --test catalog_all_ids_registered
cargo test --test gate_parity_fixture --test gate_kleisli --test gate_cbf_parity
cargo test --test gate_dual_run_parity -- --nocapture
cargo test --features formal-witness,ros2-contract,serde,gate-server-bin \
  --test formal_witness --test ros_contract_serde_roundtrip --test gate_server_http
```

### Tier 3 — Monorepo stack (R0 + bidirectional)

```bash
UMST_REQUIRE_FORMAL_EXPORT=1 \
  UMST_FORMAL_ROOT=/path/to/umst-formal-double-slit \
  bash umst-manifold/scripts/verify_umst_stack.sh
```

Full operator manual: [`VERIFY.md`](VERIFY.md). End-condition matrix M1–M9: [`END_CONDITION_REPORT.md`](END_CONDITION_REPORT.md).

---

## PASS / FAIL

| Crate | Command | Result | Notes |
|-------|---------|--------|-------|
| `umst-manifold` | `cargo test -p umst-manifold` | **PASS** | exit 0; **151** tests (65 lib + integration + doc) |
| `umst-concrete-cartridge` | `cargo test -p umst-concrete-cartridge --features manifest-bridge` | **PASS** | exit 0; **43** passed, 1 ignored (`proof_status_refresh`) |
| `umst-supercap-cartridge` | `cargo test -p umst-supercap-cartridge` | **PASS** | exit 0; **39** passed (31 lib + 6 formal + 2 contract) |

## Compile fixes

None (no trivial one-line breaks).

## Detail

### umst-manifold

- `test result: ok` — **151** passed across lib (**65**), integration suites, and doc-tests (0 doc tests).
- Full command: `cd umst-manifold && cargo test -p umst-manifold`
- **P7 highlight:** `gate_dual_run_parity` — manifold vs prototype golden **8/8 (100%)**; live `gate_dual_fixture` subprocess **8/8 (100%)** @ 2026-05-21 ([`END_CONDITION_REPORT.md`](END_CONDITION_REPORT.md) M6/M9).

### umst-concrete-cartridge

- `test result: ok` — **43** passed, **1** ignored (`proof_status_refresh_markdown_on_disk`).
- Full command: `cd umst-concrete-cartridge && cargo test -p umst-concrete-cartridge --features manifest-bridge`
- Includes `manifest_bridge_catalog_grounding` + `manifest_bridge_gate_facade` (W8).

### umst-supercap-cartridge

- `test result: ok` — **31** lib + **6** `formal_anchors` + **2** `manifold_contract`; bin targets 0 tests each.
- Warnings only (`missing_docs`, upstream `maos-opt-primitives` dead_code); no failures.
- Full command: `cd umst-supercap-cartridge && cargo test -p umst-supercap-cartridge`

---

## Pending after swarm (P0–P7 / ops)

| Item | Owner | Unblocks |
|------|-------|----------|
| **P7** adversarial in drift CI | MaOS CI | FNR=0% parity with prototype baseline |
| **P7** 1-week production dual-run monitor | ops | Plan exit “disagreement rate < threshold” |
| **W8** git publish + cartridge CI | manifold publish | R5 without `[patch]` |
| **P8** delete prototype filter body | prototype lane | [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md) parity functor → identity |

---

## Related docs

- [`AGENT_STATUS.md`](AGENT_STATUS.md) — W1–W10 / S1–S11 + P0–P7 table
- [`PARALLEL_HANDOFFS.md`](PARALLEL_HANDOFFS.md) — lane handoff narrative
- [`TODO_COMPLETION.md`](TODO_COMPLETION.md) — per-todo evidence commands
