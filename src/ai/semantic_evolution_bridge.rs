// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
//
// HCOM-009 @ 19:15 IST — runtime Burn bridge for semantic evolution (blueprint §3.1).
//
// Host-side MI lane writes + differentiable reward hook for training-loop stubs.
// Full `umst-semantics::meaning_evolution` tensor path stays in A10 domain crate;
// this module wires HCOM-006 carrier lanes on the runtime Burn home.

use burn::tensor::activation::relu;
use burn::tensor::backend::AutodiffBackend;
use burn::tensor::{backend::Backend, Tensor};

use crate::core::semantic_lane_schema::{SemanticLaneBundleV1, LANE_MI_VALUE};

/// W29 deepen cell — semantic evolution bridge honest fence bundle.
pub const W29_SEMANTIC_EVOLUTION_BRIDGE_DEEPEN_CELL: &str = "W29-015-SEMANTIC_EVOLUTION_BRIDG";

/// P3 cert-path MI gate — remains blocked at runtime bridge (semantics SSOT).
pub const P3_MI_GATE_DEFERRED_STEP: &str = "HCOM-041-P3-MI-GATE";

/// UnifiedMaterialStateTensor semantic lane write — deferred to `umst-runtime`.
pub const RUNTIME_UMST_TENSOR_WRITE_DEFERRED_STEP: &str = "HCOM-009-RUNTIME-UMST-WRITE";

/// Honest posture — runtime scaffold; cert-path P3 gate not claimed here.
pub const MEANING_EVOLUTION_RUNTIME_STUB: bool = true;

/// Runtime tensor monomorphization on `UnifiedMaterialStateTensor` — deferred.
pub const RUNTIME_TENSOR_DEFERRED: bool = true;

/// P3 cert-path MI gate — blocked at runtime bridge (mirror `umst-semantics::p3_mi_gate`).
pub const P3_MI_GATE_BLOCKED: bool = true;

/// Honest refusal — runtime bridge is scaffold, not production-wired.
pub const SEMANTIC_EVOLUTION_PRODUCTION_WIRED: bool = false;

/// Honest refusal — no physics GREEN claim at runtime bridge seam.
pub const SEMANTIC_EVOLUTION_PHYSICS_GREEN: bool = false;

/// Honest refusal — master orchestrator pin not claimed by runtime bridge.
pub const SEMANTIC_EVOLUTION_MASTER: bool = false;

/// Golden learner fixture cross-ref (`BIND_LEARNER_mi_reward_hook`).
pub const GOLDEN_LEARNER_FIXTURE_PATH: &str =
    "crates/umst-bench/fixtures/golden_learner_mi_reward_hook_v0.json";

/// Semantic evolution fence facet count (honest census).
pub const SEMANTIC_EVOLUTION_FENCE_FACET_COUNT: usize = 8;

/// Semantic evolution fence facets wired today (4/8 measured).
pub const SEMANTIC_EVOLUTION_FENCE_WIRED_COUNT: usize = 4;

/// Chair hypothesis required bits (matches `umst-semantics` P3 fixture).
pub const CHAIR_I_REQUIRED_BITS: f64 = 6.0;

/// Default learning rate for shape-update stub.
pub const EVOLUTION_STUB_LR: f32 = 0.1;

/// Default MI reward weight λ in training-loop stub.
pub const EVOLUTION_STUB_MI_WEIGHT: f32 = 0.5;

/// Honest fence string for orchestrator / census probes.
pub const HONEST_FENCE: &str =
    "runtime_stub=true|backprop_landed=true|mi_lane_write=true|p3_gate_blocked=true|runtime_tensor_deferred=true|production_wired=false|physics_green=false|master=false";

/// One facet of the semantic evolution production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticEvolutionFenceFacet {
    pub facet: &'static str,
    pub wired: bool,
    pub owning_slice: &'static str,
}

