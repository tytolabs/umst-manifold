// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Gate evidence contracts for cold-edge telemetry.

pub mod cartridge;
pub mod evidence;
pub mod explain_codes;

pub use cartridge::{CdTransitionCartridge, GateCartridge};
pub use evidence::{
    explain_cd_transition_host, AdmissibilityToken, ConstraintExplanation, TransitionEvidence,
};
pub use explain_codes::{
    fields_for_code, remediation_for_code, GateFieldIssue, MANIFEST_BRIDGE_DISABLED,
    MIX_SPEC_RATIONAL_PARSE_FAIL, MIX_SPEC_WIRE_INVALID, THERMODYNAMIC_CD_FAIL,
    THERMODYNAMIC_FAIL, TOP_GATE_EXPLAIN_CODES,
};
