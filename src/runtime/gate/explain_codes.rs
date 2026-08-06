// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Gate explain vocabulary SSOT — reject codes, remediation, and field hints.
//!
//! Cold-edge only. Cartridge MCP (`build_gate_explain`) and manifold
//! [`crate::ai::constraint_loss`] share these codes for operator diagnostics.
//!
//! # Honest boundary
//!
//! Vocabulary + remediation maps are **landed**. Live MCP/operator GREEN, production
//! wire, MASTER retick, and OP-5 clearance stay **open** — see
//! [`explain_codes_production_wired`], [`EXPLAIN_CODES_GREEN_CLAIM_BLOCKED`],
//! [`explain_codes_master_retick_eligible`], [`explain_codes_op5_cleared`].

/// Cell / deepen id for fleet posture probes.
pub const CELL_ID: &str = "W29-115-EXPLAIN_CODES";

/// Honest posture — vocabulary SSOT landed; production / GREEN / MASTER / OP-5 open.
pub const POSTURE_TAG: &str = "EXPLAIN_CODES_VOCAB_PARTIAL";

/// Whether the top-code vocabulary + remediation map is landed.
pub const VOCAB_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const HONEST_FENCE: &str =
    "vocab_landed=true production_wired=false green_claim_blocked=true master_retick=false op5_cleared=false";

/// GREEN claim blocked — honest true until measured operator certificate.
pub const EXPLAIN_CODES_GREEN_CLAIM_BLOCKED: bool = true;

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

/// Count of top gate explain codes (pinned for deepen / census).
pub const TOP_GATE_EXPLAIN_CODE_COUNT: usize = 5;

const _: () = assert!(TOP_GATE_EXPLAIN_CODES.len() == TOP_GATE_EXPLAIN_CODE_COUNT);
const _: () = assert!(!explain_codes_production_wired());
const _: () = assert!(EXPLAIN_CODES_GREEN_CLAIM_BLOCKED);
const _: () = assert!(!explain_codes_master_retick_eligible());
const _: () = assert!(!explain_codes_op5_cleared());

/// Field-level hint for gate REJECT diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateFieldIssue {
    pub path: String,
    pub issue: String,
}

/// Typed probe for explain-codes posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExplainCodesPostureProbe {
    pub cell_id: &'static str,
    pub posture_tag: &'static str,
    pub vocab_landed: bool,
    pub top_code_count: usize,
    pub production_wired: bool,
    pub green_claim_blocked: bool,
    pub master_retick_eligible: bool,
    pub op5_cleared: bool,
    pub honest_fence: &'static str,
}

/// Live production wire for gate-explain operator path — honest **false**.
#[must_use]
pub const fn explain_codes_production_wired() -> bool {
    false
}

/// MASTER retick eligibility — honest **false** (not claimed from vocab deepen).
#[must_use]
pub const fn explain_codes_master_retick_eligible() -> bool {
    false
}

/// OP-5 clearance — honest **false** (not claimed from vocab deepen).
#[must_use]
pub const fn explain_codes_op5_cleared() -> bool {
    false
}

/// Whether `code` is one of [`TOP_GATE_EXPLAIN_CODES`].
#[must_use]
pub fn is_top_gate_explain_code(code: &str) -> bool {
    TOP_GATE_EXPLAIN_CODES.iter().any(|&c| c == code)
}

/// Build introspection probe for done-when / fleet checks.
#[must_use]
pub const fn explain_codes_posture_probe() -> ExplainCodesPostureProbe {
    ExplainCodesPostureProbe {
        cell_id: CELL_ID,
        posture_tag: POSTURE_TAG,
        vocab_landed: VOCAB_LANDED,
        top_code_count: TOP_GATE_EXPLAIN_CODE_COUNT,
        production_wired: explain_codes_production_wired(),
        green_claim_blocked: EXPLAIN_CODES_GREEN_CLAIM_BLOCKED,
        master_retick_eligible: explain_codes_master_retick_eligible(),
        op5_cleared: explain_codes_op5_cleared(),
        honest_fence: HONEST_FENCE,
    }
}

