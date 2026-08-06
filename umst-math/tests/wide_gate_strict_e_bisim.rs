//! §14bis.l W-1 + W-2 — e-bisim: `umst_wide_gate_strict` and `umst_semantic_coverage_threshold_w2` rows; idempotent `REGISTRY` read.

use umst_math::constants::registry::{ConstantTier, REGISTRY};

#[test]
fn reg_has_umst_wide_gate_strict() {
    assert_eq!(REGISTRY.len(), 168);
    let names1: Vec<&'static str> = REGISTRY.iter().map(|e| e.name).collect();
    let names2: Vec<&'static str> = REGISTRY.iter().map(|e| e.name).collect();
    assert_eq!(names1, names2, "idempotent read of static REGISTRY");
    assert!(names1.contains(&"umst_wide_gate_strict"));
    assert!(names1.contains(&"umst_discovery_lru_capacity"));
    assert!(names1.contains(&"umst_tui_render_debounce_ms"));
    assert!(names1.contains(&"umst_semantic_coverage_threshold_w2"));
    assert!(names1.contains(&"umst_gpu_backend_default"));
    assert!(names1.contains(&"umst_npu_backend_default"));
    assert!(names1.contains(&"umst_cockpit_smoothing_default"));
    assert!(names1.contains(&"umst_tui_color_accent_dark"));
    assert!(names1.contains(&"umst_tui_color_accent_light"));
    let tui5a = REGISTRY
        .iter()
        .find(|e| e.name == "umst_discovery_lru_capacity")
        .expect("TUI-5 discovery LRU row");
    let tui5b = REGISTRY
        .iter()
        .find(|e| e.name == "umst_tui_render_debounce_ms")
        .expect("TUI-5 debounce row");
    assert_eq!(tui5a.tier, ConstantTier::Tier3Policy);
    assert_eq!(tui5b.tier, ConstantTier::Tier3Policy);
    assert!(
        tui5a.expression.trim_start().starts_with("16 ("),
        "LUR 16: {}",
        tui5a.expression
    );
    assert!(
        tui5b.expression.trim_start().starts_with("16 ("),
        "debounce 16: {}",
        tui5b.expression
    );

    let w2 = REGISTRY
        .iter()
        .find(|e| e.name == "umst_semantic_coverage_threshold_w2")
        .expect("W-2 G8 floor");
    assert_eq!(w2.tier, ConstantTier::Tier3Policy);
    let ev = w2.evidence.trim();
    assert!(
        ev.contains("Definition") && ev.contains("W-2"),
        "T3 `Definition` evidence: {ev}"
    );
}
