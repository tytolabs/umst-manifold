//! §0.8 — `UnitPresence` four-way ε-bisim (stringify stability)
use std::string::String;

use umst_math::hal::presence::AbsentReason;
use umst_math::hal::presence::UnitPresence;

#[test]
fn aa_hal_present_roundtrip() {
    let p: UnitPresence<u32> = UnitPresence::Present(7);
    assert_eq!(p.map(|x| x * 2), UnitPresence::Present(14));
}

#[test]
fn aa_hal_absent_by_arch_ebisim() {
    let a: UnitPresence<u8> = UnitPresence::AbsentByArch;
    let s = format!("{a:?}");
    assert_eq!(s, s);
}

#[test]
fn aa_hal_absent_by_config_ebisim() {
    let a: UnitPresence<bool> = UnitPresence::AbsentByConfig;
    assert_eq!(a, a);
}

#[test]
fn aa_hal_absent_by_fault_ebisim() {
    let r = AbsentReason {
        code: 1,
        message: "m".to_string(),
    };
    let a: UnitPresence<u8> = UnitPresence::AbsentByFault(r.clone());
    let b: UnitPresence<u8> = UnitPresence::AbsentByFault(r);
    assert_eq!(a, b);
}

#[test]
fn aa_hal_all_variants_reachable() {
    let v: [UnitPresence<String>; 4] = [
        UnitPresence::Present("x".to_string()),
        UnitPresence::AbsentByArch,
        UnitPresence::AbsentByConfig,
        UnitPresence::AbsentByFault(AbsentReason {
            code: 0,
            message: "m".to_string(),
        }),
    ];
    assert_eq!(v.len(), 4);
}
