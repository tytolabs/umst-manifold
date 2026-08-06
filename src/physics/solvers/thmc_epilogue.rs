// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP5.1 — post-step epilogue: fracture → sync → gate → time (RW-FP-P51).
//!
//! ```text
//! thmc_post_step_epilogue
//!   ok_state → and_then fracture → and_then sync → gate witness → map advance_time
//! ```
//!
//! ## Honest fences (W29-083)
//!
//! - Kleisli composition of fracture / UMST sync / gate evidence / clock advance is **wired**.
//! - [`THMC_EPILOGUE_PHYSICS_GREEN`], [`THMC_EPILOGUE_PRODUCTION_WIRED`], [`THMC_EPILOGUE_MASTER`],
//!   and [`THMC_EPILOGUE_OP5`] stay **false** — no invent GREEN / PRODUCTION_WIRED / MASTER / OP-5.
//! - Fracture skip (`apply_fracture=false`) is an honest route option, not a GREEN claim.

use burn::tensor::backend::Backend;
use burn::tensor::{Int, Tensor};

use crate::core::field::{Field, FractureEnergyField, SmallStrainField};
use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::core::traits::IScienceCartridge;
use crate::physics::error::PhysicsError;
use crate::physics::pipeline::{and_then_unit, map_result, ok_state};

use super::fracture_field::{
    strain_tensor_for_fracture_from_manifold, strain_tensor_from_bar_network_displacement,
    PhaseFieldFractureSolver,
};
use super::thmc::{ThmcSolver, ThmcState};
use super::thmc_step::{ThmcSolverStep, ThmcStepGateEvidence};

/// W29 deepen cell — thmc_epilogue honest fence bundle.
pub const W29_THMC_EPILOGUE_DEEPEN_CELL: &str = "W29-083-THMC_EPILOGUE";

/// Honest physics posture — epilogue composition is not a physics-GREEN certificate.
pub const THMC_EPILOGUE_PHYSICS_GREEN: bool = false;

/// Production orchestration pin — not claimed by the post-step Kleisli chain alone.
pub const THMC_EPILOGUE_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by the post-step Kleisli chain alone.
pub const THMC_EPILOGUE_MASTER: bool = false;

/// OP-5 pass pin — refused until orch measures production flip.
pub const THMC_EPILOGUE_OP5: bool = false;

/// Operator-visible honesty string — does **not** authorize production flip or MASTER retick.
pub const THMC_EPILOGUE_HONEST_FENCE: &str =
    "kleisli_epilogue_landed=true|fracture_optional=true|umst_sync_stage=true|gate_evidence_stage=true|advance_time_stage=true|production_wired=false|physics_green=false|master=false|op5=false";

/// Fence facet count for honest census.
pub const THMC_EPILOGUE_FENCE_FACET_COUNT: usize = 9;

/// Fence facets wired today (5/9 measured composition stages).
pub const THMC_EPILOGUE_FENCE_WIRED_COUNT: usize = 5;

/// Ordered Kleisli stage tags for the post-step epilogue (RW-FP-P51).
pub const THMC_EPILOGUE_STAGE_ORDER: &[&str] = &[
    "ok_state",
    "fracture_optional",
    "umst_sync",
    "gate_evidence",
    "advance_time",
];

/// One facet of the thmc_epilogue production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThmcEpilogueFenceFacet {
    pub facet: &'static str,
    pub wired: bool,
    pub owning_slice: &'static str,
}

