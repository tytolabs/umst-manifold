// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! THMC residual witness inventory — code-SSOT @ [`THMC_RESIDUAL_RECEIPT`].
//!
//! FP §2 totality (`thmc_residual.rs` → `PhysicsError`) and FP §6 residual-path witnesses
//! are closed on disk (56 integration tests under `thmc-coupled`). Deferred fences (post-U3
//! deepen): MP2b-U3 + XS-3-STEP5 **Closed** (operator landings on `thmc.rs`); MATPH-22
//! **Open** (material-phase ADT on `fracture_field.rs`); SCALE-JFNK **Soft** (scale-out).
//! Wave 3 THMC impl and MATPH hot-path wire stay **unauthorized** while MATPH-22 is Open —
//! this module does not invent GREEN / PRODUCTION_WIRED / MASTER.
//!
//! Authority: [`old/residuals/residuals/misc-outputs-tmp/RESEARCH_THMC_2218.md`](../../../../old/residuals/residuals/misc-outputs-tmp/RESEARCH_THMC_2218.md)
//! · [`old/residuals/residuals/misc-outputs-tmp/g_spawn_i_thmc_2252.md`](../../../../old/residuals/residuals/misc-outputs-tmp/g_spawn_i_thmc_2252.md)
//! · Deferred statuses deepened beyond the 22:52 research card (MP2b/XS closed; MATPH open).

/// Residue conjunct — THMC residual totality + FP §6 witness stack.
pub const RESIDUE_CONJUNCT: &str = "THMC-residual-totality+FP6";

/// RW-FP-TOT-3B4 receipt — explicit `PhysicsError` in `thmc_residual.rs`.
pub const TOT_3B4_RECEIPT: &str = "fp_totality_3b4";

/// FP §6 idempotency equilibrium harness receipt.
pub const IDEM_RECEIPT: &str = "g_spawn_i_thmc_0831";

/// FP §6 drying/shrinkage residual drip receipt.
pub const DRIP_RECEIPT: &str = "g_spawn_next_safe_drip_1545";

/// FP §6 harness expect hygiene receipt.
pub const WIRE_RECEIPT: &str = "g_spawn_impl_thmc_2226";

/// Latest THMC residual research + SSOT receipt @ 22:52.
pub const THMC_RESIDUAL_RECEIPT: &str = "g_spawn_i_thmc_2252";

/// `tests/verification/thmc_idempotency_equilibrium.rs` integration count.
pub const IDEM_INTEGRATION_TESTS: u8 = 13;

/// `tests/verification/thmc_drying_shrinkage.rs` integration count.
pub const DRIP_INTEGRATION_TESTS: u8 = 33;

/// Gate evidence + writeback + step wire harness integration count.
pub const WIRE_INTEGRATION_TESTS: u8 = 10;

/// Full `thmc-coupled` integration witness total (idem + drip + wire).
pub const RESIDUAL_WITNESS_TEST_TOTAL: u8 = 56;

/// Implicit `String` error sites remaining in `thmc_residual.rs` post-3B4.
pub const THMC_RESIDUAL_STRING_ERROR_SITES: u8 = 0;

/// Dense stacked-DOF cap shared by all THMC dense Newton paths (SSOT).
pub const DENSE_NEWTON_MAX_STACKED_DOFS: usize =
    super::thmc_residual::THMC_DENSE_NEWTON_MAX_STACKED_DOFS;

/// Gate status for one THMC residual witness leg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThmcResidualLegGateStatus {
    /// Witness stack incomplete for defined leg scope.
    Open,
    /// Witness stack GREEN for defined leg scope.
    Closed,
    /// Explicit deferral — not a witness gap (operator / scale-out).
    Soft,
}

impl ThmcResidualLegGateStatus {
    /// Stable tag for receipts / CI introspection.
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Open => "OPEN",
            Self::Closed => "CLOSED",
            Self::Soft => "SOFT",
        }
    }

    /// Whether this leg satisfies close precondition for residual witness ladder.
    #[must_use]
    pub const fn satisfies_leg_close_precondition(self) -> bool {
        matches!(self, Self::Closed)
    }

    /// Soft deferrals do not block Wave 3; Open does.
    #[must_use]
    pub const fn blocks_wave3(self) -> bool {
        matches!(self, Self::Open)
    }
}

/// One THMC residual witness row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThmcResidualLegRow {
    /// Leg id (`TOT-3B4` | `IDEM` | `DRIP` | `WIRE`).
    pub leg: &'static str,
    /// Integration witness count (0 for code-only legs).
    pub integration_tests: u8,
    /// Primary evidence receipt.
    pub evidence: &'static str,
    /// Witness scope status @ residual research card.
    pub status: ThmcResidualLegGateStatus,
}

