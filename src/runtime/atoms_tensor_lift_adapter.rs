// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
//
// AGAP-2127-PBM-010 — slice-3c rank-1+ adapter design scaffold for R-atoms-scalar / F1.
//
// Freezes the `umst-algebra-burn` adapter contract — one honest row per slice-3b THMC ledger
// field, documenting required `burn::Tensor` rank, phantom `Field<B, Space, D>` alignment, and
// deferred `TensorAlgebra` ops (`contract` / `grad`). Production rank-1+ monomorphization remains
// **[open]** — separate crate `umst-algebra-burn` not created. Bind status stays `UNBOUND`.
//
// **Cross-ref:** slice-3b ledger in [`atoms_tensor_lift_ledger`](super::atoms_tensor_lift_ledger);
// slice-3d op-spec ratchet in [`atoms_tensor_lift_ops`](super::atoms_tensor_lift_ops);
// slice residual rows in [`atoms_tensor_lift_residual`](super::atoms_tensor_lift_residual);
// P3 field SSOT in `umst-manifold/src/core/field.rs`.

/// PBM-010 workstream id (slice-3c deepen).
pub const PBM_ID: &str = "PBM-010";

/// Parent G40-R10 residue — F1 dual numerics still open at adapter tier.
pub const PARENT_RESIDUE_ID: &str = "R-atoms-scalar";

/// Slice-3c rank-1+ adapter scaffold identifier.
pub const SLICE_ID: &str = "slice-3c";

/// Honest posture — adapter contract rows landed; production impl **open**.
pub const POSTURE_TAG: &str = "ADAPTER_SCAFFOLD_PARTIAL";

/// Whether slice-3c adapter contract rows are on disk.
pub const ADAPTER_SCAFFOLD_LANDED: bool = true;

/// Whether rank-1+ `impl TensorAlgebra` over `burn::Tensor` is closed.
pub const RANK1_PLUS_IMPL_LANDED: bool = false;

/// Whether the planned `umst-algebra-burn` crate exists on disk.
pub const ADAPTER_CRATE_LANDED: bool = false;

/// Slice-3 0D lift step prerequisite (PBM-010 @ 19:20).
pub const SLICE3_LIFT_STEP_LANDED: bool = true;

/// Slice-3b ledger prerequisite (AGAP-2001-PBM-010).
pub const SLICE3B_LEDGER_LANDED: bool = true;

/// Slice-3d op-spec ratchet exists downstream; adapter rows stay `DEFERRED` at this tier.
pub const SLICE3D_OPS_LANDED: bool = true;

/// Primary source anchor for fleet / meta hygiene.
pub const SOURCE_ANCHOR_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_adapter.rs";

/// Planned adapter crate path (not created).
pub const ADAPTER_CRATE_PATH: &str = "umst-runtime/crates/umst-algebra-burn/";

/// Slice-3b ledger cross-ref.
pub const SLICE3B_LEDGER_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_ledger.rs";

/// Slice-3 0D lift step cross-ref.
pub const SLICE3_LIFT_STEP_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift.rs";

/// Slice-3d tensor op-spec cross-ref (downstream DESIGN_SPECIFIED ratchet).
pub const SLICE3D_OPS_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_ops.rs";

/// P3 field carrier SSOT.
pub const FIELD_SSOT_PATH: &str = "umst-manifold/src/core/field.rs";

/// C2 design SSOT.
pub const DESIGN_DOC_PATH: &str = "docs/C2_TENSOR_ALGEBRA_DESIGN.md";

/// Fleet receipt for slice-3c deepen.
pub const RECEIPT_SLUG: &str = "COMPLETION_AGAP_AGENT_PBM-010_2127";

/// Prior receipt (slice residual rows @ AGAP-2033).
pub const PRIOR_RECEIPT_SLUG: &str = "COMPLETION_AGAP_AGENT_PBM-010_2033";

/// Honest deepen fence for meta / fleet probes — scaffold yes, production no.
pub const HONEST_FENCE: &str = "adapter_scaffold_landed=true production_wired=false bind_status=UNBOUND";

/// Adapter bind posture — contract rows exist; no live `TensorAlgebra` monomorphization.
pub const BIND_STATUS: &str = "UNBOUND";

