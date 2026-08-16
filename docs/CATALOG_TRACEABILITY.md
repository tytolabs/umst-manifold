SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
# Catalog traceability (Lean export ↔ manifold)

Automation keeps the **119-module** unified Lean inventory (`umst-formal-double-slit/artifacts/catalog.json`, cross-repo merge) aligned with manifold gate/witness registration.

## What is checked

Integration test **`tests/catalog_all_ids_registered.rs`** asserts:

1. **Complete partition** — every `catalog.json` entry id (`modules[].module` or `entries[].name`) appears in exactly one of:
   - [`CATALOG_MODULE_WIRED`](../src/runtime/catalog/traceability.rs) — explicit map to runtime `catalog_id` slugs, or
   - [`ALLOW_UNUSED_CATALOG_IDS`](../src/runtime/catalog/traceability.rs) — formal-only / scaffold modules with no dedicated Rust gate yet.
2. **Gate registry** — each [`GATE_REGISTRY_CATALOG_IDS`](../src/runtime/catalog/traceability.rs) (`GateEvaluator` in `src/gate/`) is backed by a wired Lean module or [`ALLOW_UNUSED_GATE_CATALOG_IDS`](../src/runtime/catalog/traceability.rs).
3. **Wired ids are known** — each `catalog_id` in `CATALOG_MODULE_WIRED` is listed in `GateUnificationSpec.md` or [`RUNTIME_EXTRA_GATE_CATALOG_IDS`](../src/runtime/catalog/traceability.rs) (e.g. `thermodynamic_mix`).
4. **Spec table SSOT** — markdown mapping table ids match `GATE_UNIFICATION_SPEC_CATALOG_IDS`.
5. **Witness path** — wired ids may also be satisfied by embedded [`WitnessCatalog`](../src/runtime/catalog/mod.rs) witness `id` fields (build-time `witness_catalog.json`).

Drift failures print a unified diff (`+` missing, `-` stale).

## Running

From `umst-manifold`:

```bash
cargo test catalog_all_ids
# or
cargo test --test catalog_all_ids_registered
```

Optional override when the workspace layout differs:

```bash
export UMST_LEAN_CATALOG_JSON=/path/to/umst-formal-double-slit/artifacts/catalog.json
```

## When Lean export changes

1. `make lean-catalog-export` in `umst-formal-double-slit`.
2. Refresh `umst-manifold/artifacts/catalog.lock.json` digest (`upstream_catalog_digest_hex`).
3. If new modules appear, either:
   - add a row to `CATALOG_MODULE_WIRED` + ensure `catalog_id` is in `docs/GateUnificationSpec.md`, or
   - append the module name to `ALLOW_UNUSED_CATALOG_IDS` with a short rationale in `docs/claims-vs-proofs.md`.

## Related docs

- `docs/GateUnificationSpec.md` — runtime `catalog_id` registry
- `docs/claims-vs-proofs.md` — Lean module ↔ `catalog_id` ↔ Rust ledger
- `docs/DUAL_PIN_ARCHITECTURE.md` — per-fiber vs composed catalog pin (dual-pin recommended)
- `src/runtime/catalog/schema/README.md` — witness envelope vs lock bundle
