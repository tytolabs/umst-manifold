// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Surrogate / reduced-order hooks for Poisson–Nernst–Planck (PNP), gated on **`electrochemistry-pnp`**.
//!
//! The canonical physics implementation lives in [`crate::physics::solvers::electrochemistry`]. This
//! module is intentionally thin: same tensor contract as
//! [`crate::physics::solvers::electrochemistry::ElectroChemicalSolver::solve_pnp_step`],
//! with a no-op passthrough until a learned surrogate is wired in.
//!
//! **Honesty:** posture stays deferred / no-op. Do **not** invent GREEN, PRODUCTION_WIRED, or MASTER.

/// W29 continuous deepen cell for this bridge surface.
pub const PNP_BRIDGE_CELL_ID: &str = "W29-094-PNP_BRIDGE";

/// Honest posture — identity passthrough only; learned surrogate deferred.
pub const PNP_BRIDGE_POSTURE_TAG: &str = "honest-noop-surrogate-deferred";

/// Primary source anchor for fleet / meta hygiene.
pub const PNP_BRIDGE_SOURCE_ANCHOR: &str = "umst-manifold/src/pnp_bridge.rs";

/// Canonical solver surface this bridge mirrors (contract only; not a live wire claim).
pub const PNP_CANONICAL_SOLVER_SURFACE: &str =
    "physics::solvers::electrochemistry::ElectroChemicalSolver::solve_pnp_step";

/// Feature gate that enables the surrogate step symbol.
pub const PNP_BRIDGE_FEATURE_GATE: &str = "electrochemistry-pnp";

/// Learned / reduced-order surrogate is **not** production-wired.
pub const PNP_SURROGATE_PRODUCTION_WIRED: bool = false;

/// Explicit no-op passthrough is the landed behaviour under the feature gate.
pub const PNP_SURROGATE_NOOP_LANDED: bool = true;

/// Learned surrogate wiring remains deferred.
pub const PNP_LEARNED_SURROGATE_DEFERRED: bool = true;

/// Rank-3 electric potential channel count (`[B, N, 1]`).
pub const PNP_PHI_CHANNELS: usize = 1;

/// Rank-3 ion concentration species channels (`[B, N, 2]`).
pub const PNP_ION_CHANNELS: usize = 2;

/// Rank-3 permittivity channel count (`[B, N, 1]`).
pub const PNP_PERMITTIVITY_CHANNELS: usize = 1;

/// Rank-3 diffusivity species channels (`[B, N, 2]`).
pub const PNP_DIFFUSIVITY_CHANNELS: usize = 2;

/// Edge topology leading axis for `edges_b1` (`[2, E]`).
pub const PNP_EDGES_B1_LEADING: usize = 2;

/// Fleet census line — honest deferred posture (no GREEN / PRODUCTION_WIRED invent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PnpBridgeDepthSummary {
    pub cell_id: &'static str,
    pub posture_tag: &'static str,
    pub source_anchor: &'static str,
    pub canonical_solver_surface: &'static str,
    pub feature_gate: &'static str,
    pub surrogate_noop_landed: bool,
    pub learned_surrogate_deferred: bool,
    pub production_wired: bool,
    pub phi_channels: usize,
    pub ion_channels: usize,
}

/// Frozen depth summary — honest no-op / deferred learned surrogate.
#[must_use]
pub const fn pnp_bridge_depth_summary() -> PnpBridgeDepthSummary {
    PnpBridgeDepthSummary {
        cell_id: PNP_BRIDGE_CELL_ID,
        posture_tag: PNP_BRIDGE_POSTURE_TAG,
        source_anchor: PNP_BRIDGE_SOURCE_ANCHOR,
        canonical_solver_surface: PNP_CANONICAL_SOLVER_SURFACE,
        feature_gate: PNP_BRIDGE_FEATURE_GATE,
        surrogate_noop_landed: PNP_SURROGATE_NOOP_LANDED,
        learned_surrogate_deferred: PNP_LEARNED_SURROGATE_DEFERRED,
        production_wired: PNP_SURROGATE_PRODUCTION_WIRED,
        phi_channels: PNP_PHI_CHANNELS,
        ion_channels: PNP_ION_CHANNELS,
    }
}

