// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
// Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Allowlist: Lean-mechanised theorem families mirrored on a host crate without a manifold
//! `GateEvaluator` must appear as **hand-aligned** in `docs/claims-vs-proofs.md`, not `proved`
//! (runtime honesty — no per-step Lean replay on the hot path).

const CLAIMS_VS_PROOFS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/docs/claims-vs-proofs.md");

/// Module basename → required status token in the traceability table row.
const HAND_ALIGNED_NO_HOST_EVALUATOR: &[(&str, &str)] = &[("RegimeSoundness", "hand-aligned")];

#[test]
fn claims_vs_proofs_regime_soundness_runtime_honesty() {
    let claims = std::fs::read_to_string(CLAIMS_VS_PROOFS)
        .unwrap_or_else(|e| panic!("read {CLAIMS_VS_PROOFS}: {e}"));

    for (module, status) in HAND_ALIGNED_NO_HOST_EVALUATOR {
        let marker = format!("`{module}`");
        let row = claims
            .lines()
            .find(|line| line.starts_with('|') && line.contains(&marker))
            .unwrap_or_else(|| panic!("missing traceability row containing {marker}"));

        assert!(
            row.contains(status),
            "row must use status `{status}` (not runtime proved): {row}"
        );
        assert!(
            !row.trim_end().ends_with("| proved |"),
            "must not claim proved at runtime: {row}"
        );
        assert!(
            row.contains("umst-concrete-cartridge") || row.contains("regime_check_scalars"),
            "row must name cartridge SSOT mirror: {row}"
        );
    }
}
