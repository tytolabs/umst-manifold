// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Consume [`BurnLiquidPPOAgent::step_and_learn`] on an egoff hop graph.
//!
//! Hop nodes become UMST nodes; `info_gain` is I(claim;witness) bits (frugal MI).
//! I=0 is the licence not to look — occupancy hops never enter the Burn hot path.
//! This is consume-not-fork of [`crate::ai::liquid_ppo`] — not a second kernel, not chem ECO,
//! not physics GREEN.

use crate::ai::adjoint::ADJOINT_POLICY_DIM;
use crate::ai::liquid_ppo::{active_adapt_path, BurnLiquidPPOAgent};
use crate::ai::ppo::ManifoldGateway;
use crate::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
use crate::core::traits::{IScienceCartridge, PhysicalResult};
use crate::core::umst_schema::{ScalarChannelId, UMST_SCALAR_CHANNEL_COUNT};
use burn::tensor::backend::Backend;
use burn::tensor::{Data, Int, Shape, Tensor};
use serde::{Deserialize, Serialize};

/// Consume morphism — the Burn spine, not a Python stand-in.
pub const HOP_GRAPH_LIQUID_PPO_MORPHISM: &str = "BurnLiquidPPOAgent::step_and_learn";

/// Source SSOT (read-only consume).
pub const HOP_GRAPH_LIQUID_PPO_SOURCE: &str = "umst/umst-manifold/src/ai/liquid_ppo.rs";

/// Schema for the hop-graph consume receipt.
pub const HOP_GRAPH_LIQUID_PPO_SCHEMA: &str = "egoff_hop_graph_liquid_ppo_v1";

/// NamedSurrogate T — SSOT [`crate::constants::AMBIENT_REFERENCE_TEMPERATURE_K`]. Do not mint k_B.
pub const HOP_GATEWAY_TEMPERATURE_K: f64 = crate::constants::AMBIENT_REFERENCE_TEMPERATURE_K;

/// Floor credit so CBF bookkeeping can run when hop joules are sub-Landauer.
pub const HOP_GATEWAY_CREDIT_FLOOR_J: f64 = 1.0e-12;

