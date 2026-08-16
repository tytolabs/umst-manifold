// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! GateCartridge-only stub (Phase B).

use umst_manifold::core::traits::GateCartridge;

#[derive(Debug, Clone, Copy, Default)]
pub struct GateOnlySemanticStub;

impl GateCartridge for GateOnlySemanticStub {
    fn provides_spatial_physics(&self) -> bool {
        false
    }
}

#[test]
fn gate_cartridge_only_stub_does_not_claim_spatial() {
    let stub = GateOnlySemanticStub;
    assert!(!stub.provides_spatial_physics());
}
