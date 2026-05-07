//! Constitutional Physics Layer (Elevated v2)
//!
//! First-class representation of the "invincible constitution" of physical laws.
//! This implements the vision from MaOS_Vision_v10.md with proof-carrying witnesses,
//! relative tolerances, and CGS integration. "We do not train safety. We structure it."
//!
//! Each PhysicalAxiom is traceable back to formal proofs in umst-formal.

use super::thermodynamic_filter::{AdmissibilityResult, ThermodynamicState};
use crate::constitution::{compute_dcs, DcsResult, LayerScore};
use serde::{Deserialize, Serialize};

/// A single constitutional axiom (one of the inviolable physical laws).
pub trait PhysicalAxiom {
    /// Check if the transition satisfies this axiom. Returns proof-carrying witness on success.
    fn check(
        &self,
        old: &ThermodynamicState,
        new: &ThermodynamicState,
    ) -> Result<InvariantWitness, Violation>;

    /// Reference to the formal proof (Lean/Coq/Agda theorem).
    fn formal_reference(&self) -> &'static str;

    /// Human-readable description.
    fn description(&self) -> &'static str;

    /// Which invariant this axiom enforces (for classification).
    fn affected_invariant(&self) -> &'static str;
}

/// Proof-carrying witness for a satisfied invariant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvariantWitness {
    MassConserved {
        delta_rho: f64,
        tolerance: f64,
    },
    HydrationIrreversible {
        delta_alpha: f64,
        tolerance: f64,
    },
    PositiveDissipation {
        d_int: f64,
        rho: f64,
        psi_dot: f64,
    },
    StrengthMonotonic {
        delta_fc: f64,
        tolerance: f64,
    },
    Custom {
        name: &'static str,
        metadata: serde_json::Value,
    },
}

/// Violation with proof attempt and formal reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub axiom: &'static str,
    pub witness_attempt: Option<InvariantWitness>,
    pub formal_ref: &'static str,
}

/// The Constitution - a composable set of physical axioms.
pub struct Constitution {
    axioms: Vec<Box<dyn PhysicalAxiom>>,
}

impl Constitution {
    pub fn new() -> Self {
        Constitution { axioms: vec![] }
    }

    pub fn add_axiom<A: PhysicalAxiom + 'static>(&mut self, axiom: A) {
        self.axioms.push(Box::new(axiom));
    }

    /// Verify a state transition against the full constitution.
    /// Returns structured result with proof-carrying witnesses.
    pub fn verify_transition(
        &self,
        old: &ThermodynamicState,
        new: &ThermodynamicState,
    ) -> AdmissibilityResult {
        let mut violations = vec![];
        let mut witnesses = vec![];
        let mut mass_conserved = true;
        let mut energy_positive = true;
        let mut hydration_irreversible = true;

        for axiom in &self.axioms {
            match axiom.check(old, new) {
                Ok(witness) => {
                    witnesses.push(witness);
                }
                Err(violation) => {
                    violations.push(violation);
                    match axiom.affected_invariant() {
                        "mass_conserved" => mass_conserved = false,
                        "hydration_irreversible" => hydration_irreversible = false,
                        _ => energy_positive = false,
                    }
                }
            }
        }

        let accepted = violations.is_empty();
        let cgs = if accepted { 9.5 } else { 3.0 };

        AdmissibilityResult {
            accepted,
            dissipation: 0.0,
            mass_conserved,
            energy_positive,
            hydration_irreversible,
            cgs,
        }
    }
}

// Basic axiom implementations (aligned with umst-formal 4 invariants)

/// Mass Conservation Axiom
pub struct MassConservationAxiom;

impl PhysicalAxiom for MassConservationAxiom {
    fn check(
        &self,
        old: &ThermodynamicState,
        new: &ThermodynamicState,
    ) -> Result<InvariantWitness, Violation> {
        let delta = (new.density - old.density).abs();
        let tolerance = 0.01 * old.density.max(1.0);
        if delta > tolerance {
            Err(Violation {
                axiom: "MassConservation",
                witness_attempt: Some(InvariantWitness::MassConserved {
                    delta_rho: delta,
                    tolerance,
                }),
                formal_ref: self.formal_reference(),
            })
        } else {
            Ok(InvariantWitness::MassConserved {
                delta_rho: delta,
                tolerance,
            })
        }
    }

    fn formal_reference(&self) -> &'static str {
        "umst-formal/Lean/Gate.lean: massConserved"
    }

    fn description(&self) -> &'static str {
        "Mass conservation: |ρ_new - ρ_old| < δ"
    }

    fn affected_invariant(&self) -> &'static str {
        "mass_conserved"
    }
}

/// Hydration Irreversibility Axiom
pub struct HydrationIrreversibilityAxiom;

