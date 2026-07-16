// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Manifold shim — SSOT in `umst-gate` (P2.0).
pub use umst_gate::core_gate::{
    core_gate, gate, mass_conserved_between_densities, scalar_response_from_transition,
    AdmissibilityResponse, CoreGateOutcome, ScalarConstitutiveResponse,
    GATE_MASS_TOLERANCE_KG_M3,
};
