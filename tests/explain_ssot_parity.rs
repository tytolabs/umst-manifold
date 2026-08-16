// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Explain vocabulary SSOT parity — manifold codes match cartridge MCP contract.

use umst_manifold::runtime::gate::{
    fields_for_code, remediation_for_code, MANIFEST_BRIDGE_DISABLED, MIX_SPEC_RATIONAL_PARSE_FAIL,
    MIX_SPEC_WIRE_INVALID, THERMODYNAMIC_CD_FAIL, THERMODYNAMIC_FAIL, TOP_GATE_EXPLAIN_CODES,
};

/// Cartridge MCP `contribution.rs` remediation strings (frozen SSOT for parity).
const CARTRIDGE_REMEDIATION: &[(&str, &str)] = &[
    (
        MIX_SPEC_RATIONAL_PARSE_FAIL,
        "Use rational strings like \"3/4\" for all mix fields (not floats or bare numbers); ensure w_c and temperature_k are present.",
    ),
    (
        MIX_SPEC_WIRE_INVALID,
        "mix_spec failed MixSpec validation; compare field names and rational formats against umst://schemas/contribution.v1.json.",
    ),
    (
        THERMODYNAMIC_CD_FAIL,
        "Mix violates Clausius–Duhem margin; reduce w_c, adjust temperature_k, or change the thermal schedule before re-checking.",
    ),
    (
        MANIFEST_BRIDGE_DISABLED,
        "Build umst-mcp with agent-layer and manifest-bridge features so the thermodynamic gate runs.",
    ),
    (
        THERMODYNAMIC_FAIL,
        "Thermodynamic admissibility failed; run umst_gate_check with explain:true and adjust mix_spec until verdict is PASS.",
    ),
];

#[test]
fn top_five_explain_codes_have_nonempty_remediation() {
    assert_eq!(TOP_GATE_EXPLAIN_CODES.len(), 5);
    for code in TOP_GATE_EXPLAIN_CODES {
        let remediation = remediation_for_code(code);
        assert!(
            !remediation.is_empty(),
            "remediation for {code} must be non-empty"
        );
    }
}

#[test]
fn remediation_matches_cartridge_ssot() {
    for (code, expected) in CARTRIDGE_REMEDIATION {
        assert_eq!(
            remediation_for_code(code),
            *expected,
            "remediation drift for {code}"
        );
    }
}

#[test]
fn fields_for_cd_fail_includes_w_c_and_optional_temperature() {
    let without_temp = fields_for_code(THERMODYNAMIC_CD_FAIL, false);
    assert!(without_temp.iter().any(|f| f.path == "mix.w_c"));
    assert!(!without_temp.iter().any(|f| f.path == "mix.temperature_k"));

    let with_temp = fields_for_code(THERMODYNAMIC_CD_FAIL, true);
    assert!(with_temp.iter().any(|f| f.path == "mix.temperature_k"));
}

#[test]
fn mix_parse_fail_fields_non_empty() {
    let fields = fields_for_code(MIX_SPEC_RATIONAL_PARSE_FAIL, false);
    assert!(!fields.is_empty());
    assert_eq!(fields[0].path, "mix");
}