/// Frozen witness leg inventory — totality + FP §6 **CLOSED** (witness scope only).
pub const LEG_INVENTORY: &[ThmcResidualLegRow] = &[
    ThmcResidualLegRow {
        leg: "TOT-3B4",
        integration_tests: 0,
        evidence: TOT_3B4_RECEIPT,
        status: ThmcResidualLegGateStatus::Closed,
    },
    ThmcResidualLegRow {
        leg: "IDEM",
        integration_tests: IDEM_INTEGRATION_TESTS,
        evidence: IDEM_RECEIPT,
        status: ThmcResidualLegGateStatus::Closed,
    },
    ThmcResidualLegRow {
        leg: "DRIP",
        integration_tests: DRIP_INTEGRATION_TESTS,
        evidence: DRIP_RECEIPT,
        status: ThmcResidualLegGateStatus::Closed,
    },
    ThmcResidualLegRow {
        leg: "WIRE",
        integration_tests: WIRE_INTEGRATION_TESTS,
        evidence: WIRE_RECEIPT,
        status: ThmcResidualLegGateStatus::Closed,
    },
];

/// Witness leg row count (TOT-3B4 + IDEM + DRIP + WIRE).
pub const LEG_INVENTORY_COUNT: usize = 4;

/// Deferred leg row — id, gate status, evidence note.
pub type ThmcDeferredLegRow = (&'static str, ThmcResidualLegGateStatus, &'static str);

/// Deferred legs — honest OPEN/SOFT/CLOSED; not witness-ladder gaps.
///
/// MP2b-U3 + XS-3-STEP5 closed after operator landings on `thmc.rs` (post-22:52 card).
/// MATPH-22 remains Open; SCALE-JFNK Soft. Soft does not authorize Wave 3.
pub const DEFERRED_INVENTORY: &[ThmcDeferredLegRow] = &[
    (
        "MP2b-U3",
        ThmcResidualLegGateStatus::Closed,
        "thmc.rs step_envelope — operator U3 spawn @ 0721",
    ),
    (
        "XS-3-STEP5",
        ThmcResidualLegGateStatus::Closed,
        "thmc.rs StiffnessField centralizer — operator Seq 12 @ 0721",
    ),
    (
        "MATPH-22",
        ThmcResidualLegGateStatus::Open,
        "fracture_field.rs material-phase ADT — serial impl",
    ),
    (
        "SCALE-JFNK",
        ThmcResidualLegGateStatus::Soft,
        "Solver-Status #8 — >64 DOF sparse/matrix-free monolith",
    ),
];

/// Deferred inventory row count.
pub const DEFERRED_INVENTORY_COUNT: usize = 4;

/// Lookup witness leg row by id.
#[must_use]
pub fn leg_by_id(leg: &str) -> Option<&'static ThmcResidualLegRow> {
    LEG_INVENTORY.iter().find(|row| row.leg == leg)
}

/// Lookup deferred leg row by id.
#[must_use]
pub fn deferred_by_id(id: &str) -> Option<&'static ThmcDeferredLegRow> {
    DEFERRED_INVENTORY.iter().find(|(leg, _, _)| *leg == id)
}

/// Count deferred legs still Open (blocks Wave 3 / MATPH wire).
#[must_use]
pub const fn deferred_open_count() -> usize {
    let mut n = 0usize;
    let mut i = 0usize;
    while i < DEFERRED_INVENTORY.len() {
        if matches!(DEFERRED_INVENTORY[i].1, ThmcResidualLegGateStatus::Open) {
            n += 1;
        }
        i += 1;
    }
    n
}

/// Count Soft deferred legs (explicit scale-out; not Wave 3 blockers).
#[must_use]
pub const fn deferred_soft_count() -> usize {
    let mut n = 0usize;
    let mut i = 0usize;
    while i < DEFERRED_INVENTORY.len() {
        if matches!(DEFERRED_INVENTORY[i].1, ThmcResidualLegGateStatus::Soft) {
            n += 1;
        }
        i += 1;
    }
    n
}

/// Sum integration witness tests across closed legs.
#[must_use]
pub const fn residual_witness_test_sum() -> usize {
    let mut sum = 0usize;
    let mut i = 0usize;
    while i < LEG_INVENTORY.len() {
        sum += LEG_INVENTORY[i].integration_tests as usize;
        i += 1;
    }
    sum
}

/// Whether totality + FP §6 witness ladder may flip to CLOSED — **true** @ 22:52.
#[must_use]
pub fn residual_leg_close_authorized() -> bool {
    LEG_INVENTORY
        .iter()
        .all(|row| row.status.satisfies_leg_close_precondition())
        && THMC_RESIDUAL_STRING_ERROR_SITES == 0
}

