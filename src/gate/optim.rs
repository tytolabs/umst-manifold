// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Thin gate-layer atom spine visibility for **LPP-008** `adamw_step_policy` (tensor-only AdamW; no duplicated physics).
//!
//! Composition:
//! - **LPP-008** · `adamw_step_policy` · io: `weights, grad, lr, moments → (w_new, m1, m2, t)`
//!
//! **Honesty:** morphism remains **private** in [`crate::ai::liquid_ppo`]; composed into LPP-004/005/006
//! training chains (`AdjointNeuralODE::backward_adjoint` → `adamw_step_policy`). Direct gate re-export
//! deferred @ `umst-atoms::optim::adamw` atom crate alignment + LP2-C stub-kill visibility lift.

/// LPP-008 morphism id @ PORT_GRAIN_BAND `gate:optim`.
pub const LPP_008_MORPHISM_ID: &str = "adamw_step_policy";

/// IO signature witness for composition maps (`proposed_new_reorg_home: atom`).
pub const LPP_008_IO_SIGNATURE: &str = "weights, grad, lr, moments → (w_new, m1, m2, t)";

/// Honest posture — direct forwarder deferred; tests deepen only (`MASTER_RETICK=no`).
pub const LPP_008_POSTURE_TAG: &str = "honest-atom-spine-visibility-only";

/// PORT-MF-OPTIM-W2 cell id (wave-2 gate band deepen).
pub const LPP_008_CELL_ID: &str = "PORT-MF-OPTIM-W2";

/// Proposed atom-crate port home for **LPP-008** (composition map `proposed_new_reorg_home: atom`).
pub const LPP_008_PROPOSED_HOME: &str = "umst-atoms::optim::adamw";

/// Burn AdamW default β₁ @ [`crate::ai::liquid_ppo`] `adamw_step_policy`.
pub const LPP_008_BETA1: f32 = 0.9;

/// Burn AdamW default β₂ @ [`crate::ai::liquid_ppo`] `adamw_step_policy`.
pub const LPP_008_BETA2: f32 = 0.999;

/// Burn AdamW default ε @ [`crate::ai::liquid_ppo`] `adamw_step_policy`.
pub const LPP_008_EPS: f32 = 1e-5;

/// Decoupled weight-decay coefficient @ [`crate::ai::liquid_ppo`] `adamw_step_policy`.
pub const LPP_008_WEIGHT_DECAY: f32 = 1e-4;

/// Whether LPP-008 spine metadata is pinned @ HEAD (visibility only; no GREEN invent).
#[must_use]
pub fn lpp_008_spine_pinned() -> bool {
    LPP_008_MORPHISM_ID == "adamw_step_policy"
        && LPP_008_IO_SIGNATURE.contains("weights, grad, lr, moments")
        && LPP_008_IO_SIGNATURE.contains("(w_new, m1, m2, t)")
        && LPP_008_POSTURE_TAG == "honest-atom-spine-visibility-only"
        && LPP_008_CELL_ID == "PORT-MF-OPTIM-W2"
        && LPP_008_PROPOSED_HOME == "umst-atoms::optim::adamw"
        && LPP_008_BETA1 == 0.9_f32
        && LPP_008_BETA2 == 0.999_f32
        && LPP_008_EPS == 1e-5_f32
        && LPP_008_WEIGHT_DECAY == 1e-4_f32
}

