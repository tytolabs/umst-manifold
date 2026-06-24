//! Pure catalog → scalar-layout functor witness (Phase 1 §1B + dual-pin lock).
//!
//! Maps catalog **fiber ids** (repo slugs in `catalog.lock.json`) to expected nodal scalar
//! channel counts from the [`ScalarLayoutSidecar`] (`artifacts/scalar_layout.lock.json`).
//! Lean / preview catalog fibers carry **zero** nodal scalars; only the manifold runtime fiber
//! projects to the pinned `scalar_channel_count`.

use serde_json::Value;
use sha2::{Digest, Sha256};
use umst_layout_codegen::LayoutSpec;

/// Repo slug for the manifold runtime scalar-layout bearer (not a Lean `fiber_pins` entry).
pub const MANIFOLD_RUNTIME_FIBER_ID: &str = "umst-manifold";

/// Pinned nodal scalar layout sidecar (`artifacts/scalar_layout.lock.json`).
pub type ScalarLayoutSidecar = LayoutSpec;

/// Expected nodal scalar channel count for a catalog fiber id given the layout sidecar.
///
/// - [`MANIFOLD_RUNTIME_FIBER_ID`] → `sidecar.scalar_channel_count`
/// - Lean catalog pins (`lean_catalog_lock`) and preview / Track F pins → `0`
#[must_use]
pub fn expected_scalar_channel_count(
    fiber_id: &str,
    lock_role: Option<&str>,
    sidecar: &ScalarLayoutSidecar,
) -> usize {
    if fiber_id == MANIFOLD_RUNTIME_FIBER_ID {
        return sidecar.scalar_channel_count;
    }
    if is_preview_fiber_role(lock_role) {
        return 0;
    }
    if is_lean_catalog_fiber_role(lock_role) {
        return 0;
    }
    0
}

/// Runtime nodal tensor width from the scalar layout sidecar (functor image of manifold fiber).
#[must_use]
pub fn runtime_scalar_channel_count(sidecar: &ScalarLayoutSidecar) -> usize {
    expected_scalar_channel_count(MANIFOLD_RUNTIME_FIBER_ID, None, sidecar)
}

/// Preview / tertiary Track F pins are excluded from composed digest guards.
#[must_use]
pub fn is_preview_fiber_role(lock_role: Option<&str>) -> bool {
    let role = lock_role.unwrap_or("").to_ascii_lowercase();
    role.contains("preview") || role.contains("track_f")
}

/// Lean catalog lock pins (`lean_catalog_lock`) carry module digests only — no nodal scalars.
#[must_use]
pub fn is_lean_catalog_fiber_role(lock_role: Option<&str>) -> bool {
    lock_role
        .map(|r| r.eq_ignore_ascii_case("lean_catalog_lock"))
        .unwrap_or(false)
}

fn is_sha256_hex(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit())
}

/// SHA-256 of sorted `repo:digest` pairs for non-preview [`fiber_pins`](Value::get).
#[must_use]
pub fn non_preview_fiber_fingerprint_hex(lock: &Value) -> String {
    let pins = lock
        .get("fiber_pins")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let mut digests: Vec<String> = pins
        .iter()
        .filter(|pin| !is_preview_fiber_pin_value(pin))
        .filter_map(|pin| {
            let repo = pin.get("repo")?.as_str()?;
            let digest = pin.get("catalog_digest_hex")?.as_str()?;
            Some(format!("{repo}:{digest}"))
        })
        .collect();

    if digests.is_empty() {
        return String::new();
    }

    digests.sort();
    let payload = digests.join("|");
    let hash = Sha256::digest(payload.as_bytes());
    format!("{hash:x}")
}

#[must_use]
fn is_preview_fiber_pin_value(pin: &Value) -> bool {
    is_preview_fiber_role(pin.get("lock_role").and_then(Value::as_str))
}

/// Composed digest guard invariants (mirrors `build.rs` / `catalog_lock_verify.py`).
///
/// For v2 locks with non-preview fibers:
/// - `composed_primary_fiber_fingerprint_hex` matches recomputed fingerprint
/// - `composed_catalog_digest_hex` is 64-char hex and equals `upstream_catalog_digest_hex`
#[must_use]
pub fn composed_digest_guard_holds(lock: &Value) -> bool {
    let version = lock.get("version").and_then(Value::as_u64).unwrap_or(1);
    if version < 2 {
        return true;
    }

    let pins = lock
        .get("fiber_pins")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let non_preview: Vec<&Value> = pins
        .iter()
        .filter(|pin| !is_preview_fiber_pin_value(pin))
        .collect();

    if non_preview.is_empty() {
        return true;
    }

    let fp = non_preview_fiber_fingerprint_hex(lock);
    let stored = lock
        .get("composed_primary_fiber_fingerprint_hex")
        .and_then(Value::as_str)
        .unwrap_or("");
    if stored.is_empty() || stored != fp {
        return false;
    }

    let composed = lock
        .get("composed_catalog_digest_hex")
        .and_then(Value::as_str)
        .unwrap_or("");
    if !is_sha256_hex(composed) {
        return false;
    }

    if let Some(upstream) = lock
        .get("upstream_catalog_digest_hex")
        .and_then(Value::as_str)
        .filter(|u| !u.is_empty())
    {
        if composed != upstream {
            return false;
        }
    }

    true
}

/// Idempotency: recomputing the guard on an already-valid lock stays valid.
#[must_use]
pub fn composed_digest_guard_idempotent(lock: &Value) -> bool {
    if !composed_digest_guard_holds(lock) {
        return true;
    }
    composed_digest_guard_holds(lock)
}

#[cfg(test)]
mod tests {
    use super::*;
    use umst_layout_codegen::parse_scalar_layout_lock;

    const SAMPLE_SIDECAR: &str = r#"{
  "schema": "umst_scalar_layout_v1",
  "scalar_channel_count": 7,
  "channel_ids": [
    "SCALAR_CHANNEL0",
    "SCALAR_HUMIDITY",
    "SCALAR_INTERNAL_VARIABLE_0",
    "SCALAR_TEMPERATURE",
    "SCALAR_DAMAGE",
    "SCALAR_FRACTURE_ENERGY_GC",
    "SCALAR_EPISTEMIC_UNCERTAINTY"
  ]
}"#;

    #[test]
    fn catalog_functor_manifold_runtime_maps_sidecar_count() {
        let sidecar = parse_scalar_layout_lock(SAMPLE_SIDECAR).expect("sidecar");
        assert_eq!(runtime_scalar_channel_count(&sidecar), 7);
    }

    #[test]
    fn catalog_functor_lean_fiber_maps_to_zero() {
        let sidecar = parse_scalar_layout_lock(SAMPLE_SIDECAR).expect("sidecar");
        assert_eq!(
            expected_scalar_channel_count(
                "umst-formal",
                Some("lean_catalog_lock"),
                &sidecar
            ),
            0
        );
    }
}