/// Posture honesty gate — vocabulary real; production / GREEN / MASTER / OP-5 fenced.
#[must_use]
pub fn explain_codes_posture_honest(probe: &ExplainCodesPostureProbe) -> bool {
    probe.cell_id == CELL_ID
        && probe.posture_tag == POSTURE_TAG
        && probe.vocab_landed
        && probe.top_code_count == TOP_GATE_EXPLAIN_CODES.len()
        && !probe.production_wired
        && probe.green_claim_blocked
        && !probe.master_retick_eligible
        && !probe.op5_cleared
        && probe.honest_fence.contains("vocab_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("green_claim_blocked=true")
        && probe.honest_fence.contains("master_retick=false")
        && probe.honest_fence.contains("op5_cleared=false")
}

/// Validate posture honesty — fail closed on fake GREEN / production / MASTER / OP-5 claims.
pub fn validate_explain_codes_posture_honesty() -> Result<(), &'static str> {
    let probe = explain_codes_posture_probe();
    if probe.production_wired {
        return Err("explain_codes_production_wired must stay false until live MCP wire");
    }
    if !probe.green_claim_blocked {
        return Err("EXPLAIN_CODES_GREEN_CLAIM_BLOCKED must stay true");
    }
    if probe.master_retick_eligible {
        return Err("explain_codes_master_retick_eligible must stay false");
    }
    if probe.op5_cleared {
        return Err("explain_codes_op5_cleared must stay false");
    }
    if !probe.vocab_landed {
        return Err("vocab_landed must stay true at W29-115");
    }
    if !explain_codes_posture_honest(&probe) {
        return Err("explain_codes_posture_honest failed");
    }
    Ok(())
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
            "Mix violates Clausius–Duhem margin; reduce w_c, adjust temperature_k, or change the thermal schedule before re-checking."
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_codes_count_and_membership() {
        assert_eq!(TOP_GATE_EXPLAIN_CODES.len(), TOP_GATE_EXPLAIN_CODE_COUNT);
        for code in TOP_GATE_EXPLAIN_CODES {
            assert!(is_top_gate_explain_code(code));
            assert!(!remediation_for_code(code).is_empty());
        }
        assert!(!is_top_gate_explain_code("not_a_real_code"));
    }

    #[test]
    fn remediation_and_fields_cover_top_codes() {
        assert_eq!(
            remediation_for_code(MIX_SPEC_RATIONAL_PARSE_FAIL),
            "Use rational strings like \"3/4\" for all mix fields (not floats or bare numbers); ensure w_c and temperature_k are present."
        );
        let parse_fields = fields_for_code(MIX_SPEC_RATIONAL_PARSE_FAIL, false);
        assert_eq!(parse_fields[0].path, "mix");

        let cd_no_t = fields_for_code(THERMODYNAMIC_CD_FAIL, false);
        assert!(cd_no_t.iter().any(|f| f.path == "mix.w_c"));
        assert!(!cd_no_t.iter().any(|f| f.path == "mix.temperature_k"));

        let cd_with_t = fields_for_code(THERMODYNAMIC_CD_FAIL, true);
        assert!(cd_with_t.iter().any(|f| f.path == "mix.temperature_k"));

        let thermo = fields_for_code(THERMODYNAMIC_FAIL, true);
        assert_eq!(thermo.len(), 2);

        let bridge = fields_for_code(MANIFEST_BRIDGE_DISABLED, false);
        assert_eq!(bridge[0].path, "build.features");

        assert!(fields_for_code("unknown_reject", false).is_empty());
        assert!(remediation_for_code("unknown_reject").contains("gate_reject"));
    }

    #[test]
    fn explain_codes_honest_fences_no_green_production_master_op5() {
        assert!(!explain_codes_production_wired());
        assert!(EXPLAIN_CODES_GREEN_CLAIM_BLOCKED);
        assert!(!explain_codes_master_retick_eligible());
        assert!(!explain_codes_op5_cleared());

        let probe = explain_codes_posture_probe();
        assert_eq!(probe.cell_id, CELL_ID);
        assert!(probe.vocab_landed);
        assert!(!probe.production_wired);
        assert!(probe.green_claim_blocked);
        assert!(!probe.master_retick_eligible);
        assert!(!probe.op5_cleared);
        assert!(explain_codes_posture_honest(&probe));
        validate_explain_codes_posture_honesty().expect("honest explain_codes posture");
    }
}