/// Deferred adapter contract row count — all six THMC ledger fields.
pub const ADAPTER_DEFERRED_ROW_COUNT: usize = 6;

/// Honest production rank-1+ tensor path — **false** until `umst-algebra-burn` lands.
#[must_use]
pub const fn atoms_tensor_lift_adapter_production_wired() -> bool {
    false
}

/// Compile-time fence — production flip not authorized at adapter scaffold tier.
const _: () = assert!(!atoms_tensor_lift_adapter_production_wired());
const _: () = assert!(ADAPTER_SCAFFOLD_LANDED);
const _: () = assert!(!RANK1_PLUS_IMPL_LANDED);
const _: () = assert!(!ADAPTER_CRATE_LANDED);

/// One adapter contract row — maps a slice-3b ledger field to deferred Burn tensor ops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdapterContractRow {
    /// Matching sub-residue id in [`super::atoms_tensor_lift_ledger::RANK1_PLUS_LEDGER_ROWS`].
    pub ledger_sub_id: &'static str,
    /// THMC channel label (T/H/M/C) — mirror of ledger census.
    pub thmc_channel: &'static str,
    /// Phantom space marker name in `field.rs`.
    pub field_marker: &'static str,
    /// Burn tensor rank `D` for `Field<B, Space, D>`.
    pub tensor_rank: u8,
    /// Typical shape note from field census (not enforced at runtime).
    pub typical_shape_note: &'static str,
    /// Adapter bind posture for this carrier — always `UNBOUND` at scaffold tier.
    pub bind_status: &'static str,
    /// Whether `impl TensorAlgebra` over this carrier is landed.
    pub impl_landed: bool,
    /// `contract` semantics — deferred until adapter crate lands.
    pub contract_status: &'static str,
    /// `grad` semantics — deferred until adapter crate lands.
    pub grad_status: &'static str,
}

/// Frozen adapter contract — aligned 1:1 with slice-3b THMC ledger rows.
pub const ADAPTER_CONTRACT_ROWS: &[AdapterContractRow] = &[
    AdapterContractRow {
        ledger_sub_id: "R-ATOMS-F1-T",
        thmc_channel: "T",
        field_marker: "Temperature",
        tensor_rank: 3,
        typical_shape_note: "[B, N, F_T]",
        bind_status: BIND_STATUS,
        impl_landed: false,
        contract_status: "DEFERRED",
        grad_status: "DEFERRED",
    },
    AdapterContractRow {
        ledger_sub_id: "R-ATOMS-F1-H",
        thmc_channel: "H",
        field_marker: "Humidity",
        tensor_rank: 3,
        typical_shape_note: "[B, N, F_h]",
        bind_status: BIND_STATUS,
        impl_landed: false,
        contract_status: "DEFERRED",
        grad_status: "DEFERRED",
    },
    AdapterContractRow {
        ledger_sub_id: "R-ATOMS-F1-u",
        thmc_channel: "M",
        field_marker: "Displacement",
        tensor_rank: 3,
        typical_shape_note: "[B, N, 3]",
        bind_status: BIND_STATUS,
        impl_landed: false,
        contract_status: "DEFERRED",
        grad_status: "DEFERRED",
    },
    AdapterContractRow {
        ledger_sub_id: "R-ATOMS-F1-d",
        thmc_channel: "C",
        field_marker: "Damage",
        tensor_rank: 3,
        typical_shape_note: "[B, N, 1]",
        bind_status: BIND_STATUS,
        impl_landed: false,
        contract_status: "DEFERRED",
        grad_status: "DEFERRED",
    },
    AdapterContractRow {
        ledger_sub_id: "R-ATOMS-F1-alpha",
        thmc_channel: "C",
        field_marker: "ReactionExtent",
        tensor_rank: 3,
        typical_shape_note: "[B, N, F_alpha]",
        bind_status: BIND_STATUS,
        impl_landed: false,
        contract_status: "DEFERRED",
        grad_status: "DEFERRED",
    },
    AdapterContractRow {
        ledger_sub_id: "R-ATOMS-F1-eps",
        thmc_channel: "M",
        field_marker: "SmallStrain",
        tensor_rank: 4,
        typical_shape_note: "[B, N, 3, 3]",
        bind_status: BIND_STATUS,
        impl_landed: false,
        contract_status: "DEFERRED",
        grad_status: "DEFERRED",
    },
];

