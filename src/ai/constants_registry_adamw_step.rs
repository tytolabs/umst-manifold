// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! AdamW optimizer coefficient registry for liquid-PPO policy steps.
//!
//! The AdamW coefficients are the Burn AdamW defaults, recorded as training choices (Policy).

use crate::constants_registry::{Derivation, GroundedConst};

/// Reference substrate density for constraint-penalty tensor fills (kg/m³); the penalty runs only
/// in the reward-shaping lanes (`epistemic-ppo`, `kleisli-ppo-hot-bind`).
#[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
pub const LIQUID_PPO_SUBSTRATE_REFERENCE_DENSITY_KG_M3: GroundedConst<f32> = GroundedConst {
    name: "liquid_ppo_substrate_reference_density_kg_m3",
    value: crate::gate::transition_proposal::SUBSTRATE_REFERENCE_DENSITY_KG_M3.value as f32,
    evidence: "gate SUBSTRATE_REFERENCE_DENSITY_KG_M3 in f32 fills",
    derivation: Some(Derivation::Absent {
        reason: "reads gate_substrate_reference_density_kg_m3, which no source or measurement grounds; follow-up W-119 MANIFOLD-GROUNDEDCONST-DERIVATION",
    }),
};

/// AdamW first-moment decay β₁.
pub const ADAMW_BETA1_COEFF: GroundedConst<f32> = GroundedConst {
    name: "adamw_beta1_coeff",
    value: 0.9,
    evidence: "Burn AdamW default β₁",
    derivation: Some(Derivation::Policy {
        rationale: "first-moment decay of the AdamW policy optimiser, the Burn AdamW default; a training choice",
    }),
};

/// AdamW second-moment decay β₂.
pub const ADAMW_BETA2_COEFF: GroundedConst<f32> = GroundedConst {
    name: "adamw_beta2_coeff",
    value: 0.999,
    evidence: "Burn AdamW default β₂",
    derivation: Some(Derivation::Policy {
        rationale: "second-moment decay of the AdamW policy optimiser, the Burn AdamW default; a training choice",
    }),
};

/// AdamW ε stabilizer.
pub const ADAMW_EPSILON: GroundedConst<f32> = GroundedConst {
    name: "adamw_epsilon",
    value: 1e-5,
    evidence: "Burn AdamW default ε",
    derivation: Some(Derivation::Policy {
        rationale:
            "denominator guard of the AdamW update, the Burn AdamW default; a numerical choice",
    }),
};

/// AdamW decoupled weight decay.
pub const ADAMW_WEIGHT_DECAY: GroundedConst<f32> = GroundedConst {
    name: "adamw_weight_decay",
    value: 1e-4,
    evidence: "Burn AdamW default weight decay",
    derivation: Some(Derivation::Policy {
        rationale: "decoupled weight decay of the AdamW policy optimiser, the Burn AdamW default; a training choice",
    }),
};

#[cfg(test)]
mod adamw_step_coeffs_match_burn_defaults {}
