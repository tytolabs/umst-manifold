// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
//
// AGAP-2037-LIB-NALGEBRA — slice-4 `nalgebra` 3×3 `TensorAlgebra` lift **scaffold** (F-09).
//
// First non-scalar `TensorAlgebra` backend on the runtime home — fixed `Matrix3<f64>` carrier
// for small-strain / Cauchy-stress probes. Cartridge B1 monomorphization, Voigt-6 production
// paths, and `num-dual` AD remain **[open]** under `R-faithful-decomp-B1`.
//
// **Cross-ref:** slice-3 `BurnAtomAlgebra` in [`atoms_tensor_lift`](super::atoms_tensor_lift);
// Voigt layout SSOT in [`VectorMechanicsSolver::voigt_strain_from_edge_displacement`](../../physics/mechanics.rs).

use nalgebra::Matrix3;
use umst_cartridge_api::TensorAlgebra;

/// LIB-LEARN-F-NALGEBRA workstream id.
pub const LIB_ID: &str = "LIB-LEARN-F-NALGEBRA";

/// Research blueprint row (F-09).
pub const BLUEPRINT_ROW: &str = "F-09";

/// Parent residue — B1 tensor production path not closed.
pub const PARENT_RESIDUE_ID: &str = "R-faithful-decomp-B1";

/// Slice-4 runtime nalgebra tensor lift scaffold id.
pub const SLICE_ID: &str = "slice-4";

/// Honest posture — 3×3 scaffold landed; cartridge lift + AD **open**.
pub const POSTURE_TAG: &str = "HONEST_PARTIAL";

/// Whether the slice-4 nalgebra scaffold module is on disk.
pub const SCAFFOLD_LANDED: bool = true;

/// Whether B1 cartridge monomorphization over `NalgebraAlgebra` is closed.
pub const CARTRIDGE_LIFT_DEFERRED: bool = true;

/// Whether `num-dual` σ = ∂ψ/∂ε scalar AD is landed (F-10 @ AGAP-2037-LIB-NUMDUAL).
pub const NUM_DUAL_DEFERRED: bool = false;

/// Whether scalar AD σ = ∂ψ/∂ε is landed in continuum fence.
pub const NUM_DUAL_SCALAR_AD_LANDED: bool = true;

/// Honest fence — B1 production wire over `NalgebraAlgebra` **not** claimed.
pub const PRODUCTION_WIRED: bool = false;

/// Honest fence — MASTER / fleet-close tag **not** claimed on this scaffold.
pub const MASTER_CLAIMED: bool = false;

/// Honest fence — `grad` is identity stub (no spatial mesh / FD stencil).
pub const GRAD_IS_IDENTITY_STUB: bool = true;

/// Honest fence — `mul` is Hadamard (component-wise), not matrix product.
pub const MATMUL_PRODUCT_DEFERRED: bool = true;

/// Honest fence — full 3×3 tensor AD σ_ij pairing still open.
pub const NUM_DUAL_TENSOR_AD_LANDED: bool = false;

/// Continuum F-10 module anchor.
pub const NUM_DUAL_MODULE_PATH: &str =
    "umst-cartridges/crates/umst-cartridge-continuum/src/tensor_lift/num_dual_ad.rs";

/// Primary source anchor for fleet / meta hygiene.
pub const SOURCE_ANCHOR_PATH: &str = "umst-manifold/src/runtime/nalgebra_algebra.rs";

/// C2 design SSOT.
pub const DESIGN_DOC_PATH: &str = "docs/C2_TENSOR_ALGEBRA_DESIGN.md";

/// Research libs SSOT.
pub const RESEARCH_DOC_PATH: &str = "archived/residuals/misc-outputs-tmp/RESEARCH_LIBS_FORMAL_FIELD_CHEM_JUL2026.md";

/// Absolute tolerance for symmetry / Voigt round-trip probes.
pub const SYMMETRY_ABS_TOL: f64 = 1e-12;

