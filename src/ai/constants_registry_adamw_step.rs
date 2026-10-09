// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! AdamW optimizer coefficient registry for liquid-PPO policy steps.
//!
//! Values mirror Burn AdamW defaults. Fixture SSOT only — not physics GREEN.

use crate::constants_registry::GroundedConst;

/// Reference substrate density for constraint-penalty tensor fills (kg/m³); the penalty runs only
/// in the reward-shaping lanes (`epistemic-ppo`, `kleisli-ppo-hot-bind`).
#[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
pub const LIQUID_PPO_SUBSTRATE_REFERENCE_DENSITY_KG_M3: GroundedConst<f32> = GroundedConst {
    name: "liquid_ppo_substrate_reference_density_kg_m3",
    value: 2400.0,
    evidence: "aligns with gate SUBSTRATE_REFERENCE_DENSITY_KG_M3 in f32 fills",
};

/// AdamW first-moment decay β₁.
pub const ADAMW_BETA1_COEFF: GroundedConst<f32> = GroundedConst {
    name: "adamw_beta1_coeff",
    value: 0.9,
    evidence: "Burn AdamW default β₁",
};

/// AdamW second-moment decay β₂.
pub const ADAMW_BETA2_COEFF: GroundedConst<f32> = GroundedConst {
    name: "adamw_beta2_coeff",
    value: 0.999,
    evidence: "Burn AdamW default β₂",
};

/// AdamW ε stabilizer.
pub const ADAMW_EPSILON: GroundedConst<f32> = GroundedConst {
    name: "adamw_epsilon",
    value: 1e-5,
    evidence: "Burn AdamW default ε",
};

/// AdamW decoupled weight decay.
pub const ADAMW_WEIGHT_DECAY: GroundedConst<f32> = GroundedConst {
    name: "adamw_weight_decay",
    value: 1e-4,
    evidence: "Burn AdamW default weight decay",
};

#[cfg(test)]
mod adamw_step_coeffs_match_burn_defaults {}
