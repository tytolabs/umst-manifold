// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Neural implicit field decode scaffold (R4) — decode-only, no CutFEM.
//!
//! **Honesty:** MLP `(z, x) → φ` with sigmoid density mapping only. Not eikonal-regularized,
//! not mesh-extracted, not physics-GREEN. Feature gate: [`IMPLICIT_FIELD_CARGO_FEATURE`].

#![cfg(feature = "design-implicit-field")]

use burn::module::Module;
use burn::nn::{Linear, LinearConfig};
use burn::tensor::activation::{sigmoid, tanh};
use burn::tensor::{backend::Backend, Tensor};

use crate::core::traits::{DesignDecodeError, DesignLatent, DesignRepresentation, Geometry};

/// Cargo feature gate for this module (`umst-manifold/Cargo.toml`).
pub const IMPLICIT_FIELD_CARGO_FEATURE: &str = "design-implicit-field";

/// W29 deepen cell id — decode scaffold only (`MASTER_RETICK=no`).
pub const IMPLICIT_FIELD_CELL_ID: &str = "W29-010-IMPLICIT_FIELD";

/// Honest posture — tests deepen only; no GREEN / production invent.
pub const IMPLICIT_FIELD_POSTURE_TAG: &str = "honest-decode-scaffold-only";

/// [`DesignRepresentation::repr_id`] witness string.
pub const IMPLICIT_FIELD_REPR_ID: &str = "umst.design.implicit_field";

/// Production wiring blocked — CutFEM / mesh oracle deferred.
pub const IMPLICIT_FIELD_PRODUCTION_WIRED: bool = false;

/// Physics GREEN blocked — no eikonal / SDF oracle on this path.
pub const IMPLICIT_FIELD_PHYSICS_GREEN: bool = false;

/// Default sigmoid sharpness β when callers omit an explicit value.
pub const IMPLICIT_FIELD_DEFAULT_BETA: f32 = 1.0;

/// Whether implicit-field honesty fences are pinned @ HEAD (no GREEN invent).
#[must_use]
pub fn implicit_field_honesty_fence_pinned() -> bool {
    IMPLICIT_FIELD_CARGO_FEATURE == "design-implicit-field"
        && IMPLICIT_FIELD_CELL_ID == "W29-010-IMPLICIT_FIELD"
        && IMPLICIT_FIELD_POSTURE_TAG == "honest-decode-scaffold-only"
        && IMPLICIT_FIELD_REPR_ID == "umst.design.implicit_field"
        && !IMPLICIT_FIELD_PRODUCTION_WIRED
        && !IMPLICIT_FIELD_PHYSICS_GREEN
        && IMPLICIT_FIELD_DEFAULT_BETA.is_finite()
        && IMPLICIT_FIELD_DEFAULT_BETA > 0.0
}

/// Map raw field output φ to nodal density ρ = σ(−β·φ) ∈ (0, 1).
#[must_use]
pub fn phi_to_density<B: Backend<FloatElem = f32>>(phi: Tensor<B, 3>, beta: f32) -> Tensor<B, 3> {
    sigmoid(phi.mul_scalar(-beta))
}

/// MLP `(z, x) → φ` scaffold — not eikonal-regularized (future work).
#[derive(Module, Debug)]
pub struct ImplicitFieldNet<B: Backend> {
    lin_z: Linear<B>,
    lin_x: Linear<B>,
    lin_h: Linear<B>,
    lin_out: Linear<B>,
}

impl<B: Backend<FloatElem = f32>> ImplicitFieldNet<B> {
    pub fn new(latent_dim: usize, hidden_dim: usize, device: &B::Device) -> Self {
        Self {
            lin_z: LinearConfig::new(latent_dim, hidden_dim).init(device),
            lin_x: LinearConfig::new(3, hidden_dim).init(device),
            lin_h: LinearConfig::new(hidden_dim, hidden_dim).init(device),
            lin_out: LinearConfig::new(hidden_dim, 1).init(device),
        }
    }

