// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! lean_extract_stub — MANIFOLD-LEAN-EXTRACT-STUB consumer for formal_pins census.
//!
//! **Honest boundary:** `rustc` cannot `use` `.lean` sources. This `lean_extract_stub`
//! records the intended consumer surface targeting Urge `formal_pins` census
//! (`umst/umst-urge/src/formal_pins.rs`) — NOT proof replay, NOT physics GREEN,
//! NOT extraction-complete.

/// Cell id for this consumer stub.
pub const CELL_ID: &str = "MANIFOLD-LEAN-EXTRACT-STUB";

/// Non-claim fence — stub records intent; Lean terms stay in Lean.
pub const NON_CLAIM: &str =
    "MANIFOLD-LEAN-EXTRACT-STUB Lean→Rust extraction consumer stub — rustc cannot use .lean; targets formal_pins census; not proof replay; not physics GREEN; not extraction_complete; not production_wired";

/// Machine-readable stub marker.
pub const LEAN_EXTRACT_STUB_MARKER: &str = "lean_extract_stub_v1";

/// Honest physics GREEN posture.
pub const PHYSICS_GREEN: bool = false;

/// Lean→Rust term extraction is not landed — refuse invented complete.
pub const EXTRACTION_COMPLETE: bool = false;

/// No production wire from this stub alone.
pub const PRODUCTION_WIRED: bool = false;

/// Urge `formal_pins` authority — census SSOT, not a runtime `use` edge.
pub const FORMAL_PINS_AUTHORITY: &str = "umst/umst-urge/src/formal_pins.rs";

/// Upstream Urge cell that owns the pin table.
pub const FORMAL_PINS_CELL_ID: &str = "URGE-INT-FORMAL-PINS";

/// Pin table marker @ Urge census.
pub const FORMAL_PIN_TABLE_MARKER: &str = "urge_int_formal_pins_v1";

/// Pipeline spine documenting rustc `.lean` refusal + consumer intent.
pub const PIPELINE_DOC_PATH: &str = "umst/umst-manifold/docs/FORMAL_BIDIRECTIONAL_ALIGNMENT.md";

/// Offline Lean catalog exporter (digest pin, not term extraction).
pub const MANIFEST_EXPORT_TOOL: &str =
    "umst-formal-double-slit/tools/lean_export/export_catalog.py";

/// Expected meso identity count @ Urge census.
pub const MESO_PIN_COUNT_TARGET: usize = 50;

/// Expected knowing identity count @ Urge census.
pub const KNOWING_PIN_COUNT_TARGET: usize = 10;

/// Expected total lockstep identities (meso + knowing).
pub const FORMAL_PIN_COUNT_TARGET: usize = MESO_PIN_COUNT_TARGET + KNOWING_PIN_COUNT_TARGET;

/// Refuse inventing extraction-complete — Lean proof terms are not Rust modules.
#[must_use]
pub const fn refuse_invented_extraction_complete() -> bool {
    !EXTRACTION_COMPLETE && !PHYSICS_GREEN && !PRODUCTION_WIRED
}

/// Typed consumer stub — intended Lean→Rust extraction consumer @ `formal_pins` census.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeanExtractStub {
    pub cell_id: &'static str,
    pub marker: &'static str,
    pub physics_green: bool,
    pub extraction_complete: bool,
    pub production_wired: bool,
    pub formal_pins_authority: &'static str,
    pub formal_pins_cell_id: &'static str,
    pub formal_pin_table_marker: &'static str,
    pub pipeline_doc_path: &'static str,
    pub manifest_export_tool: &'static str,
    pub formal_pin_count_target: usize,
    pub meso_pin_count_target: usize,
    pub knowing_pin_count_target: usize,
    pub rustc_cannot_use_lean: bool,
}

/// Frozen consumer stub — records intended extraction consumer, not landed extraction.
#[must_use]
pub const fn lean_extract_stub() -> LeanExtractStub {
    LeanExtractStub {
        cell_id: CELL_ID,
        marker: LEAN_EXTRACT_STUB_MARKER,
        physics_green: PHYSICS_GREEN,
        extraction_complete: EXTRACTION_COMPLETE,
        production_wired: PRODUCTION_WIRED,
        formal_pins_authority: FORMAL_PINS_AUTHORITY,
        formal_pins_cell_id: FORMAL_PINS_CELL_ID,
        formal_pin_table_marker: FORMAL_PIN_TABLE_MARKER,
        pipeline_doc_path: PIPELINE_DOC_PATH,
        manifest_export_tool: MANIFEST_EXPORT_TOOL,
        formal_pin_count_target: FORMAL_PIN_COUNT_TARGET,
        meso_pin_count_target: MESO_PIN_COUNT_TARGET,
        knowing_pin_count_target: KNOWING_PIN_COUNT_TARGET,
        rustc_cannot_use_lean: true,
    }
}

/// Probe alias for census / meta consumers.
#[must_use]
pub fn lean_extract_stub_probe() -> LeanExtractStub {
    lean_extract_stub()
}

/// Honest deepen — stub wired, extraction and physics GREEN both open.
#[must_use]
pub fn lean_extract_stub_honest() -> bool {
    let s = lean_extract_stub();
    !s.physics_green
        && !s.extraction_complete
        && !s.production_wired
        && s.rustc_cannot_use_lean
        && refuse_invented_extraction_complete()
        && s.formal_pins_authority.contains("formal_pins")
        && s.formal_pin_count_target
            == s.meso_pin_count_target + s.knowing_pin_count_target
}

#[cfg(test)]
mod lean_extract {
    use super::*;

    #[test]
    fn lean_extract_stub_physics_green_false() {
        assert!(!PHYSICS_GREEN);
        assert!(!EXTRACTION_COMPLETE);
        assert!(!PRODUCTION_WIRED);
        assert!(refuse_invented_extraction_complete());
    }

    #[test]
    fn lean_extract_stub_targets_formal_pins_census() {
        let s = lean_extract_stub();
        assert_eq!(s.formal_pins_authority, FORMAL_PINS_AUTHORITY);
        assert!(s.formal_pins_authority.ends_with("formal_pins.rs"));
        assert_eq!(s.formal_pins_cell_id, "URGE-INT-FORMAL-PINS");
        assert_eq!(s.formal_pin_table_marker, FORMAL_PIN_TABLE_MARKER);
        assert_eq!(s.formal_pin_count_target, 60);
        assert_eq!(s.meso_pin_count_target, 50);
        assert_eq!(s.knowing_pin_count_target, 10);
    }

    #[test]
    fn lean_extract_stub_rustc_cannot_use_lean() {
        let s = lean_extract_stub();
        assert!(s.rustc_cannot_use_lean);
        assert!(!s.extraction_complete);
        assert!(s.pipeline_doc_path.contains("FORMAL_BIDIRECTIONAL_ALIGNMENT"));
        assert!(s.manifest_export_tool.contains("export_catalog.py"));
    }

    #[test]
    fn lean_extract_stub_honest_posture() {
        assert!(lean_extract_stub_honest());
        let probe = lean_extract_stub_probe();
        assert_eq!(probe.cell_id, CELL_ID);
        assert_eq!(probe.marker, LEAN_EXTRACT_STUB_MARKER);
        assert!(!probe.physics_green);
    }

    #[test]
    fn lean_extract_stub_refuses_invented_complete() {
        assert!(!EXTRACTION_COMPLETE);
        assert!(refuse_invented_extraction_complete());
    }
}
