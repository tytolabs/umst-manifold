// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Neural implicit field decode scaffold (R4) — decode-only, no CutFEM.

#![cfg(feature = "design-implicit-field")]

use burn::module::Module;
use burn::nn::{Linear, LinearConfig};
use burn::tensor::activation::{sigmoid, tanh};
use burn::tensor::{backend::Backend, Tensor};

use crate::core::traits::{DesignDecodeError, DesignLatent, DesignRepresentation, Geometry};

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
}

impl<B: Backend<FloatElem = f32>> DesignRepresentation<B> for ImplicitField<B> {
    fn repr_id(&self) -> &'static str {
        "umst.design.implicit_field"
    }

    fn decode(
        &self,
        latent: &DesignLatent<B>,
        query_coords: Tensor<B, 3>,
    ) -> Result<Geometry<B>, DesignDecodeError> {
        let phi = self
            .field_net
            .forward_phi(latent.tensor.clone(), query_coords.clone());
        let density = sigmoid(phi.clone().mul_scalar(-self.beta));
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
