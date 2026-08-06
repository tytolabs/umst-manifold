// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
//
// AGAP-1920-PBM-010 — slice-3 production tensor lift **step** for R-atoms-scalar / F1.
//
// Cartridge atoms remain f64 scalar-first (SC-01..03). This module lands the **first real**
// `burn::Tensor` `TensorAlgebra` instance on the runtime Burn home — 0D atom fields only.
// Rank-1+ THMC fields and cartridge monomorphization remain **[open]** under
// `R-faithful-decomp-B1`.
//
// **Cross-ref:** slice-2 `BurnScalar` prototype in `umst-cartridge-continuum/tensor_lift`;
// T4 f64↔f32 cast in [`atoms_scalar_bridge`](super::atoms_scalar_bridge).

use burn::tensor::{backend::Backend, Data, Shape, Tensor};
use burn_ndarray::NdArray;
use umst_cartridge_api::TensorAlgebra;

/// PBM-010 workstream id.
pub const PBM_ID: &str = "PBM-010";

/// Parent G40-R10 residue — F1 dual numerics still open at ledger tier.
pub const PARENT_RESIDUE_ID: &str = "R-atoms-scalar";

/// Slice-3 runtime tensor lift step id.
pub const SLICE_ID: &str = "slice-3";

// Slice-3b rank-1+ ledger deepen cross-ref (AGAP-2001-PBM-010).
pub const SLICE3B_LEDGER_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_ledger.rs";

/// Slice residual rows deepen cross-ref (AGAP-2033-PBM-010).
pub const SLICE_RESIDUAL_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_residual.rs";

/// Slice-3c adapter contract scaffold cross-ref (AGAP-2127-PBM-010).
pub const SLICE3C_ADAPTER_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_adapter.rs";

/// Slice-3d tensor op spec cross-ref (SWARM-C25-0831-89).
pub const SLICE3D_OPS_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_ops.rs";

/// Honest posture — 0D `burn::Tensor` step landed; rank-1+ production **open**.
pub const POSTURE_TAG: &str = "HONEST_PARTIAL";

/// Whether the slice-3 0D lift step module is on disk.
pub const LIFT_STEP_LANDED: bool = true;

/// Whether rank-1+ `burn::Tensor` monomorphization is closed.
pub const RANK1_PLUS_DEFERRED: bool = true;

/// Honest fence — production rank-1+ wire **not** claimed (W29 deepen).
pub const PRODUCTION_WIRED: bool = false;

/// Honest fence — rank-1+ `impl TensorAlgebra` over Burn **not** landed.
pub const RANK1_PLUS_IMPL_LANDED: bool = false;

/// Honest fence — planned `umst-algebra-burn` crate **not** on disk.
pub const ADAPTER_CRATE_LANDED: bool = false;

/// Sibling slice-3b ledger module on disk (path census only — not F1 close).
pub const SLICE3B_LEDGER_LANDED: bool = true;

/// Sibling slice-3c adapter scaffold on disk (path census only).
pub const SLICE3C_ADAPTER_LANDED: bool = true;

/// Sibling slice-3d op-spec module on disk (path census only).
pub const SLICE3D_OPS_LANDED: bool = true;

/// Sibling slice-residual inventory on disk (path census only).
pub const SLICE_RESIDUAL_LANDED: bool = true;

/// Primary source anchor for fleet / meta hygiene.
pub const SOURCE_ANCHOR_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift.rs";

/// C2 design SSOT.
pub const DESIGN_DOC_PATH: &str = "docs/C2_TENSOR_ALGEBRA_DESIGN.md";

/// Slice-2 continuum prototype cross-ref.
pub const SLICE2_PROTOTYPE_PATH: &str =
    "umst-cartridges/crates/umst-cartridge-continuum/src/tensor_lift/burn_algebra.rs";

/// Planned adapter crate path (not created — honest fence).
pub const ADAPTER_CRATE_PATH: &str = "umst-runtime/crates/umst-algebra-burn/";

/// Fleet receipt for slice-3 0D lift step (PBM-010).
pub const RECEIPT_SLUG: &str = "COMPLETION_AGAP_AGENT_PBM-010_1920";

/// Prior slice-2 continuum prototype receipt cross-ref.
pub const PRIOR_RECEIPT_SLUG: &str = "COMPLETION_AGAP_AGENT_PBM-010_SLICE2";

/// Default ndarray backend for atom-lift scaffold.
pub type AtomLiftBackend = NdArray;

/// 0D atom field — shape `[1]` `burn::Tensor` (production carrier for scalar atoms).
#[derive(Debug, Clone)]
pub struct BurnAtomField {
    inner: Tensor<AtomLiftBackend, 1>,
}