    pub fn forward_phi(&self, latent_bn: Tensor<B, 2>, coords_bn3: Tensor<B, 3>) -> Tensor<B, 3> {
        let [b, n, three] = coords_bn3.dims();
        debug_assert_eq!(three, 3);
        let z = latent_bn.clone().unsqueeze_dim::<3>(1);
        let z_rep = z.expand([b, n, latent_bn.dims()[1]]);
        let z_flat = z_rep.reshape([b * n, latent_bn.dims()[1]]);
        let x_flat = coords_bn3.reshape([b * n, 3]);
        let h = tanh(self.lin_z.forward(z_flat).add(self.lin_x.forward(x_flat)));
        let h = tanh(self.lin_h.forward(h));
        let phi = self.lin_out.forward(h);
        phi.reshape([b, n, 1])
    }
}

/// Implicit SDF representation scaffold (`f(z, x) → φ`).
#[derive(Clone, Debug)]
pub struct ImplicitField<B: Backend> {
    pub field_net: ImplicitFieldNet<B>,
    pub beta: f32,
}

impl<B: Backend<FloatElem = f32>> ImplicitField<B> {
    pub fn new(field_net: ImplicitFieldNet<B>, beta: f32) -> Self {
        Self { field_net, beta }
    }

    /// Single choke point for harness / B6 decode (parity with [`crate::ai::topology::VoxelDensity::decode_voxel_density`]).
    pub fn decode_implicit_field(
        &self,
        latent: &DesignLatent<B>,
        query_coords: Tensor<B, 3>,
    ) -> Result<Geometry<B>, DesignDecodeError> {
        self.decode(latent, query_coords)
    }

    fn validate_decode_inputs(
        &self,
        latent: &DesignLatent<B>,
        query_coords: &Tensor<B, 3>,
    ) -> Result<(), DesignDecodeError> {
        let [b_lat, _d] = latent.tensor.dims();
        let [b_coord, _n, three] = query_coords.dims();
        if three != 3 {
            return Err(DesignDecodeError::ShapeMismatch);
        }
        if b_lat != b_coord {
            return Err(DesignDecodeError::ShapeMismatch);
        }
        if !self.beta.is_finite() || self.beta <= 0.0 {
            return Err(DesignDecodeError::NonFinite);
        }
        Ok(())
    }
}

