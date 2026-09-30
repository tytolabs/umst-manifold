// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Schrödinger (unitary) branch — reversible single-Kraus step.

/// Witness for the cited identity unitary Kraus step (no tabulated `u8` channel index in Rust).
///
/// Proof: `SchrodingerDynamics` — unitary single-Kraus factor.
/// DOI: 10_5281/zenodo.19159660
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitaryStepIdentity {
    /// Identity channel named in `theorem_registry` (`UMST.FormalDoubleSlit.SchrodingerDynamics::unitary_channel`).
    CitedIdentity,
}

/// Returns the cited identity step when the formal pin is wired; no numeric Kraus index is asserted here.
pub fn unitary_step_identity() -> Option<UnitaryStepIdentity> {
    Some(UnitaryStepIdentity::CitedIdentity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schrodinger_identity_step_is_cited_witness() {
        assert_eq!(
            unitary_step_identity(),
            Some(UnitaryStepIdentity::CitedIdentity)
        );
    }
}
