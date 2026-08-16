SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
# Prototype audit — paths → manifold module

Audit of UMST prototypes and embodied bridge code vs **canonical Rust SSOT** in `umst-manifold`. ROS2 CLI usage elsewhere (e.g. `zeroclaw/crates/robot-kit`) is **not** a UMST theorem pipeline; manifold carries only serde **contracts** for cross-language alignment.

Integration status for thin prototypes: **`umst-prototype/docs/THIN_PROTOTYPE_STATUS.md`**.

## `umst-prototype` (`umst-core` crate)

| Prototype path | Role | Canonical manifold destination |
|----------------|------|-------------------------------|
| `umst-prototype/src/rust/core/src/science/thermodynamic_filter.rs` | Clausius–Duhem transition filter, hydration free-energy model | `umst_manifold::gate::thermo_transition` (`ThermodynamicState`, `ThermodynamicGate`) |
| `umst-prototype/src/rust/core/src/tensors/kleisli.rs` | Admissibility monad, Kleisli composition on mix tensors | `umst_manifold::gate::kleisli` (generic over `Clone` carriers; manifold `MixTensor<B>` lacks `Clone` — use predicates on slices / host summaries or wrap IDs) |
| `umst-prototype/src/rust/core/src/bin/gate_server.rs` | Minimal HTTP `/gate`, `/health` | `umst_manifold::` binary `gate_server` (features **`gate-server-bin`** / `gate-server`), JSON via **`gate::http_manifest`** |
| `umst-prototype/src/rust/core/src/tensors/mix.rs` | Legacy homogeneous mix tensor | `umst_manifold::core::tensors::MixTensor<B>` (Burn-backed) |

## `umst-prototype-2a`

| Prototype path | Role | Canonical manifold destination |
|----------------|------|-------------------------------|
| `umst-prototype-2a/prototype/src/rust/core/src/science/thermodynamic_filter.rs` | Same filter as prototype v1 | `umst_manifold::gate::thermo_transition` |
| `umst-prototype-2a/prototype/src/rust/core/src/tensors/kleisli.rs` | Same Kleisli layer | `umst_manifold::gate::kleisli` |

## UMST embodied / robotics (informative)

| Path | Role | Manifold linkage |
|------|------|------------------|
| `zeroclaw/crates/robot-kit/src/drive.rs`, `sense.rs` | Shell to `ros2` CLI (`/cmd_vel`, lidar) | **`umst_manifold::ros::contract`** (feature `ros2-contract`) exposes `catalog_hash` on serde DTOs only — runtime ROS stays in application crates. |

## Formal / Lean axis

| Path | Role | Manifold linkage |
|------|------|------------------|
| `umst-formal-double-slit/Lean/**/*.lean` | Epistemic + gate compat lemmas | `tools/lean_export/export_catalog.py` → `artifacts/catalog.json`; digest pins `artifacts/catalog.lock.json` consumed by **`build.rs`**. |
