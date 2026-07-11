<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# Developer verification (`umst-manifold`)

Copy-paste commands for local checks. All paths assume the **crate root**:

```bash
cd /path/to/umst-manifold   # or workspace root/umst-manifold
```

**Toolchain:** `rust-toolchain.toml` pins **1.88** (required for `cargo clippy --all-targets` with the full optional graph). Install with `rustup toolchain install 1.88`.

**Catalog lock (current, v2 dual-pin):** `artifacts/catalog.lock.json` — composed R0 digest `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227` (**119** modules); per-fiber pins in `fiber_pins[]` (69 + 62). Policy: [`DUAL_PIN_ARCHITECTURE.md`](DUAL_PIN_ARCHITECTURE.md). Stale-doc audit: [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md). Override at build: `UMST_CATALOG=/path/to/lock.json`.

### Exit-0 ledger (`scripts/verify_umst_stack.sh`)

| Verified (UTC) | Command | Exit | Notes |
|----------------|---------|------|-------|
| **2026-05-29** | `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` | **0** | Stack verify OK; GitHub CI [run 26649667467](https://github.com/tytolabs/umst-manifold/actions/runs/26649667467) success; clippy fix [`fe22437`](https://github.com/tytolabs/umst-manifold/commit/fe22437); **G-02** concrete cartridge remote `manifest-bridge` closed; scoped blockers **G-03 + FFI** only |
| **2026-05-21T22:24:58Z** | `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` | **0** | Recursive pass: epistemic+trace **log guard** (`VERIFY_STEP_LOG`); `w8_publish_readiness.sh` bash `[[` array check; M* matrix all exit 0 |
| **2026-05-21T22:20:07Z** | `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` | **0** | R0 pin **119**; G.2/G.3 echoed + log guard; `ci_quality_profile` epistemic wiring test; solver tests gated |
| **2026-05-21T22:01:20Z** | `bash scripts/verify_umst_stack.sh` | **0** | Monorepo sibling formal + prototype E6; fixed duplicate `trace_calibration` integration target |
| 2026-05-21T21:18:04Z | `bash scripts/verify_umst_stack.sh` | 0 | Prior pin — see [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) |

Guard: `cargo test --test catalog_all_ids_registered catalog_lock_module_count_matches_upstream_export_119` asserts lock `module_count` and upstream `catalog.json` row count both stay **119**.

---

## 1. Check (compile / lint)

### 1.1 Default (CI portable path)

```bash
cargo check
cargo build
cargo build --examples
```

### 1.2 Format + clippy (matches `rust-solvers.yml` / manifold `rust.yml` lint job)

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo clippy --all-targets --features solver-experimental -- -D warnings
```

### 1.3 Solver research compile-only (PR gate in `rust.yml`)

```bash
cargo check --all-targets --features solver-stable,solver-research
```

### 1.4 Docs / solver table guard

```bash
python3 scripts/check_solver_status.py \
  --check-paths \
  --check-memo-links \
  --check-statmech-verification-set
```

### 1.5 Physics gradient guard (optional PR lane)

```bash
bash scripts/check_physics_no_gradient_break.sh
```

---

## 2. Test (default + gate lane)

### 2.1 Default integration + unit tests

```bash
cargo test
```

Equivalent explicit target sweep:

```bash
cargo test --tests
```

### 2.2 Gate / catalog parity (workspace root `umst-catalog-drift.yml`)

**Transition evidence (`GateCartridge`):** cold-edge witnesses for Clausius–Duhem transitions
live in `src/runtime/gate/` (`CdTransitionCartridge::transition_evidence` → catalog id
`umst.gate.cd_transition` + [`AdmissibilityToken`](src/runtime/gate/evidence.rs)). Host path
mirrors [`ai::constraint_loss::ConstraintExplanation`](src/ai/constraint_loss.rs); verify with:

```bash
cargo test runtime::gate::cartridge::
```

**Golden fixtures (researcher path):** hand-built transition vectors, vendored adversarial JSON, expected verdict tokens, and one-shot commands — [`GOLDEN_FIXTURES.md`](GOLDEN_FIXTURES.md).

From **multi-repo workspace** root:

```bash
cargo test -p umst-manifold --manifest-path umst-manifold/Cargo.toml

cargo test -p umst-manifold --manifest-path umst-manifold/Cargo.toml \
  --test gate_parity_fixture --test gate_kleisli --test gate_cbf_parity

cargo test -p umst-manifold --manifest-path umst-manifold/Cargo.toml \
  --features formal-witness,ros2-contract,serde \
  --test formal_witness --test manifest_strict_witness --test ros_contract_serde_roundtrip

cargo test -p umst-manifold --manifest-path umst-manifold/Cargo.toml \
  --features gate-server-bin --test gate_server_http
```

Same gate parity block from **inside** `umst-manifold/`:

```bash
cargo test --test gate_parity_fixture --test gate_kleisli --test gate_cbf_parity
cargo test --features formal-witness --test formal_witness
cargo test --features ros2-contract,serde --test ros_contract_serde_roundtrip
cargo test --features gate-server-bin --test gate_server_http
```

### 2.3 Solver lanes

```bash
# PR-stable subset (topology-density-evolution + statmech-vinet)
cargo test --features solver-stable

# Full experimental union (main-branch release job; slow)
cargo test --features solver-experimental
cargo test --release --features solver-experimental

# Meta feature used in rust-solvers.yml
cargo test --features solver-tests
```

### 2.4 Phase-4 verification subset (`rust.yml` PR job)

```bash
cargo test --release --features solver-research --test thmc_monolithic_newton_chain
cargo test --release --features solver-research,photonics-fdfd --test photonics_curl_curl_2d_patch
cargo test --release --features solver-research,photonics-fdfd --test photonics_curl_curl_3d_brick
cargo test --release --features solver-research --test statmech_lj_johnson_upscale_bridge
```

### 2.5 Core physics validation envelope

See [`Validation.md`](Validation.md). Quick replay:

```bash
cargo test --test dec_identities --test conservation --test cbf
```

### 2.6 Q1-hex hardware perf (Sprint 1–2 instrumentation)

Fast PR witnesses (parity + drift guards; **not** wall-clock benchmarks):

```bash
cargo test --features mechanics-adjoint-q1-hex \
  --test hardware_perf_adversarial \
  --test solver_region_parity \
  --test grid_witness_catalog \
  --test q1_hex_pcg_warm_start_ab \
  --test q1_hex_forward_perf_instrument
```

Local preconditioner ladder A/B (slow, ~minutes):

```bash
cargo test --features mechanics-adjoint-q1-hex --test q1_hex_perf_levers_ab -- --nocapture
```

Ecosystem-wide battery (MaOS workspace): `bash scripts/verify-hardware-perf.sh`.

---

## 3. Features (optional Cargo flags)

| Feature | Enables | Verify with |
|---------|---------|-------------|
| `ndarray` | Default CPU backend | `cargo test` |
| `mac-fast` | `ndarray` + Apple Accelerate (macOS) | `cargo test --features mac-fast` |
| `formal-witness` | `src/ai/formal.rs` + formal witness hook | `cargo test --features formal-witness --test formal_witness` |
| `ros2-contract` | `src/ros/contract.rs` DTOs | needs `serde` for roundtrip test |
| `serde` | Serde on ROS wire types | `cargo test --features ros2-contract,serde --test ros_contract_serde_roundtrip` |
| `gate-server-bin` / `gate-server` | `gate_server` binary + HTTP tests | `cargo test --features gate-server-bin --test gate_server_http` |
| `gate-full` | Alias → `gate-server-bin` | same as gate server |
| `manifest-bridge` | Downstream bridge hook (empty flag today) | cartridge check below |
| `manifold-manifest` | Manifest façade forward | `cargo doc -p umst-manifold --features manifold-manifest` |
| `solver-stable` | Topology + Vinet statmech | `cargo test --features solver-stable` |
| `solver-research` | Research solver union | `cargo check --all-targets --features solver-stable,solver-research` |
| `solver-experimental` | `solver-stable` ∪ `solver-research` | `cargo test --features solver-experimental` |
| `solver-tests` | Same graph as experimental (CI meta) | `cargo test --features solver-tests` |

Run the **full optional graph** (local only; pulls GPU/transitive pins):

```bash
cargo test --all-features --release
```

### 3.1 Run `gate_server` locally

```bash
cargo run --features gate-server-bin --bin gate_server
# POST JSON to http://127.0.0.1:<port>/gate  (see tests/gate_server_http.rs)
```

### 3.2 Downstream cartridge (W8) — git pin (G-02) or optional `[patch]`

**Production / CI (G-02 closed 2026-05-29):** `umst-concrete-cartridge` pins `umst-manifold` by git **`rev = fe22437`** (no workspace `[patch]`). Remote GHA runs `manifest-bridge` on the git dependency alone.

```bash
cd ../umst-concrete-cartridge
cargo test -p umst-concrete-cartridge --features manifest-bridge
cargo test -p umst-concrete-cartridge --features manifest-bridge --test manifest_bridge_catalog_grounding
```

**Monorepo dev (optional):** add workspace `[patch]` in `umst-concrete-cartridge/Cargo.toml` to sibling `../umst-manifold` when testing unpublished manifold changes before bumping the cartridge `rev`.

Prep gate (no push): `bash scripts/w8_publish_readiness.sh` — accepts git pin **or** workspace patch.

### 3.3 Release grounding (`StrictCatalogMatch` + `formal-witness`)

**Default builds are unchanged:** `cargo check` / `cargo test` use [`UmstManifestBuilder::default`](../../src/manifest/umst_manifest.rs) with [`GroundingContract::CatalogPinnedRos2`](../../src/manifest/umst_manifest.rs) and **no** `formal-witness` feature. Digest mismatch does not hard-fail.

**God-grade / production release profile** (explicit opt-in — see [`RELEASE_WITNESS_LADDER.md`](RELEASE_WITNESS_LADDER.md) R5 v1):

| Layer | Setting |
|-------|---------|
| Manifest | [`UmstManifestBuilder::for_release_witness()`](../../src/manifest/umst_manifest.rs) → [`GroundingContract::StrictCatalogMatch`](../../src/manifest/umst_manifest.rs) + lock-pinned `catalog_hash` |
| Crate | `--features formal-witness` |
| Gateway | [`UmstManifest::apply_witness_to_gateway`](../../src/manifest/umst_manifest.rs) or [`EmbodiedOrchestrator::from_manifest`](../../src/manifest/orchestrator.rs) sets `expected_catalog_schema_digest` |
| Cartridge | `manifest-bridge` on `umst-concrete-cartridge` (or sibling) against the same git-pinned manifold revision |

When both gateway and UMST carry `Some(digest)` and they differ, evaluation returns [`FormalReject::CatalogSchemaDigestMismatch`](../../src/ai/formal.rs) (`catalog_id`: `umst.formal.catalog_lock`). `None` on either side skips the witness (dev-safe).

```bash
# Manifest unit tests (default features — includes strict/mismatch builder fixtures)
cargo test manifest::

# Release witness integration (StrictCatalogMatch + digest mismatch reject)
cargo test --features formal-witness --test manifest_strict_witness

# CI / release witness lane (paired with verify_umst_stack.sh formal-witness block)
cargo test --features formal-witness --test formal_witness --test manifest_strict_witness

# Full R5 + export digest (monorepo sibling formal)
export UMST_REQUIRE_FORMAL_EXPORT=1
export UMST_FORMAL_ROOT=../umst-formal-double-slit
bash scripts/verify_umst_stack.sh
```

#### 3.3.1 Optional CI profile (`verify_umst_stack.sh` release lane)

`scripts/verify_umst_stack.sh` runs the **release manifest profile** in the same feature-gated block as `formal_witness` and ROS contract tests:

| Env | Behavior |
|-----|----------|
| *(unset or `1`)* | `cargo test --features formal-witness,… --test manifest_strict_witness` (default in stack verify) |
| `UMST_RELEASE_MANIFEST_PROFILE=0` | Skip `manifest_strict_witness` only (witness smoke + ROS roundtrip still run) |

Local one-shot:

```bash
cargo test --features formal-witness --test manifest_strict_witness
```

Cartridge release check (git-pinned manifold; optional local `[patch]` for pre-publish dev):

```bash
cd ../umst-concrete-cartridge
cargo test -p umst-concrete-cartridge --features manifest-bridge,manifold-manifest
```

---

## 4. Catalog / formal export

Refresh Lean catalog artifact (upstream repo):

```bash
cd ../umst-formal-double-slit
make lean-catalog-export
# writes artifacts/catalog.json — update umst-manifold/artifacts/catalog.lock.json digest when promoting
```


**Bidirectional catalog guard:** `scripts/bidirectional_catalog_check.sh` regenerates the Lean export from `UMST_FORMAL_ROOT`, compares its digest to `artifacts/catalog.lock.json`, and checks that every `catalog_id` implemented under `src/gate` is anchored in `umst-formal-double-slit/artifacts/catalog.json` (literal id or mapped Lean module via `grep`). Fails closed on drift; sibling default: `UMST_FORMAL_ROOT=../umst-formal-double-slit bash scripts/bidirectional_catalog_check.sh`.

After lock change, rebuild manifold so `build.rs` re-emits `UMST_CATALOG_LOCK_SHA256_HEX`:

```bash
cd ../umst-manifold
cargo clean -p umst-manifold && cargo check
```

---

## 5. CI workflow map

### 5.0 Catalog drift — repo layout (`verify_umst_stack.sh`)

`scripts/verify_umst_stack.sh` resolves the Lean export tree in this order:

1. **`UMST_FORMAL_ROOT`** — absolute path to `umst-formal-double-slit` (set in CI env).
2. **Sibling checkout** — `../umst-formal-double-slit` relative to the **crate root** (local monorepo / multi-repo folder).

| Where CI runs | Workflow file | Formal repo on the runner |
|---------------|---------------|---------------------------|
| **multi-repo workspace** (manifold is `umst-manifold/`) | `.github/workflows/umst-catalog-drift.yml` at repo root | Same `actions/checkout` as the monorepo; formal path is `${{ github.workspace }}/umst-formal-double-slit`, script runs with `working-directory: umst-manifold`. **No second checkout** — both trees must be committed (or otherwise present) in the monorepo. |
| **`tytolabs/umst-manifold`** (crate is repo root) | `.github/workflows/umst-catalog-drift.yml` | Second `actions/checkout` of `tytolabs/umst-formal-double-slit` into `${{ github.workspace }}/umst-formal-double-slit` (not a filesystem sibling of the runner workspace parent). `UMST_FORMAL_ROOT` points at that directory. |

Local parity without nesting formal inside the manifold repo:

```bash
# sibling layout (matches resolve_formal_root fallback)
git clone …/umst-manifold && git clone …/umst-formal-double-slit
cd umst-manifold
export UMST_REQUIRE_FORMAL_EXPORT=1
bash scripts/verify_umst_stack.sh
```


### 5.1 Bidirectional catalog check (`bidirectional_catalog_check.sh`)

After **`verify_umst_stack.sh`**, CI runs **`scripts/bidirectional_catalog_check.sh`** as a separate workflow step (same job, `UMST_FORMAL_ROOT` / `UMST_REQUIRE_FORMAL_EXPORT` as §5.0):

| Step | What it checks |
|------|----------------|
| (1) | Regenerate Lean export via `export_catalog.py` from `UMST_FORMAL_ROOT`. |
| (2) | Export `digest` and `module_count` match `artifacts/catalog.lock.json` and committed `umst-formal-double-slit/artifacts/catalog.json`. |
| (3) | Each `catalog_id()` under `src/gate/` is anchored in formal `catalog.json` (grep / Lean module map). |
| (4) | `cargo test --test catalog_all_ids_registered` — Lean `modules[].module` ↔ `CATALOG_MODULE_WIRED` ∪ `ALLOW_UNUSED_CATALOG_IDS`. |

**Workflows:** `workspace root/.github/workflows/umst-catalog-drift.yml` (step after `verify_umst_stack.sh`, `working-directory: umst-manifold`) and `umst-manifold/.github/workflows/umst-catalog-drift.yml` (standalone repo; second checkout of `tytolabs/umst-formal-double-slit`).

Local parity (sibling formal tree):

```bash
cd umst-manifold
export UMST_REQUIRE_FORMAL_EXPORT=1
export UMST_FORMAL_ROOT=../umst-formal-double-slit
bash scripts/verify_umst_stack.sh
bash scripts/bidirectional_catalog_check.sh
```


If formal is missing and `UMST_REQUIRE_FORMAL_EXPORT=1`, the script fails; when unset/`0`, the export digest step is skipped (gate tests still run).

### 5.1.1 Optional adversarial gate (prototype E6)

See also [`GOLDEN_FIXTURES.md`](GOLDEN_FIXTURES.md) §2 for the vendored Rust golden (`gate_adversarial`) and case-field reference.

After the gate-server HTTP tests, `scripts/verify_umst_stack.sh` may run the prototype **Experiment E6** adversarial stress script:

| Order | Prototype root |
|-------|----------------|
| 1 | **`UMST_PROTOTYPE_ROOT`** — directory that contains `scripts/test_gate_adversarial.py` |
| 2 | **Sibling checkout** — `../umst-prototype` or `../umst-prototype_2` next to the crate root (multi-repo workspace) |

| Prototype tree | Behavior |
|----------------|----------|
| **Script missing** | Prints `SKIP: umst-prototype adversarial gate …` and continues (set `UMST_REQUIRE_ADVERSARIAL_GATE=1` to fail closed). |
| **Script present** | Runs `python3 scripts/test_gate_adversarial.py`, then asserts `results/adversarial_gate_test.json` → `summary.false_negatives == 0` (hard safety / FNR). |

Typical monorepo path: `umst-prototype_2/scripts/test_gate_adversarial.py` (thin `umst-prototype` has no E6 script).

```bash
export UMST_PROTOTYPE_ROOT=../umst-prototype_2
bash scripts/verify_umst_stack.sh
# … ends with: OK: adversarial gate FNR=0 (75 cases)
```



### 5.2 Optional gate parity in `rust.yml` (parity-ci / W10-a)

Job **`verify-umst-stack-optional`** in `umst-manifold/.github/workflows/rust.yml` mirrors local parity without replacing `umst-catalog-drift.yml`:

| Formal tree | CI behavior |
|-------------|-------------|
| **Present** — `UMST_FORMAL_ROOT`, monorepo sibling `../umst-formal-double-slit`, or `${{ github.workspace }}/umst-formal-double-slit` | `bash scripts/verify_umst_stack.sh` with `UMST_REQUIRE_FORMAL_EXPORT=1` (export digest + bidirectional script when present + full gate matrix). |
| **Absent** | **Subset:** §2.2 gate / formal-witness / gate-server integration tests only (no Lean export digest). |

The job uses `continue-on-error: true` — signal only; branch protection should keep requiring `build-test`, `lint`, and solver PR jobs.

**Local:**

```bash
# Full stack (sibling formal)
export UMST_REQUIRE_FORMAL_EXPORT=1
export UMST_FORMAL_ROOT=../umst-formal-double-slit
bash scripts/verify_umst_stack.sh

# Same subset as rust.yml when formal is not on disk
cargo test --test gate_parity_fixture --test gate_kleisli --test gate_cbf_parity --test gate_dual_run_parity
cargo test --features formal-witness,ros2-contract,serde --test formal_witness --test ros_contract_serde_roundtrip
cargo test --features gate-server-bin --test gate_server_http
```

For authoritative catalog drift on every Lean/manifold change, use **`umst-catalog-drift.yml`** (§5.0–§5.1).


| Workflow | Scope | Mirrors |
|----------|-------|---------|
| `umst-manifold/.github/workflows/rust.yml` | Default build/test, solver-stable PR, lint; **optional** `verify-umst-stack-optional` (full `verify_umst_stack.sh` or §2.2 subset) | §1.1–§2.3, §5.2 |
| `workspace root/.github/workflows/rust-solvers.yml` | fmt, clippy, solver-tests, cartridge | §1.2, §2.3, §3.2 |
| `workspace root/.github/workflows/umst-catalog-drift.yml` | `verify_umst_stack.sh` + `bidirectional_catalog_check.sh` | §2.2, §5.1 |

---

## 6. One-shot “pre-push” bundle

```bash
cd umst-manifold
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo test --test gate_parity_fixture --test gate_kleisli --test gate_cbf_parity
cargo test --features formal-witness,ros2-contract,serde \
  --test formal_witness --test manifest_strict_witness --test ros_contract_serde_roundtrip
cargo test --features gate-server-bin --test gate_server_http
python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set
```

From multi-repo workspace root, also run the drift file parity:

```bash
cargo test -p umst-manifold --manifest-path umst-manifold/Cargo.toml \
  --test gate_parity_fixture --test gate_kleisli --test gate_cbf_parity
```
