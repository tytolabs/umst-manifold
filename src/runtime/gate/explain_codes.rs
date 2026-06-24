// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Gate explain vocabulary SSOT — reject codes, remediation, and field hints.
//!
//! Cold-edge only. Cartridge MCP (`build_gate_explain`) and manifold
//! [`crate::ai::constraint_loss`] share these codes for operator diagnostics.

/// Rational mix field failed to parse as `a/b`.
pub const MIX_SPEC_RATIONAL_PARSE_FAIL: &str = "mix_spec_rational_parse_fail";
/// `MixSpec` wire validation failed.
pub const MIX_SPEC_WIRE_INVALID: &str = "mix_spec_wire_invalid";
/// Clausius–Duhem thermodynamic margin negative.
pub const THERMODYNAMIC_CD_FAIL: &str = "thermodynamic_cd_fail";
/// Generic thermodynamic admissibility failure.
pub const THERMODYNAMIC_FAIL: &str = "thermodynamic_fail";
/// MCP built without manifest-bridge / thermodynamic gate.
pub const MANIFEST_BRIDGE_DISABLED: &str = "manifest_bridge_disabled";

/// Top operator-facing reject codes (parity-tested with cartridge MCP).
pub const TOP_GATE_EXPLAIN_CODES: &[&str] = &[
    MIX_SPEC_RATIONAL_PARSE_FAIL,
    MIX_SPEC_WIRE_INVALID,
    THERMODYNAMIC_CD_FAIL,
    THERMODYNAMIC_FAIL,
    MANIFEST_BRIDGE_DISABLED,
];

/// Field-level hint for gate REJECT diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateFieldIssue {
    pub path: String,
    pub issue: String,
}

/// One-line remediation for a [`regime_violations`] code.
#[must_use]
pub fn remediation_for_code(code: &str) -> &'static str {
    match code {
        MIX_SPEC_RATIONAL_PARSE_FAIL => {
            "Use rational strings like \"3/4\" for all mix fields (not floats or bare numbers); ensure w_c and temperature_k are present."
        }
        MIX_SPEC_WIRE_INVALID => {
            "mix_spec failed MixSpec validation; compare field names and rational formats against umst://schemas/contribution.v1.json."
        }
        THERMODYNAMIC_CD_FAIL => {
            "Mix violates Clausius–Duhem margin; reduce w_c, adjust temperature_k, or change curing regime before re-checking."
        }
        MANIFEST_BRIDGE_DISABLED => {
            "Build umst-mcp with agent-layer and manifest-bridge features so the thermodynamic gate runs."
        }
        THERMODYNAMIC_FAIL => {
            "Thermodynamic admissibility failed; run umst_gate_check with explain:true and adjust mix_spec until verdict is PASS."
        }
        _ => "See regime_violations codes and umst://schemas/gate_reject.v1.json; fix mix_spec and re-run gate check.",
    }
}

/// Field paths implicated by a reject code (mix-json agnostic — caller supplies presence hints).
#[must_use]
pub fn fields_for_code(code: &str, mix_has_temperature_k: bool) -> Vec<GateFieldIssue> {
    match code {
        MIX_SPEC_RATIONAL_PARSE_FAIL => vec![GateFieldIssue {
            path: "mix".into(),
            issue: "rational_parse_fail".into(),
        }],
        MIX_SPEC_WIRE_INVALID => vec![GateFieldIssue {
            path: "mix".into(),
            issue: "wire_invalid".into(),
        }],
        THERMODYNAMIC_CD_FAIL | THERMODYNAMIC_FAIL => {
            let mut fields = vec![GateFieldIssue {
                path: "mix.w_c".into(),
                issue: "cd_margin_negative".into(),
            }];
            if mix_has_temperature_k {
                fields.push(GateFieldIssue {
                    path: "mix.temperature_k".into(),
                    issue: "regime_out_of_envelope".into(),
                });
            }
            fields
        }
        MANIFEST_BRIDGE_DISABLED => vec![GateFieldIssue {
            path: "build.features".into(),
            issue: "manifest_bridge_disabled".into(),
        }],
        _ => Vec::new(),
    }
}
