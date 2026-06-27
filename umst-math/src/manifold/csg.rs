//! Combinatorics: hard CSG and smooth min (Haskell `SDFGate` discipline).

// SPDX-License-Identifier: MIT
// Algebra reference: umst-formal/Haskell/SDFGate.hs (intersectSDF, rUnionSDF, smoothUnionSDF)

const CSG_SMOOTH_K: f64 = 0.05; // see `manifold_csg_smooth_k_default` in REGISTRY (Tier-3 policy)

/// Hard / exact CSG **union** for SDFs (outside-positive convention): `max(f,g)`.
#[inline]
pub fn hard_union(a: f64, b: f64) -> f64 {
    a.max(b)
}

/// Hard / exact CSG **intersection**: `min(f,g)`.
#[inline]
pub fn hard_intersection(a: f64, b: f64) -> f64 {
    a.min(b)
}

/// Quilez polynomial `smoothMin` (SDFGate.smoothUnionSDF) with blend parameter `k > 0`.
/// As `k → 0^+`, converges to `min` (I2: monoid literature uses smooth appx for union; here the signed field follows Haskell).
pub fn smooth_min(a: f64, b: f64, k: f64) -> f64 {
    let k = k.max(1e-12);
    let h = (k - (a - b).abs()).max(0.0) / k;
    a.min(b) - h * h * h * k / 6.0
}

/// Default smoothness from REGISTRY (Tier-3 policy placeholder; B-Arc may tune).
pub fn default_smooth_k() -> f64 {
    CSG_SMOOTH_K
}

/// 1D Helmholtz SDF from `umst-formal/Haskell/SDFGate.hs` — `-(q * α)`.
pub fn helmholtz_sdf_1d(alpha: f64, q_hyd: f64) -> f64 {
    -(q_hyd * alpha)
}

/// Constant `∂ψ/∂α` from SDFGate (`helmholtzGradient` = -q_hyd).
pub fn helmholtz_gradient(q_hyd: f64) -> f64 {
    -q_hyd
}

// --- UMST gate SDF (ThermodynamicState product space; sign agrees with SDFGate.hs) ---

/// Haskell `qHydration` reference for SDF gate parity (formal layer; not W9 cement SSOT).
pub const Q_HYDRATION_J_PER_KG: f64 = 4.5e2;
const Q_HYDR: f64 = Q_HYDRATION_J_PER_KG;
const M_TOL: f64 = 100.0;

/// `ThermodynamicState` mirror for pure gate SDF only (5 fields, Haskell `UMST.ThermodynamicState`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermoGateState {
    /// ρ kg/m³
    pub density: f64,
    /// ψ J/kg
    pub free_energy: f64,
    /// α
    pub hydration: f64,
    /// f_c MPa
    pub strength: f64,
    /// f_c max MPa
    pub max_strength: f64,
}

pub fn mass_conservation_sdf(old: &ThermoGateState, new: &ThermoGateState) -> f64 {
    (new.density - old.density).abs() - M_TOL
}

pub fn clausius_duhem_sdf(old: &ThermoGateState, new: &ThermoGateState) -> f64 {
    new.free_energy - old.free_energy
}

/// Clausius–Duhem admissibility conjunct (`Gate.lean`: `new.freeEnergy ≤ old.freeEnergy`).
///
/// Parametric over any scalar ψ (material free energy, clock desync energy, etc.).
#[must_use]
pub fn clausius_duhem_admissible(old_free_energy: f64, new_free_energy: f64) -> bool {
    new_free_energy <= old_free_energy
}

pub fn hydration_irreversibility_sdf(old: &ThermoGateState, new: &ThermoGateState) -> f64 {
    old.hydration - new.hydration
}

pub fn strength_monotonicity_sdf(old: &ThermoGateState, new: &ThermoGateState) -> f64 {
    old.strength - new.strength
}

/// `gateSDF` = `maximum` of the four (SDFGate.gateSDF).
pub fn gate_sdf(old: &ThermoGateState, new: &ThermoGateState) -> f64 {
    hard_union(
        mass_conservation_sdf(old, new),
        hard_union(
            clausius_duhem_sdf(old, new),
            hard_union(
                hydration_irreversibility_sdf(old, new),
                strength_monotonicity_sdf(old, new),
            ),
        ),
    )
}

/// `helmholtzSDF` at α with canonical `Q_hyd` from `UMST.qHydration`.
pub fn umst_helmholtz_sdf(alpha: f64) -> f64 {
    helmholtz_sdf_1d(alpha, Q_HYDR)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_associates_three() {
        let a: f64 = 0.1;
        let b: f64 = 0.2;
        let c: f64 = 0.15;
        let u1 = hard_union(a, hard_union(b, c));
        let u2 = hard_union(hard_union(a, b), c);
        assert!(
            (u1 - u2).abs() < 1e-15,
            "I2 monoid: associativity of max on f64"
        );
    }
}