/// Scalar AdamW contract reference (decoupled WD) — audit witness only; production tensor step stays private in `liquid_ppo`.
///
/// Mirrors the hyperparameter schedule documented on **LPP-008** without duplicating the Burn tensor hot path.
#[must_use]
pub fn lpp_008_scalar_adamw_step(
    weight: f64,
    grad: f64,
    lr: f64,
    m1: Option<f64>,
    m2: Option<f64>,
    t: usize,
) -> (f64, f64, f64, usize) {
    let beta1 = f64::from(LPP_008_BETA1);
    let beta2 = f64::from(LPP_008_BETA2);
    let eps = f64::from(LPP_008_EPS);
    let wd = f64::from(LPP_008_WEIGHT_DECAY);

    let decayed = weight - weight * lr * wd;

    let (moment_1, moment_2, time) = match (m1, m2) {
        (Some(m1_prev), Some(m2_prev)) => {
            let m1_new = m1_prev * beta1 + grad * (1.0 - beta1);
            let m2_new = m2_prev * beta2 + grad * grad * (1.0 - beta2);
            (m1_new, m2_new, t + 1)
        }
        _ => {
            let m1_new = grad * (1.0 - beta1);
            let m2_new = grad * grad * (1.0 - beta2);
            (m1_new, m2_new, 1)
        }
    };

    let time_i = time as i32;
    let m1c = moment_1 / (1.0 - beta1.powi(time_i));
    let m2c = moment_2 / (1.0 - beta2.powi(time_i));
    let raw_delta = m1c / (m2c.sqrt() + eps);
    let new_w = decayed - raw_delta * lr;

    (new_w, moment_1, moment_2, time)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use burn::tensor::{Data, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn device() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    /// Test-only tensor reference mirroring [`crate::ai::liquid_ppo`] `adamw_step_policy` contract.
    fn adamw_step_policy_ref(
        weights: Tensor<B, 1>,
        grad: Tensor<B, 1>,
        lr: f32,
        m1: Option<Tensor<B, 1>>,
        m2: Option<Tensor<B, 1>>,
        t: usize,
    ) -> (Tensor<B, 1>, Tensor<B, 1>, Tensor<B, 1>, usize) {
        let tensor_updated = weights
            .clone()
            .sub(weights.mul_scalar(lr * LPP_008_WEIGHT_DECAY));

        let (moment_1, moment_2, time) = match (m1, m2) {
            (Some(m1_prev), Some(m2_prev)) => {
                let m1_new = m1_prev
                    .mul_scalar(LPP_008_BETA1)
                    .add(grad.clone().mul_scalar(1.0_f32 - LPP_008_BETA1));
                let m2_new = m2_prev.mul_scalar(LPP_008_BETA2).add(
                    grad.clone()
                        .powf_scalar(2.0_f32)
                        .mul_scalar(1.0_f32 - LPP_008_BETA2),
                );
                (m1_new, m2_new, t + 1)
            }
            _ => {
                let m1_new = grad.clone().mul_scalar(1.0_f32 - LPP_008_BETA1);
                let m2_new = grad
                    .powf_scalar(2.0_f32)
                    .mul_scalar(1.0_f32 - LPP_008_BETA2);
                (m1_new, m2_new, 1)
            }
        };

        let time_i = time as i32;
        let m1c = moment_1
            .clone()
            .div_scalar(1.0_f32 - LPP_008_BETA1.powi(time_i));
        let m2c = moment_2
            .clone()
            .div_scalar(1.0_f32 - LPP_008_BETA2.powi(time_i));
        let raw_delta = m1c.div(m2c.sqrt().add_scalar(LPP_008_EPS));
        let new_w = tensor_updated.sub(raw_delta.mul_scalar(lr));

        (new_w, moment_1, moment_2, time)
    }

    #[test]
    fn gate_optim_lpp_008_spine_metadata_pinned() {
        assert!(lpp_008_spine_pinned());
        assert_eq!(LPP_008_MORPHISM_ID, "adamw_step_policy");
        assert_eq!(LPP_008_CELL_ID, "PORT-MF-OPTIM-W2");
        assert_eq!(LPP_008_PROPOSED_HOME, "umst-atoms::optim::adamw");
    }

    #[test]
    fn gate_optim_io_signature_documents_tensor_io() {
        assert!(LPP_008_IO_SIGNATURE.contains("weights"));
        assert!(LPP_008_IO_SIGNATURE.contains("grad"));
        assert!(LPP_008_IO_SIGNATURE.contains("lr"));
        assert!(LPP_008_IO_SIGNATURE.contains("moments"));
        assert!(LPP_008_IO_SIGNATURE.contains("w_new"));
        assert!(LPP_008_IO_SIGNATURE.contains("m1"));
        assert!(LPP_008_IO_SIGNATURE.contains("m2"));
        assert!(LPP_008_IO_SIGNATURE.contains('t'));
    }

    #[test]
    fn gate_optim_hyperparameters_match_burn_adamw_defaults() {
        assert_relative_eq!(f64::from(LPP_008_BETA1), 0.9, epsilon = 1.0e-6);
        assert_relative_eq!(f64::from(LPP_008_BETA2), 0.999, epsilon = 1.0e-6);
        assert_relative_eq!(f64::from(LPP_008_EPS), 1.0e-5, epsilon = 1.0e-8);
        assert_relative_eq!(f64::from(LPP_008_WEIGHT_DECAY), 1.0e-4, epsilon = 1.0e-8);
    }

    #[test]
    fn gate_optim_scalar_first_step_initializes_moments_at_t_one() {
        let (w_new, m1, m2, t) = lpp_008_scalar_adamw_step(1.0, 0.5, 0.01, None, None, 0);
        assert_eq!(t, 1);
        assert_relative_eq!(m1, 0.05, epsilon = 1.0e-6);
        assert_relative_eq!(m2, 0.00025, epsilon = 1.0e-6);
        assert!(w_new.is_finite());
        assert!(
            w_new < 1.0,
            "decoupled WD + grad step must move weight down from 1.0"
        );
    }

    #[test]
    fn gate_optim_scalar_second_step_increments_time() {
        let (_, m1, m2, t1) = lpp_008_scalar_adamw_step(1.0, 0.5, 0.01, None, None, 0);
        let (_, _, _, t2) = lpp_008_scalar_adamw_step(1.0, 0.5, 0.01, Some(m1), Some(m2), t1);
        assert_eq!(t2, 2);
    }

    #[test]
    fn gate_optim_scalar_zero_grad_applies_weight_decay_only() {
        let lr = 0.1_f64;
        let w0 = 2.0_f64;
        let (w_new, m1, m2, t) = lpp_008_scalar_adamw_step(w0, 0.0, lr, None, None, 0);
        let expected_decayed = w0 - w0 * lr * f64::from(LPP_008_WEIGHT_DECAY);
        assert_relative_eq!(w_new, expected_decayed, epsilon = 1.0e-9);
        assert_relative_eq!(m1, 0.0, epsilon = 1.0e-30);
        assert_relative_eq!(m2, 0.0, epsilon = 1.0e-30);
        assert_eq!(t, 1);
    }

    #[test]
    fn gate_optim_scalar_positive_grad_moves_weight_against_gradient() {
        let (w_pos, _, _, _) = lpp_008_scalar_adamw_step(1.0, 1.0, 0.01, None, None, 0);
        let (w_neg, _, _, _) = lpp_008_scalar_adamw_step(1.0, -1.0, 0.01, None, None, 0);
        assert!(w_pos < 1.0, "positive grad must decrease weight");
        assert!(w_neg > 1.0, "negative grad must increase weight");
        assert!(w_pos < w_neg, "sign of grad must flip update direction");
    }

    #[test]
    fn gate_optim_scalar_moments_accumulate_across_steps() {
        let (w1, m1_1, m2_1, t1) = lpp_008_scalar_adamw_step(1.0, 0.4, 0.01, None, None, 0);
        let (w2, m1_2, m2_2, t2) =
            lpp_008_scalar_adamw_step(w1, 0.4, 0.01, Some(m1_1), Some(m2_1), t1);
        assert_eq!(t2, 2);
        assert!(m1_2.abs() > m1_1.abs(), "m1 must accumulate");
        assert!(m2_2 > m2_1, "m2 must accumulate for nonzero grad");
        assert!(w2.is_finite());
    }

    #[test]
    fn gate_optim_scalar_matches_manual_bias_correction_at_t_two() {
        let grad = 0.25_f64;
        let lr = 0.01_f64;
        let w0 = 1.0_f64;
        let (_, m1_1, m2_1, t1) = lpp_008_scalar_adamw_step(w0, grad, lr, None, None, 0);
        let (w2, _, _, t2) = lpp_008_scalar_adamw_step(w0, grad, lr, Some(m1_1), Some(m2_1), t1);
        assert_eq!(t2, 2);

        let beta1 = f64::from(LPP_008_BETA1);
        let beta2 = f64::from(LPP_008_BETA2);
        let eps = f64::from(LPP_008_EPS);
        let wd = f64::from(LPP_008_WEIGHT_DECAY);
        let m1_new = m1_1 * beta1 + grad * (1.0 - beta1);
        let m2_new = m2_1 * beta2 + grad * grad * (1.0 - beta2);
        let m1c = m1_new / (1.0 - beta1.powi(2));
        let m2c = m2_new / (1.0 - beta2.powi(2));
        let decayed = w0 - w0 * lr * wd;
        let expected = decayed - (m1c / (m2c.sqrt() + eps)) * lr;
        assert_relative_eq!(w2, expected, epsilon = 1.0e-9);
    }

    #[test]
    fn gate_optim_tensor_ref_agrees_with_scalar_on_single_element() {
        let dev = device();
        let w = Tensor::<B, 1>::from_data(Data::new(vec![1.0_f32], Shape::new([1])), &dev);
        let g = Tensor::<B, 1>::from_data(Data::new(vec![0.5_f32], Shape::new([1])), &dev);
        let lr = 0.01_f32;
        let (w_tensor, m1_t, m2_t, t_tensor) =
            adamw_step_policy_ref(w, g.clone(), lr, None, None, 0);
        let (w_scalar, m1_s, m2_s, t_scalar) =
            lpp_008_scalar_adamw_step(1.0, 0.5, f64::from(lr), None, None, 0);
        assert_eq!(t_tensor, t_scalar);
        assert_relative_eq!(
            w_tensor.into_data().value[0],
            w_scalar as f32,
            epsilon = 1.0e-5
        );
        assert_relative_eq!(m1_t.into_data().value[0], m1_s as f32, epsilon = 1.0e-5);
        assert_relative_eq!(m2_t.into_data().value[0], m2_s as f32, epsilon = 1.0e-5);
    }

    #[test]
    fn gate_optim_tensor_ref_multi_element_independent_rows() {
        let dev = device();
        let w = Tensor::<B, 1>::from_data(
            Data::new(vec![1.0_f32, 2.0_f32, -1.0_f32], Shape::new([3])),
            &dev,
        );
        let g = Tensor::<B, 1>::from_data(
            Data::new(vec![0.1_f32, -0.2_f32, 0.3_f32], Shape::new([3])),
            &dev,
        );
        let (w_new, _, _, _) = adamw_step_policy_ref(w, g, 0.01, None, None, 0);
        let v: Vec<f32> = w_new.into_data().value;
        assert_eq!(v.len(), 3);
        for &x in &v {
            assert!(x.is_finite(), "each element must stay finite");
        }
        assert_ne!(v[0], v[1]);
        assert_ne!(v[1], v[2]);
    }

    #[test]
    fn gate_optim_tensor_ref_two_step_chain_finite() {
        let dev = device();
        let w0 =
            Tensor::<B, 1>::from_data(Data::new(vec![0.5_f32, -0.5_f32], Shape::new([2])), &dev);
        let g =
            Tensor::<B, 1>::from_data(Data::new(vec![0.2_f32, -0.1_f32], Shape::new([2])), &dev);
        let lr = 0.001_f32;
        let (w1, m1, m2, t1) = adamw_step_policy_ref(w0, g.clone(), lr, None, None, 0);
        let (w2, _, _, t2) = adamw_step_policy_ref(w1, g, lr, Some(m1), Some(m2), t1);
        assert_eq!(t2, 2);
        for x in w2.into_data().value {
            assert!(x.is_finite());
        }
    }

    #[test]
    fn gate_optim_proposed_home_is_atom_crate_path() {
        assert!(LPP_008_PROPOSED_HOME.starts_with("umst-atoms::"));
        assert!(LPP_008_PROPOSED_HOME.ends_with("adamw"));
        assert!(LPP_008_PROPOSED_HOME.contains("optim"));
    }

    #[test]
    fn gate_optim_posture_tag_blocks_green_invent() {
        assert!(LPP_008_POSTURE_TAG.contains("honest"));
        assert!(LPP_008_POSTURE_TAG.contains("visibility"));
        assert!(!LPP_008_POSTURE_TAG.contains("green"));
    }

    #[test]
    fn w8e14_gate_optim_adamw_step_zero_gradient_is_noop() {
        // Honest AdamW: zero grad still applies decoupled weight decay (not a pure noop).
        let dev = device();
        let lr = 0.01_f32;
        let w = Tensor::<B, 1>::from_data(Data::new(vec![1.0_f32, 2.0_f32], Shape::new([2])), &dev);
        let g = Tensor::<B, 1>::zeros([2], &dev);
        let (w1, _, _, t) = adamw_step_policy_ref(w.clone(), g, lr, None, None, 0);
        assert_eq!(t, 1);
        let v0: Vec<f32> = w.into_data().value;
        let v1: Vec<f32> = w1.into_data().value;
        let wd = LPP_008_WEIGHT_DECAY;
        for i in 0..2 {
            let expected = v0[i] * (1.0 - lr * wd);
            assert!(
                (v1[i] - expected).abs() < 1e-6,
                "zero-grad AdamW must apply WD only: got {} want {expected}",
                v1[i]
            );
        }
    }
}
