// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//
//! Deprecated compatibility facade for `umst-runtime`.
//!
//! Inactive until A3 GitHub rename — workspace root `umst-manifold` currently serves this role.

#![allow(deprecated)]

#[deprecated(
    since = "0.2.0",
    note = "crate renamed to umst-runtime; depend on `umst-runtime` instead"
)]
pub use umst_runtime::*;