pub const HOP_GRAPH_LIQUID_PPO_NON_CLAIM: &str =
    "BurnLiquidPPOAgent::step_and_learn consume on egoff hop graph; info_gain = I_witness bits; I=0 occupancy refused; not physics GREEN; not CHEM_ECO_LIQUID_PPO_ON_CHEM_WIRED; not production_wired";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HopNode {
    #[serde(default)]
    pub attempt: u32,
    #[serde(default)]
    pub i_proxy: f64,
    #[serde(default)]
    pub i_witness_bits: f64,
    #[serde(default)]
    pub compiler_ok: bool,
    #[serde(default)]
    pub goal_reached: bool,
    #[serde(default)]
    pub eval_count: u64,
    #[serde(default)]
    pub landauer_joules: f64,
    #[serde(default)]
    pub energy_unmeasured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HopGraph {
    #[serde(default)]
    pub cell_id: String,
    #[serde(default)]
    pub nodes: Vec<HopNode>,
    /// Flat AdamW θ — restored across consume-not-fork hops (dim [`ADJOINT_POLICY_DIM`]).
    #[serde(default)]
    pub policy_weights: Option<Vec<f32>>,
    #[serde(default)]
    pub adam_m1: Option<Vec<f32>>,
    #[serde(default)]
    pub adam_m2: Option<Vec<f32>>,
    #[serde(default)]
    pub adam_t: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HopPpoReceipt {
    pub schema: &'static str,
    pub morphism: &'static str,
    pub source: &'static str,
    pub adapt_path: &'static str,
    pub cell_id: String,
    pub step_and_learn_ran: bool,
    pub occupancy_refused: bool,
    pub i_witness_bits: f32,
    pub policy_weight_before: Option<f32>,
    pub policy_weight_after: Option<f32>,
    pub weights_moved: bool,
    pub policy_weights: Option<Vec<f32>>,
    pub adam_m1: Option<Vec<f32>>,
    pub adam_m2: Option<Vec<f32>>,
    pub adam_t: usize,
    pub error: Option<String>,
    pub physics_green: bool,
    pub chem_eco_liquid_ppo_on_chem_wired: bool,
    pub production_wired: bool,
    pub non_claim: &'static str,
}

impl HopPpoReceipt {
    fn refuse_occupancy(cell_id: String, i_witness_bits: f32) -> Self {
        Self {
            schema: HOP_GRAPH_LIQUID_PPO_SCHEMA,
            morphism: HOP_GRAPH_LIQUID_PPO_MORPHISM,
            source: HOP_GRAPH_LIQUID_PPO_SOURCE,
            adapt_path: active_adapt_path(),
            cell_id,
            step_and_learn_ran: false,
            occupancy_refused: true,
            i_witness_bits,
            policy_weight_before: None,
            policy_weight_after: None,
            weights_moved: false,
            policy_weights: None,
            adam_m1: None,
            adam_m2: None,
            adam_t: 0,
            error: None,
            physics_green: false,
            chem_eco_liquid_ppo_on_chem_wired: false,
            production_wired: false,
            non_claim: HOP_GRAPH_LIQUID_PPO_NON_CLAIM,
        }
    }
}

/// Last-hop I_witness bits. Mean I dilutes Burn (occupancy theater).
#[must_use]
pub fn graph_i_witness_bits(graph: &HopGraph) -> f32 {
    graph
        .nodes
        .last()
        .map(|n| n.i_witness_bits.max(0.0) as f32)
        .unwrap_or(0.0)
}

/// Digest of AdamW θ — persist is digest+file, not a 1024-float JSON tax.
#[must_use]
pub fn theta_digest(weights: &[f32]) -> String {
    let mut h = 0xCBF2_9CE4_8422_2325_u64;
    h ^= weights.len() as u64;
    for (i, w) in weights.iter().enumerate() {
        h = h.wrapping_mul(0x0100_0000_01B3).wrapping_add(u64::from(w.to_bits()) ^ (i as u64));
    }
    format!("{h:016x}")
}

/// Write θ bytes next to a digest. CORPUS should store the digest, not the vec.
pub fn persist_theta(
    dir: &std::path::Path,
    cell_id: &str,
    weights: &[f32],
) -> std::io::Result<(String, std::path::PathBuf)> {
    let digest = theta_digest(weights);
    std::fs::create_dir_all(dir)?;
    let safe: String = cell_id
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '_' })
        .collect();
    let path = dir.join(format!("{safe}.{digest}.f32bin"));
    let mut bytes = Vec::with_capacity(weights.len() * 4);
    for w in weights {
        bytes.extend_from_slice(&w.to_le_bytes());
    }
    std::fs::write(&path, &bytes)?;
    Ok((digest, path))
}

/// Cartridge: hop-graph remaining-I / paid joules into [`PhysicalResult`] for the gateway.
/// Not chem G. Not a Burn kernel copy.
pub struct EgoffHopGraphCartridge;

impl<B: Backend<FloatElem = f32>> IScienceCartridge<B> for EgoffHopGraphCartridge {
    fn compute_all(&self, mix: &MaterialCompositionTensor<B>) -> PhysicalResult<B> {
        let d = mix.fractions.device();
        PhysicalResult {
            free_energy: Tensor::zeros([1, 1], &d),
            dissipation: Tensor::zeros([1, 1], &d),
            safety_margin: Tensor::zeros([1, 1], &d),
            cost: Tensor::zeros([1, 1], &d),
            damage: Tensor::zeros([1, 1], &d),
            temperature_delta: None,
            #[cfg(feature = "information_density")]
            information_density: Tensor::zeros([1, 1], &d),
        }
    }

    fn compute_topology(&self, m: &UnifiedMaterialStateTensor<B>) -> PhysicalResult<B> {
        let d = m.scalar_features.device();
        let n = m.scalar_features.dims()[0];
        let f = m.scalar_features.dims()[1];
        let vals = m.scalar_features.clone().into_data().value;
        let ch0 = ScalarChannelId::Channel0.index();
        let dmg = ScalarChannelId::Damage.index();
        let mut free = vec![0.0_f32; n];
        let mut damage = vec![0.0_f32; n];
        for i in 0..n {
            let i_proxy = vals[i * f + ch0].clamp(0.0, 1.0);
            free[i] = (1.0 - i_proxy) * 1.0e-12_f32;
            damage[i] = vals[i * f + dmg].clamp(0.0, 1.0) * 1.0e-12_f32;
        }
        PhysicalResult {
            free_energy: Tensor::from_data(Data::new(free, Shape::new([1, n])), &d),
            dissipation: Tensor::full([1, n], 1.0e-20_f32, &d),
            safety_margin: Tensor::ones([1, n], &d),
            cost: Tensor::full([1, n], 1.0e-18_f32, &d),
            damage: Tensor::from_data(Data::new(damage, Shape::new([1, n])), &d),
            temperature_delta: None,
            #[cfg(feature = "information_density")]
            information_density: Tensor::zeros([1, n], &d),
        }
    }
}

