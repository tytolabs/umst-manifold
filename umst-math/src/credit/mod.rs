//! Scalar credit aggregates (formal companion: `umst-formal/Lean/CreditGreedyOptimal.lean`).
//!
//! `G8` semantic script counts `pub fn|struct|…` in `**/*.rs`, not `pub mod`; W-3 theorem-bound rows live in
//! [`greedy`](crate::credit::greedy) (and see §14bis.l-W3 `M0-allowlist-extension.txt`).

/// ZCI-EXEMPT: `pub mod` re-export only; no G8 `pub` line in this file — bindings are on `greedy` (`§0.3` I-B; slice W-3)
pub mod greedy;
