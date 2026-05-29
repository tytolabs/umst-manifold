<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# End-condition report (UMST gate / manifest matrix)

**Generated:** 2026-05-21T22:24:58Z (UTC) (full M1–M11 + MX matrix replay; `UMST_REQUIRE_FORMAL_EXPORT=1 verify_umst_stack.sh` exit 0; epistemic+trace log guard in stack script)  
**Toolchain (host):** Rust **1.86** (Homebrew); pin in `rust-toolchain.toml` is **1.88** (CI) — see yellow note below  
**Crate root:** `umst-manifold/`  
**Catalog lock:** `artifacts/catalog.lock.json` — composed digest `0697014fb5b90a3a…` (full `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227`), **119** modules (unified cross-repo pin; primary fiber **69** @ `c1d9ba2aa402…`)  

## Verdict

| Overall | Result |
|---------|--------|
| **End condition** | **PASS** — all matrix steps exited 0; `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` exit 0 |

**Legend:** 🟢 GREEN = exit 0, clean · 🟡 YELLOW = exit 0 with warnings or intentional skip · 🔴 RED = non-zero exit

## Matrix (M*)

| ID | Step | Command (summary) | Exit | Status | Tests / notes |
|----|------|-------------------|------|--------|----------------|
| M1 | Default check | `cargo check` | 0 | 🟢 | Compile-only default features |
| M2 | `formal-witness` | `cargo test --features formal-witness --test formal_witness` | 0 | 🟢 | 3 passed |
| M3 | `ros2-contract` + `serde` | `cargo test --features ros2-contract,serde --test ros_contract_serde_roundtrip` | 0 | 🟢 | 3 passed |
| M4 | `gate-server-bin` | `cargo test --features gate-server-bin --test gate_server_http` | 0 | 🟢 | 1 passed (`POST /gate` localhost) |
| M5 | `manifold-manifest` on supercap | `cargo check -p umst-supercap-cartridge --features manifold-manifest` (path dep `../../../umst-manifold`) | 0 | 🟢 | Check only; no manifold warnings |
| M6 | `dual_run_parity` | `cargo test --test gate_dual_run_parity -- --nocapture` | 0 | 🟢 | Golden **8/8 (100%)**; live subprocess **8/8 (100%)** |
| M7 | `embodied_orchestrator` | `cargo test --test embodied_orchestrator` | 0 | 🟢 | 8 passed (incl. Kleisli host route) |
| M8 | `gate_dual_run` (fixture) | `cargo build --bin gate_dual_fixture` in `umst-prototype/src/rust/core` | 0 | 🟡 | Subprocess helper; `umst-core` unused_import warning (1) |
| M9 | `gate_dual_run` (re-run) | Repeat M6 after M8 | 0 | 🟢 | Same 100% golden + live agreement |
| M10 | `gate_kleisli` + `gate_reject_catalog_id` | `cargo test --test gate_kleisli --test gate_reject_catalog_id` | 0 | 🟢 | 6 + 6 passed |
| M11 | `gate_adversarial` | `cargo test --test gate_adversarial` | 0 | 🟢 | FNR=0 on 75-case golden |
| MX | `ndarray` integration sweep | `cargo test --tests --features ndarray` | 0 | 🟢 | All integration binaries + lib unit lane |
| MX | Release manifest witness | `cargo test --features formal-witness --test manifest_strict_witness` | 0 | 🟢 | StrictCatalogMatch + digest mismatch reject (3 tests) |
| MX | Dual-pin catalog lock | `scripts/catalog_lock_verify.py` vs Lean export (composed + both fibers) | 0 | 🟢 | Via `verify_umst_stack.sh` when `UMST_REQUIRE_FORMAL_EXPORT=1` |
| MX | `catalog_lock_119` | `cargo test --test catalog_all_ids_registered catalog_lock_module_count_matches_upstream_export_119` | 0 | 🟢 | Lock `module_count` == upstream export **119** |
| MX | `witness_priority_queue` | `cargo test --test witness_priority_queue` | 0 | 🟢 | 4 passed; adaptive priority tests only (not hot path) |
| MX | `catalog_incremental_graph_drift` | `cargo test --test catalog_incremental_graph_drift` | 0 | 🟢 | 1 passed |
| MX | `ci_god_grade_profile` | `cargo test --test ci_god_grade_profile` | 0 | 🟢 | 2 passed |
| MX | `epistemic_trace_schema` | `cargo test --features ros2-contract,serde --test epistemic_trace_schema` | 0 | 🟢 | 13 passed (G.2) |
| MX | `trace_calibration` | `cargo test --features trace-calibration --test trace_calibration` | 0 | 🟢 | 8 passed (G.3) |
| MX | Prototype adversarial gate | `umst-prototype_2/scripts/test_gate_adversarial.py` | 0 | 🟢 | FNR=0 (75 cases) via `verify_umst_stack.sh` |
| MX | Stack verify bundle | `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` | 0 | 🟢 | Full recursive lane; epistemic G.2/G.3/R6 **log guard** at tail |
| MX | `regime_soundness_claims_allowlist` | `cargo test --features ros2-contract,serde,trace-calibration --test regime_soundness_claims_allowlist` | 0 | 🟢 | 1 passed (J.3) |

