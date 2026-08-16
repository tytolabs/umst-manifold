// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Constants grounding registry (Phase N-CGD-scaffold).
//!
//! The [`registry`] submodule holds the compile-time `REGISTRY` table and helpers; the human-readable
//! mirror lives in `docs/CGD_REGISTRY.md` §24a (`docs/HSAD_PLAN.md` §0.4 taxonomy).

pub mod derivation;
pub mod provenance;
pub mod registry;
pub mod tier1_derivation;
pub mod tier2_derivation;
pub mod tier3_derivation;
pub mod toolchain_pin;