/// thmc_epilogue production fence facet inventory (honest posture SSOT).
pub const THMC_EPILOGUE_FENCE_FACETS: &[ThmcEpilogueFenceFacet] = &[
    ThmcEpilogueFenceFacet {
        facet: "kleisli_epilogue_composition",
        wired: true,
        owning_slice: W29_THMC_EPILOGUE_DEEPEN_CELL,
    },
    ThmcEpilogueFenceFacet {
        facet: "optional_fracture_damage_stage",
        wired: true,
        owning_slice: W29_THMC_EPILOGUE_DEEPEN_CELL,
    },
    ThmcEpilogueFenceFacet {
        facet: "umst_sync_writeback_stage",
        wired: true,
        owning_slice: W29_THMC_EPILOGUE_DEEPEN_CELL,
    },
    ThmcEpilogueFenceFacet {
        facet: "gate_evidence_attachment_stage",
        wired: true,
        owning_slice: W29_THMC_EPILOGUE_DEEPEN_CELL,
    },
    ThmcEpilogueFenceFacet {
        facet: "advance_time_map_stage",
        wired: true,
        owning_slice: W29_THMC_EPILOGUE_DEEPEN_CELL,
    },
    ThmcEpilogueFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
    ThmcEpilogueFenceFacet {
        facet: "physics_green",
        wired: false,
        owning_slice: "refused — composition ≠ physics GREEN",
    },
    ThmcEpilogueFenceFacet {
        facet: "master_orchestrator_pin",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
    ThmcEpilogueFenceFacet {
        facet: "op5_pass",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
];

const _: () = assert!(!THMC_EPILOGUE_PHYSICS_GREEN);
const _: () = assert!(!THMC_EPILOGUE_PRODUCTION_WIRED);
const _: () = assert!(!THMC_EPILOGUE_MASTER);
const _: () = assert!(!THMC_EPILOGUE_OP5);

/// Count wired thmc_epilogue fence facets (must match [`THMC_EPILOGUE_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn thmc_epilogue_fence_wired_count() -> usize {
    THMC_EPILOGUE_FENCE_FACETS.iter().filter(|f| f.wired).count()
}

/// Typed probe for thmc_epilogue posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThmcEpilogueProbe {
    pub deepen_cell: &'static str,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub kleisli_epilogue: bool,
    pub fracture_optional: bool,
    pub umst_sync_stage: bool,
    pub gate_evidence_stage: bool,
    pub advance_time_stage: bool,
    pub production_wired: bool,
    pub master: bool,
    pub physics_green: bool,
    pub op5: bool,
    pub honest_fence: &'static str,
    pub stage_count: usize,
}

/// Build introspection probe for thmc_epilogue done-when checks.
#[must_use]
pub const fn thmc_epilogue_probe() -> ThmcEpilogueProbe {
    ThmcEpilogueProbe {
        deepen_cell: W29_THMC_EPILOGUE_DEEPEN_CELL,
        fence_facet_count: THMC_EPILOGUE_FENCE_FACET_COUNT,
        fence_wired_count: THMC_EPILOGUE_FENCE_WIRED_COUNT,
        kleisli_epilogue: true,
        fracture_optional: true,
        umst_sync_stage: true,
        gate_evidence_stage: true,
        advance_time_stage: true,
        production_wired: THMC_EPILOGUE_PRODUCTION_WIRED,
        master: THMC_EPILOGUE_MASTER,
        physics_green: THMC_EPILOGUE_PHYSICS_GREEN,
        op5: THMC_EPILOGUE_OP5,
        honest_fence: THMC_EPILOGUE_HONEST_FENCE,
        stage_count: THMC_EPILOGUE_STAGE_ORDER.len(),
    }
}

/// thmc_epilogue landed with production/master/GREEN/OP-5 honestly open.
#[must_use]
pub fn thmc_epilogue_honest(probe: &ThmcEpilogueProbe) -> bool {
    probe.deepen_cell == W29_THMC_EPILOGUE_DEEPEN_CELL
        && probe.fence_facet_count == THMC_EPILOGUE_FENCE_FACET_COUNT
        && probe.fence_wired_count == THMC_EPILOGUE_FENCE_WIRED_COUNT
        && probe.kleisli_epilogue
        && probe.fracture_optional
        && probe.umst_sync_stage
        && probe.gate_evidence_stage
        && probe.advance_time_stage
        && !probe.production_wired
        && !probe.master
        && !probe.physics_green
        && !probe.op5
        && probe.stage_count == THMC_EPILOGUE_STAGE_ORDER.len()
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("op5=false")
}

