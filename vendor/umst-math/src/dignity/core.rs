//! Engineering mirror of `umst-formal/Lean/Dignity.lean`.

use crate::landauer::K_B;

/// Operator UX upper bound (egoff `dignity_scalar` scale, `10 · RCC`).
pub const D_MAX: f64 = 10.0;

#[inline]
fn landauer_joules_per_bit(temperature_k: f64) -> f64 {
    K_B * temperature_k.max(0.0) * std::f64::consts::LN_2
}

/// Landauer honest-spend predicate: `k_B T ln 2 * ΔMI ≤ ΔE`.
#[must_use]
pub fn honest_spend(temperature_k: f64, delta_mi_bits: f64, delta_energy_j: f64) -> bool {
    landauer_joules_per_bit(temperature_k) * delta_mi_bits <= delta_energy_j
}

/// `Some v` iff `v ∈ [0, D_MAX]`, else `None` (Lean `tryDignity`).
#[must_use]
pub fn try_dignity(value: f64) -> Option<f64> {
    if (0.0..=D_MAX).contains(&value) {
        Some(value)
    } else {
        None
    }
}

/// One gated dignity update (Lean `dignity_step` honest branch shape).
///
/// Proof: `UMST.Formal.Dignity::dignity_monotone_under_mi_gain` (Zenodo **10.5281/zenodo.19159660**).
#[must_use]
pub fn dignity_step(
    temperature_k: f64,
    current_dignity: f64,
    delta_mi_bits: f64,
    delta_energy_j: f64,
) -> f64 {
    if honest_spend(temperature_k, delta_mi_bits, delta_energy_j) {
        (current_dignity + delta_mi_bits).min(D_MAX)
    } else {
        current_dignity
    }
}

/// Proof: `UMST.Formal.Dignity::dignity_monotone_under_mi_gain` (same DOI family).
#[must_use]
pub fn dignity_monotone_under_mi_gain_check(
    temperature_k: f64,
    current: f64,
    mi1: f64,
    e1: f64,
    mi2: f64,
    e2: f64,
) -> bool {
    if !(temperature_k > 0.0
        && (0.0..=D_MAX).contains(&current)
        && mi1 >= 0.0
        && mi2 >= 0.0
        && e1 >= 0.0
        && e2 >= 0.0
        && honest_spend(temperature_k, mi1, e1)
        && honest_spend(temperature_k, mi2, e2)
        && mi1 <= mi2)
    {
        return true;
    }
    let v1 = dignity_step(temperature_k, current, mi1, e1);
    let v2 = dignity_step(temperature_k, current, mi2, e2);
    v1 <= v2 + 1e-12
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_dignity_rejects_negative() {
        assert_eq!(try_dignity(-1.0), None);
    }

    #[test]
    fn try_dignity_rejects_above_max() {
        assert_eq!(try_dignity(10.5), None);
    }

    #[test]
    fn try_dignity_accepts_interior() {
        assert_eq!(try_dignity(3.0), Some(3.0));
    }

    #[test]
    fn honest_step_non_decreasing() {
        let t = 300.0;
        let d = 2.0;
        let mi = 0.5;
        let e = landauer_joules_per_bit(t) * mi + 1.0;
        let d2 = dignity_step(t, d, mi, e);
        assert!(d2 + 1e-12 >= d && d2 <= D_MAX + 1e-12);
    }

    #[test]
    fn sub_landauer_does_not_increase() {
        let t = 300.0;
        let d = 4.0;
        let mi = 2.0;
        let e = landauer_joules_per_bit(t) * mi * 0.5;
        let d2 = dignity_step(t, d, mi, e);
        assert!((d2 - d).abs() < 1e-12);
    }

    #[test]
    fn avg_bounds_midpoint() {
        let a = 1.0;
        let b = 9.0;
        let m = 0.5 * a + 0.5 * b;
        assert!((0.0..=D_MAX).contains(&m));
    }

    #[test]
    fn determinism() {
        let x = dignity_step(280.0, 3.0, 0.25, 1e-18);
        assert_eq!(x, dignity_step(280.0, 3.0, 0.25, 1e-18));
    }

    #[test]
    fn monotone_checker_on_random_feasible_pair() {
        assert!(dignity_monotone_under_mi_gain_check(
            295.0, 1.0, 0.1, 1e-15, 0.2, 2e-15
        ));
    }
}