/// Semantic evolution production fence facet inventory (honest posture SSOT).
pub const SEMANTIC_EVOLUTION_FENCE_FACETS: &[SemanticEvolutionFenceFacet] = &[
    SemanticEvolutionFenceFacet {
        facet: "shape_update_backprop",
        wired: true,
        owning_slice: W29_SEMANTIC_EVOLUTION_BRIDGE_DEEPEN_CELL,
    },
    SemanticEvolutionFenceFacet {
        facet: "mi_reward_hook",
        wired: true,
        owning_slice: W29_SEMANTIC_EVOLUTION_BRIDGE_DEEPEN_CELL,
    },
    SemanticEvolutionFenceFacet {
        facet: "hcom006_mi_lane_write",
        wired: true,
        owning_slice: W29_SEMANTIC_EVOLUTION_BRIDGE_DEEPEN_CELL,
    },
    SemanticEvolutionFenceFacet {
        facet: "training_loop_stub",
        wired: true,
        owning_slice: W29_SEMANTIC_EVOLUTION_BRIDGE_DEEPEN_CELL,
    },
    SemanticEvolutionFenceFacet {
        facet: "p3_cert_path_mi_gate",
        wired: false,
        owning_slice: P3_MI_GATE_DEFERRED_STEP,
    },
    SemanticEvolutionFenceFacet {
        facet: "runtime_umst_tensor_write",
        wired: false,
        owning_slice: RUNTIME_UMST_TENSOR_WRITE_DEFERRED_STEP,
    },
    SemanticEvolutionFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: P3_MI_GATE_DEFERRED_STEP,
    },
    SemanticEvolutionFenceFacet {
        facet: "master_orchestrator_pin",
        wired: false,
        owning_slice: P3_MI_GATE_DEFERRED_STEP,
    },
];

/// Compile-time fence — production/master/physics GREEN flip not authorized.
const _: () = assert!(MEANING_EVOLUTION_RUNTIME_STUB);
const _: () = assert!(RUNTIME_TENSOR_DEFERRED);
const _: () = assert!(P3_MI_GATE_BLOCKED);
const _: () = assert!(!SEMANTIC_EVOLUTION_PRODUCTION_WIRED);
const _: () = assert!(!SEMANTIC_EVOLUTION_PHYSICS_GREEN);
const _: () = assert!(!SEMANTIC_EVOLUTION_MASTER);

/// Count wired semantic evolution fence facets.
#[must_use]
pub fn semantic_evolution_fence_wired_count() -> usize {
    SEMANTIC_EVOLUTION_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Measured honest-posture snapshot for semantic evolution bridge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticEvolutionHonestPosture {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub runtime_stub: bool,
    pub p3_gate_blocked: bool,
    pub runtime_tensor_deferred: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub deferred_p3_step: &'static str,
    pub deferred_umst_write_step: &'static str,
}

/// Honest posture bundle — no invented GREEN / PRODUCTION_WIRED / MASTER.
#[must_use]
pub fn semantic_evolution_honest_posture_bundle() -> SemanticEvolutionHonestPosture {
    SemanticEvolutionHonestPosture {
        physics_green: SEMANTIC_EVOLUTION_PHYSICS_GREEN,
        production_wired: SEMANTIC_EVOLUTION_PRODUCTION_WIRED,
        master: SEMANTIC_EVOLUTION_MASTER,
        runtime_stub: MEANING_EVOLUTION_RUNTIME_STUB,
        p3_gate_blocked: P3_MI_GATE_BLOCKED,
        runtime_tensor_deferred: RUNTIME_TENSOR_DEFERRED,
        fence_facet_count: SEMANTIC_EVOLUTION_FENCE_FACET_COUNT,
        fence_wired_count: semantic_evolution_fence_wired_count(),
        deferred_p3_step: P3_MI_GATE_DEFERRED_STEP,
        deferred_umst_write_step: RUNTIME_UMST_TENSOR_WRITE_DEFERRED_STEP,
    }
}

/// Typed probe for semantic evolution bridge done-when checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticEvolutionProbe {
    pub deepen_cell: &'static str,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub backprop_landed: bool,
    pub mi_lane_write_landed: bool,
    pub training_loop_stub_landed: bool,
    pub p3_gate_blocked: bool,
    pub runtime_tensor_deferred: bool,
    pub production_wired: bool,
    pub master: bool,
    pub physics_green: bool,
    pub honest_fence: &'static str,
}

