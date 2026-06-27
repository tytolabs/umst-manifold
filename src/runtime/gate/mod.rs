// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Gate evidence contracts for cold-edge telemetry.

pub mod admissibility_margin;
pub mod cartridge;
#[cfg(feature = "ucrs-provenance")]
pub mod cold_wire;
pub mod evidence;
pub mod explain_codes;

pub use admissibility_margin::{
    admissibility_from_margin, admissibility_margin_from_dissipation, AdmissibilityMargin,
    ADMISSIBILITY_MARGIN_EPS,
};
pub use cartridge::{CdTransitionCartridge, GateCartridge};
#[cfg(feature = "ucrs-provenance")]
pub use cold_wire::{
    transition_evidence_to_wire, SpineEventCost, TransitionEvidenceWire,
};
pub use evidence::{
    explain_cd_transition_host, AdmissibilityToken, ConstraintExplanation, TransitionEvidence,
    UcrsObservedAtWire,
};
pub use explain_codes::{
    fields_for_code, remediation_for_code, GateFieldIssue, MANIFEST_BRIDGE_DISABLED,
    MIX_SPEC_RATIONAL_PARSE_FAIL, MIX_SPEC_WIRE_INVALID, THERMODYNAMIC_CD_FAIL, THERMODYNAMIC_FAIL,
    TOP_GATE_EXPLAIN_CODES,
};
