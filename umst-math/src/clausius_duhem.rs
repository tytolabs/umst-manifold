//! Clausius–Duhem conjunct SSOT (mirrors `gateCheck` dissipative conjunct on ψ).

/// Returns true when `new_ψ ≤ old_ψ` (dissipative / non-increasing free-energy head).
///
/// Shared predicate family for `umst-formal/Lean/Gate.lean` and `umst-ucrs::gate_check`.
#[must_use]
pub fn clausius_duhem_admissible(old_psi: f64, new_psi: f64) -> bool {
    old_psi.is_finite() && new_psi.is_finite() && new_psi <= old_psi
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dissipative_admits() {
        assert!(clausius_duhem_admissible(1.0, 0.5));
        assert!(clausius_duhem_admissible(1.0, 1.0));
    }

    #[test]
    fn increase_rejects() {
        assert!(!clausius_duhem_admissible(0.5, 1.0));
    }
}