/// Fleet census row for slice-3c adapter deepen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtomsTensorLiftAdapterDepthSummary {
    pub pbm_id: &'static str,
    pub parent_residue_id: &'static str,
    pub slice_id: &'static str,
    pub posture_tag: &'static str,
    pub adapter_scaffold_landed: bool,
    pub rank1_plus_impl_landed: bool,
    pub adapter_crate_landed: bool,
    pub slice3_lift_step_landed: bool,
    pub slice3b_ledger_landed: bool,
    pub slice3d_ops_landed: bool,
    pub deferred_row_count: usize,
    pub production_wired: bool,
    pub bind_status: &'static str,
    pub honest_fence: &'static str,
}

/// Typed probe for slice-3c adapter honesty (mirrors runtime posture witness).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtomsTensorLiftAdapterHonestyProbe {
    pub slice_id: &'static str,
    pub posture_tag: &'static str,
    pub adapter_scaffold_landed: bool,
    pub rank1_plus_impl_landed: bool,
    pub adapter_crate_landed: bool,
    pub deferred_row_count: usize,
    pub production_wired: bool,
    pub bind_status: &'static str,
    pub honest_fence: &'static str,
}

/// Lookup adapter contract row by ledger sub-id.
#[must_use]
pub fn adapter_contract_row(ledger_sub_id: &str) -> Option<&'static AdapterContractRow> {
    let mut i = 0;
    while i < ADAPTER_CONTRACT_ROWS.len() {
        if ADAPTER_CONTRACT_ROWS[i].ledger_sub_id == ledger_sub_id {
            return Some(&ADAPTER_CONTRACT_ROWS[i]);
        }
        i += 1;
    }
    None
}

/// Count contract rows with deferred `impl TensorAlgebra`.
#[must_use]
pub const fn adapter_deferred_row_count() -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < ADAPTER_CONTRACT_ROWS.len() {
        if !ADAPTER_CONTRACT_ROWS[i].impl_landed {
            count += 1;
        }
        i += 1;
    }
    count
}

/// Count unbound contract rows (scaffold-tier bind fence).
#[must_use]
pub const fn adapter_unbound_row_count() -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < ADAPTER_CONTRACT_ROWS.len() {
        // Const equality on &str via byte compare of known BIND_STATUS.
        let status = ADAPTER_CONTRACT_ROWS[i].bind_status;
        if status.len() == BIND_STATUS.len() {
            let mut j = 0;
            let mut eq = true;
            while j < BIND_STATUS.len() {
                if status.as_bytes()[j] != BIND_STATUS.as_bytes()[j] {
                    eq = false;
                    break;
                }
                j += 1;
            }
            if eq {
                count += 1;
            }
        }
        i += 1;
    }
    count
}

/// Frozen depth summary — honest adapter scaffold on contract census only.
#[must_use]
pub const fn atoms_tensor_lift_adapter_depth_summary() -> AtomsTensorLiftAdapterDepthSummary {
    AtomsTensorLiftAdapterDepthSummary {
        pbm_id: PBM_ID,
        parent_residue_id: PARENT_RESIDUE_ID,
        slice_id: SLICE_ID,
        posture_tag: POSTURE_TAG,
        adapter_scaffold_landed: ADAPTER_SCAFFOLD_LANDED,
        rank1_plus_impl_landed: RANK1_PLUS_IMPL_LANDED,
        adapter_crate_landed: ADAPTER_CRATE_LANDED,
        slice3_lift_step_landed: SLICE3_LIFT_STEP_LANDED,
        slice3b_ledger_landed: SLICE3B_LEDGER_LANDED,
        slice3d_ops_landed: SLICE3D_OPS_LANDED,
        deferred_row_count: adapter_deferred_row_count(),
        production_wired: atoms_tensor_lift_adapter_production_wired(),
        bind_status: BIND_STATUS,
        honest_fence: HONEST_FENCE,
    }
}

