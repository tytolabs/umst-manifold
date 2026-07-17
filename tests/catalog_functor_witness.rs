//! Root re-export witness — delegates to `umst-math::catalog_functor` + fixture locks.

use serde_json::Value;
use umst_layout_codegen::parse_scalar_layout_lock;
use umst_math::catalog_functor::{composed_digest_guard_holds, expected_scalar_channel_count};

#[test]
fn catalog_functor_fixture_lock_passes_composed_digest_guard() {
    let lock_json = include_str!("../artifacts/catalog.lock.json");
    let lock: Value = serde_json::from_str(lock_json).expect("parse artifacts/catalog.lock.json for catalog_functor composed_digest_guard witness (FP §6 Track G catalog harness)");
    assert!(composed_digest_guard_holds(&lock));
}

#[test]
fn catalog_functor_fixture_fibers_and_sidecar_agree() {
    let sidecar = parse_scalar_layout_lock(include_str!("../artifacts/scalar_layout.lock.json"))
        .expect("parse artifacts/scalar_layout.lock.json sidecar for catalog_functor fiber pin witness (FP §6 Track G catalog harness)");
    let lock: Value =
        serde_json::from_str(include_str!("../artifacts/catalog.lock.json")).expect("parse artifacts/catalog.lock.json for catalog_functor fiber pin witness (FP §6 Track G catalog harness)");
    for pin in lock
        .get("fiber_pins")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let repo = pin.get("repo").and_then(Value::as_str).expect("catalog.lock.json fiber_pins[].repo field for scalar channel count witness (FP §6 Track G catalog harness)");
        let role = pin.get("lock_role").and_then(Value::as_str);
        assert_eq!(expected_scalar_channel_count(repo, role, &sidecar), 0);
    }
}
