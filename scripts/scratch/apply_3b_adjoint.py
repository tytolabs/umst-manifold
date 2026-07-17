#!/usr/bin/env python3
"""One-shot patch for FP Wave 3b-2 adjoint Result migration."""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

def patch_adjoint_rs():
    p = ROOT / "src/physics/adjoint.rs"
    t = p.read_text()
    if "use super::error::PhysicsError;" not in t:
        t = t.replace(
            "use super::topology::EdgeTopology;\n",
            "use super::topology::EdgeTopology;\nuse super::error::PhysicsError;\n",
        )
    t = t.replace(
        "    ) -> (Tensor<B, 1>, f32)\n    where\n        B: AutodiffBackend<FloatElem = f32>,\n        B::InnerBackend: Backend<FloatElem = f32>,\n    {\n        let (surrogate, c_raw, _) = Self::forward_loss_with_diagnostics(",
        "    ) -> Result<(Tensor<B, 1>, f32), PhysicsError>\n    where\n        B: AutodiffBackend<FloatElem = f32>,\n        B::InnerBackend: Backend<FloatElem = f32>,\n    {\n        let (surrogate, c_raw, _) = Self::forward_loss_with_diagnostics(",
    )
    t = t.replace(
        "            cross_section_area,\n        );\n        (surrogate, c_raw)\n    }\n\n    /// Same as [`Self::forward_and_loss`]",
        "            cross_section_area,\n        )?;\n        Ok((surrogate, c_raw))\n    }\n\n    /// Same as [`Self::forward_and_loss`]",
    )
    t = t.replace(
        "    ) -> (Tensor<B, 1>, f32, AdjointComplianceDiagnostics)\n    where\n        B: AutodiffBackend<FloatElem = f32>,\n        B::InnerBackend: Backend<FloatElem = f32>,\n    {",
        "    ) -> Result<(Tensor<B, 1>, f32, AdjointComplianceDiagnostics), PhysicsError>\n    where\n        B: AutodiffBackend<FloatElem = f32>,\n        B::InnerBackend: Backend<FloatElem = f32>,\n    {",
    )
    t = t.replace(
        "            );\n\n        let eq_rel = VectorMechanicsSolver::bar_network_equilibrium_rel_residual(",
        "            );\n        pcg.ensure_converged(cg)?;\n\n        let eq_rel = VectorMechanicsSolver::bar_network_equilibrium_rel_residual(",
    )
    t = t.replace(
        "        let c_raw = comp.into_scalar();\n\n        let diag = AdjointComplianceDiagnostics {",
        "        let c_raw = comp.into_scalar();\n        if !c_raw.is_finite() {\n            return Err(PhysicsError::NonFiniteCompliance);\n        }\n\n        let diag = AdjointComplianceDiagnostics {",
    )
    t = t.replace(
        "        (surrogate, c_raw, diag)\n    }\n}",
        "        Ok((surrogate, c_raw, diag))\n    }\n}",
    )
    p.write_text(t)

def patch_adjoint_q1_hex_rs():
    p = ROOT / "src/physics/adjoint_q1_hex.rs"
    t = p.read_text()
    if "use super::error::PhysicsError;" not in t:
        t = t.replace(
            "use super::linear::masked_dot;\n",
            "use super::error::PhysicsError;\nuse super::linear::masked_dot;\n",
        )
    helper = '''
fn ensure_hex_equilibrium(
    eq_rel: f32,
    pcg: &BarNetworkPcgReport,
    cg: &MechanicsInnerLoopConfig,
) -> Result<(), PhysicsError> {
    if pcg.converged_with_cfg(cg) && eq_rel.is_finite() {
        let rel_tol = BarNetworkPcgReport::rel_tol_from_cfg(cg);
        if rel_tol <= 0.0 || eq_rel <= rel_tol {
            return Ok(());
        }
    }
    Err(PhysicsError::Diverged {
        eq_rel,
        pcg_iterations: pcg.iterations,
    })
}

'''
    if "fn ensure_hex_equilibrium" not in t:
        t = t.replace("fn count_nonfinite(v: &[f32]) -> usize {", helper + "fn count_nonfinite(v: &[f32]) -> usize {")
    t = t.replace(
        "    ) -> (Q1HexComplianceAudit, Vec<f32>) {",
        "    ) -> Result<(Q1HexComplianceAudit, Vec<f32>), PhysicsError> {",
    )
    t = t.replace(
        "            None,\n        );\n        let mut compliance = 0.0_f32;",
        "            None,\n        );\n        ensure_hex_equilibrium(state.eq_rel, &state.pcg, cg)?;\n        let mut compliance = 0.0_f32;",
        1,
    )
    t = t.replace(
        "        (audit, state.u)\n    }\n\n    /// Top-layer void-column fractions",
        "        Ok((audit, state.u))\n    }\n\n    /// Top-layer void-column fractions",
    )
    t = t.replace(
        "    ) -> (Tensor<B, 1>, f32)\n    where\n        B: AutodiffBackend<FloatElem = f32>,\n        B::InnerBackend: Backend<FloatElem = f32>,\n    {\n        let (surrogate, c_raw, _) = Self::forward_loss_with_diagnostics(",
        "    ) -> Result<(Tensor<B, 1>, f32), PhysicsError>\n    where\n        B: AutodiffBackend<FloatElem = f32>,\n        B::InnerBackend: Backend<FloatElem = f32>,\n    {\n        let (surrogate, c_raw, _) = Self::forward_loss_with_diagnostics(",
    )
    t = t.replace(
        "            None,\n            None,\n        );\n        (surrogate, c_raw)\n    }\n\n    /// Same as [`Self::forward_and_loss`]",
        "            None,\n            None,\n        )?;\n        Ok((surrogate, c_raw))\n    }\n\n    /// Same as [`Self::forward_and_loss`]",
    )
    t = t.replace(
        "    ) -> (Tensor<B, 1>, f32, AdjointComplianceDiagnostics)\n    where\n        B: AutodiffBackend<FloatElem = f32>,\n        B::InnerBackend: Backend<FloatElem = f32>,\n    {",
        "    ) -> Result<(Tensor<B, 1>, f32, AdjointComplianceDiagnostics), PhysicsError>\n    where\n        B: AutodiffBackend<FloatElem = f32>,\n        B::InnerBackend: Backend<FloatElem = f32>,\n    {",
    )
    t = t.replace(
        "        let c_raw = comp.into_scalar();\n\n        let diag = AdjointComplianceDiagnostics {",
        "        let c_raw = comp.into_scalar();\n        ensure_hex_equilibrium(state.eq_rel, &state.pcg, cg)?;\n        if !c_raw.is_finite() {\n            return Err(PhysicsError::NonFiniteCompliance);\n        }\n\n        let diag = AdjointComplianceDiagnostics {",
    )
    t = t.replace(
        "        (surrogate, c_raw, diag)\n    }\n\n    /// Host diagnostics bundle",
        "        Ok((surrogate, c_raw, diag))\n    }\n\n    /// Host diagnostics bundle",
    )
    p.write_text(t)

if __name__ == "__main__":
    patch_adjoint_rs()
    patch_adjoint_q1_hex_rs()
    print("patched adjoint + adjoint_q1_hex")
