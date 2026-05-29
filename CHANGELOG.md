<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

**Impact:** Lean **69-module** catalog export pins at build time; runtime gates reject inadmissible transitions without replaying Lean on the hot path — cartridges inherit digest parity via `manifest-bridge` / `IScienceCartridge`.

### Added

- **Gate unification** — registry-first `catalog_id` evaluators under `src/gate/` (Kleisli `Admissible`, `ThermodynamicMixEvaluator`, CD transition + CBF dual-run parity); `UmstManifest` defaults and `gate_server` HTTP shim; spec [`docs/GateUnificationSpec.md`](docs/GateUnificationSpec.md); integration tests `gate_kleisli`, `gate_cbf_parity`, `gate_dual_run_parity`, `gate_parity_fixture`, `gate_server_http` (feature `gate-server-bin`).
- **Catalog lock** — `artifacts/catalog.lock.json` pins upstream Lean export digest `c1d9ba2aa402106a3477f454dd6d28015eb399c1160d8a2e2ba7d16788fdbfcc` (**69** modules from `umst-formal-double-slit`); `build.rs` / `src/runtime/catalog/` emit `UMST_CATALOG_DIGEST_HEX` (override with `UMST_CATALOG=/path/to/lock.json`).
- **`EmbodiedOrchestrator`** — `src/manifest/orchestrator.rs` composes `ManifoldGateway` (tensor / CBF) with optional host transition + mix registry gates; `tests/embodied_orchestrator.rs`.
- **`formal-witness`** feature — `src/ai/formal.rs` (`FormalReject`, optional `CatalogSchemaDigestMismatch`); `ManifoldGateway::evaluate_topology_step_formal`; smoke `tests/formal_witness.rs`.
- **`verify_umst_stack.sh`** — local / CI parity script (`cargo check`, Lean export vs lock, gate + formal + ROS + gate-server tests); wired in `.github/workflows/umst-catalog-drift.yml`; commands catalog [`docs/VERIFY.md`](docs/VERIFY.md).
- **ROS / manifest wire** — `src/manifest/`, `src/ros/contract.rs` (`catalog_hash` on DTOs); features `ros2-contract`, `serde`, `manifest-bridge`, `manifold-gate`.
- **Documentation** — parallel handoff status [`docs/AGENT_STATUS.md`](docs/AGENT_STATUS.md), claims traceability [`docs/claims-vs-proofs.md`](docs/claims-vs-proofs.md), layout SSOT [`docs/REPO_LAYOUT_SSOT.md`](docs/REPO_LAYOUT_SSOT.md). Catalog ↔ Rust usage matrix (when published): [`docs/CATALOG_COVERAGE_AUDIT.md`](docs/CATALOG_COVERAGE_AUDIT.md).
- Helmholtz PDE density filter (`topology_filter`), Heaviside projection with continuation, Bruyneel–Duysinx self-weight q-norm, augmented Lagrangian volume constraint, extruded-plate mechanics scaffold, and integration tests (`topology_filter`, `heaviside_projection`, `self_weight_topology`, `aug_lagrangian_volume`, `extruded_plate_mechanics`) — all behind `solver-experimental` / `solver-tests`.
- Striatus-class shell storyline in the concrete cartridge: `optimize_shell_3d`, hero GIF `notebooks/_artifacts/striatus_emergence.gif`, and print-ready STL export (`docs/Striatus.md` in that repository).

### Changed

- Gate HTTP surface routes through `gate_server_router.rs` (`POST /gate`, `GET /health`) aligned with unified `catalog_id` evaluators (see [`docs/GateUnificationSpec.md`](docs/GateUnificationSpec.md)).

## [0.1.0] — 2026-05-07

### Added
- Initial public release of the UMST Manifold.
- `core::tensors::UnifiedMaterialStateTensor` — sparse spacetime cellular sheaf with
  scalar / vector / matrix feature spaces.
- `core::tensors::VerifiedUMST<P>` — type-state pattern carrying a phantom proof
  witness so unchecked states cannot reach downstream APIs.
- `core::traits::IScienceCartridge` — trait surface for plugging domain physics
  (concrete, polymers, alloys, biomaterials) into the manifold.
- `physics::laplacian` — discrete `d`, `d*`, and the Hodge Laplacian
  `Δ = d*d + dd*` over the 1-skeleton.
- `ai::adjoint` — adjoint sensitivity method for Neural ODE training with
  constant activation memory in the integration horizon.
- `ai::cbf::ThermodynamicCBF` — control barrier function enforcing the
  Clausius–Duhem inequality and the Landauer erasure limit.
- `ai::ppo::ManifoldGateway` and `ai::liquid_ppo::BurnLiquidPPOAgent` — agent
  surfaces that consume any `IScienceCartridge` implementation.
- Documentation: README, `docs/Mathematical-Foundations.md`, `docs/Validation.md`.
- Worked example: `examples/basic_topology.rs`.
- CI: `cargo fmt`, `cargo clippy -D warnings`, `cargo test`, `cargo doc -D warnings`
  on `ubuntu-latest` and `macos-latest` against `stable` and the MSRV.
- Governance: `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1),
  `SECURITY.md`, `CITATION.cff`, issue and pull-request templates, dependabot.

[Unreleased]: https://github.com/tytolabs/umst-manifold/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/tytolabs/umst-manifold/releases/tag/v0.1.0