### Intentional skips (honest)

| ID | Step | Reason | Status |
|----|------|--------|--------|
| SKIP | Combined feature one-shot | `formal-witness,ros2-contract,serde,gate-server-bin` in single `cargo test` not run; lanes exercised **individually** (M2–M4, MX) per [`VERIFY.md`](VERIFY.md) §2.2 | 🟡 |
| SKIP | Host toolchain 1.88 | Local run used Homebrew **1.86**; CI uses `rust-toolchain.toml` **1.88** — matrix still exit 0 on 1.86 | 🟡 |

### Feature combo coverage

The optional gate lane features were exercised **individually** (matches [`VERIFY.md`](VERIFY.md) §2.2 and `scripts/verify_umst_stack.sh`):

- `formal-witness` → `formal_witness` integration test  
- `ros2-contract` + `serde` → `ros_contract_serde_roundtrip`  
- `gate-server-bin` → `gate_server_http`  
- `manifold-manifest` → downstream **supercap** cartridge check (M5)  
- `trace-calibration` → `trace_calibration` (MX)  
- `witness_priority_queue` + `catalog_lock_119` → explicit in `verify_umst_stack.sh` (recursive, 2026-05-21)

Combined one-shot (not run; prior art in VERIFY):

```bash
cargo test --features formal-witness,ros2-contract,serde,gate-server-bin \
  --test formal_witness --test ros_contract_serde_roundtrip --test gate_server_http
```

## Gate composition (commutative diagram)

**Reading:** Integration targets `gate_kleisli`, `gate_reject_catalog_id`, `embodied_orchestrator`, and `gate_dual_run_parity` are the executable witness that Kleisli registration, mix-registry hydration, and dual-run evaluation **commute** on the same fixture graph (same inputs → same reject/accept catalog ids whether composed via registry route or direct evaluator).

| Vertex (test) | Morphism checked |
|---------------|------------------|
| `gate_kleisli` | Kleisli unit + registry slug routing |
| `gate_reject_catalog_id` | Reject object arrows stable under composition |
| `embodied_orchestrator` | Host/gateway/registry triangle (8 scenarios) |
| `gate_dual_run_parity` | Manifold vs prototype golden + live subprocess |

## Gate dual-run parity (detail)

Target: `tests/gate_dual_run_parity.rs` (`[[test]]` name `gate_dual_run_parity`).

| Lane | Agreement |
|------|-----------|
| Manifold mix-proposal vs **prototype golden** fixtures | 8/8 (100%) |
| Manifold vs **live** `gate_dual_fixture` subprocess | 8/8 (100%) |

Fixtures: `tests/data/gate_dual_run_fixtures.json`.  
Prototype helper: `umst-prototype/src/rust/core` binary `gate_dual_fixture`.

## Embodied orchestrator (detail)

Target: `tests/embodied_orchestrator.rs` (auto-discovered integration test).

All eight scenarios passed: gateway-only accept, mix registry forward/reverse hydration, host CD mass reject, Kleisli unit route + missing-registry slug, dual-run accept/require host step.

## Warnings recorded (non-failing)

No `cargo` warnings on **umst-manifold** in this matrix. Warnings appeared only on **transitive / sibling** crates during M8:

| Source | Kind | Count (approx.) |
|--------|------|-----------------|
| `umst-core` (M8 build) | `unused_imports`: `DEFAULT_S_INTRINSIC_MPA` | 1 |

**Policy:** warnings logged on sibling crates (M8); compile fixes applied only where matrix/tests **failed** (none this run).

## Fixes applied (this run)

- **`scripts/verify_umst_stack.sh`:** epistemic+trace steps recorded via `verify_step_echo` + fail-closed log guard (`VERIFY_STEP_LOG`); merged `EXIT` trap for catalog temps.
- **`scripts/w8_publish_readiness.sh`:** `if [[ ${#missing[@]} -gt 0 ]]` (bash 3.2-safe; fixes `syntax error near '('` under integration-test invoke).
- **Report only:** M* matrix all exit 0 after fixes; `MX_ndarray` sweep green (was failing via `w8_publish_readiness` script parse).

## Integration sweep

| Command | Exit | Notes |
|---------|------|-------|
| `cargo test --tests --features ndarray` | 0 | Integration binaries + lib unit lane `test result: ok` |

## Artifacts

- Full console log: `/tmp/end_condition_matrix_20260521T222352Z.log` (local run; not committed)
- Stack verify log: `/tmp/verify_umst_stack_run_final.log`

## Related docs

- Developer commands: [`VERIFY.md`](VERIFY.md)  
- Agent deliverable status: [`AGENT_STATUS.md`](AGENT_STATUS.md)  
- Prototype gate map: [`PROTOTYPE_GATE_MAP.md`](PROTOTYPE_GATE_MAP.md), `umst-prototype/docs/GATE_SERVER.md`
