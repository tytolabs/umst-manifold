// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Oracle harvest for `golden_learner_mi_reward_hook_v0` — pinned inputs only.
//!
//! Trajectory cites existing test:
//! - `semantic_evolution_bridge::tests::runtime_shape_update_backprop_nonzero`

use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::NdArray;
use umst_manifold::ai::semantic_evolution_bridge::{mi_reward_hook, CHAIR_I_REQUIRED_BITS};

type B = NdArray<f32>;

/// Harvest oracle: `mi_reward_hook` on smoke-1 vector (chair hypothesis pins).
#[must_use]
pub fn harvest_chair_mi_reward_hook() -> f32 {
    let device = Default::default();
    let i_required = Tensor::<B, 1>::from_data(
        Data::new(vec![CHAIR_I_REQUIRED_BITS as f32], Shape::new([1])),
        &device,
    );
    let witness_before =
        Tensor::<B, 1>::from_data(Data::new(vec![1.0_f32], Shape::new([1])), &device);
    let witness_after =
        Tensor::<B, 1>::from_data(Data::new(vec![4.0_f32], Shape::new([1])), &device);
    let reward = mi_reward_hook(i_required, witness_before, witness_after);
    reward.into_data().value[0]
}

fn f32_hex(bits: f32) -> String {
    format!("{:016x}", bits.to_bits())
}

#[test]
fn mi_reward_hook_oracle_harvest_pins_from_runtime_shape_update() {
    let reward_hook_scalar = harvest_chair_mi_reward_hook();
    eprintln!("HARVEST chair_mi_reward_hook reward_hook_scalar={reward_hook_scalar:.17e}");
    eprintln!(
        "HARVEST_HEX reward_hook_scalar={}",
        f32_hex(reward_hook_scalar)
    );
    // relu(6-1) - relu(6-4) = 5 - 2 = 3
    assert!((reward_hook_scalar - 3.0_f32).abs() < 1e-6);
}
