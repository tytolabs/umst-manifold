// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! R1b: optimizer ≡ readout ≡ gate audit on one [`ComplianceFunctional`] kernel.

#![cfg(feature = "mechanics-adjoint-q1-hex")]
#![allow(clippy::too_many_arguments)]

use burn::backend::Autodiff;
use burn::tensor::{backend::AutodiffBackend, Data, Shape, Tensor};
use burn_ndarray::NdArray;
use rand::{rngs::StdRng, Rng, SeedableRng};

use umst_manifold::physics::adjoint::SimpElasticMaterial;
use umst_manifold::physics::adjoint_q1_hex::AdjointComplianceQ1Hex;
use umst_manifold::physics::compliance_functional::{
    ComplianceContext, ComplianceFunctional, ComplianceHostInput, CompliancePenalization,
    Q1HexBrickSpec, Q1HexComplianceFunctional,
};
use umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type AD = Autodiff<NdArray<f32>>;
type Inner = <AD as AutodiffBackend>::InnerBackend;

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
fn compliance_functional_identity_optimizer_readout_gate() {
    let nx = 4_usize;
    let ny = 4_usize;
    let nz = 1_usize;
    let plate = ExtrudedPlateMechanics {
        nx,
        ny,
        nz,
        dx: 0.25,
        dy: 0.25,
        dz: 0.05,
    };
    let n_nodes = (nx + 1) * (ny + 1) * (nz + 1);
    let mut rng = StdRng::seed_from_u64(42);
    let rho_flat: Vec<f32> = (0..n_nodes).map(|_| rng.gen_range(0.3_f32..0.9)).collect();

    let mut bf = vec![0.0_f32; n_nodes * 3];
    let top_nid = nx / 2 + (ny / 2) * (nx + 1) + nz * (nx + 1) * (ny + 1);
    bf[top_nid * 3 + 2] = -1.0;
    let bm = plate_bottom_uz_mask(nx, ny, nz);

    let cg = MechanicsInnerLoopConfig::default();
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
            dx: plate.dx,
            dy: plate.dy,
            dz: plate.dz,
        },
        cg: cg.clone(),
        self_weight: None,
    };

    let dev = <Inner as burn::tensor::backend::Backend>::Device::default();
    let rho_ad: Tensor<AD, 3> = Tensor::from_data(
        Data::new(rho_flat.clone(), Shape::new([1, n_nodes, 1])),
        &dev,
    );
    let bf_t: Tensor<Inner, 3> =
        Tensor::from_data(Data::new(bf.clone(), Shape::new([1, n_nodes, 3])), &dev);
    let bm_t: Tensor<Inner, 3> =
        Tensor::from_data(Data::new(bm.clone(), Shape::new([1, n_nodes, 3])), &dev);

    let penalization_opt = CompliancePenalization::Schedule {
        outer: 20,
        total: 200,
    };
    let penalization_gate = CompliancePenalization::Gate(3.0);

    let (surrogate, value_opt) = Q1HexComplianceFunctional
        .eval_autodiff(
            &ctx,
            rho_ad.clone(),
            bf_t.clone(),
            bm_t.clone(),
            penalization_opt,
        )
        .expect(
            "Q1HexComplianceFunctional::eval_autodiff optimizer penalization schedule on Q1 hex plate (FP §6 Track A4 solid elasticity harness)",
        );
    let c_surrogate: f32 = surrogate.into_scalar();

    let inner_opt = Q1HexComplianceFunctional
        .eval_inner(
            &ctx,
            ComplianceHostInput {
                rho_flat: &rho_flat,
                body_force: &bf,
                boundary_mask: &bm,
                penalization: penalization_opt,
            },
        )
        .expect(
            "Q1HexComplianceFunctional::eval_inner optimizer-mode parity vs autodiff surrogate (FP §6 Track A4 solid elasticity harness)",
        );

    let inner_gate = Q1HexComplianceFunctional
        .eval_inner(
            &ctx,
            ComplianceHostInput {
                rho_flat: &rho_flat,
                body_force: &bf,
                boundary_mask: &bm,
                penalization: penalization_gate,
            },
        )
        .expect(
            "Q1HexComplianceFunctional::eval_inner gate penalization vs legacy raw_compliance (FP §6 Track A4 solid elasticity harness)",
        );

    let legacy_gate = AdjointComplianceQ1Hex::raw_compliance_at_rho(
        &rho_flat,
        nx,
        ny,
        nz,
        plate.dx,
        plate.dy,
        plate.dz,
        &bf,
        &bm,
        SimpElasticMaterial {
            p: 3.0,
            ..ctx.material
        },
        &cg,
        None,
    );

    let eps = 1e-4_f32;
    assert!(
        (c_surrogate - value_opt.c_raw).abs() < eps,
        "surrogate {c_surrogate} != c_raw {}",
        value_opt.c_raw
    );
    assert!(
        (inner_opt.c_raw - value_opt.c_raw).abs() < eps,
        "optimizer inner {} != autodiff {}",
        inner_opt.c_raw,
        value_opt.c_raw
    );
    assert!(
        (inner_gate.c_raw - legacy_gate).abs() < eps,
        "gate functional {} != legacy raw_compliance {}",
        inner_gate.c_raw,
        legacy_gate
    );
    assert!(
        (inner_gate.penalization_p - 3.0).abs() < 1e-6,
        "gate p must be 3.0, got {}",
        inner_gate.penalization_p
    );
    assert!(
        (inner_opt.penalization_p - penalization_opt.resolve_p()).abs() < 1e-6,
        "optimizer p mismatch"
    );
}
