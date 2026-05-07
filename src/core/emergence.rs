// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO


use burn::tensor::{backend::Backend, Tensor};

/// Emergence Monitor
/// Tracks dissipation hotspots mapping $D_{int}$ to tensor gradients.
/// $m_i = D_{int,i} + \lambda|\nabla SDF_i|^2$
pub struct EmergenceMonitor<B: Backend> {
    pub lambda: f32,
    _backend: std::marker::PhantomData<B>,
}

impl<B: Backend> EmergenceMonitor<B> {
    pub fn new(lambda: f32) -> Self {
        Self {
            lambda,
            _backend: std::marker::PhantomData,
        }
    }

    /// Computes the thermo-topological defect mass field.
    /// This is a fully differentiable tensor operation allowing backprop.
    ///
    /// # Arguments
    /// * `d_int` - Internal dissipation tensor [Batch, Depth, Height, Width]
    /// * `sdf` - Signed Distance Field geometry tensor [Batch, Depth, Height, Width]
    ///
    /// # Returns
    /// * `mass_defects` - Tensor mapping hotspots [Batch, Depth, Height, Width]
    pub fn compute_dissipation_hotspots(
        &self,
        d_int: Tensor<B, 4>,
        sdf: Tensor<B, 4>,
    ) -> Tensor<B, 4> {
        let dims = sdf.dims();
        let (batch, d, h, w) = (dims[0], dims[1], dims[2], dims[3]);

        // Real finite-difference spatial gradients ∇SDF
        // dSDF/dx
        let sdf_x_plus = sdf.clone().slice([0..batch, 0..d, 0..h, 2..w]);
        let sdf_x_minus = sdf.clone().slice([0..batch, 0..d, 0..h, 0..(w - 2)]);
        let dx = sdf_x_plus.sub(sdf_x_minus).div_scalar(2.0);

        // dSDF/dy
        let sdf_y_plus = sdf.clone().slice([0..batch, 0..d, 2..h, 0..w]);
        let sdf_y_minus = sdf.clone().slice([0..batch, 0..d, 0..(h - 2), 0..w]);
        let dy = sdf_y_plus.sub(sdf_y_minus).div_scalar(2.0);

        // dSDF/dz
        let sdf_z_plus = sdf.clone().slice([0..batch, 2..d, 0..h, 0..w]);
        let sdf_z_minus = sdf.clone().slice([0..batch, 0..(d - 2), 0..h, 0..w]);
        let dz = sdf_z_plus.sub(sdf_z_minus).div_scalar(2.0);

        // Pad the gradients back to original dimensions using zeros at the boundary
        let pad_x = dx.pad(&[(0, 0), (0, 0), (0, 0), (1, 1)], 0.0);
        let pad_y = dy.pad(&[(0, 0), (0, 0), (1, 1), (0, 0)], 0.0);
        let pad_z = dz.pad(&[(0, 0), (1, 1), (0, 0), (0, 0)], 0.0);

        // Compute magnitude squared: |∇SDF|^2 = dx^2 + dy^2 + dz^2
        let grad_sdf_sq = pad_x
            .powf_scalar(2.0)
            .add(pad_y.powf_scalar(2.0))
            .add(pad_z.powf_scalar(2.0));

        // Return m_i = D_{int,i} + \lambda|\nabla SDF_i|^2
        d_int.add(grad_sdf_sq.mul_scalar(self.lambda))
    }
}