fn hop_umst<B: Backend<FloatElem = f32, IntElem = i64>>(
    graph: &HopGraph,
    device: &B::Device,
) -> UnifiedMaterialStateTensor<B> {
    let n = graph.nodes.len().max(2);
    let f = UMST_SCALAR_CHANNEL_COUNT;
    let mut scalars = vec![0.0_f32; n * f];
    for (i, node) in graph.nodes.iter().enumerate() {
        let base = i * f;
        scalars[base + ScalarChannelId::Channel0.index()] = node.i_proxy.clamp(0.0, 1.0) as f32;
        scalars[base + ScalarChannelId::EpistemicUncertainty.index()] =
            (1.0 - node.i_proxy.clamp(0.0, 1.0)) as f32;
        scalars[base + ScalarChannelId::Damage.index()] = if node.compiler_ok { 0.0 } else { 1.0 };
    }
    if graph.nodes.len() == 1 {
        let row0 = scalars[..f].to_vec();
        scalars[f..2 * f].copy_from_slice(&row0);
    }
    let e = (n - 1).max(1) * 2;
    let mut edge_vals = Vec::with_capacity(2 * e);
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    for i in 0..(n - 1).max(1) {
        let a = i as i64;
        let b = ((i + 1) % n) as i64;
        starts.push(a);
        ends.push(b);
        starts.push(b);
        ends.push(a);
    }
    edge_vals.extend_from_slice(&starts);
    edge_vals.extend_from_slice(&ends);
    let coords: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), device);
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edge_vals, Shape::new([2, e])), device);
    let faces_b2: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), device);
    UnifiedMaterialStateTensor {
        coords,
        edges_b1,
        faces_b2,
        scalar_features: Tensor::from_data(Data::new(scalars, Shape::new([n, f])), device),
        vector_features: Tensor::zeros([n, 1, 3], device),
        matrix_features: Tensor::zeros([n, 1, 3, 3], device),
        resolution_mm: [1.0, 1.0, 1.0],
        node_positions: None,
        displacement_bc_mask: Tensor::ones([1, n, 3], device),
        policy_editable_mask: Tensor::ones([n, 1], device),
        #[cfg(feature = "formal-witness")]
        catalog_schema_digest: None,
    }
}

/// Run Burn `step_and_learn` on the hop graph. I=0 occupancy is refused before tensors.
pub fn step_and_learn_hop_graph(graph: &HopGraph) -> HopPpoReceipt {
    step_and_learn_hop_graph_on::<burn_ndarray::NdArray<f32>>(graph, &burn_ndarray::NdArrayDevice::default())
}

fn load_policy_vec<B: Backend<FloatElem = f32>>(
    values: &[f32],
    device: &B::Device,
) -> Option<Tensor<B, 1>> {
    if values.len() == ADJOINT_POLICY_DIM {
        Some(Tensor::from_data(
            Data::new(values.to_vec(), Shape::new([ADJOINT_POLICY_DIM])),
            device,
        ))
    } else {
        None
    }
}

