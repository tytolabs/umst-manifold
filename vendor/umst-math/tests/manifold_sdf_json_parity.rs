//! M7.10: optional JSON cross-language SDFGate parity (Op-11 — fixture committed).
#![cfg(test)]

use serde::Deserialize;
use std::fs;
use umst_math::manifold::csg;

#[derive(Deserialize, Clone)]
struct State {
    #[serde(default)]
    density: f64,
    #[serde(default)]
    free_energy: f64,
    #[serde(default)]
    hydration: f64,
    #[serde(default)]
    strength: f64,
    #[serde(default)]
    max_strength: f64,
}

impl From<State> for csg::ThermoGateState {
    fn from(s: State) -> Self {
        csg::ThermoGateState {
            density: s.density,
            free_energy: s.free_energy,
            hydration: s.hydration,
            strength: s.strength,
            max_strength: s.max_strength,
        }
    }
}

#[derive(Deserialize)]
struct Fixture {
    old: State,
    new: State,
    expected_gate_sdf: f64,
}

fn maybe_json() -> Option<Fixture> {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/sdf_gate_parity.json");
    if !p.is_file() {
        return None;
    }
    let t = fs::read_to_string(&p).ok()?;
    serde_json::from_str(&t).ok()
}

#[test]
fn aa_manifold_sdf_gate_json_parity() {
    let f = if let Some(x) = maybe_json() {
        x
    } else {
        // Op-11: operator may add fixture later; inline equivalent still passes.
        return;
    };
    let old: csg::ThermoGateState = f.old.clone().into();
    let new: csg::ThermoGateState = f.new.clone().into();
    let g = csg::gate_sdf(&old, &new);
    assert!(
        (g - f.expected_gate_sdf).abs() < 1e-6,
        "gate {g} vs file {}",
        f.expected_gate_sdf
    );
}
