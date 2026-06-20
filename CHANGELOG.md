<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Removed (gate hygiene)

- **GitHub release + tags `v2.0.0` / `v2.0.0-rc1` withdrawn** (2026-06-20): no Zenodo software DOI was minted for the release; premature public ship claim. Re-tag after user sign-off and trust-surface close. Code on `main` unchanged.

## [2.0.0] — 2026-06-15 (unreleased — tags withdrawn 2026-06-20)

**Tags:** ~~`v2.0.0-rc1`~~ / ~~`v2.0.0`~~ (deleted; see Unreleased)  
**Manifold `main`:** `7431c1b` · **Catalog R0:** **122** modules, digest `c61b1bef…`

Material-agnostic **W9** wave: kernel renames (`StatePoint`, `transition_proposal`), cement literals evicted to cartridge injection, lexicon + agnostic-on-fork CI green, formal prime-spectral mirror (+2 Lean roots), cartridge registry split, Phase B `GateCartridge` / `SpatialCartridge` stub + [`docs/CARTRIDGE_PORT.md`](docs/CARTRIDGE_PORT.md). Plan: [`docs/W9_PLAN.md`](docs/W9_PLAN.md).

### Added

- **Phase B port docs** — `GateCartridge` marker trait, `SpatialCartridge` alias, `tests/gate_cartridge_only_stub.rs` (injection-only gate contract).
- **Tier-2c injection fixture** — `tests/injection_mechanism_fixture.rs` (111/222 sentinels); cartridge `tier2c_closure_parity`.
- **Prime-spectral formal mirror** — `PrimeSpectralGuidance` + `PrimeSpectralCategory` in `umst-formal` (53 lake roots; catalog allowlist on manifold).
- **Theorem count SSOT** — `scripts/theorem_counts_snapshot.json` → formal **261** theorem / double-slit **540** theorem (regen via `check_theorem_counts_ssot.py`).

### Changed

- **Agnostic kernel** — `MixTensor` → `StatePoint` (deprecated alias); `mix_proposal` → `transition_proposal`; `ThmcHydrationKinetics` → `ReactionExtentKinetics` with byte-equivalent THMC parity.
- **Gate registry** — CARTRIDGE badge rows split from universal `GATE_REGISTRY_CATALOG_IDS`; kernel `gate::ConcreteCartridge` retired (cartridge owns policy `GateEvaluator`).
- **Catalog pin** — unified export **119 → 122** modules (`c61b1bef…`); `.umst-pins.toml` bumps formal `b09d4a0`, double-slit `72a6fe9`.
- **Cartridge git pin** — `umst-concrete-cartridge` pins manifold `cfc683f` (`v2.0.0-rc1` embed; Phase B on manifold is docs/tests only).

### Verification

`UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` exit **0** (rustup **1.88** on PATH). Acceptance: `check_domain_lexicon.sh` 0, `check_agnostic_on_fork.sh` 0, `cargo test --lib` 81/81, `thmc_drying_shrinkage` 32/32, `lake build UMST` green on `umst-formal` `main`.

## [2.0.0-rc1] — 2026-06-15

Phase A only (`cfc683f`): W9 agnostic port + catalog 122 allowlist before Phase B cherry-pick.


Plain-language snapshot for this wave — read the bullets below for detail.

- **God-grade stack** landed in [`2a28eb5`](https://github.com/tytolabs/umst-manifold/commit/2a28eb5) (unified catalog lock, gate evaluators, manifest/orchestrator, `verify_umst_stack.sh`, drift CI). Follow-up commits through **`fe22437`** fixed rustfmt, pinned-catalog fallbacks, W8 integration-test skips without a sibling cartridge checkout, and **clippy `-D warnings`** on CI stable (`manual_contains` in `witness_priority.rs`). Docs alignment for **G-02 closed** is [`8b97af7`](https://github.com/tytolabs/umst-manifold/commit/8b97af7).
- **Proof inventory pin:** **119 / 119** Lean modules (dual-pin v2: primary fiber **69** + secondary **62**); digest **`0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227`** (`0697014f…` in logs). This is a **fingerprint lock**, not “every module runs on the hot path” — do **not** cite **69** as the live catalog module count.
- **In-repo automation:** **16 / 16** checklist rows green on last full `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` run (2026-05-29). That is **not** the same as org-wide cartridge CI completion and **not** hot-path 100% (**~26%** primary / **~15%** unified by design).
- **W8 Phase 1 — done:** manifest API and `manifest-bridge` surface are on **`main`** @ **`fe22437`**; manifold GitHub CI green ([run 26649667467](https://github.com/tytolabs/umst-manifold/actions/runs/26649667467)).
- **G-02 — done (cartridge):** `umst-concrete-cartridge` pins git **`rev = fe22437`**, removes workspace `[patch]`, and runs `cargo test -p umst-concrete-cartridge --features manifest-bridge` in GHA — remote digest-grounded facade tests without monorepo sibling.
- **Scoped blockers (v1):** **G-03** supercap remote `manifest-bridge` (optional) and **FFI** horizon (G-26); hot-path **~26%** remains by design.

**Impact:** Unified **119-module** catalog export pins at build time (digest `0697014f…`); runtime gates reject inadmissible transitions without replaying Lean on the hot path — cartridges inherit digest parity via `manifest-bridge` / `IScienceCartridge`. Hot-path wiring remains **~26%** (18/69 primary) by design.

### Fixed

- **CI chain (2026-05-29)** — after god-grade `2a28eb5`: rustfmt + pinned upstream catalog (`debba0a`); verify fallback without live export tool (`debba0a`); bidirectional catalog check fallback (`910d7b3`); skip W8 integration test when concrete cartridge sibling is absent (`fe2c80d`); manifest/test clippy (`17d439f`); `witness_priority.rs` uses `Vec::contains` for stable `clippy::manual_contains` on CI stable (`fe22437`).

### Added

- **Gate unification** — registry-first `catalog_id` evaluators under `src/gate/` (Kleisli `Admissible`, `ThermodynamicMixEvaluator`, CD transition + CBF dual-run parity); `UmstManifest` defaults and `gate_server` HTTP shim; spec [`docs/GateUnificationSpec.md`](docs/GateUnificationSpec.md); integration tests `gate_kleisli`, `gate_cbf_parity`, `gate_dual_run_parity`, `gate_parity_fixture`, `gate_server_http` (feature `gate-server-bin`).
- **Catalog lock** — `artifacts/catalog.lock.json` pins unified Lean export digest `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227` (**119** modules, `cross_repo_merge: true`); `build.rs` / `src/runtime/catalog/` emit `UMST_CATALOG_DIGEST_HEX` (override with `UMST_CATALOG=/path/to/lock.json`).
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
