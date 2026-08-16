// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! `ArchClass` closed-enum totality
use umst_math::hal::profile::ArchClass;

fn tag(c: &ArchClass) -> u8 {
    match c {
        ArchClass::LinuxIntelP14sGen5 => 0,
        ArchClass::MacosM3Max => 1,
        ArchClass::Unknown {
            host_fingerprint_hash: h,
        } => 2 + (h.len() & 0xff) as u8,
    }
}

#[test]
fn aa_hal_arch_class_exhaustive() {
    let a = [
        ArchClass::LinuxIntelP14sGen5,
        ArchClass::MacosM3Max,
        ArchClass::Unknown {
            host_fingerprint_hash: "a".to_string(),
        },
    ];
    for c in a {
        let _ = tag(&c);
    }
}

#[test]
fn aa_hal_arch_unknown_fingerprint_holds() {
    let c = ArchClass::Unknown {
        host_fingerprint_hash: "abc".to_string(),
    };
    assert!(tag(&c) > 0);
}

#[test]
fn aa_hal_arch_class_ebisim() {
    let a = ArchClass::LinuxIntelP14sGen5;
    let b = ArchClass::LinuxIntelP14sGen5;
    assert_eq!(a, b);
}
