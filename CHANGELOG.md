<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Helmholtz PDE density filter (`topology_filter`), Heaviside projection with continuation, Bruyneel–Duysinx self-weight q-norm, augmented Lagrangian volume constraint, extruded-plate mechanics scaffold, and integration tests (`topology_filter`, `heaviside_projection`, `self_weight_topology`, `aug_lagrangian_volume`, `extruded_plate_mechanics`) — all behind `solver-experimental` / `solver-tests`.
- Striatus-class shell storyline in the concrete cartridge: `optimize_shell_3d`, hero GIF `notebooks/_artifacts/striatus_emergence.gif`, and print-ready STL export (`docs/Striatus.md` in that repository).

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