/// Fixed 3×3 symmetric tensor carrier (small strain / Cauchy stress scaffold).
#[derive(Debug, Clone, PartialEq)]
pub struct NalgebraTensorField {
    inner: Matrix3<f64>,
}

impl NalgebraTensorField {
    /// Borrow the underlying `Matrix3<f64>`.
    #[must_use]
    pub fn as_matrix(&self) -> &Matrix3<f64> {
        &self.inner
    }

    /// Lift a host `Matrix3<f64>` into the runtime carrier.
    #[must_use]
    pub fn from_matrix(matrix: Matrix3<f64>) -> Self {
        Self { inner: matrix }
    }

    /// Project carrier back to host matrix (cold boundary).
    #[must_use]
    pub fn to_matrix(&self) -> Matrix3<f64> {
        self.inner
    }

    /// Rank-2 identity (I₃) — continuum volumetric probe basis.
    #[must_use]
    pub fn identity() -> Self {
        Self::from_matrix(Matrix3::identity())
    }

    /// Scale every component by `alpha` (linear probe; not constitutive law).
    #[must_use]
    pub fn scale(&self, alpha: f64) -> Self {
        Self::from_matrix(self.inner * alpha)
    }

    /// Build a symmetric strain tensor from host Voigt-6 layout
    /// (`[ε_xx, ε_yy, ε_zz, ε_xy, ε_yz, ε_xz]` — matches mechanics Voigt SSOT).
    #[must_use]
    pub fn from_voigt6_symmetric(voigt: [f64; 6]) -> Self {
        let [exx, eyy, ezz, exy, eyz, exz] = voigt;
        Self::from_matrix(Matrix3::new(
            exx, exy, exz, exy, eyy, eyz, exz, eyz, ezz,
        ))
    }

    /// Symmetric Voigt-6 projection (upper-triangle block, `i ≤ j`).
    #[must_use]
    pub fn to_voigt6_symmetric(&self) -> [f64; 6] {
        let m = &self.inner;
        [m[(0, 0)], m[(1, 1)], m[(2, 2)], m[(0, 1)], m[(1, 2)], m[(0, 2)]]
    }

    /// Frobenius norm — scalar probe for parity stubs.
    #[must_use]
    pub fn frobenius_norm(&self) -> f64 {
        self.inner.norm()
    }

    /// Frobenius inner product `A:B = tr(Aᵀ B)` — matches `TensorAlgebra::contract` scalar.
    #[must_use]
    pub fn frobenius_inner(&self, other: &Self) -> f64 {
        (self.inner.transpose() * other.inner).trace()
    }

    /// Trace of the 3×3 carrier (volumetric probe).
    #[must_use]
    pub fn trace(&self) -> f64 {
        self.inner.trace()
    }

    /// Mean hydrostatic / volumetric part `(tr/3) I`.
    #[must_use]
    pub fn volumetric(&self) -> Self {
        let mean = self.trace() / 3.0;
        Self::from_matrix(Matrix3::from_diagonal_element(mean))
    }

    /// Deviatoric part `A − (tr/3) I` — continuum small-strain probe.
    #[must_use]
    pub fn deviatoric(&self) -> Self {
        let vol = self.volumetric();
        Self::from_matrix(self.inner - vol.inner)
    }

    /// Symmetry probe — `|A − Aᵀ|_∞ ≤ tol`.
    #[must_use]
    pub fn is_symmetric(&self, tol: f64) -> bool {
        let skew = self.inner - self.inner.transpose();
        skew.iter().all(|x| x.abs() <= tol)
    }

    /// Force symmetry via `(A + Aᵀ)/2` — Voigt host boundary hygiene.
    #[must_use]
    pub fn symmetrize(&self) -> Self {
        Self::from_matrix((self.inner + self.inner.transpose()) * 0.5)
    }
}

/// Slice-4 `TensorAlgebra` over fixed 3×3 `nalgebra` fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NalgebraAlgebra;

