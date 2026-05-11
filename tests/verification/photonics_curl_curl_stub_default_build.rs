// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Default-lane contract: without `--features photonics`, [`PhotonicsSolver::solve_maxwell_curl_curl`]
//! is a documented no-op (returns `e_field` unchanged). See `docs/Solver-Status.md` (**DEFERRAL — Photonics**)
//! and module docs in `src/physics/solvers/photonics.rs`.

#[cfg(not(feature = "photonics"))]
mod photonics_off {
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};
    use umst_manifold::physics::solvers::PhotonicsSolver;
    use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

    type B = NdArray<f32>;

    fn device() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    fn chain_edges(n: usize) -> Tensor<B, 2, Int> {
        let e = n - 1;
        let mut v = Vec::with_capacity(2 * e);
        for i in 0..e {
            v.push(i as i64);
        }
        for i in 0..e {
            v.push((i + 1) as i64);
        }
        Tensor::from_data(Data::new(v, Shape::new([2, e])), &device())
    }

    fn coords_line_x(n: usize, h: f32) -> Tensor<B, 2> {
        let mut v = Vec::with_capacity(n * 3);
        for i in 0..n {
            v.push(i as f32 * h);
            v.push(0.0);
            v.push(0.0);
        }
        Tensor::from_data(Data::new(v, Shape::new([n, 3])), &device())
    }

    /// Without `photonics`, curl–curl must not mutate the incoming field (compile-checked API + runtime identity).
    #[test]
    fn solve_maxwell_curl_curl_returns_e_field_unchanged_when_photonics_feature_off() {
        let dev = device();
        let n = 11usize;
        let h = 1e-3_f32;
        let edges = chain_edges(n);
        let coords = coords_line_x(n, h);

        let mut e0 = vec![0.0_f32; n * 3];
        for i in 0..n {
            e0[i * 3] = 0.1 * i as f32;
            e0[i * 3 + 1] = 0.2 * i as f32;
            e0[i * 3 + 2] = 0.3 * i as f32;
        }
        let e_field = Tensor::<B, 3>::from_data(Data::new(e0.clone(), Shape::new([1, n, 3])), &dev);
        let eps_r = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let eps_i = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let mut j = vec![0.0_f32; n * 3];
        j[3 * (n / 2) + 1] = 1.0_f32;
        let j = Tensor::<B, 3>::from_data(Data::new(j, Shape::new([1, n, 3])), &dev);
        let cg = MechanicsInnerLoopConfig::default();
        let ps = PhotonicsSolver { frequency_hz: 1e9_f32 };

        let out = ps.solve_maxwell_curl_curl(e_field, eps_r, eps_i, j, edges, coords, &cg);

        let got = out.into_data().value;
        assert_eq!(got.len(), e0.len());
        for i in 0..got.len() {
            assert_eq!(got[i], e0[i], "stub must preserve e_field[{i}]");
        }
    }
}