/// Whether Wave 3 THMC impl may proceed — derived from deferred fences.
///
/// Authorized only when no deferred Open remains (Soft alone does not authorize).
/// Currently **false**: MATPH-22 Open. MP2b-U3 Closed is insufficient alone.
#[must_use]
pub fn wave3_impl_authorized() -> bool {
    residual_leg_close_authorized()
        && DEFERRED_INVENTORY
            .iter()
            .all(|(_, status, _)| !status.blocks_wave3())
}

/// Whether MATPH material-phase hot-path wire is authorized — derived from MATPH-22.
#[must_use]
pub fn matph_wire_authorized() -> bool {
    matches!(
        deferred_by_id("MATPH-22").map(|(_, status, _)| *status),
        Some(ThmcResidualLegGateStatus::Closed)
    )
}

/// Runtime honesty probe — inventory sums + closed legs + deferred fence consistency.
#[must_use]
pub fn thmc_residual_honesty_holds() -> bool {
    residual_leg_close_authorized()
        && residual_witness_test_sum() == RESIDUAL_WITNESS_TEST_TOTAL as usize
        && DEFERRED_INVENTORY.len() == DEFERRED_INVENTORY_COUNT
        && deferred_open_count() == 1
        && deferred_soft_count() == 1
        && !wave3_impl_authorized()
        && !matph_wire_authorized()
        && DENSE_NEWTON_MAX_STACKED_DOFS == 64
        && deferred_by_id("MP2b-U3")
            .is_some_and(|(_, s, _)| *s == ThmcResidualLegGateStatus::Closed)
        && deferred_by_id("XS-3-STEP5")
            .is_some_and(|(_, s, _)| *s == ThmcResidualLegGateStatus::Closed)
        && deferred_by_id("MATPH-22").is_some_and(|(_, s, _)| *s == ThmcResidualLegGateStatus::Open)
        && deferred_by_id("SCALE-JFNK")
            .is_some_and(|(_, s, _)| *s == ThmcResidualLegGateStatus::Soft)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thmc_residual_inventory_honest_at_2252_probe() {
        assert_eq!(LEG_INVENTORY.len(), LEG_INVENTORY_COUNT);
        assert_eq!(
            residual_witness_test_sum(),
            RESIDUAL_WITNESS_TEST_TOTAL as usize
        );
        assert_eq!(THMC_RESIDUAL_STRING_ERROR_SITES, 0);
        assert!(residual_leg_close_authorized());
        assert!(!wave3_impl_authorized());
        assert!(!matph_wire_authorized());
        assert!(thmc_residual_honesty_holds());

        for leg in ["TOT-3B4", "IDEM", "DRIP", "WIRE"] {
            let row = leg_by_id(leg).expect("leg row");
            assert_eq!(row.status, ThmcResidualLegGateStatus::Closed);
        }

        assert_eq!(DEFERRED_INVENTORY.len(), DEFERRED_INVENTORY_COUNT);
        assert_eq!(deferred_open_count(), 1);
        assert_eq!(deferred_soft_count(), 1);

        let mp2b = deferred_by_id("MP2b-U3").expect("mp2b row");
        assert_eq!(mp2b.1, ThmcResidualLegGateStatus::Closed);
        assert_eq!(
            deferred_by_id("XS-3-STEP5").expect("xs3").1,
            ThmcResidualLegGateStatus::Closed
        );
        assert_eq!(
            deferred_by_id("MATPH-22").expect("matph").1,
            ThmcResidualLegGateStatus::Open
        );
        assert_eq!(
            deferred_by_id("SCALE-JFNK").expect("scale").1,
            ThmcResidualLegGateStatus::Soft
        );
        assert!(ThmcResidualLegGateStatus::Open.blocks_wave3());
        assert!(!ThmcResidualLegGateStatus::Soft.blocks_wave3());
        assert!(!ThmcResidualLegGateStatus::Closed.blocks_wave3());
    }

    #[test]
    fn thmc_residual_receipt_pinned() {
        assert_eq!(THMC_RESIDUAL_RECEIPT, "g_spawn_i_thmc_2252");
        assert_eq!(DENSE_NEWTON_MAX_STACKED_DOFS, 64);
        assert_eq!(RESIDUE_CONJUNCT, "THMC-residual-totality+FP6");
    }

    #[test]
    fn thmc_residual_deferred_fences_block_wave3_without_inventing_green() {
        // Operator landings closed; MATPH Open alone keeps Wave 3 / MATPH wire refused.
        assert!(
            deferred_by_id("MP2b-U3").is_some_and(|(_, s, _)| s.satisfies_leg_close_precondition())
        );
        assert!(!matph_wire_authorized());
        assert!(!wave3_impl_authorized());
        assert_eq!(deferred_open_count(), 1);
    }
}
