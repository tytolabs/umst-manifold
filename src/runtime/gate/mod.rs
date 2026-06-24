// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Gate evidence contracts for cold-edge telemetry.

pub mod cartridge;
pub mod evidence;

pub use cartridge::{CdTransitionCartridge, GateCartridge};
pub use evidence::{
    explain_cd_transition_host, AdmissibilityToken, ConstraintExplanation, TransitionEvidence,
};
