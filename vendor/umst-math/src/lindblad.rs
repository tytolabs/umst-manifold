//! Lindblad CPTP semigroup — dephasing / stream-D hooks (numerical layer for Oracle v2).
//!
//! **Registry:** `theorem_registry::THEOREM_REGISTRY` — add a `LindbladStreamD/…` row when the
//! Lean export for `streamD_limit_to_Lueders_states` is bridged numerically.

/// Dephasing rate parameter (1/s) placeholder for Lindblad generator calibration.
///
/// Proof: `LindbladDynamics` / `dephasingSolution_tendsto_diagonal`.
/// DOI: 10.5281/zenodo.19159660
pub fn dephasing_rate_placeholder() -> f64 {
    1.0
}
