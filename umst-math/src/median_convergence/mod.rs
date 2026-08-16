// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Median-convergence warmup budget — mirror of `UMST.Formal.MedianConvergence`.
//!
//! The theorem-derived count is `N_warmup = ⌈(2 / (ε² ρ_min²)) · ln(2/δ)⌉` (natural log).
//! The cockpit ships the pragmatic gate **`max(3, ⌈√W⌉)`**; see [`sqrt_window_threshold`] and
//! Lean `sqrt_window_warmup_is_admissible`.

pub mod core;

pub use core::{n_warmup, sqrt_window_threshold};
