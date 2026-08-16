SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
# Gate unification — catalog_id and evaluator strategy

This spec defines how **multiple gate implementations** (prototype thermodynamic transition filter, differentiable `ManifoldGateway` + `ThermodynamicCBF`, future Lean-exported obligations) unify under stable Rust types in `umst-manifold`.

## Identifiers (`catalog_id`)

Every obligation that can be **witnessed**, **exported from Lean**, or **asserted in CI** MUST carry:

- **`catalog_id`**: Stable string slug, namespaced (`umst.gate.cd_transition`, `umst.gate.landauer_cbf`, `umst.gate.http_shim`).
- **`catalog_revision`**: Monotonic revision string (semver or Lean export epoch).
- **`catalog_digest_hex`**: SHA-256 hex of canonical JSON row (from `WitnessCatalog::compiled_digest_hex()` at build).

The runtime lock file **`artifacts/catalog.lock.json`** aggregates module entries; `build.rs` hashes the bundle for `cargo:rustc-env=UMST_CATALOG_DIGEST_HEX`.

## Registry-first evaluator selection

Default policy in `UmstManifest`:

1. **Registry-first**: Select `GateEvaluator` by `catalog_id` from manifest defaults (`UmstManifest::default()` wires `ThermodynamicTransitionEvaluator`).
2. **Dual-run mode** (`UmstManifest::dual_run`): Run **transition gate** (`gate::thermo_transition`) and **CBF scalar gate** (`ai::cbf::ThermodynamicCBF`) independently; reject if **either** reports inadmissible (used in parity tests and migration).
3. **Replace mode**: Swap default evaluator with a downstream `Box<dyn GateEvaluator>` (e.g. cartridge-specific strength bound) while keeping `catalog_id` stable for telemetry.

## Mapping table (initial)

| `catalog_id` | Source | Manifold type | Notes |
|--------------|--------|---------------|-------|
| `umst.gate.cd_transition` | Prototype `ThermodynamicFilter` | `gate::thermo_transition::ThermodynamicGate` | Host `f64` Clausius–Duhem + mass + strength monotonicity |
| `umst.gate.kleisli_unit` | Prototype `kleisli` | `gate::kleisli::Admissible<A>` | Categorical composition; no WASM |
| `umst.gate.landauer_cbf` | `ai::cbf::ThermodynamicCBF` | `ManifoldGateway::evaluate_topology_step` | Tensor reductions → scalar Landauer + dissipation |
| `umst.gate.http_shim` | Legacy `gate_server` | `bins/gate_server` | JSON request → transition gate |
| `umst.formal.catalog_lock` | Lean export + lock | `runtime::catalog` | Build-time digest; optional `formal-witness` reject enum |

## CI traceability

Lean `catalog.json` module ids must partition across wired maps, `GateUnificationSpec` `catalog_id`s, witness registry, or `ALLOW_UNUSED_CATALOG_IDS` — see **`docs/CATALOG_TRACEABILITY.md`**, **`docs/DUAL_PIN_ARCHITECTURE.md`**, and `tests/catalog_all_ids_registered.rs`.

Default [`GateRegistry`](../src/manifest/umst_manifest.rs) declared lanes include all spec slugs plus registry evaluators (`thermodynamic_mix`, `umst.cartridge.concrete.policy`) for unified **119-module** manifests.

## Migration notes

- Prototype crates keep the name `umst-core`; they do **not** publish the SSOT API. Downstream should depend on **`umst-manifold`**.
- Until prototypes add path dependencies, use this spec + `PROTOTYPE_GATE_MAP.md` for traceability (see `docs/THIN_PROTOTYPES_PATH_DEPS.md` in `umst-prototype` when present).
- **`umst.gate.prediction_vs_physics`** (deprecated): never a separate `GateEvaluator` registry row. Mix prediction vs physics obligations use **`umst.gate.http_shim`** with telemetry `gate_family` **`mix_prediction_vs_physics`** (`HttpMixGateEvaluator` in `src/gate/http_manifest.rs`).
- **HTTP manifest literals (deprecated, P5):** `default_gate_manifest()`, `GateHttpRuntime::from_defaults()`, and `HttpTransitionEvaluator::from_domain_policy_defaults()` embed prototype closure defaults. Prefer **`GateManifest::from(&UmstManifest)`** and **`GateHttpRuntime::from_umst_manifest`** (injection-only). Full table: **`docs/RUNTIME_TOPOLOGY.md`** § HTTP gate defaults.
