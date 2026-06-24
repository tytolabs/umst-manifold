//! Witness spike: catalog fiber id → scalar channel count + composed digest guard.

use quickcheck::quickcheck;
use serde_json::Value;
use umst_layout_codegen::parse_scalar_layout_lock;
use umst_math::catalog_functor::{
    composed_digest_guard_holds, composed_digest_guard_idempotent,
    expected_scalar_channel_count, is_preview_fiber_role, runtime_scalar_channel_count,
    MANIFOLD_RUNTIME_FIBER_ID,
};

fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("umst-math inside umst-manifold workspace")
        .to_path_buf()
}

fn fixture_catalog_lock_json() -> String {
    std::fs::read_to_string(workspace_root().join("artifacts/catalog.lock.json"))
        .expect("artifacts/catalog.lock.json")
}

fn fixture_scalar_layout_json() -> String {
    std::fs::read_to_string(workspace_root().join("artifacts/scalar_layout.lock.json"))
        .expect("artifacts/scalar_layout.lock.json")
}

#[test]
fn catalog_functor_fiber_id_maps_to_expected_scalar_channel_count_from_sidecar() {
    let sidecar = parse_scalar_layout_lock(&fixture_scalar_layout_json()).expect("sidecar parse");
    assert_eq!(sidecar.scalar_channel_count, 7);
    assert_eq!(
        runtime_scalar_channel_count(&sidecar),
        sidecar.scalar_channel_count
    );
    assert_eq!(
        expected_scalar_channel_count(MANIFOLD_RUNTIME_FIBER_ID, None, &sidecar),
        7
    );

    let lock: Value = serde_json::from_str(&fixture_catalog_lock_json()).expect("catalog lock json");
    let pins = lock
        .get("fiber_pins")
        .and_then(Value::as_array)
        .expect("fiber_pins");

    for pin in pins {
        let repo = pin.get("repo").and_then(Value::as_str).expect("repo");
        let role = pin.get("lock_role").and_then(Value::as_str);
        let count = expected_scalar_channel_count(repo, role, &sidecar);
        if is_preview_fiber_role(role) {
            assert_eq!(count, 0, "preview fiber {repo}");
        } else if role == Some("lean_catalog_lock") {
            assert_eq!(count, 0, "lean catalog fiber {repo}");
        } else {
            assert_eq!(count, 0, "unknown role for {repo}");
        }
    }
}

#[test]
fn catalog_functor_composed_digest_guard_holds_for_fixture_lock() {
    let lock: Value = serde_json::from_str(&fixture_catalog_lock_json()).expect("catalog lock json");
    assert!(
        composed_digest_guard_holds(&lock),
        "fixture catalog.lock.json must satisfy T1 composed digest guard"
    );
    assert!(composed_digest_guard_idempotent(&lock));
}

quickcheck! {
    fn catalog_functor_digest_guard_idempotent_on_fixture_clone(_pad: u8) -> bool {
        let lock: Value = serde_json::from_str(&fixture_catalog_lock_json()).expect("lock");
        composed_digest_guard_idempotent(&lock)
    }

    fn catalog_functor_preview_roles_map_to_zero_scalars(role_suffix: String) -> bool {
        let sidecar = parse_scalar_layout_lock(&fixture_scalar_layout_json()).expect("sidecar");
        let role = format!("ucrs_preview_{role_suffix}");
        if !is_preview_fiber_role(Some(&role)) {
            return true;
        }
        expected_scalar_channel_count("umst-ucrs", Some(&role), &sidecar) == 0
    }
}