/// Build introspection probe for semantic evolution done-when checks.
#[must_use]
pub const fn semantic_evolution_probe() -> SemanticEvolutionProbe {
    SemanticEvolutionProbe {
        deepen_cell: W29_SEMANTIC_EVOLUTION_BRIDGE_DEEPEN_CELL,
        fence_facet_count: SEMANTIC_EVOLUTION_FENCE_FACET_COUNT,
        fence_wired_count: SEMANTIC_EVOLUTION_FENCE_WIRED_COUNT,
        backprop_landed: true,
        mi_lane_write_landed: true,
        training_loop_stub_landed: true,
        p3_gate_blocked: P3_MI_GATE_BLOCKED,
        runtime_tensor_deferred: RUNTIME_TENSOR_DEFERRED,
        production_wired: SEMANTIC_EVOLUTION_PRODUCTION_WIRED,
        master: SEMANTIC_EVOLUTION_MASTER,
        physics_green: SEMANTIC_EVOLUTION_PHYSICS_GREEN,
        honest_fence: HONEST_FENCE,
    }
}

/// Runtime bridge landed with production/master/P3 composition honestly open.
#[must_use]
pub fn semantic_evolution_honest(probe: &SemanticEvolutionProbe) -> bool {
    probe.deepen_cell == W29_SEMANTIC_EVOLUTION_BRIDGE_DEEPEN_CELL
        && probe.fence_facet_count == SEMANTIC_EVOLUTION_FENCE_FACET_COUNT
        && probe.fence_wired_count == SEMANTIC_EVOLUTION_FENCE_WIRED_COUNT
        && probe.backprop_landed
        && probe.mi_lane_write_landed
        && probe.training_loop_stub_landed
        && probe.p3_gate_blocked
        && probe.runtime_tensor_deferred
        && !probe.production_wired
        && !probe.master
        && !probe.physics_green
}

/// One training-loop stub report (fixture metrics only).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EvolutionStepReport {
    pub loss: f32,
    pub mi_reward_mean: f32,
    pub grad_center_l2: f32,
    pub grad_radius_l2: f32,
}

/// Inputs for one differentiable evolution training step (fixture-closed).
#[derive(Debug, Clone)]
pub struct EvolutionStepInputs<B: Backend> {
    pub center: Tensor<B, 2>,
    pub radius: Tensor<B, 1>,
    pub delta_center: Tensor<B, 2>,
    pub delta_radius: Tensor<B, 1>,
    pub probes: Tensor<B, 3>,
    pub target_sdf: Tensor<B, 2>,
    pub i_required: Tensor<B, 1>,
    pub i_witness_before: Tensor<B, 1>,
    pub i_witness_after: Tensor<B, 1>,
    pub lr: f32,
    pub mi_weight: f32,
}

/// MI deficit: `max(0, i_required − i_witness)` (host algebra).
#[must_use]
pub fn mi_deficit_from_bits(i_required: f64, i_witness: f64) -> f64 {
    (i_required - i_witness).max(0.0)
}

/// Project witness bits into HCOM-006 `LANE_MI_VALUE`.
#[must_use]
pub fn witness_to_mi_lane_scalar(i_witness_bits: f64) -> f64 {
    i_witness_bits
}

/// Write MI witness into a v1 carrier row.
pub fn write_mi_witness_lane(row: &mut [f64], i_witness_bits: f64) {
    if row.len() > LANE_MI_VALUE {
        row[LANE_MI_VALUE] = witness_to_mi_lane_scalar(i_witness_bits);
    }
}