impl TensorAlgebra for NalgebraAlgebra {
    type Field = NalgebraTensorField;

    fn zero() -> Self::Field {
        NalgebraTensorField::from_matrix(Matrix3::zeros())
    }

    fn add(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        NalgebraTensorField::from_matrix(lhs.inner + rhs.inner)
    }

    /// Hadamard (component-wise) product — **not** matrix multiply
    /// (`MATMUL_PRODUCT_DEFERRED`).
    fn mul(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        NalgebraTensorField::from_matrix(lhs.inner.component_mul(&rhs.inner))
    }

    /// Double contraction as diagonal carrier of `tr(Aᵀ B)`.
    fn contract(lhs: Self::Field, rhs: Self::Field) -> Self::Field {
        let trace = lhs.frobenius_inner(&rhs);
        NalgebraTensorField::from_matrix(Matrix3::from_diagonal_element(trace))
    }

    /// Identity stub — spatial `grad` not wired (`GRAD_IS_IDENTITY_STUB`).
    fn grad(field: Self::Field) -> Self::Field {
        field
    }
}

/// Production lift step — symmetric Voigt-6 host snapshot → `Matrix3` field.
#[must_use]
pub fn lift_strain_voigt6(voigt: [f64; 6]) -> NalgebraTensorField {
    NalgebraTensorField::from_voigt6_symmetric(voigt)
}

/// Matrix-product probe kept **outside** `TensorAlgebra::mul` (honest fence).
#[must_use]
pub fn matrix_product_probe(lhs: &NalgebraTensorField, rhs: &NalgebraTensorField) -> NalgebraTensorField {
    NalgebraTensorField::from_matrix(lhs.inner * rhs.inner)
}

/// Fleet census row for LIB-NALGEBRA / F-09 hygiene.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NalgebraTensorLiftDepthSummary {
    pub lib_id: &'static str,
    pub blueprint_row: &'static str,
    pub parent_residue_id: &'static str,
    pub slice_id: &'static str,
    pub posture_tag: &'static str,
    pub scaffold_landed: bool,
    pub cartridge_lift_deferred: bool,
    pub num_dual_deferred: bool,
    /// Honest fence — never affirms production wire.
    pub production_wired: bool,
    /// Honest fence — MASTER tag stays false.
    pub master_claimed: bool,
    /// Honest fence — grad remains identity stub.
    pub grad_is_identity_stub: bool,
    /// Honest fence — TensorAlgebra::mul is Hadamard only.
    pub matmul_product_deferred: bool,
    /// Honest fence — full tensor AD pairing still open.
    pub num_dual_tensor_ad_landed: bool,
}

/// Frozen depth summary — honest partial on 3×3 scaffold only.
#[must_use]
pub const fn nalgebra_tensor_lift_depth_summary() -> NalgebraTensorLiftDepthSummary {
    NalgebraTensorLiftDepthSummary {
        lib_id: LIB_ID,
        blueprint_row: BLUEPRINT_ROW,
        parent_residue_id: PARENT_RESIDUE_ID,
        slice_id: SLICE_ID,
        posture_tag: POSTURE_TAG,
        scaffold_landed: SCAFFOLD_LANDED,
        cartridge_lift_deferred: CARTRIDGE_LIFT_DEFERRED,
        num_dual_deferred: NUM_DUAL_DEFERRED,
        production_wired: PRODUCTION_WIRED,
        master_claimed: MASTER_CLAIMED,
        grad_is_identity_stub: GRAD_IS_IDENTITY_STUB,
        matmul_product_deferred: MATMUL_PRODUCT_DEFERRED,
        num_dual_tensor_ad_landed: NUM_DUAL_TENSOR_AD_LANDED,
    }
}

