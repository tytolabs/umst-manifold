// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT

//! Theorem ↔ constant crosswalk: computed from the registry's `Derivation::Theorem` rows.

use umst_math::constants::derivation::Derivation;
use umst_math::constants::registry::REGISTRY;
use umst_math::theorem_registry::{constant_for_theorem, theorem_for_constant};

#[test]
fn every_theorem_row_round_trips_through_the_crosswalk() {
    for e in REGISTRY {
        if let Derivation::Theorem { decl, .. } = e.derivation {
            assert_eq!(theorem_for_constant(e.name), Some(decl));
            assert_eq!(constant_for_theorem(decl), Some(e.name));
        }
    }
}

#[test]
fn rows_no_theorem_fixes_have_no_crosswalk_entry() {
    for e in REGISTRY {
        if !matches!(e.derivation, Derivation::Theorem { .. }) {
            assert_eq!(theorem_for_constant(e.name), None, "{}", e.name);
        }
    }
}
