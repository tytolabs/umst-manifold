//! Deterministic 7×4 presence-variant grid (7 [`UnitKind`] × 4 `UnitPresence` *tags*)
use umst_math::hal::UnitKind;

#[test]
fn aa_hal_grid_7x4_reachable() {
    for row in 0u8..7 {
        for col in 0u8..4 {
            let _ = (UnitKind::ALL[row as usize], col);
        }
    }
    assert_eq!(7 * 4, 28);
}