/// Validate thmc_epilogue honesty — fail closed on fake production/master/GREEN/OP-5 claims.
pub fn validate_thmc_epilogue_honesty() -> Result<(), &'static str> {
    let probe = thmc_epilogue_probe();
    if probe.production_wired {
        return Err("THMC_EPILOGUE_PRODUCTION_WIRED must stay false — Kleisli composition only");
    }
    if probe.master {
        return Err("THMC_EPILOGUE_MASTER must stay false until orchestrator pin lands");
    }
    if probe.physics_green {
        return Err("THMC_EPILOGUE_PHYSICS_GREEN must stay false — no invent GREEN");
    }
    if probe.op5 {
        return Err("THMC_EPILOGUE_OP5 must stay false until measured production flip");
    }
    if thmc_epilogue_fence_wired_count() != THMC_EPILOGUE_FENCE_WIRED_COUNT {
        return Err("thmc_epilogue_fence_wired_count drifted from THMC_EPILOGUE_FENCE_WIRED_COUNT");
    }
    if THMC_EPILOGUE_STAGE_ORDER.len() != 5 {
        return Err("THMC_EPILOGUE_STAGE_ORDER must stay length 5 (RW-FP-P51)");
    }
    if !thmc_epilogue_honest(&probe) {
        return Err("thmc_epilogue_probe failed thmc_epilogue_honest gate");
    }
    Ok(())
}

/// Post-step context for [`thmc_post_step_epilogue`].
#[derive(Clone)]
pub struct ThmcPostStepCtx<B: Backend> {
    pub batch: usize,
    pub n: usize,
    pub edges_b1: Tensor<B, 2, Int>,
    pub dt: f32,
}

/// Kleisli post-step epilogue: optional fracture → UMST sync → gate evidence → advance time.
pub fn thmc_post_step_epilogue<B, C>(
    solver: &ThmcSolver,
    cartridge: &C,
    pre_step: &ThmcState<B>,
    state: ThmcState<B>,
    manifold: &mut UnifiedMaterialStateTensor<B>,
    ctx: &ThmcPostStepCtx<B>,
    apply_fracture: bool,
) -> Result<(ThmcState<B>, ThmcStepGateEvidence), PhysicsError>
where
    B: Backend<FloatElem = f32>,
    C: IScienceCartridge<B>,
{
    let edges_b1 = ctx.edges_b1.clone();
    let state = ok_state(state);
    let state = if apply_fracture {
        state.and_then(|s| apply_fracture_damage(s, manifold, ctx.batch, ctx.n, edges_b1.clone()))
    } else {
        state
    };
    let state = state
        .and_then(|s| and_then_unit(s, |st| crate::physics::thmc_umst_sync::sync_thmc_to_umst(st, manifold)))?;
    let gate_evidence = ThmcSolverStep::attach_gate_evidence(
        solver, cartridge, pre_step, &state, manifold, ctx.dt,
    )?;
    let state = map_result(ok_state(state), |mut s| {
        s.time += ctx.dt;
        s
    })?;
    Ok((state, gate_evidence))
}