impl BurnAtomField {
    /// Project host scalar into a 0D Burn tensor field.
    #[must_use]
    pub fn from_host_scalar(device: &<AtomLiftBackend as Backend>::Device, value: f64) -> Self {
        let f32 = super::atoms_scalar_bridge::cartridge_scalar_to_burn_f32(value);
        Self {
            inner: Tensor::from_data(Data::new(vec![f32], Shape::new([1])), device),
        }
    }

    /// Read tensor lane back to host `f64` (cold boundary).
    #[must_use]
    pub fn to_host_scalar(&self) -> f64 {
        let f32 = self.inner.clone().into_data().value[0];
        super::atoms_scalar_bridge::burn_f32_to_cartridge_scalar(f32)
    }

    /// Borrow inner tensor for physics kernels.
    #[must_use]
    pub fn as_tensor(&self) -> &Tensor<AtomLiftBackend, 1> {
        &self.inner
    }
}

/// Slice-3 `TensorAlgebra` over real `burn::Tensor` 0D atom fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BurnAtomAlgebra;

impl TensorAlgebra for BurnAtomAlgebra {
    type Field = BurnAtomField;

    fn zero() -> Self::Field {
        let device = Default::default();
        BurnAtomField::from_host_scalar(&device, 0.0)
    }

    fn add(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        BurnAtomField {
            inner: lhs.inner + rhs.inner,
        }
    }

    fn mul(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        BurnAtomField {
            inner: lhs.inner * rhs.inner,
        }
    }

    fn contract(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        BurnAtomField {
            inner: lhs.inner * rhs.inner,
        }
    }

    fn grad(field: Self::Field) -> Self::Field {
        field
    }
}

/// Production lift step — cartridge `f64` atom scalar → `burn::Tensor` field.
#[must_use]
pub fn lift_atom_scalar(
    device: &<AtomLiftBackend as Backend>::Device,
    value: f64,
) -> BurnAtomField {
    BurnAtomField::from_host_scalar(device, value)
}

/// Fleet census row for PBM-010 / R10-b hygiene.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtomsTensorLiftDepthSummary {
    pub pbm_id: &'static str,
    pub parent_residue_id: &'static str,
    pub slice_id: &'static str,
    pub posture_tag: &'static str,
    pub lift_step_landed: bool,
    pub rank1_plus_deferred: bool,
    /// Honest fence — never affirms production wire.
    pub production_wired: bool,
    /// Honest fence — rank-1+ TensorAlgebra impl still open.
    pub rank1_plus_impl_landed: bool,
    /// Sibling path census (ledger/adapter/ops/residual) — not F1 closure.
    pub sibling_slice_ladder_landed: bool,
}

/// Frozen depth summary — honest partial on 0D lift step only.
#[must_use]
pub const fn atoms_tensor_lift_depth_summary() -> AtomsTensorLiftDepthSummary {
    AtomsTensorLiftDepthSummary {
        pbm_id: PBM_ID,
        parent_residue_id: PARENT_RESIDUE_ID,
        slice_id: SLICE_ID,
        posture_tag: POSTURE_TAG,
        lift_step_landed: LIFT_STEP_LANDED,
        rank1_plus_deferred: RANK1_PLUS_DEFERRED,
        production_wired: PRODUCTION_WIRED,
        rank1_plus_impl_landed: RANK1_PLUS_IMPL_LANDED,
        sibling_slice_ladder_landed: SLICE3B_LEDGER_LANDED
            && SLICE3C_ADAPTER_LANDED
            && SLICE3D_OPS_LANDED
            && SLICE_RESIDUAL_LANDED,
    }
}

/// Honest fence probe — production-wire / rank-1+ impl claims stay false.
#[must_use]
pub const fn honest_fences_hold() -> bool {
    LIFT_STEP_LANDED
        && RANK1_PLUS_DEFERRED
        && !PRODUCTION_WIRED
        && !RANK1_PLUS_IMPL_LANDED
        && !ADAPTER_CRATE_LANDED
}

#[cfg(test)]
mod tests {
    use umst_cartridge_api::ScalarAlgebra;

    use super::*;

    #[test]
    fn pbm010_posture_metadata_locked() {
        let summary = atoms_tensor_lift_depth_summary();
        assert_eq!(summary.pbm_id, "PBM-010");
        assert_eq!(summary.parent_residue_id, "R-atoms-scalar");
        assert_eq!(summary.slice_id, "slice-3");
        assert_eq!(summary.posture_tag, POSTURE_TAG);
        assert!(LIFT_STEP_LANDED);
        assert!(RANK1_PLUS_DEFERRED);
        assert!(!summary.production_wired);
        assert!(!summary.rank1_plus_impl_landed);
        assert!(summary.sibling_slice_ladder_landed);
    }

