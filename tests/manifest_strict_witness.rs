// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! R5 v1 release profile: [`GroundingContract::StrictCatalogMatch`] + `formal-witness` digest reject.

use burn::tensor::backend::Backend;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::ai::formal::FormalReject;
use umst_manifold::ai::ppo::ManifoldGateway;
use umst_manifold::core::tensors::UnifiedMaterialStateTensor;
use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
use umst_manifold::manifest::{GroundingContract, UmstManifestBuilder};
use umst_manifold::runtime::catalog::lock_upstream_catalog_digest_bytes;

type B = NdArray<f32>;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

fn tiny_umst(digest: Option<[u8; 32]>) -> UnifiedMaterialStateTensor<B> {
    let dev = device();
    let n = 2usize;
    let f = 5usize;
    let coords: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &dev);
    let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
        &dev,
    );
    let faces_b2: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &dev);
    UnifiedMaterialStateTensor {
        coords,
        edges_b1,
        faces_b2,
        scalar_features: Tensor::<B, 2>::zeros([n, f], &dev),
        vector_features: Tensor::<B, 3>::zeros([n, 1, 3], &dev),
        matrix_features: Tensor::<B, 4>::zeros([n, 1, 3, 3], &dev),
        resolution_mm: [1.0, 1.0, 1.0],
        node_positions: None,
        displacement_bc_mask: Tensor::<B, 3>::ones([1, n, 3], &dev),
        policy_editable_mask: Tensor::<B, 2>::ones([n, 1], &dev),
        catalog_schema_digest: digest,
    }
}

struct GatewayStubCartridge;

impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for GatewayStubCartridge {
    fn compute_all(
        &self,
        mix: &umst_manifold::core::tensors::MixTensor<Bk>,
    ) -> umst_manifold::core::traits::PhysicalResult<Bk> {
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

#[test]
fn release_manifest_strict_pins_lock_digest() {
    let manifest = UmstManifestBuilder::default().for_release_witness().build();
    assert_eq!(
        manifest.grounding_contract,
        GroundingContract::StrictCatalogMatch
    );
    assert_eq!(manifest.catalog_hash, lock_upstream_catalog_digest_bytes());
    assert_eq!(
        manifest.witness_catalog_digest(),
        Some(lock_upstream_catalog_digest_bytes())
    );
}

#[test]
fn gateway_new_auto_pins_lock_digest_without_manual_wiring() {
    let gateway: ManifoldGateway<B, GatewayStubCartridge> =
        ManifoldGateway::new(GatewayStubCartridge, 300.0_f64, 1.0e-12_f64);
    assert_eq!(
        gateway.expected_catalog_schema_digest,
        Some(lock_upstream_catalog_digest_bytes())
    );
}

#[test]
fn strict_manifest_matching_digest_accepts_topology_step() {
    let staging = UmstManifestBuilder::default().for_staging().build();
    let release = UmstManifestBuilder::default().for_release_witness().build();
    assert_ne!(
        release.grounding_contract, staging.grounding_contract,
        "staging must differ from release profile"
    );

    let lock = lock_upstream_catalog_digest_bytes();
    let mut gateway = ManifoldGateway::new(GatewayStubCartridge, 300.0_f64, 1.0e-12_f64);

    let umst = tiny_umst(Some(lock)).with_lock_catalog_schema_digest();
    let info_gain = Tensor::<B, 1>::zeros([1], &device());
    let result = gateway.evaluate_topology_step_formal(umst, info_gain);
    assert!(
        result.is_ok(),
        "matching digest should pass witness: {:?}",
        result.err()
    );
}

#[test]
fn strict_manifest_digest_mismatch_rejects() {
    let release = UmstManifestBuilder::default().for_release_witness().build();
    let mut wrong = lock_upstream_catalog_digest_bytes();
    wrong[0] ^= 0xff;

    let mut gateway = ManifoldGateway::new(GatewayStubCartridge, 300.0_f64, 1.0e-12_f64);
    release.apply_witness_to_gateway(&mut gateway);

    let umst = tiny_umst(Some(wrong));
    let info_gain = Tensor::<B, 1>::zeros([1], &device());
    let Err(err) = gateway.evaluate_topology_step_formal(umst, info_gain) else {
        panic!("mismatched catalog_schema_digest must reject");
    };

    match err {
        FormalReject::CatalogSchemaDigestMismatch { expected, observed } => {
            assert_eq!(expected, lock_upstream_catalog_digest_bytes());
            assert_eq!(observed, wrong);
        }
        other => panic!("expected CatalogSchemaDigestMismatch, got {other:?}"),
    }
    assert_eq!(err.catalog_id(), "umst.formal.catalog_lock");
}