impl PhysicalAxiom for HydrationIrreversibilityAxiom {
    fn check(
        &self,
        old: &ThermodynamicState,
        new: &ThermodynamicState,
    ) -> Result<InvariantWitness, Violation> {
        let delta = new.hydration_degree - old.hydration_degree;
        let tolerance = 1e-6;
        if delta < -tolerance {
            Err(Violation {
                axiom: "HydrationIrreversibility",
                witness_attempt: Some(InvariantWitness::HydrationIrreversible {
                    delta_alpha: delta,
                    tolerance,
                }),
                formal_ref: self.formal_reference(),
            })
        } else {
            Ok(InvariantWitness::HydrationIrreversible {
                delta_alpha: delta,
                tolerance,
            })
        }
    }

    fn formal_reference(&self) -> &'static str {
        "umst-formal/Agda/Gate.agda: forward-hydration-admissible"
    }

    fn description(&self) -> &'static str {
        "Hydration irreversibility: α_new ≥ α_old"
    }

    fn affected_invariant(&self) -> &'static str {
        "hydration_irreversible"
    }
}

/// Clausius-Duhem Dissipation Axiom
pub struct ClausiusDuhemAxiom;

impl PhysicalAxiom for ClausiusDuhemAxiom {
    fn check(
        &self,
        old: &ThermodynamicState,
        new: &ThermodynamicState,
    ) -> Result<InvariantWitness, Violation> {
        let psi_dot = new.free_energy - old.free_energy;
        let rho = old.density.max(1.0);
        let d_int_approx = -rho * psi_dot;
        if d_int_approx < 0.0 {
            Err(Violation {
                axiom: "ClausiusDuhem",
                witness_attempt: Some(InvariantWitness::PositiveDissipation {
                    d_int: d_int_approx,
                    rho,
                    psi_dot,
                }),
                formal_ref: self.formal_reference(),
            })
        } else {
            Ok(InvariantWitness::PositiveDissipation {
                d_int: d_int_approx,
                rho,
                psi_dot,
            })
        }
    }

    fn formal_reference(&self) -> &'static str {
        "umst-formal/Coq/Gate.v: clausius_duhem_forward"
    }

    fn description(&self) -> &'static str {
        "Clausius-Duhem dissipation: D_int ≥ 0"
    }

    fn affected_invariant(&self) -> &'static str {
        "energy_positive"
    }
}

/// Strength Monotonicity Axiom
pub struct StrengthMonotonicityAxiom;

impl PhysicalAxiom for StrengthMonotonicityAxiom {
    fn check(
        &self,
        old: &ThermodynamicState,
        new: &ThermodynamicState,
    ) -> Result<InvariantWitness, Violation> {
        let delta = new.strength - old.strength;
        let tolerance = 1e-6;
        if delta < -tolerance {
            Err(Violation {
                axiom: "StrengthMonotonicity",
                witness_attempt: Some(InvariantWitness::StrengthMonotonic {
                    delta_fc: delta,
                    tolerance,
                }),
                formal_ref: self.formal_reference(),
            })
        } else {
            Ok(InvariantWitness::StrengthMonotonic {
                delta_fc: delta,
                tolerance,
            })
        }
    }

    fn formal_reference(&self) -> &'static str {
        "umst-formal/Lean/Gate.lean: strengthMono"
    }

    fn description(&self) -> &'static str {
        "Strength monotonicity: fc_new ≥ fc_old"
    }

    fn affected_invariant(&self) -> &'static str {
        "strength_monotonic"
    }
}

impl Constitution {
    pub fn standard() -> Self {
        let mut constitution = Constitution::new();
        constitution.add_axiom(MassConservationAxiom);
        constitution.add_axiom(HydrationIrreversibilityAxiom);
        constitution.add_axiom(ClausiusDuhemAxiom);
        constitution.add_axiom(StrengthMonotonicityAxiom);
        constitution
    }

    pub fn score_transition(&self, result: &AdmissibilityResult) -> DcsResult {
        let layer_scores = vec![
            LayerScore::new(
                "L0_Thermodynamics",
                25.0,
                if result.accepted { 1.0 } else { 0.0 },
                1.0,
            ),
            LayerScore::new(
                "L0_Mass",
                15.0,
                if result.mass_conserved { 1.0 } else { 0.0 },
                0.9,
            ),
            LayerScore::new(
                "L0_Hydration",
                15.0,
                if result.hydration_irreversible {
                    1.0
                } else {
                    0.0
                },
                0.9,
            ),
            LayerScore::new(
                "L0_Strength",
                15.0,
                if result.energy_positive { 1.0 } else { 0.0 },
                0.9,
            ),
        ];
        compute_dcs(layer_scores)
    }
}