fn apply_fracture_damage<B: Backend<FloatElem = f32>>(
    mut state: ThmcState<B>,
    manifold: &UnifiedMaterialStateTensor<B>,
    batch: usize,
    n: usize,
    edges_b1: Tensor<B, 2, Int>,
) -> Result<ThmcState<B>, PhysicsError> {
    let device = state.thermal.temperature.as_tensor().device();
    let strain_tensor = if let Some(coords_n3) = manifold.node_positions.as_ref() {
        if coords_n3.dims() == [n, 3] {
            strain_tensor_from_bar_network_displacement::<B>(
                state.mechanical.displacement.as_tensor().clone(),
                coords_n3.clone(), edges_b1.clone(), n,
            )
        } else {
            strain_tensor_for_fracture_from_manifold::<B>(manifold, batch, n, &device)
        }
    } else {
        strain_tensor_for_fracture_from_manifold::<B>(manifold, batch, n, &device)
    };
    let strain = SmallStrainField::from_tensor(strain_tensor);
    let gc = FractureEnergyField::from_tensor(Tensor::<B, 3>::ones([batch, n, 1], &device));
    let fracture = PhaseFieldFractureSolver { length_scale: 1.0 };
    let d_last = state.damage.as_tensor().dims()[2];
    let damage_core = match d_last {
        1 => state.damage.clone(),
        _ => state.damage.clone().map(|t| t.slice([0..batch, 0..n, 0..1])),
    };
    let damage_new = fracture.update_damage(strain, damage_core, gc, edges_b1)?;
    state.damage = if d_last == 1 {
        damage_new
    } else {
        let tail = state.damage.as_tensor().clone().slice([0..batch, 0..n, 1..d_last]);
        Field::new(Tensor::cat(
            vec![damage_new.as_tensor().clone(), tail],
            2,
        ))
    };
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::backend::Backend;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    use crate::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
    use crate::core::traits::{IScienceCartridge, PhysicalResult};
    use crate::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
    use crate::physics::solvers::{ThmcSolver, ThmcState};

    type B = NdArray<f32>;

    fn dev() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    struct Stub;

    impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for Stub {
        fn compute_all(&self, mix: &MaterialCompositionTensor<Bk>) -> PhysicalResult<Bk> {
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

        fn compute_topology(&self, m: &UnifiedMaterialStateTensor<Bk>) -> PhysicalResult<Bk> {
            let d = m.scalar_features.device();
            let n = m.scalar_features.dims()[0];
            PhysicalResult {
                free_energy: Tensor::zeros([1, n], &d),
                dissipation: Tensor::zeros([1, n], &d),
                safety_margin: Tensor::zeros([1, n], &d),
                cost: Tensor::zeros([1, n], &d),
                damage: Tensor::zeros([1, n], &d),
                temperature_delta: None,
                #[cfg(feature = "information_density")]
                information_density: Tensor::zeros([1, n], &d),
            }
        }
    }

    fn umst(n: usize) -> UnifiedMaterialStateTensor<B> {
        let device = dev();
        let f = UMST_SCALAR_CHANNEL_COUNT;
        let coords: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &device);
        let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
            Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
            &device,
        );
        let faces_b2: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &device);
        UnifiedMaterialStateTensor {
            coords,
            edges_b1,
            faces_b2,
            scalar_features: Tensor::<B, 2>::zeros([n, f], &device),
            vector_features: Tensor::<B, 3>::zeros([n, 1, 3], &device),
            matrix_features: Tensor::<B, 4>::zeros([n, 1, 3, 3], &device),
            resolution_mm: [1.0, 1.0, 1.0],
            node_positions: None,
            displacement_bc_mask: Tensor::<B, 3>::ones([n, 3, 1], &device),
            policy_editable_mask: Tensor::<B, 2>::ones([n, 1], &device),
            #[cfg(feature = "formal-witness")]
            catalog_schema_digest: None,
        }
    }

    fn mk_state(device: &NdArrayDevice, n: usize, time: f32) -> ThmcState<B> {
        ThmcState::from_tensors(
            Tensor::full([1, n, 1], 293.0_f32, device),
            Tensor::full([1, n, 1], 0.5_f32, device),
            Tensor::zeros([1, n, 3], device),
            Tensor::full([1, n, 1], 0.42_f32, device),
            Tensor::zeros([1, n, 1], device),
            time,
        )
    }

    #[test]
    fn thmc_epilogue_honest_fence_bundle() {
        validate_thmc_epilogue_honesty().expect("honest fence");
        let probe = thmc_epilogue_probe();
        assert!(thmc_epilogue_honest(&probe));
        assert_eq!(
            thmc_epilogue_fence_wired_count(),
            THMC_EPILOGUE_FENCE_WIRED_COUNT
        );
        assert!(!THMC_EPILOGUE_PHYSICS_GREEN);
        assert!(!THMC_EPILOGUE_PRODUCTION_WIRED);
        assert!(!THMC_EPILOGUE_MASTER);
        assert!(!THMC_EPILOGUE_OP5);
    }

    #[test]
    fn thmc_epilogue_honest_fence_string_locked() {
        assert!(THMC_EPILOGUE_HONEST_FENCE.contains("kleisli_epilogue_landed=true"));
        assert!(THMC_EPILOGUE_HONEST_FENCE.contains("production_wired=false"));
        assert!(THMC_EPILOGUE_HONEST_FENCE.contains("physics_green=false"));
        assert!(THMC_EPILOGUE_HONEST_FENCE.contains("master=false"));
        assert!(THMC_EPILOGUE_HONEST_FENCE.contains("op5=false"));
        assert_eq!(W29_THMC_EPILOGUE_DEEPEN_CELL, "W29-083-THMC_EPILOGUE");
    }

    #[test]
    fn thmc_epilogue_stage_order_rw_fp_p51() {
        assert_eq!(THMC_EPILOGUE_STAGE_ORDER.len(), 5);
        assert_eq!(THMC_EPILOGUE_STAGE_ORDER[0], "ok_state");
        assert_eq!(THMC_EPILOGUE_STAGE_ORDER[1], "fracture_optional");
        assert_eq!(THMC_EPILOGUE_STAGE_ORDER[2], "umst_sync");
        assert_eq!(THMC_EPILOGUE_STAGE_ORDER[3], "gate_evidence");
        assert_eq!(THMC_EPILOGUE_STAGE_ORDER[4], "advance_time");
    }

    #[test]
    fn thmc_epilogue_fence_facets_refuse_green_production_master_op5() {
        assert_eq!(THMC_EPILOGUE_FENCE_FACETS.len(), THMC_EPILOGUE_FENCE_FACET_COUNT);
        for facet in THMC_EPILOGUE_FENCE_FACETS {
            if matches!(
                facet.facet,
                "production_wired" | "physics_green" | "master_orchestrator_pin" | "op5_pass"
            ) {
                assert!(
                    !facet.wired,
                    "facet {} must stay unwired (no invent GREEN/PRODUCTION_WIRED/MASTER/OP-5)",
                    facet.facet
                );
            }
        }
        assert_eq!(thmc_epilogue_fence_wired_count(), 5);
    }

    #[test]
    fn thmc_epilogue_advance_time_without_fracture() {
        let n = 2usize;
        let mut manifold = umst(n);
        let device = dev();
        let edges_b1 = manifold.edges_b1.clone();
        let pre = mk_state(&device, n, 1.0_f32);
        let state = pre.clone();
        let solver = ThmcSolver::default();
        let ctx = ThmcPostStepCtx {
            batch: 1,
            n,
            edges_b1,
            dt: 0.25_f32,
        };
        let (out, evidence) = thmc_post_step_epilogue(
            &solver,
            &Stub,
            &pre,
            state,
            &mut manifold,
            &ctx,
            false,
        )
        .expect(
            "thmc_post_step_epilogue(apply_fracture=false) must advance time + attach gate evidence \
             (W29-083 THMC_EPILOGUE deepen witness)",
        );
        assert!(
            (out.time - 1.25_f32).abs() < f32::EPSILON,
            "expected time 1.25 after dt=0.25, got {}",
            out.time
        );
        assert!(
            (evidence.dt_seconds - 0.25_f32).abs() < f32::EPSILON,
            "gate evidence dt must match ctx.dt"
        );
        assert_eq!(out.damage.as_tensor().dims(), [1, n, 1]);
    }

    #[test]
    fn thmc_epilogue_probe_refuses_invented_green() {
        let mut probe = thmc_epilogue_probe();
        assert!(thmc_epilogue_honest(&probe));
        probe.physics_green = true;
        assert!(!thmc_epilogue_honest(&probe));
        probe = thmc_epilogue_probe();
        probe.production_wired = true;
        assert!(!thmc_epilogue_honest(&probe));
        probe = thmc_epilogue_probe();
        probe.master = true;
        assert!(!thmc_epilogue_honest(&probe));
        probe = thmc_epilogue_probe();
        probe.op5 = true;
        assert!(!thmc_epilogue_honest(&probe));
    }
}