/// Whether the PNP bridge morphism / honesty fence is pinned at HEAD.
#[must_use]
pub const fn pnp_bridge_morphism_pinned() -> bool {
    const_str_eq(PNP_BRIDGE_CELL_ID, "W29-094-PNP_BRIDGE")
        && const_str_eq(PNP_BRIDGE_POSTURE_TAG, "honest-noop-surrogate-deferred")
        && !PNP_SURROGATE_PRODUCTION_WIRED
        && PNP_LEARNED_SURROGATE_DEFERRED
        && PNP_SURROGATE_NOOP_LANDED
        && pnp_bridge_tensor_contract_pinned()
}

#[must_use]
const fn const_str_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// Production wire claim — **always false** until a measured learned surrogate lands.
#[must_use]
pub const fn pnp_bridge_production_wired() -> bool {
    PNP_SURROGATE_PRODUCTION_WIRED
}

/// Shape-contract witness matching `solve_pnp_step` tensor ranks / channel counts.
#[must_use]
pub const fn pnp_bridge_tensor_contract_pinned() -> bool {
    PNP_PHI_CHANNELS == 1
        && PNP_ION_CHANNELS == 2
        && PNP_PERMITTIVITY_CHANNELS == 1
        && PNP_DIFFUSIVITY_CHANNELS == 2
        && PNP_EDGES_B1_LEADING == 2
        && const_str_eq(PNP_BRIDGE_FEATURE_GATE, "electrochemistry-pnp")
}

#[cfg(feature = "electrochemistry-pnp")]
use crate::physics::solvers::electrochemistry::ElectroChemicalSolver;
#[cfg(feature = "electrochemistry-pnp")]
use burn::tensor::{backend::Backend, Int, Tensor};

