/* Re-implemented in cockpit §14bis.f-H-9; inspired by
 * tytolabs/umst-prototype-2a@9c0434d3ebade8f697bbd402bb080ea00da76914
 * /prototype/src/rust/core/src/hardware/rapl.rs (read-only inspiration; no code lifted).
 * SPDX-License-Identifier: MIT
 */
//! RAPL `energy_uj` read (no writes). NED: never fabricate; `None` if Denied or parse fail.

use std::path::Path;

/// Returns microjoules if readable; `None` if missing or permission / parse error.
pub fn read_package_energy_uj(energy_path: &Path) -> Option<u64> {
    let s = std::fs::read_to_string(energy_path).ok()?;
    s.trim().parse().ok()
}
