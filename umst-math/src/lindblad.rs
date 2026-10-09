// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Lindblad CPTP semigroup — dephasing / stream-D hooks (numerical layer for Oracle v2).
//!
//! **Registry:** `theorem_registry::THEOREM_REGISTRY` — add a `LindbladStreamD/…` row when the
//! Lean export for `streamD_limit_to_Lueders_states` is bridged numerically.
//!
//! No dephasing rate is calibrated: the generator will take the rate as an input when the bridge lands
//! (proof `LindbladDynamics` / `dephasingSolution_tendsto_diagonal`, DOI 10.5281/zenodo.19159660), so this
//! module carries no rate value.