impl<B: Backend<FloatElem = f32>> DesignRepresentation<B> for ImplicitField<B> {
    fn repr_id(&self) -> &'static str {
        IMPLICIT_FIELD_REPR_ID
    }

    fn decode(
        &self,
        latent: &DesignLatent<B>,
        query_coords: Tensor<B, 3>,
    ) -> Result<Geometry<B>, DesignDecodeError> {
        self.validate_decode_inputs(latent, &query_coords)?;
        let phi = self
            .field_net
            .forward_phi(latent.tensor.clone(), query_coords.clone());
        let density = phi_to_density(phi.clone(), self.beta);
        if density
            .clone()
            .into_data()
            .value
            .iter()
            .any(|x| !x.is_finite())
        {
            return Err(DesignDecodeError::NonFinite);
        }
        Ok(Geometry {
            density,
            signed_distance: Some(phi),
            coords: query_coords,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::{Data, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn device() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    fn bar_coords(n: usize, dev: &NdArrayDevice) -> Tensor<B, 3> {
        let mut data = Vec::with_capacity(n * 3);
        for i in 0..n {
            let t = i as f32 / (n - 1).max(1) as f32;
            data.extend_from_slice(&[t, 0.0, 0.0]);
        }
        Tensor::<B, 3>::from_data(Data::new(data, Shape::new([1, n, 3])), dev)
    }

    #[test]
    fn implicit_field_honesty_fence_pinned_not_green() {
        assert!(implicit_field_honesty_fence_pinned());
        assert!(!IMPLICIT_FIELD_PRODUCTION_WIRED);
        assert!(!IMPLICIT_FIELD_PHYSICS_GREEN);
        assert!(IMPLICIT_FIELD_POSTURE_TAG.contains("honest"));
        assert!(!IMPLICIT_FIELD_POSTURE_TAG
            .to_ascii_lowercase()
            .contains("green"));
    }

    #[test]
    fn implicit_field_repr_id_and_feature_gate_documented() {
        assert_eq!(IMPLICIT_FIELD_REPR_ID, "umst.design.implicit_field");
        assert_eq!(IMPLICIT_FIELD_CARGO_FEATURE, "design-implicit-field");
        assert_eq!(IMPLICIT_FIELD_CELL_ID, "W29-010-IMPLICIT_FIELD");
    }

    #[test]
    fn implicit_field_decode_finite_bar_fixture() {
        let dev = device();
        let net = ImplicitFieldNet::<B>::new(4, 8, &dev);
        let field = ImplicitField::new(net, IMPLICIT_FIELD_DEFAULT_BETA);
        let n = 5_usize;
        let coords = bar_coords(n, &dev);
        let latent = DesignLatent {
            tensor: Tensor::<B, 2>::zeros([1, 4], &dev),
        };
        let geom = field
            .decode_implicit_field(&latent, coords)
            .expect("decode");
        assert_eq!(geom.density.dims(), [1, n, 1]);
        assert!(geom.signed_distance.is_some());
        let vals: Vec<f32> = geom.density.into_data().value;
        assert!(vals.iter().all(|x| x.is_finite()));
        for &rho in &vals {
            assert!(
                rho > 0.0 && rho < 1.0,
                "sigmoid density must lie in (0,1), got {rho}"
            );
        }
    }

    #[test]
    fn implicit_field_batch_mismatch_refuses_shape() {
        let dev = device();
        let net = ImplicitFieldNet::<B>::new(2, 4, &dev);
        let field = ImplicitField::new(net, 1.0);
        let coords = Tensor::<B, 3>::zeros([2, 3, 3], &dev);
        let latent = DesignLatent {
            tensor: Tensor::<B, 2>::zeros([1, 2], &dev),
        };
        assert_eq!(
            field.decode(&latent, coords),
            Err(DesignDecodeError::ShapeMismatch)
        );
    }

    #[test]
    fn implicit_field_non_positive_beta_refuses_non_finite() {
        let dev = device();
        let net = ImplicitFieldNet::<B>::new(2, 4, &dev);
        let field = ImplicitField::new(net, 0.0);
        let coords = Tensor::<B, 3>::zeros([1, 2, 3], &dev);
        let latent = DesignLatent {
            tensor: Tensor::<B, 2>::zeros([1, 2], &dev),
        };
        assert_eq!(
            field.decode(&latent, coords),
            Err(DesignDecodeError::NonFinite)
        );
    }

    #[test]
    fn implicit_field_phi_to_density_sigmoid_bounds() {
        let dev = device();
        let phi = Tensor::<B, 3>::from_data(
            Data::new(vec![-10.0_f32, 0.0, 10.0], Shape::new([1, 3, 1])),
            &dev,
        );
        let rho = phi_to_density(phi, 1.0);
        let vals: Vec<f32> = rho.into_data().value;
        assert!(vals[0] > vals[1] && vals[1] > vals[2]);
        for &v in &vals {
            assert!(v > 0.0 && v < 1.0);
        }
    }

    #[test]
    fn implicit_field_forward_phi_matches_batch_latent_broadcast() {
        let dev = device();
        let net = ImplicitFieldNet::<B>::new(3, 6, &dev);
        let coords = Tensor::<B, 3>::from_data(
            Data::new(
                vec![0.0_f32, 0.0, 0.0, 1.0, 0.0, 0.0],
                Shape::new([2, 1, 3]),
            ),
            &dev,
        );
        let latent = DesignLatent {
            tensor: Tensor::<B, 2>::from_data(
                Data::new(vec![0.1_f32, 0.2, 0.3, 0.4, 0.5, 0.6], Shape::new([2, 3])),
                &dev,
            ),
        };
        let phi = net.forward_phi(latent.tensor, coords);
        assert_eq!(phi.dims(), [2, 1, 1]);
        assert!(phi.into_data().value.iter().all(|x| x.is_finite()));
    }
}