fn step_and_learn_hop_graph_on<B: Backend<FloatElem = f32, IntElem = i64>>(
    graph: &HopGraph,
    device: &B::Device,
) -> HopPpoReceipt
where
    B::Device: Default,
{
    let i_w = graph_i_witness_bits(graph);
    if i_w <= 0.0 {
        return HopPpoReceipt::refuse_occupancy(graph.cell_id.clone(), i_w);
    }
    let credit = graph
        .nodes
        .iter()
        .map(|n| n.landauer_joules)
        .sum::<f64>()
        .max(HOP_GATEWAY_CREDIT_FLOOR_J);
    let gateway = ManifoldGateway::new(
        EgoffHopGraphCartridge,
        HOP_GATEWAY_TEMPERATURE_K,
        credit,
    );
    let mut agent = BurnLiquidPPOAgent::new(gateway);
    if let Some(w) = graph.policy_weights.as_ref() {
        if let Some(t) = load_policy_vec::<B>(w, device) {
            agent.ode_solver.policy_weights = t;
        }
    }
    if let Some(m1) = graph.adam_m1.as_ref() {
        agent.ode_solver.adam_m1 = load_policy_vec::<B>(m1, device);
    }
    if let Some(m2) = graph.adam_m2.as_ref() {
        agent.ode_solver.adam_m2 = load_policy_vec::<B>(m2, device);
    }
    agent.ode_solver.adam_t = graph.adam_t;
    let state = hop_umst::<B>(graph, device);
    let info = Tensor::<B, 1>::full([1], i_w, device);
    let dt = Tensor::<B, 1>::full([1], 1.0_f32, device);
    let w0 = agent.ode_solver.policy_weights.clone().into_data().value[0];
    match agent.step_and_learn(state, 0.0_f32, 1.0_f32, info, dt) {
        Ok(_) => {
            let weights = agent.ode_solver.policy_weights.clone().into_data().value;
            let w1 = weights[0];
            let adam_m1 = agent
                .ode_solver
                .adam_m1
                .as_ref()
                .map(|t| t.clone().into_data().value);
            let adam_m2 = agent
                .ode_solver
                .adam_m2
                .as_ref()
                .map(|t| t.clone().into_data().value);
            HopPpoReceipt {
                schema: HOP_GRAPH_LIQUID_PPO_SCHEMA,
                morphism: HOP_GRAPH_LIQUID_PPO_MORPHISM,
                source: HOP_GRAPH_LIQUID_PPO_SOURCE,
                adapt_path: active_adapt_path(),
                cell_id: graph.cell_id.clone(),
                step_and_learn_ran: true,
                occupancy_refused: false,
                i_witness_bits: i_w,
                policy_weight_before: Some(w0),
                policy_weight_after: Some(w1),
                weights_moved: w0 != w1 && w0.is_finite() && w1.is_finite(),
                policy_weights: Some(weights),
                adam_m1,
                adam_m2,
                adam_t: agent.ode_solver.adam_t,
                error: None,
                physics_green: false,
                chem_eco_liquid_ppo_on_chem_wired: false,
                production_wired: false,
                non_claim: HOP_GRAPH_LIQUID_PPO_NON_CLAIM,
            }
        }
        Err(e) => HopPpoReceipt {
            schema: HOP_GRAPH_LIQUID_PPO_SCHEMA,
            morphism: HOP_GRAPH_LIQUID_PPO_MORPHISM,
            source: HOP_GRAPH_LIQUID_PPO_SOURCE,
            adapt_path: active_adapt_path(),
            cell_id: graph.cell_id.clone(),
            step_and_learn_ran: false,
            occupancy_refused: false,
            i_witness_bits: i_w,
            policy_weight_before: Some(w0),
            policy_weight_after: None,
            weights_moved: false,
            policy_weights: graph.policy_weights.clone(),
            adam_m1: graph.adam_m1.clone(),
            adam_m2: graph.adam_m2.clone(),
            adam_t: graph.adam_t,
            error: Some(e),
            physics_green: false,
            chem_eco_liquid_ppo_on_chem_wired: false,
            production_wired: false,
            non_claim: HOP_GRAPH_LIQUID_PPO_NON_CLAIM,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measured_node(attempt: u32, i_proxy: f64, compiler_ok: bool) -> HopNode {
        HopNode {
            attempt,
            i_proxy,
            i_witness_bits: if compiler_ok { 4.0 } else { 1.0 },
            compiler_ok,
            goal_reached: i_proxy >= 1.0,
            eval_count: 16,
            landauer_joules: 1.0e-20,
            energy_unmeasured: false,
        }
    }

    #[test]
    fn occupancy_i_zero_refuses_before_burn() {
        let g = HopGraph {
            cell_id: "EGOFF-ADK-OCCUPANCY".into(),
            nodes: vec![HopNode {
                attempt: 1,
                i_proxy: 0.0,
                i_witness_bits: 0.0,
                energy_unmeasured: true,
                ..HopNode::default()
            }],
            ..HopGraph::default()
        };
        let r = step_and_learn_hop_graph(&g);
        assert!(r.occupancy_refused);
        assert!(!r.step_and_learn_ran);
        assert!(!r.physics_green);
        assert!(!r.chem_eco_liquid_ppo_on_chem_wired);
        assert_eq!(r.morphism, "BurnLiquidPPOAgent::step_and_learn");
    }

    #[test]
    fn i_positive_runs_burn_step_and_learn_and_moves_adamw_weights() {
        let g = HopGraph {
            cell_id: "EGOFF-ADK-BURN-PPO".into(),
            nodes: vec![
                measured_node(1, 0.25, false),
                measured_node(2, 1.0, true),
            ],
            ..HopGraph::default()
        };
        let r = step_and_learn_hop_graph(&g);
        assert!(
            r.step_and_learn_ran,
            "Burn step_and_learn must run: {:?}",
            r.error
        );
        assert!(!r.occupancy_refused);
        assert!(r.weights_moved, "AdamW must move policy_weights");
        assert!(!r.physics_green);
        assert!(!r.chem_eco_liquid_ppo_on_chem_wired);
        assert!(!r.production_wired);
        assert_eq!(r.morphism, HOP_GRAPH_LIQUID_PPO_MORPHISM);
        assert_eq!(
            r.policy_weights.as_ref().map(|w| w.len()),
            Some(ADJOINT_POLICY_DIM)
        );
        assert!(r.adam_t >= 1);
    }

    #[test]
    fn consume_not_fork_restores_adamw_moments() {
        let g = HopGraph {
            cell_id: "EGOFF-ADK-BURN-PPO".into(),
            nodes: vec![measured_node(1, 0.5, true)],
            policy_weights: None,
            adam_m1: None,
            adam_m2: None,
            adam_t: 0,
        };
        let r1 = step_and_learn_hop_graph(&g);
        assert!(r1.step_and_learn_ran, "first step: {:?}", r1.error);
        let g2 = HopGraph {
            cell_id: g.cell_id.clone(),
            nodes: vec![measured_node(1, 0.5, true), measured_node(2, 1.0, true)],
            policy_weights: r1.policy_weights.clone(),
            adam_m1: r1.adam_m1.clone(),
            adam_m2: r1.adam_m2.clone(),
            adam_t: r1.adam_t,
        };
        let r2 = step_and_learn_hop_graph(&g2);
        assert!(r2.step_and_learn_ran, "restored AdamW: {:?}", r2.error);
        assert!(r2.adam_t > r1.adam_t, "AdamW time must continue, not reset");
        assert!(!r2.chem_eco_liquid_ppo_on_chem_wired);
    }

    #[test]
    fn chem_eco_stays_false_after_consume() {
        let g = HopGraph {
            cell_id: "EGOFF-ADK-BURN-PPO".into(),
            nodes: vec![measured_node(1, 0.5, true)],
            ..HopGraph::default()
        };
        let r = step_and_learn_hop_graph(&g);
        assert!(!r.chem_eco_liquid_ppo_on_chem_wired);
        assert!(!r.physics_green);
    }

    #[test]
    fn last_hop_i_not_mean_dilutes_burn() {
        let g = HopGraph {
            cell_id: "EGOFF-ADK-MEAN-I".into(),
            nodes: vec![
                measured_node(1, 1.0, true),
                HopNode {
                    attempt: 2,
                    i_proxy: 0.0,
                    i_witness_bits: 0.0,
                    energy_unmeasured: true,
                    ..HopNode::default()
                },
            ],
            ..HopGraph::default()
        };
        let r = step_and_learn_hop_graph(&g);
        assert!(
            r.occupancy_refused,
            "last hop I=0 must refuse even if earlier hops had I>0 (mean dilution)"
        );
        assert!(!r.step_and_learn_ran);
        assert_eq!(graph_i_witness_bits(&g), 0.0);
    }

    #[test]
    fn theta_persist_is_digest_and_file_not_1024_json_tax() {
        let weights = vec![0.1_f32, 0.2, 0.3];
        let digest = theta_digest(&weights);
        assert_eq!(digest.len(), 16);
        let dir = std::env::temp_dir().join(format!(
            "egoff_theta_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        let (d2, path) = persist_theta(&dir, "CELL-A", &weights).expect("persist");
        assert_eq!(d2, digest);
        assert!(path.is_file());
        assert!(path.extension().is_some());
        let bytes = std::fs::read(&path).expect("read");
        assert_eq!(bytes.len(), 12);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn hop_gateway_temperature_matches_ambient_reference() {
        use crate::constants::AMBIENT_REFERENCE_TEMPERATURE_K;
        assert_eq!(HOP_GATEWAY_TEMPERATURE_K, AMBIENT_REFERENCE_TEMPERATURE_K);
    }
}