/// Honest fence probe — production / MASTER / matmul / tensor-AD claims stay false.
#[must_use]
pub const fn honest_fences_hold() -> bool {
    SCAFFOLD_LANDED
        && CARTRIDGE_LIFT_DEFERRED
        && !PRODUCTION_WIRED
        && !MASTER_CLAIMED
        && GRAD_IS_IDENTITY_STUB
        && MATMUL_PRODUCT_DEFERRED
        && !NUM_DUAL_TENSOR_AD_LANDED
        && NUM_DUAL_SCALAR_AD_LANDED
        && !NUM_DUAL_DEFERRED
}


#[cfg(test)]
mod tests {
    use umst_cartridge_api::ScalarAlgebra;

    use super::*;

    #[test]
    fn lib_nalgebra_posture_metadata_locked() {
        let summary = nalgebra_tensor_lift_depth_summary();
        assert_eq!(summary.lib_id, "LIB-LEARN-F-NALGEBRA");
        assert_eq!(summary.blueprint_row, "F-09");
        assert_eq!(summary.parent_residue_id, "R-faithful-decomp-B1");
        assert_eq!(summary.slice_id, "slice-4");
        assert_eq!(summary.posture_tag, POSTURE_TAG);
        assert!(SCAFFOLD_LANDED);
        assert!(CARTRIDGE_LIFT_DEFERRED);
        assert!(!NUM_DUAL_DEFERRED);
        assert!(NUM_DUAL_SCALAR_AD_LANDED);
        assert!(!summary.production_wired);
        assert!(!summary.master_claimed);
        assert!(summary.grad_is_identity_stub);
        assert!(summary.matmul_product_deferred);
        assert!(!summary.num_dual_tensor_ad_landed);
        assert!(honest_fences_hold());
    }

    #[test]
    fn honest_fences_refuse_green_production_master() {
        assert_ne!(POSTURE_TAG, "GREEN");
        assert_ne!(POSTURE_TAG, "PRODUCTION_WIRED");
        assert_ne!(POSTURE_TAG, "MASTER");
        assert!(!PRODUCTION_WIRED);
        assert!(!MASTER_CLAIMED);
        assert!(GRAD_IS_IDENTITY_STUB);
        assert!(MATMUL_PRODUCT_DEFERRED);
    }

    #[test]
    fn voigt6_roundtrip_preserves_symmetric_components() {
        let voigt = [1.0e-4, -2.0e-4, 3.0e-4, 0.5e-4, -0.25e-4, 0.1e-4];
        let field = lift_strain_voigt6(voigt);
        assert!(field.is_symmetric(SYMMETRY_ABS_TOL));
        let back = field.to_voigt6_symmetric();
        for (a, b) in voigt.iter().zip(back.iter()) {
            assert!((a - b).abs() < 1e-12);
        }
    }

    #[test]
    fn symmetrize_kills_skew_component() {
        let skewish = NalgebraTensorField::from_matrix(Matrix3::new(
            1.0, 2.0, 0.0, 0.5, 1.0, 0.0, 0.0, 0.0, 1.0,
        ));
        assert!(!skewish.is_symmetric(SYMMETRY_ABS_TOL));
        let sym = skewish.symmetrize();
        assert!(sym.is_symmetric(SYMMETRY_ABS_TOL));
        assert!((sym.as_matrix()[(0, 1)] - 1.25).abs() < 1e-12);
    }