/// Populate semantic lane bundle with MI witness (HCOM-006 bridge).
#[must_use]
pub fn semantic_lane_bundle_with_mi(i_witness_bits: f64) -> SemanticLaneBundleV1 {
    SemanticLaneBundleV1 {
        mi_value: witness_to_mi_lane_scalar(i_witness_bits),
        ..SemanticLaneBundleV1::default()
    }
}

/// Differentiable MI reward: `relu(deficit_before) − relu(deficit_after)`.
#[must_use]
pub fn mi_reward_hook<B: Backend>(
    i_required: Tensor<B, 1>,
    i_witness_before: Tensor<B, 1>,
    i_witness_after: Tensor<B, 1>,
) -> Tensor<B, 1> {
    let deficit_before = relu(i_required.clone().sub(i_witness_before));
    let deficit_after = relu(i_required.sub(i_witness_after));
    deficit_before.sub(deficit_after)
}

/// Differentiable shape update: `params' = params + lr · delta`.
#[must_use]
pub fn shape_update_forward<B: Backend>(
    center: Tensor<B, 2>,
    radius: Tensor<B, 1>,
    delta_center: Tensor<B, 2>,
    delta_radius: Tensor<B, 1>,
    lr: f32,
) -> (Tensor<B, 2>, Tensor<B, 1>) {
    (
        center.add(delta_center.mul_scalar(lr)),
        radius.add(delta_radius.mul_scalar(lr)),
    )
}

/// Sphere SDF surrogate error — per-batch mean absolute error `[B]`.
#[must_use]
pub fn sphere_sdf_surrogate<B: Backend>(
    center: Tensor<B, 2>,
    radius: Tensor<B, 1>,
    probes: Tensor<B, 3>,
    target_sdf: Tensor<B, 2>,
) -> Tensor<B, 1> {
    let batch = center.dims()[0];
    let probe_count = probes.dims()[1];

    let center_expanded = center.unsqueeze_dim::<3>(1).expand([batch, probe_count, 3]);
    let diff = probes.sub(center_expanded);
    let dist = diff.clone().mul(diff).sum_dim(2).sqrt().squeeze(2);

    let radius_expanded = radius.unsqueeze_dim::<2>(1).expand([batch, probe_count]);
    let sdf = dist.sub(radius_expanded);
    let err = sdf.sub(target_sdf).abs();
    err.mean_dim(1).squeeze(1)
}

/// Combined semantic evolution loss: shape error minus weighted MI reward.
#[must_use]
pub fn semantic_evolution_loss<B: Backend>(
    shape_err: Tensor<B, 1>,
    mi_reward: Tensor<B, 1>,
    mi_weight: f32,
) -> Tensor<B, 1> {
    shape_err.sub(mi_reward.mul_scalar(mi_weight))
}

/// One training-loop stub: forward shape update → MI reward → backward → report.
#[must_use]
pub fn run_evolution_training_step<B: AutodiffBackend<FloatElem = f32>>(
    inputs: EvolutionStepInputs<B>,
) -> EvolutionStepReport {
    let (new_center, new_radius) = shape_update_forward(
        inputs.center.clone(),
        inputs.radius.clone(),
        inputs.delta_center,
        inputs.delta_radius,
        inputs.lr,
    );
    let shape_err = sphere_sdf_surrogate(new_center, new_radius, inputs.probes, inputs.target_sdf);
    let mi_reward = mi_reward_hook(
        inputs.i_required,
        inputs.i_witness_before,
        inputs.i_witness_after,
    );
    let mi_reward_mean = mi_reward.clone().mean().into_data().value[0];

    let loss = semantic_evolution_loss(shape_err, mi_reward, inputs.mi_weight).mean();
    let loss_v = loss.clone().into_data().value[0];

    let grads = loss.backward();
    let grad_center_l2 = tensor_l2_grad_2d(&inputs.center, &grads);
    let grad_radius_l2 = tensor_l2_grad_1d(&inputs.radius, &grads);

    EvolutionStepReport {
        loss: loss_v,
        mi_reward_mean,
        grad_center_l2,
        grad_radius_l2,
    }
}

