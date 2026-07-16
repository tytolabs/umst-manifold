// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
//
//! `umst-runtime` — executor crate (A3 alias of workspace-root `umst-manifold` during transition).
//!
//! New code should depend on `umst-runtime`; legacy path deps on `umst-manifold` remain valid via the
//! root facade package until the GitHub rename + `crates/umst-manifold` facade swap (see A3 status doc).

#![allow(deprecated)]

pub use umst_manifold::*;
