// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Pure compile-time functor: `artifacts/scalar_layout.lock.json` → Rust layout constants.
//!
//! IO stays in `umst-manifold/build.rs`; this crate only parses and emits source text.

mod emit;
mod parse;

pub use emit::{emit_scalar_layout_guard, emit_scalar_layout_rs};
pub use parse::{parse_scalar_layout_lock, LayoutCodegenError, LayoutSpec};
