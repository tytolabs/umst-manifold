// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Host-side `f32` Krylov helpers for THMC JFNK slices (`solver-experimental`).
//!
//! Implementation lives in [`super::krylov_host`] so other solver lanes (acoustics) can share
//! GMRES without pulling THMC-only symbols.

pub use super::krylov_host::{gmres_f32, gmres_f32_try};
