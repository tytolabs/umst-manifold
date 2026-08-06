// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Thin gate-layer re-export of [`crate::ai::info_gain`] MSE surrogates (LP1 atom spine; no duplicated physics).
//!
//! Composition: **IG-002** · reward hook **M-RH-059** (`suggested_info_gain_from_batched_nodal_scalars`).

pub use crate::ai::info_gain::{
    suggested_info_gain_from_batched_nodal_scalars,
    suggested_info_gain_from_state_delta,
};

#[cfg(feature = "epistemic-ppo")]
pub use crate::ai::info_gain::nodal_scalar_means;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::info_gain as inner;
    use approx::assert_abs_diff_eq;
    use burn::tensor::{Data, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn device() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    #[test]
    fn gate_info_gain_reexports_delegate_to_inner_on_same_inputs() {
        let dev = device();
        let baseline = Tensor::<B, 2>::from_data(
            Data::new(vec![0.5_f32, 1.5_f32, 2.5_f32], Shape::new([1, 3])),
            &dev,
        );
        let proposed = Tensor::<B, 2>::from_data(
            Data::new(vec![1.5_f32, 2.5_f32, 3.5_f32], Shape::new([1, 3])),
            &dev,
        );
        let gate_g = suggested_info_gain_from_state_delta(baseline.clone(), proposed.clone());
        let inner_g = inner::suggested_info_gain_from_state_delta(baseline, proposed);
        let gate_v: Vec<f32> = gate_g.into_data().value;
        let inner_v: Vec<f32> = inner_g.into_data().value;
        assert_eq!(gate_v.len(), inner_v.len());
        for (g, i) in gate_v.iter().zip(inner_v.iter()) {
            assert_abs_diff_eq!(*g, *i, epsilon = 1.0e-6);
        }
    }

    #[test]
    fn gate_state_delta_zero_when_baseline_equals_proposed() {
        let dev = device();
        let baseline = Tensor::<B, 2>::from_data(
            Data::new(vec![1.0_f32, 2.0_f32, 3.0_f32, 4.0_f32], Shape::new([2, 2])),
            &dev,
        );
        let proposed = baseline.clone();
        let g = suggested_info_gain_from_state_delta(baseline, proposed);
        assert_eq!(g.dims(), [2]);
        let v: Vec<f32> = g.into_data().value;
        for &row in &v {
            assert_abs_diff_eq!(row, 0.0_f32, epsilon = 1.0e-6);
        }
    }

    #[test]
    fn gate_state_delta_matches_manual_mse_per_batch_row() {
        let dev = device();
        let baseline = Tensor::<B, 2>::from_data(
            Data::new(vec![0.0_f32, 0.0_f32, 1.0_f32, 1.0_f32], Shape::new([2, 2])),
            &dev,
        );
        let proposed = Tensor::<B, 2>::from_data(
            Data::new(vec![1.0_f32, 1.0_f32, 3.0_f32, 3.0_f32], Shape::new([2, 2])),
            &dev,
        );
        let gate_g = suggested_info_gain_from_state_delta(baseline.clone(), proposed.clone());
        let inner_g = inner::suggested_info_gain_from_state_delta(baseline, proposed);
        let gate_v: Vec<f32> = gate_g.into_data().value;
        let inner_v: Vec<f32> = inner_g.into_data().value;
        let row0 = (1.0_f32 + 1.0_f32) / 2.0_f32;
        let row1 = (4.0_f32 + 4.0_f32) / 2.0_f32;
        assert_abs_diff_eq!(gate_v[0], row0, epsilon = 1.0e-5);
        assert_abs_diff_eq!(gate_v[1], row1, epsilon = 1.0e-5);
        assert_abs_diff_eq!(gate_v[0], inner_v[0], epsilon = 1.0e-6);
        assert_abs_diff_eq!(gate_v[1], inner_v[1], epsilon = 1.0e-6);
    }

    #[test]
    fn gate_state_delta_outputs_are_non_negative() {
        let dev = device();
        let baseline = Tensor::<B, 2>::from_data(
            Data::new(vec![-2.0_f32, 0.5_f32, 1.0_f32, -1.0_f32], Shape::new([2, 2])),
            &dev,
        );
        let proposed = Tensor::<B, 2>::from_data(
            Data::new(vec![2.0_f32, -0.5_f32, -3.0_f32, 1.0_f32], Shape::new([2, 2])),
            &dev,
        );
        let g = suggested_info_gain_from_state_delta(baseline, proposed);
        let v: Vec<f32> = g.into_data().value;
        for &row in &v {
            assert!(row >= 0.0_f32, "MSE surrogate must be non-negative, got {row}");
        }
    }

    #[test]
    fn gate_state_delta_larger_delta_yields_larger_surrogate() {
        let dev = device();
        let baseline = Tensor::<B, 2>::zeros([1, 3], &dev);
        let small = Tensor::<B, 2>::from_data(
            Data::new(vec![0.1_f32, 0.1_f32, 0.1_f32], Shape::new([1, 3])),
            &dev,
        );
        let large = Tensor::<B, 2>::from_data(
            Data::new(vec![1.0_f32, 1.0_f32, 1.0_f32], Shape::new([1, 3])),
            &dev,
        );
        let g_small = suggested_info_gain_from_state_delta(baseline.clone(), small);
        let g_large = suggested_info_gain_from_state_delta(baseline, large);
        let small_v = g_small.into_data().value[0];
        let large_v = g_large.into_data().value[0];
        assert!(large_v > small_v, "larger delta must yield larger surrogate");
    }

    #[test]
    fn gate_state_delta_single_feature_dimension() {
        let dev = device();
        let baseline = Tensor::<B, 2>::from_data(
            Data::new(vec![2.0_f32, 4.0_f32], Shape::new([2, 1])),
            &dev,
        );
        let proposed = Tensor::<B, 2>::from_data(
            Data::new(vec![5.0_f32, 1.0_f32], Shape::new([2, 1])),
            &dev,
        );
        let g = suggested_info_gain_from_state_delta(baseline, proposed);
        assert_eq!(g.dims(), [2]);
        let v: Vec<f32> = g.into_data().value;
        assert_abs_diff_eq!(v[0], 9.0_f32, epsilon = 1.0e-5);
        assert_abs_diff_eq!(v[1], 9.0_f32, epsilon = 1.0e-5);
    }

    #[test]
    fn gate_nodal_scalars_agree_with_flattened_state_delta() {
        let dev = device();
        let b = Tensor::<B, 3>::from_data(
            Data::new(
                vec![0.0_f32, 2.0_f32, 4.0_f32, 6.0_f32],
                Shape::new([1, 2, 2]),
            ),
            &dev,
        );
        let p = Tensor::<B, 3>::from_data(
            Data::new(
                vec![1.0_f32, 3.0_f32, 5.0_f32, 7.0_f32],
                Shape::new([1, 2, 2]),
            ),
            &dev,
        );
        let inner_g3 = inner::suggested_info_gain_from_batched_nodal_scalars(b.clone(), p.clone());
        let g3 = suggested_info_gain_from_batched_nodal_scalars(b.clone(), p.clone());
        let g2 = suggested_info_gain_from_state_delta(b.reshape([1, 4]), p.reshape([1, 4]));
        let a: Vec<f32> = g3.into_data().value;
        let c: Vec<f32> = g2.into_data().value;
        let d: Vec<f32> = inner_g3.into_data().value;
        assert_abs_diff_eq!(a[0], c[0], epsilon = 1.0e-5);
        assert_abs_diff_eq!(a[0], d[0], epsilon = 1.0e-6);
    }

    #[test]
    fn gate_nodal_scalars_multi_batch_rows_independent() {
        let dev = device();
        let b = Tensor::<B, 3>::from_data(
            Data::new(
                vec![
                    0.0_f32, 0.0_f32, //
                    1.0_f32, 1.0_f32, //
                    0.0_f32, 0.0_f32, //
                    0.0_f32, 0.0_f32,
                ],
                Shape::new([2, 2, 2]),
            ),
            &dev,
        );
        let p = Tensor::<B, 3>::from_data(
            Data::new(
                vec![
                    1.0_f32, 1.0_f32, //
                    1.0_f32, 1.0_f32, //
                    2.0_f32, 0.0_f32, //
                    0.0_f32, 0.0_f32,
                ],
                Shape::new([2, 2, 2]),
            ),
            &dev,
        );
        let g = suggested_info_gain_from_batched_nodal_scalars(b, p);
        assert_eq!(g.dims(), [2]);
        let v: Vec<f32> = g.into_data().value;
        assert_abs_diff_eq!(v[0], 0.5_f32, epsilon = 1.0e-5);
        assert_abs_diff_eq!(v[1], 1.0_f32, epsilon = 1.0e-5);
    }

    #[test]
    fn gate_nodal_scalars_zero_when_unchanged() {
        let dev = device();
        let b = Tensor::<B, 3>::from_data(
            Data::new(
                vec![0.5_f32, 1.5_f32, 2.5_f32, 3.5_f32],
                Shape::new([1, 2, 2]),
            ),
            &dev,
        );
        let p = b.clone();
        let g = suggested_info_gain_from_batched_nodal_scalars(b, p);
        let v: Vec<f32> = g.into_data().value;
        assert_abs_diff_eq!(v[0], 0.0_f32, epsilon = 1.0e-6);
    }

    #[test]
    fn gate_nodal_scalars_single_node_single_channel() {
        let dev = device();
        let b = Tensor::<B, 3>::from_data(
            Data::new(vec![3.0_f32], Shape::new([1, 1, 1])),
            &dev,
        );
        let p = Tensor::<B, 3>::from_data(
            Data::new(vec![7.0_f32], Shape::new([1, 1, 1])),
            &dev,
        );
        let g = suggested_info_gain_from_batched_nodal_scalars(b, p);
        let v: Vec<f32> = g.into_data().value;
        assert_abs_diff_eq!(v[0], 16.0_f32, epsilon = 1.0e-5);
    }

    #[test]
    fn gate_info_gain_batch_axis_matches_gateway_contract() {
        let dev = device();
        let b = 4usize;
        let d = 3usize;
        let baseline = Tensor::<B, 2>::zeros([b, d], &dev);
        let proposed = Tensor::<B, 2>::from_data(
            Data::new(vec![1.0_f32; b * d], Shape::new([b, d])),
            &dev,
        );
        let g = suggested_info_gain_from_state_delta(baseline, proposed);
        assert_eq!(g.dims(), [b]);
        let v: Vec<f32> = g.into_data().value;
        assert_eq!(v.len(), b);
        for &row in &v {
            assert_abs_diff_eq!(row, 1.0_f32, epsilon = 1.0e-5);
        }
    }

    #[test]
    fn gate_info_gain_symmetric_under_baseline_proposed_swap() {
        let dev = device();
        let baseline = Tensor::<B, 2>::from_data(
            Data::new(vec![1.0_f32, 2.0_f32, 3.0_f32], Shape::new([1, 3])),
            &dev,
        );
        let proposed = Tensor::<B, 2>::from_data(
            Data::new(vec![4.0_f32, 5.0_f32, 6.0_f32], Shape::new([1, 3])),
            &dev,
        );
        let g_fwd = suggested_info_gain_from_state_delta(baseline.clone(), proposed.clone());
        let g_rev = suggested_info_gain_from_state_delta(proposed, baseline);
        let fwd: Vec<f32> = g_fwd.into_data().value;
        let rev: Vec<f32> = g_rev.into_data().value;
        assert_abs_diff_eq!(fwd[0], rev[0], epsilon = 1.0e-6);
    }

    #[test]
    fn w8e14_gate_info_gain_zero_delta_yields_zero() {
        let dev = device();
        let state = Tensor::<B, 2>::from_data(
            Data::new(vec![2.0_f32, 3.0_f32], Shape::new([1, 2])),
            &dev,
        );
        let g = suggested_info_gain_from_state_delta(state.clone(), state);
        let v: Vec<f32> = g.into_data().value;
        assert_abs_diff_eq!(v[0], 0.0_f32, epsilon = 1.0e-6);
    }
}