/// Build honesty probe for adapter scaffold done-when checks.
#[must_use]
pub const fn atoms_tensor_lift_adapter_honesty_probe() -> AtomsTensorLiftAdapterHonestyProbe {
    AtomsTensorLiftAdapterHonestyProbe {
        slice_id: SLICE_ID,
        posture_tag: POSTURE_TAG,
        adapter_scaffold_landed: ADAPTER_SCAFFOLD_LANDED,
        rank1_plus_impl_landed: RANK1_PLUS_IMPL_LANDED,
        adapter_crate_landed: ADAPTER_CRATE_LANDED,
        deferred_row_count: adapter_deferred_row_count(),
        production_wired: atoms_tensor_lift_adapter_production_wired(),
        bind_status: BIND_STATUS,
        honest_fence: HONEST_FENCE,
    }
}

/// Adapter scaffold landed with production path honestly open / unbound.
#[must_use]
pub fn atoms_tensor_lift_adapter_honesty_holds(probe: &AtomsTensorLiftAdapterHonestyProbe) -> bool {
    probe.slice_id == SLICE_ID
        && probe.posture_tag == POSTURE_TAG
        && probe.adapter_scaffold_landed
        && !probe.rank1_plus_impl_landed
        && !probe.adapter_crate_landed
        && probe.deferred_row_count == ADAPTER_DEFERRED_ROW_COUNT
        && !probe.production_wired
        && probe.bind_status == BIND_STATUS
        && probe.honest_fence.contains("adapter_scaffold_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("bind_status=UNBOUND")
}

/// Validate adapter honesty — fail closed on fake production / bind claims.
pub fn validate_atoms_tensor_lift_adapter_honesty() -> Result<(), &'static str> {
    let probe = atoms_tensor_lift_adapter_honesty_probe();
    if probe.production_wired {
        return Err(
            "atoms_tensor_lift_adapter_production_wired must stay false until umst-algebra-burn",
        );
    }
    if !probe.adapter_scaffold_landed {
        return Err("ADAPTER_SCAFFOLD_LANDED must stay true at AGAP-2127-PBM-010");
    }
    if probe.bind_status != BIND_STATUS {
        return Err("BIND_STATUS must stay UNBOUND at adapter scaffold tier");
    }
    if adapter_unbound_row_count() != ADAPTER_DEFERRED_ROW_COUNT {
        return Err("all adapter contract rows must remain UNBOUND");
    }
    if !atoms_tensor_lift_adapter_honesty_holds(&probe) {
        return Err("atoms_tensor_lift_adapter_honesty_holds failed");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::atoms_tensor_lift_ledger::RANK1_PLUS_LEDGER_ROWS;
    use super::super::atoms_tensor_lift_ops::TENSOR_OP_SPEC_ROWS;
    use crate::core::field::FIELD_CENSUS_ROWS;
    use super::*;

    #[test]
    fn pbm010_slice3c_adapter_metadata_locked() {
        let summary = atoms_tensor_lift_adapter_depth_summary();
        assert_eq!(summary.pbm_id, "PBM-010");
        assert_eq!(summary.parent_residue_id, "R-atoms-scalar");
        assert_eq!(summary.slice_id, "slice-3c");
        assert_eq!(summary.posture_tag, POSTURE_TAG);
        assert!(ADAPTER_SCAFFOLD_LANDED);
        assert!(!RANK1_PLUS_IMPL_LANDED);
        assert!(!ADAPTER_CRATE_LANDED);
        assert!(SLICE3_LIFT_STEP_LANDED);
        assert!(SLICE3B_LEDGER_LANDED);
        assert!(SLICE3D_OPS_LANDED);
        assert!(!summary.production_wired);
        assert_eq!(summary.bind_status, "UNBOUND");
        assert_eq!(summary.deferred_row_count, 6);
        assert_eq!(ADAPTER_DEFERRED_ROW_COUNT, 6);
    }

    #[test]
    fn adapter_contract_rows_align_with_ledger() {
        assert_eq!(ADAPTER_CONTRACT_ROWS.len(), 6);
        assert_eq!(ADAPTER_CONTRACT_ROWS.len(), RANK1_PLUS_LEDGER_ROWS.len());
        assert_eq!(adapter_deferred_row_count(), 6);
        assert_eq!(adapter_unbound_row_count(), 6);
        for ledger_row in RANK1_PLUS_LEDGER_ROWS {
            let contract = adapter_contract_row(ledger_row.sub_id)
                .unwrap_or_else(|| panic!("missing contract for {}", ledger_row.sub_id));
            assert_eq!(contract.field_marker, ledger_row.field_marker);
            assert_eq!(contract.tensor_rank, ledger_row.tensor_rank);
            assert_eq!(contract.thmc_channel, ledger_row.thmc_channel);
            assert!(!contract.impl_landed);
            assert_eq!(contract.contract_status, "DEFERRED");
            assert_eq!(contract.grad_status, "DEFERRED");
            assert_eq!(contract.bind_status, "UNBOUND");
        }
    }

    #[test]
    fn adapter_contract_aligns_with_field_census_shapes() {
        for contract in ADAPTER_CONTRACT_ROWS {
            let census = FIELD_CENSUS_ROWS
                .iter()
                .find(|r| r.marker_name == contract.field_marker)
                .unwrap_or_else(|| panic!("missing field census for {}", contract.field_marker));
            assert_eq!(census.tensor_rank, contract.tensor_rank);
            assert_eq!(census.typical_shape_note, contract.typical_shape_note);
            assert_eq!(census.ledger_sub_id, Some(contract.ledger_sub_id));
        }
    }

    #[test]
    fn adapter_stays_deferred_while_ops_design_specified() {
        assert_eq!(ADAPTER_CONTRACT_ROWS.len(), TENSOR_OP_SPEC_ROWS.len());
        for contract in ADAPTER_CONTRACT_ROWS {
            let spec = TENSOR_OP_SPEC_ROWS
                .iter()
                .find(|r| r.ledger_sub_id == contract.ledger_sub_id)
                .expect("op-spec row");
            assert_eq!(spec.contract_status, "DESIGN_SPECIFIED");
            assert_eq!(spec.grad_status, "DESIGN_SPECIFIED");
            assert_eq!(contract.contract_status, "DEFERRED");
            assert_eq!(contract.grad_status, "DEFERRED");
            assert!(!contract.impl_landed);
            assert!(!spec.impl_landed);
        }
    }

    #[test]
    fn adapter_contract_small_strain_is_rank4() {
        let eps = adapter_contract_row("R-ATOMS-F1-eps").expect("eps row");
        assert_eq!(eps.tensor_rank, 4);
        assert_eq!(eps.field_marker, "SmallStrain");
        assert_eq!(eps.thmc_channel, "M");
        assert_eq!(eps.typical_shape_note, "[B, N, 3, 3]");
    }

    #[test]
    fn adapter_thmc_channels_cover_thmc() {
        let channels: [&str; 6] = ADAPTER_CONTRACT_ROWS
            .iter()
            .map(|r| r.thmc_channel)
            .collect::<Vec<_>>()
            .try_into()
            .expect("six rows");
        assert!(channels.contains(&"T"));
        assert!(channels.contains(&"H"));
        assert!(channels.contains(&"M"));
        assert!(channels.contains(&"C"));
    }

    #[test]
    fn adapter_honesty_fence_holds() {
        assert!(!atoms_tensor_lift_adapter_production_wired());
        assert_eq!(BIND_STATUS, "UNBOUND");
        assert_eq!(
            HONEST_FENCE,
            "adapter_scaffold_landed=true production_wired=false bind_status=UNBOUND"
        );
        let probe = atoms_tensor_lift_adapter_honesty_probe();
        assert!(atoms_tensor_lift_adapter_honesty_holds(&probe));
        assert!(validate_atoms_tensor_lift_adapter_honesty().is_ok());
        let summary = atoms_tensor_lift_adapter_depth_summary();
        assert!(!summary.production_wired);
        assert!(summary.honest_fence.contains("production_wired=false"));
    }

    #[test]
    fn adapter_paths_honest() {
        assert!(SOURCE_ANCHOR_PATH.contains("atoms_tensor_lift_adapter"));
        assert!(ADAPTER_CRATE_PATH.contains("umst-algebra-burn"));
        assert!(SLICE3B_LEDGER_PATH.contains("atoms_tensor_lift_ledger"));
        assert!(SLICE3D_OPS_PATH.contains("atoms_tensor_lift_ops"));
        assert!(FIELD_SSOT_PATH.contains("field.rs"));
        assert_eq!(RECEIPT_SLUG, "COMPLETION_AGAP_AGENT_PBM-010_2127");
        assert_eq!(PRIOR_RECEIPT_SLUG, "COMPLETION_AGAP_AGENT_PBM-010_2033");
    }
}
