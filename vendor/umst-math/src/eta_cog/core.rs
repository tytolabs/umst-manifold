//! Engineering mirror of `umst-formal/Lean/EtaCog.lean`.

use crate::dignity::dignity_step;
use crate::landauer::K_B;

/// One cockpit step: temperature (K) and nonnegative MI / energy deltas (bits / joules).
#[derive(Clone, Debug, PartialEq)]
pub struct EtaCogClaim {
    /// Bath temperature (K); must be > 0 for a nonzero Landauer floor.
    pub temperature_k: f64,
    /// Claimed epistemic MI gain (bits), nonnegative.
    pub delta_mi_bits: f64,
    /// Measured or attributed energy expenditure (J), nonnegative.
    pub delta_energy_j: f64,
}

#[inline]
fn landauer_floor_j(temperature_k: f64) -> f64 {
    K_B * temperature_k.max(0.0) * std::f64::consts::LN_2
}

/// η_cog = dignity · ΔMI / (ΔE + k_B T ln 2) — denominator case **(i)** (COCKPIT_DESIGN_BRIEF §5).
///
/// Proof: `UMST.Formal.EtaCog::eta_cog_nonneg` (Zenodo **10.5281/zenodo.19159660**).
#[must_use]
pub fn eta_cog(dignity_value: f64, c: &EtaCogClaim) -> f64 {
    let lb = landauer_floor_j(c.temperature_k);
    let denom = c.delta_energy_j + lb;
    if !(c.temperature_k > 0.0
        && c.delta_mi_bits >= 0.0
        && c.delta_energy_j >= 0.0
        && dignity_value >= 0.0
        && denom > 0.0)
    {
        return 0.0;
    }
    dignity_value * c.delta_mi_bits / denom
}

/// η_cog using dignity **after** a Landauer-gated `dignity_step` (dishonest branch freezes).
///
/// Proof: `UMST.Formal.EtaCog::eta_cog_frozen_under_dishonest_claim`.
#[must_use]
pub fn eta_cog_after_dishonest_dignity_step(
    temperature_k: f64,
    current_dignity: f64,
    dignity_claim_mi: f64,
    dignity_claim_e: f64,
    cockpit: &EtaCogClaim,
) -> f64 {
    let d2 = dignity_step(
        temperature_k,
        current_dignity,
        dignity_claim_mi,
        dignity_claim_e,
    );
    eta_cog(d2, cockpit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dignity::honest_spend;

    fn claim(t: f64, mi: f64, e: f64) -> EtaCogClaim {
        EtaCogClaim {
            temperature_k: t,
            delta_mi_bits: mi,
            delta_energy_j: e,
        }
    }

    #[test]
    fn eta_cog_nonneg_interior() {
        let c = claim(300.0, 0.5, 1e-18);
        let y = eta_cog(2.0, &c);
        assert!(y >= 0.0 && y.is_finite());
    }

    #[test]
    fn eta_cog_zero_energy_matches_floor_ratio() {
        let t = 300.0;
        let lb = landauer_floor_j(t);
        let c = claim(t, 0.25, 0.0);
        let d = 4.0;
        let y = eta_cog(d, &c);
        assert!((y - d * 0.25 / lb).abs() < 1e-24);
    }

    #[test]
    fn eta_cog_monotone_in_dignity() {
        let c = claim(290.0, 0.3, 1e-19);
        let y1 = eta_cog(1.0, &c);
        let y2 = eta_cog(3.0, &c);
        assert!(y1 <= y2 + 1e-15);
    }

    #[test]
    fn eta_cog_monotone_in_mi() {
        let c1 = claim(295.0, 0.1, 2e-19);
        let c2 = claim(295.0, 0.4, 2e-19);
        let d = 2.5;
        assert!(eta_cog(d, &c1) <= eta_cog(d, &c2) + 1e-15);
    }

    #[test]
    fn eta_cog_antitone_in_energy() {
        let c_lo = claim(301.0, 0.2, 0.0);
        let c_hi = claim(301.0, 0.2, 5e-18);
        let d = 3.0;
        assert!(eta_cog(d, &c_hi) <= eta_cog(d, &c_lo) + 1e-15);
    }

    #[test]
    fn eta_cog_frozen_under_dishonest_dignity_step() {
        let t = 300.0;
        let cur = 2.0;
        let mi = 10.0;
        let e = 1e-25;
        assert!(!honest_spend(t, mi, e));
        let cockpit = claim(t, 0.15, landauer_floor_j(t) * 0.15 + 1e-20);
        let y0 = eta_cog(cur, &cockpit);
        let y1 = eta_cog_after_dishonest_dignity_step(t, cur, mi, e, &cockpit);
        assert!((y0 - y1).abs() < 1e-18);
    }

    #[test]
    fn eta_cog_invalid_inputs_zero() {
        let c = claim(-1.0, 1.0, 1.0);
        assert_eq!(eta_cog(1.0, &c), 0.0);
    }
}
