# UMST manifold — repository layout SSOT

This document is the workspace-level **single source of truth** for where formal artifacts, gates, manifests, and prototype code map into `umst-manifold` after the UMST unification lane.

## Target directories (`umst-manifold`)

| Concern | Path | Purpose |
|---------|------|---------|
| **Formal catalog lock** | `umst-manifold/artifacts/catalog.lock.json` | Pinned JSON describing Lean-exported entries (digests + module graph skeleton). Consumed by `build.rs` and `runtime::catalog`. |
| **Formal catalog schema** | `umst-manifold/artifacts/catalog.schema.json` | JSON Schema stub for tooling and CI validators. |
| **Runtime catalog** | `umst-manifold/src/runtime/catalog/` | `WitnessCatalog`, loader helpers, codegen surface (`catalog_digest`). |
| **Gate (thermo + Kleisli + CBF bridge)** | `umst-manifold/src/gate/` | `GateEvaluator`, Clausius–Duhem transition gate (ported from prototypes), Kleisli admissibility monoid, CBF bridging notes. |
| **Manifest composition** | `umst-manifold/src/manifest/` | `UmstManifest` defaults (gate policy + pinned digest expectation). |
| **ROS serde contract** | `umst-manifold/src/ros/` (feature `ros2-contract`) | DTO stubs with `catalog_hash` — no ROS runtime dependency. |
| **HTTP gate shim** | `umst-manifold/src/bin/gate_server.rs` (features **`gate-server-bin`** / `gate-server`) | Minimal `POST /gate` JSON via **`crate::gate::http_manifest`** + **`gate_server_router`**. |
| **Policy gateway (existing)** | `umst-manifold/src/ai/ppo.rs` | `ManifoldGateway` — CBF + cartridge; optional `formal-witness` hook. |
| **Concrete cartridge façade** | `umst-concrete-cartridge/crates/umst-concrete-cartridge/src/lib.rs` | Optional re-exports behind `manifold-manifest` / `manifold-gate` features (path / git `umst-manifold` unchanged by default). |
| **Lean export tool** | `umst-formal-double-slit/tools/lean_export/` | Emits `artifacts/catalog.json` for CI and for refreshing the lock fingerprint workflow. |

## Related workspaces (consume, do not duplicate)

- **`umst-concrete-cartridge`** — Differentiable cartridge; dependency on manifold via git (see workspace `Cargo.toml` + optional `[patch]` in local `.cargo/config.toml`).
- **`umst-prototype` / `umst-prototype-2a`** — Historical `umst-core` crates with `ThermodynamicFilter`, `kleisli`, `gate_server`; **source ports** landed in `umst-manifold::gate`.
- **`umst-formal-double-slit/Lean`** — Primary Lean SSOT library (`lake build`).

## Operational notes

- Override catalog input for builds: **`UMST_CATALOG`** env var → path to catalog lock JSON (otherwise `artifacts/catalog.lock.json` relative to the manifold crate root).
- Downstream parity tests live under **`umst-manifold/tests/`** (`cbf.rs` + gate parity).
