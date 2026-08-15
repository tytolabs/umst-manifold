// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Manifold shim — SSOT in `umst-gate` (P2.0).
pub use umst_gate::admissibility_census::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn w8e14_admissibility_census_shim_open_deltas_cleared() {
        assert!(OPEN_RECONCILIATION_DELTAS.is_empty());
        assert!(format_open_deltas().is_empty());
    }

    #[test]
    fn w8e14_admissibility_census_compute_sites_pin_core_gate() {
        assert!(ADMISSIBILITY_COMPUTE_SITES
            .iter()
            .any(|s| s.symbol == "core_gate" && s.repo == "umst-foundations"));
    }

    #[test]
    fn w8e14_gate_parity_digest_prefix_matches_full_hash() {
        assert!(GATE_PARITY_V0_SHA256.starts_with(GATE_PARITY_V0_SHA256_PREFIX));
        assert_eq!(GATE_PARITY_V0_SHA256.len(), 64);
    }
}
