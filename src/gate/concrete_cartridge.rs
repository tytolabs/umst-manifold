// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Host-side policy defaults aligned with **`umst-concrete-cartridge`** Powers / D1 calibration
//! when the Burn cartridge is not linked into the `gate_server` binary.

use super::evaluator::GateEvaluator;
use super::http_manifest::{default_gate_manifest, GateManifest};

/// Zero-sized marker: HTTP gate uses cartridge-aligned literals without pulling `burn`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ConcreteCartridge;

impl ConcreteCartridge {
    /// Powers closure defaults (prototype `PhysicsConfig::default` / UCI D1 **`s_intrinsic`**).
    #[must_use]
    pub fn default_gate_manifest() -> GateManifest {
        default_gate_manifest()
    }
}

impl GateEvaluator for ConcreteCartridge {
    fn catalog_id(&self) -> &'static str {
        "umst.cartridge.concrete.policy"
    }

    fn gate_family(&self) -> &'static str {
        "concrete_powers_manifest_defaults"
    }
}