/// Placeholder **PNP surrogate** step: same arguments and rank-3 tensor contract as
/// [`ElectroChemicalSolver::solve_pnp_step`](crate::physics::solvers::electrochemistry::ElectroChemicalSolver::solve_pnp_step);
/// currently returns inputs unchanged (explicit no-op).
///
/// # Tensor shapes (must match `solve_pnp_step`)
/// - `electric_potential`: **`[B, N, 1]`**
/// - `ion_concentration`: **`[B, N, 2]`** (e.g. two species channels)
/// - `edges_b1`: **`[2, E]`** (`Int` topology)
/// - `permittivity`: **`[B, N, 1]`**
/// - `diffusivity`: **`[B, N, 2]`**
///
/// Returns **`(electric_potential, ion_concentration)`** with the same shapes as the inputs.
#[cfg(feature = "electrochemistry-pnp")]
#[allow(unused_variables)]
pub fn pnp_surrogate_step<B: Backend<FloatElem = f32>>(
    solver: &ElectroChemicalSolver,
    dt: f32,
    electric_potential: Tensor<B, 3>,
    ion_concentration: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    permittivity: Tensor<B, 3>,
    diffusivity: Tensor<B, 3>,
) -> (Tensor<B, 3>, Tensor<B, 3>) {
    let _ = (solver, dt, edges_b1, permittivity, diffusivity);
    (electric_potential, ion_concentration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pnp_bridge_morphism_identity_pinned() {
        assert!(pnp_bridge_morphism_pinned());
        assert_eq!(PNP_BRIDGE_CELL_ID, "W29-094-PNP_BRIDGE");
        assert_eq!(PNP_BRIDGE_POSTURE_TAG, "honest-noop-surrogate-deferred");
        assert!(PNP_BRIDGE_SOURCE_ANCHOR.contains("pnp_bridge.rs"));
        assert!(PNP_CANONICAL_SOLVER_SURFACE.contains("solve_pnp_step"));
    }

    #[test]
    fn pnp_bridge_depth_summary_honest_deferred() {
        let summary = pnp_bridge_depth_summary();
        assert_eq!(summary.cell_id, PNP_BRIDGE_CELL_ID);
        assert_eq!(summary.posture_tag, PNP_BRIDGE_POSTURE_TAG);
        assert_eq!(summary.source_anchor, PNP_BRIDGE_SOURCE_ANCHOR);
        assert_eq!(summary.feature_gate, "electrochemistry-pnp");
        assert!(summary.surrogate_noop_landed);
        assert!(summary.learned_surrogate_deferred);
        assert!(!summary.production_wired);
        assert_eq!(summary.phi_channels, 1);
        assert_eq!(summary.ion_channels, 2);
    }

    #[test]
    fn pnp_bridge_posture_tag_honest_not_green() {
        let tag = PNP_BRIDGE_POSTURE_TAG.to_ascii_lowercase();
        assert!(tag.contains("honest"));
        assert!(tag.contains("deferred") || tag.contains("noop"));
        assert!(!tag.contains("green"));
        assert!(!tag.contains("master"));
        assert!(!tag.contains("production_wired"));
        assert!(pnp_bridge_morphism_pinned());
    }

    #[test]
    fn pnp_bridge_production_stays_false() {
        assert!(!pnp_bridge_production_wired());
        assert!(!PNP_SURROGATE_PRODUCTION_WIRED);
        assert!(PNP_LEARNED_SURROGATE_DEFERRED);
        assert!(PNP_SURROGATE_NOOP_LANDED);
    }

    #[test]
    fn pnp_bridge_tensor_contract_matches_solve_pnp_step() {
        assert!(pnp_bridge_tensor_contract_pinned());
        assert_eq!(PNP_PHI_CHANNELS, 1);
        assert_eq!(PNP_ION_CHANNELS, 2);
        assert_eq!(PNP_PERMITTIVITY_CHANNELS, 1);
        assert_eq!(PNP_DIFFUSIVITY_CHANNELS, 2);
        assert_eq!(PNP_EDGES_B1_LEADING, 2);
    }

    #[test]
    fn pnp_bridge_refuses_invented_master_or_green_tokens() {
        let blob = format!(
            "{}|{}|{}",
            PNP_BRIDGE_POSTURE_TAG, PNP_BRIDGE_CELL_ID, PNP_CANONICAL_SOLVER_SURFACE
        );
        let lower = blob.to_ascii_lowercase();
        assert!(!lower.contains("production_wired=true"));
        assert!(!lower.contains("green_swarm"));
        assert!(!lower.contains("master_retick"));
        // Cell id may contain PNP_BRIDGE; that is a name, not a GREEN invent.
        assert!(lower.contains("honest") || PNP_BRIDGE_POSTURE_TAG.contains("honest"));
    }

    #[cfg(feature = "electrochemistry-pnp")]
    #[test]
    fn pnp_bridge_surrogate_step_identity_passthrough() {
        use burn::backend::NdArray;
        use burn::tensor::{Int, TensorData};

        type B = NdArray<f32>;

        let device = Default::default();
        let solver = ElectroChemicalSolver::default();
        let phi = Tensor::<B, 3>::from_data(
            TensorData::new(vec![0.1_f32, 0.2, 0.3, 0.4], [1, 4, 1]),
            &device,
        );
        let c = Tensor::<B, 3>::from_data(
            TensorData::new(
                vec![1.0_f32, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                [1, 4, 2],
            ),
            &device,
        );
        let edges = Tensor::<B, 2, Int>::from_data(
            TensorData::new(vec![0_i32, 1, 2, 1, 2, 3], [2, 3]),
            &device,
        );
        let eps = Tensor::<B, 3>::from_data(
            TensorData::new(vec![1.0_f32; 4], [1, 4, 1]),
            &device,
        );
        let d = Tensor::<B, 3>::from_data(
            TensorData::new(vec![1.0_f32; 8], [1, 4, 2]),
            &device,
        );

        let phi_in = phi.clone();
        let c_in = c.clone();
        let (phi_out, c_out) =
            pnp_surrogate_step(&solver, 1e-3_f32, phi, c, edges, eps, d);

        let phi_in_v = phi_in.into_data().to_vec::<f32>().unwrap();
        let phi_out_v = phi_out.into_data().to_vec::<f32>().unwrap();
        let c_in_v = c_in.into_data().to_vec::<f32>().unwrap();
        let c_out_v = c_out.into_data().to_vec::<f32>().unwrap();
        assert_eq!(phi_in_v, phi_out_v, "noop surrogate must preserve Φ");
        assert_eq!(c_in_v, c_out_v, "noop surrogate must preserve c");
        assert!(!pnp_bridge_production_wired());
    }
}
