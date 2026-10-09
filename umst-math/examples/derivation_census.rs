// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Print the derivation census of `umst_math::constants::registry::REGISTRY` as one JSON line (read by the
//! egoff K-7 gate `scripts/check_no_pending_derivation.sh`).

fn main() {
    println!("{}", umst_math::constants::derivation::DerivationCensus::of_registry().to_json());
}