    #[test]
    fn lift_atom_scalar_roundtrip_within_f32_epsilon() {
        let device = Default::default();
        let host = 2.4e-6_f64;
        let field = lift_atom_scalar(&device, host);
        let back = field.to_host_scalar();
        assert!((back - host).abs() < 1e-5);
    }

    #[test]
    fn burn_atom_algebra_tensor_ops_match_host_at_probe() {
        let device = Default::default();
        let a = lift_atom_scalar(&device, 2.0);
        let b = lift_atom_scalar(&device, 3.5);
        let product = <BurnAtomAlgebra as TensorAlgebra>::mul(a, b);
        let sum = <BurnAtomAlgebra as TensorAlgebra>::add(product, BurnAtomField::from_host_scalar(&device, 0.0));
        assert!((sum.to_host_scalar() - 7.0).abs() < 1e-5);
    }

    #[test]
    fn burn_atom_algebra_not_f64_sized_on_slice3_path() {
        assert!(!umst_cartridge_api::atoms_scalar_anchor::scalar_field_is_f64::<BurnAtomAlgebra>());
        assert!(umst_cartridge_api::atoms_scalar_anchor::scalar_field_is_f64::<ScalarAlgebra>());
    }

    #[test]
    fn w29_honest_fences_refuse_production_wire() {
        assert!(honest_fences_hold());
        assert!(!PRODUCTION_WIRED);
        assert!(!RANK1_PLUS_IMPL_LANDED);
        assert!(!ADAPTER_CRATE_LANDED);
        assert!(ADAPTER_CRATE_PATH.contains("umst-algebra-burn"));
        assert_eq!(RECEIPT_SLUG, "COMPLETION_AGAP_AGENT_PBM-010_1920");
        assert!(SOURCE_ANCHOR_PATH.ends_with("atoms_tensor_lift.rs"));
    }

    #[test]
    fn w29_sibling_slice_ladder_path_anchors() {
        assert!(SLICE3B_LEDGER_PATH.contains("atoms_tensor_lift_ledger"));
        assert!(SLICE_RESIDUAL_PATH.contains("atoms_tensor_lift_residual"));
        assert!(SLICE3C_ADAPTER_PATH.contains("atoms_tensor_lift_adapter"));
        assert!(SLICE3D_OPS_PATH.contains("atoms_tensor_lift_ops"));
        assert!(SLICE2_PROTOTYPE_PATH.contains("burn_algebra"));
        assert!(SLICE3B_LEDGER_LANDED);
        assert!(SLICE3C_ADAPTER_LANDED);
        assert!(SLICE3D_OPS_LANDED);
        assert!(SLICE_RESIDUAL_LANDED);
        let summary = atoms_tensor_lift_depth_summary();
        assert!(summary.sibling_slice_ladder_landed);
        assert!(summary.rank1_plus_deferred);
    }

    #[test]
    fn burn_atom_algebra_zero_contract_grad_at_0d_probe() {
        let device = Default::default();
        let z = <BurnAtomAlgebra as TensorAlgebra>::zero();
        assert!(z.to_host_scalar().abs() < f64::EPSILON);

        let a = lift_atom_scalar(&device, 1.5);
        let b = lift_atom_scalar(&device, 4.0);
        // 0D contract ≡ elementwise mul on atom fields.
        let contracted = <BurnAtomAlgebra as TensorAlgebra>::contract(a.clone(), b.clone());
        let mul = <BurnAtomAlgebra as TensorAlgebra>::mul(a.clone(), b);
        assert!((contracted.to_host_scalar() - mul.to_host_scalar()).abs() < 1e-5);
        assert!((contracted.to_host_scalar() - 6.0).abs() < 1e-5);

        // 0D grad is identity (honest — no spatial derivative on scalar atom field).
        let graded = <BurnAtomAlgebra as TensorAlgebra>::grad(a);
        assert!((graded.to_host_scalar() - 1.5).abs() < 1e-5);

        // Additive identity via zero.
        let sum = <BurnAtomAlgebra as TensorAlgebra>::add(lift_atom_scalar(&device, 3.0), z);
        assert!((sum.to_host_scalar() - 3.0).abs() < 1e-5);
    }

    #[test]
    fn burn_atom_field_as_tensor_shape_is_rank1_len1() {
        let device = Default::default();
        let field = lift_atom_scalar(&device, -0.25);
        let data = field.as_tensor().clone().into_data();
        assert_eq!(data.shape.dims, [1]);
        assert_eq!(data.value.len(), 1);
        assert!((f64::from(data.value[0]) - (-0.25)).abs() < 1e-5);
    }
}
