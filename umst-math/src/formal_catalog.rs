// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! The composed formal catalog pin, derived once here and re-exported by every consumer.
//!
//! `build.rs` recomputes the digest and row counts of the pinned formal export
//! (umst-manifold `artifacts/upstream_catalog.json`, umst-formal-double-slit's merged catalog at the
//! `.umst-pins.toml` commit) and hashes the umst-manifold catalog lock. umst-manifold
//! `runtime::catalog`, umst-algebra (and through it umst-trust) and umst-gateway re-export these
//! constants, so a pin change changes every binary. Registry row: `manifold_formal_catalog_digest`
//! (Pin form).

include!(concat!(env!("OUT_DIR"), "/formal_catalog_pin.rs"));
