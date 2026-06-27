// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! R3: deterministic query replay — identical inputs yield identical witnesses.

#![cfg(feature = "design-query")]

use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::ai::topology::{DensityNet, VoxelDensity};
use umst_manifold::core::traits::{DesignLatent, DesignRepresentation};
use umst_manifold::design::query::{
    DesignQueryContext, DesignQueryPort, StructuralDesignQuery,
};
use umst_manifold::physics::adjoint::SimpElasticMaterial;
use umst_manifold::physics::compliance_functional::{
    ComplianceContext, CompliancePenalization, Q1HexBrickSpec,
};
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type B = NdArray<f32>;

fn plate_bottom_uz_mask(nx: usize, ny: usize, nz: usize) -> Vec<f32> {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n = nx1 * ny1 * (nz + 1);
    let mut m = vec![1.0_f32; n * 3];
    for iz in 0..=nz {
        for iy in 0..=ny {
            for ix in 0..=nx {
                let nid = ix + iy * nx1 + iz * nx1 * ny1;
                if iz == 0 {
                    m[nid * 3 + 2] = 0.0;
                }
            }
        }
    }
    m
}

#[test]
fn design_query_deterministic_replay() {
    let nx = 4_usize;
    let ny = 4_usize;
    let nz = 1_usize;
    let n_nodes = (nx + 1) * (ny + 1) * (nz + 1);
    let dev = NdArrayDevice::default();
    let rep = VoxelDensity::new(DensityNet::new(8, &dev));
    let coords: Vec<f32> = (0..n_nodes)
        .flat_map(|i| {
            let fi = i as f32;
            [fi * 0.01, fi * 0.02, fi * 0.03]
        })
        .collect();
    let coords_t = Tensor::<B, 3>::from_data(Data::new(coords, Shape::new([1, n_nodes, 3])), &dev);
    let latent = DesignLatent {
        tensor: Tensor::<B, 2>::zeros([1, 1], &dev),
    };
    let mut bf = vec![0.0_f32; n_nodes * 3];
    bf[bf.len() / 2] = -1.0;
    let bm = plate_bottom_uz_mask(nx, ny, nz);
    let ctx = ComplianceContext {
        material: SimpElasticMaterial {
            e0: 1.0,
            nu: 0.3,
            p: 1.0,
            e_min: 1e-9,
        },
        mesh: Q1HexBrickSpec {
            nx,
            ny,
            nz,
            dx: 0.25,
            dy: 0.25,
            dz: 0.05,
        },
        cg: MechanicsInnerLoopConfig::default(),
        self_weight: None,
    };
    let bf_t = Tensor::<B, 3>::from_data(Data::new(bf.clone(), Shape::new([1, n_nodes, 3])), &dev);
    let bm_t = Tensor::<B, 3>::from_data(Data::new(bm.clone(), Shape::new([1, n_nodes, 3])), &dev);
    let qctx = DesignQueryContext {
        seed: 42,
        compliance_ctx: &ctx,
        penalization_optimizer: CompliancePenalization::Gate(3.0),
        penalization_gate: CompliancePenalization::Gate(3.0),
        representation: &rep,
        body_force: bf_t,
        boundary_mask: bm_t,
        old_density: Tensor::<B, 1>::ones([1], &dev) * 2400.0,
        new_density: Tensor::<B, 1>::ones([1], &dev) * 2400.0,
        old_free_energy: Tensor::<B, 1>::ones([1], &dev) * -1.0e5,
        new_free_energy: Tensor::<B, 1>::ones([1], &dev) * -1.1e5,
        dt_s: Tensor::<B, 1>::ones([1], &dev),
    };
    let q = StructuralDesignQuery;
    let r1 = q.query_v0(&qctx, &latent, coords_t.clone()).expect("q1");
    let r2 = q.query_v0(&qctx, &latent, coords_t).expect("q2");
    assert_eq!(r1.witness.seed, r2.witness.seed);
    assert_eq!(r1.witness.repr_id, r2.witness.repr_id);
    assert!((r1.metrics.compliance_gate - r2.metrics.compliance_gate).abs() < 1e-5);
    assert!((r1.margin.value() - r2.margin.value()).abs() < 1e-5);
}