fn tensor_l2_grad_2d<B: AutodiffBackend<FloatElem = f32>>(
    param: &Tensor<B, 2>,
    grads: &B::Gradients,
) -> f32 {
    param
        .grad(grads)
        .map(|g| l2_norm(&g.into_data().value))
        .unwrap_or(0.0)
}

fn tensor_l2_grad_1d<B: AutodiffBackend<FloatElem = f32>>(
    param: &Tensor<B, 1>,
    grads: &B::Gradients,
) -> f32 {
    param
        .grad(grads)
        .map(|g| l2_norm(&g.into_data().value))
        .unwrap_or(0.0)
}

fn l2_norm(values: &[f32]) -> f32 {
    values.iter().map(|x| x * x).sum::<f32>().sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::backend::Autodiff;
    use burn::tensor::{Data, Shape, Tensor};
    use burn_ndarray::NdArray;

    type B = Autodiff<NdArray<f32>>;

    #[test]
    fn honest_fence_consts_refuse_green_production_master() {
        assert!(MEANING_EVOLUTION_RUNTIME_STUB);
        assert!(RUNTIME_TENSOR_DEFERRED);
        assert!(P3_MI_GATE_BLOCKED);
        assert!(!SEMANTIC_EVOLUTION_PRODUCTION_WIRED);
        assert!(!SEMANTIC_EVOLUTION_PHYSICS_GREEN);
        assert!(!SEMANTIC_EVOLUTION_MASTER);
        assert_eq!(
            semantic_evolution_fence_wired_count(),
            SEMANTIC_EVOLUTION_FENCE_WIRED_COUNT
        );
    }

    #[test]
    fn honest_posture_bundle_and_probe() {
        let bundle = semantic_evolution_honest_posture_bundle();
        assert!(bundle.runtime_stub);
        assert!(bundle.p3_gate_blocked);
        assert!(!bundle.production_wired);
        assert!(!bundle.master);
        assert!(!bundle.physics_green);
        assert_eq!(
            bundle.fence_facet_count,
            SEMANTIC_EVOLUTION_FENCE_FACET_COUNT
        );

        let probe = semantic_evolution_probe();
        assert!(semantic_evolution_honest(&probe));
        assert!(probe.honest_fence.contains("production_wired=false"));
        assert!(probe.honest_fence.contains("master=false"));
    }

    #[test]
    fn mi_deficit_from_bits_matches_p3_algebra() {
        assert!((mi_deficit_from_bits(6.0, 4.0) - 2.0).abs() < f64::EPSILON);
        assert!(mi_deficit_from_bits(4.0, 6.0).abs() < f64::EPSILON);
        assert!((mi_deficit_from_bits(CHAIR_I_REQUIRED_BITS, 1.0) - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mi_lane_write_preserves_witness() {
        use crate::core::semantic_lane_schema::UMST_CARRIER_LANE_COUNT;
        let mut row = vec![0.0_f64; UMST_CARRIER_LANE_COUNT];
        write_mi_witness_lane(&mut row, 4.5);
        assert!((row[LANE_MI_VALUE] - 4.5).abs() < f64::EPSILON);
        let bundle = semantic_lane_bundle_with_mi(3.5);
        assert!((bundle.mi_value - 3.5).abs() < f64::EPSILON);
    }

    #[test]
    fn mi_reward_hook_positive_on_improvement() {
        let device = Default::default();
        let i_required = Tensor::<B, 1>::from_data(
            Data::new(vec![CHAIR_I_REQUIRED_BITS as f32], Shape::new([1])),
            &device,
        );
        let before = Tensor::<B, 1>::from_data(Data::new(vec![1.0_f32], Shape::new([1])), &device);
        let after = Tensor::<B, 1>::from_data(Data::new(vec![4.0_f32], Shape::new([1])), &device);
        let reward = mi_reward_hook(i_required.clone(), before, after)
            .into_data()
            .value[0];
        // relu(6-1) - relu(6-4) = 5 - 2 = 3
        assert!((reward - 3.0_f32).abs() < 1e-6);

        let flat = Tensor::<B, 1>::from_data(Data::new(vec![6.0_f32], Shape::new([1])), &device);
        let zero_reward = mi_reward_hook(i_required, flat.clone(), flat)
            .into_data()
            .value[0];
        assert!(zero_reward.abs() < 1e-6);
    }

    #[test]
    fn runtime_shape_update_backprop_nonzero() {
        let device = Default::default();
        let center = Tensor::<B, 2>::from_data(
            Data::new(vec![0.0_f32, 0.0, 0.0], Shape::new([1, 3])),
            &device,
        )
        .require_grad();
        let radius = Tensor::<B, 1>::from_data(Data::new(vec![0.20_f32], Shape::new([1])), &device)
            .require_grad();
        let delta_center = Tensor::<B, 2>::from_data(
            Data::new(vec![0.05_f32, 0.0, 0.0], Shape::new([1, 3])),
            &device,
        )
        .require_grad();
        let delta_radius =
            Tensor::<B, 1>::from_data(Data::new(vec![0.02_f32], Shape::new([1])), &device)
                .require_grad();

        let probes = Tensor::<B, 3>::from_data(
            Data::new(
                vec![0.25_f32, 0.0, 0.0, 0.0_f32, 0.0, 0.0],
                Shape::new([1, 2, 3]),
            ),
            &device,
        );
        let target = Tensor::<B, 2>::from_data(
            Data::new(vec![0.05_f32, -0.20], Shape::new([1, 2])),
            &device,
        );

        let report = run_evolution_training_step(EvolutionStepInputs {
            center: center.clone(),
            radius: radius.clone(),
            delta_center,
            delta_radius,
            probes,
            target_sdf: target,
            i_required: Tensor::<B, 1>::from_data(
                Data::new(vec![CHAIR_I_REQUIRED_BITS as f32], Shape::new([1])),
                &device,
            ),
            i_witness_before: Tensor::<B, 1>::from_data(
                Data::new(vec![1.0_f32], Shape::new([1])),
                &device,
            ),
            i_witness_after: Tensor::<B, 1>::from_data(
                Data::new(vec![4.0_f32], Shape::new([1])),
                &device,
            ),
            lr: EVOLUTION_STUB_LR,
            mi_weight: EVOLUTION_STUB_MI_WEIGHT,
        });

        assert!(report.loss.is_finite());
        assert!((report.mi_reward_mean - 3.0_f32).abs() < 1e-6);
        assert!(
            report.grad_center_l2 > 1e-8 || report.grad_radius_l2 > 1e-8,
            "runtime backprop must be non-zero"
        );
        assert!(MEANING_EVOLUTION_RUNTIME_STUB);
        assert!(P3_MI_GATE_BLOCKED);
        assert!(RUNTIME_TENSOR_DEFERRED);
    }

    #[test]
    fn fence_facet_inventory_matches_wired_count() {
        let wired: Vec<_> = SEMANTIC_EVOLUTION_FENCE_FACETS
            .iter()
            .filter(|f| f.wired)
            .map(|f| f.facet)
            .collect();
        assert_eq!(wired.len(), SEMANTIC_EVOLUTION_FENCE_WIRED_COUNT);
        assert!(wired.contains(&"shape_update_backprop"));
        assert!(wired.contains(&"mi_reward_hook"));
        assert!(wired.contains(&"hcom006_mi_lane_write"));
        assert!(wired.contains(&"training_loop_stub"));

        let open: Vec<_> = SEMANTIC_EVOLUTION_FENCE_FACETS
            .iter()
            .filter(|f| !f.wired)
            .map(|f| f.facet)
            .collect();
        assert!(open.contains(&"p3_cert_path_mi_gate"));
        assert!(open.contains(&"production_wired"));
        assert!(open.contains(&"master_orchestrator_pin"));
    }
}