    #[test]
    fn volumetric_deviatoric_split_reconstructs() {
        let field = lift_strain_voigt6([3.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let vol = field.volumetric();
        let dev = field.deviatoric();
        let recon = <NalgebraAlgebra as TensorAlgebra>::add(vol, dev);
        for i in 0..3 {
            for j in 0..3 {
                assert!((recon.as_matrix()[(i, j)] - field.as_matrix()[(i, j)]).abs() < 1e-12);
            }
        }
        assert!((dev.trace()).abs() < 1e-12);
        assert!((vol.trace() - field.trace()).abs() < 1e-12);
    }

    #[test]
    fn frobenius_inner_matches_contract_diagonal() {
        let a = lift_strain_voigt6([1.0, 2.0, 3.0, 0.5, -0.25, 0.1]);
        let b = lift_strain_voigt6([0.5, 1.0, 1.5, 0.0, 0.0, 0.0]);
        let inner = a.frobenius_inner(&b);
        let contracted = <NalgebraAlgebra as TensorAlgebra>::contract(a.clone(), b);
        let voigt = contracted.to_voigt6_symmetric();
        assert!((voigt[0] - inner).abs() < 1e-12);
        assert!((voigt[1] - inner).abs() < 1e-12);
        assert!((voigt[2] - inner).abs() < 1e-12);
    }

    #[test]
    fn nalgebra_algebra_tensor_ops_at_probe() {
        let a = lift_strain_voigt6([1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let b = lift_strain_voigt6([2.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let product = <NalgebraAlgebra as TensorAlgebra>::mul(a.clone(), b.clone());
        let sum = <NalgebraAlgebra as TensorAlgebra>::add(
            product,
            NalgebraTensorField::from_matrix(Matrix3::zeros()),
        );
        assert!((sum.to_voigt6_symmetric()[0] - 2.0).abs() < 1e-12);

        // Hadamard ≠ matrix product when off-diagonals present.
        let c = lift_strain_voigt6([0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
        let d = lift_strain_voigt6([0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
        let hadamard = <NalgebraAlgebra as TensorAlgebra>::mul(c.clone(), d.clone());
        let matmul = matrix_product_probe(&c, &d);
        assert!((hadamard.to_voigt6_symmetric()[3] - 1.0).abs() < 1e-12);
        assert!((matmul.as_matrix()[(0, 0)] - 1.0).abs() < 1e-12);
        assert!((matmul.to_voigt6_symmetric()[3]).abs() < 1e-12);
    }

    #[test]
    fn nalgebra_algebra_contract_yields_trace_on_diagonal() {
        let a = lift_strain_voigt6([1.0, 2.0, 3.0, 0.0, 0.0, 0.0]);
        let b = lift_strain_voigt6([1.0, 1.0, 1.0, 0.0, 0.0, 0.0]);
        let contracted = <NalgebraAlgebra as TensorAlgebra>::contract(a, b);
        let voigt = contracted.to_voigt6_symmetric();
        assert!((voigt[0] - 6.0).abs() < 1e-12);
        assert!((voigt[1] - 6.0).abs() < 1e-12);
        assert!((voigt[2] - 6.0).abs() < 1e-12);
    }

    #[test]
    fn grad_is_identity_stub_measured() {
        let a = lift_strain_voigt6([1.0, -1.0, 0.5, 0.1, 0.0, 0.0]);
        let graded = <NalgebraAlgebra as TensorAlgebra>::grad(a.clone());
        assert_eq!(graded.to_voigt6_symmetric(), a.to_voigt6_symmetric());
        assert!(GRAD_IS_IDENTITY_STUB);
    }

    #[test]
    fn identity_scale_and_zero_algebra() {
        let i = NalgebraTensorField::identity();
        assert!((i.trace() - 3.0).abs() < 1e-12);
        let scaled = i.scale(2.0);
        assert!((scaled.trace() - 6.0).abs() < 1e-12);
        let z = <NalgebraAlgebra as TensorAlgebra>::zero();
        let sum = <NalgebraAlgebra as TensorAlgebra>::add(scaled, z);
        assert!((sum.frobenius_norm() - scaled.frobenius_norm()).abs() < 1e-12);
    }

    #[test]
    fn nalgebra_algebra_not_f64_sized_on_slice4_path() {
        assert!(!umst_cartridge_api::atoms_scalar_anchor::scalar_field_is_f64::<NalgebraAlgebra>());
        assert!(umst_cartridge_api::atoms_scalar_anchor::scalar_field_is_f64::<ScalarAlgebra>());
    }
}
