//! Englert complementarity **V² + I² ≤ 1** (fringe visibility vs which-way information).

use ordered_float::NotNan;

/// Left-hand side **V² + I²** (dimensionless).
///
/// Proof: `umst-formal-double-slit/Lean/QuantumClassicalBridge.lean` — `complementarity_fringe_path`.
/// DOI: 10.5281/zenodo.19159660
pub fn englert_lhs(visibility: NotNan<f64>, which_way_information: NotNan<f64>) -> NotNan<f64> {
    let v = visibility.into_inner();
    let i = which_way_information.into_inner();
    NotNan::new(v * v + i * i).expect("sum of squares")
}

/// Predicate: Englert inequality **V² + I² ≤ 1** (ε tolerance).
///
/// Proof: same as [`englert_lhs`].
/// DOI: 10.5281/zenodo.19159660
pub fn englert_bound_holds(visibility: NotNan<f64>, which_way_information: NotNan<f64>) -> bool {
    englert_lhs(visibility, which_way_information).into_inner() <= 1.0 + 1e-12
}
