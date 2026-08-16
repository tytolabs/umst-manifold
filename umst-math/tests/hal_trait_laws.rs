// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! §0.8 RED — category **𝓗** routing laws + `HardwareUnit` smoke on [`umst_math::hal::mocks::MockHardwareUnit`]
use umst_math::hal::laws::{
    check_associative, check_identity_law, f_cpu_igpu, g_igpu_ram, h_ram_port, id_route, route_id,
};
use umst_math::hal::mocks::MockHardwareUnit;
use umst_math::hal::traits::HardwareUnit;
use umst_math::hal::traits::{AllocationId, AllocationSpec, InferenceBatch};
use umst_math::hal::UnitKind;

#[test]
fn aa_hal_law_identity_fn_cpu_igpu() {
    let (l, r) = check_identity_law(f_cpu_igpu);
    assert!(l);
    assert!(r);
}

#[test]
fn aa_hal_law_identity_fn_id_is_id() {
    let (l, r) = check_identity_law(id_route);
    assert!(l);
    assert!(r);
}

#[test]
fn aa_hal_law_associative_fgh() {
    assert!(check_associative(f_cpu_igpu, g_igpu_ram, h_ram_port));
}

#[test]
fn aa_hal_law_kcompose_cpu_to_port() {
    let k = UnitKind::Cpu;
    // Cpu → Igpu → Ram → Port
    let chain = f_cpu_igpu(k).and_then(g_igpu_ram).and_then(h_ram_port);
    assert_eq!(chain, Some(UnitKind::Port));
}

#[test]
fn aa_hal_law_route_id_spans_all_kinds() {
    let id = route_id();
    for k in UnitKind::ALL {
        assert_eq!(id(k), Some(k));
    }
}

#[test]
fn aa_hal_mock_unit_trait_roundtrip_alloc_infer() {
    let u = MockHardwareUnit::new(UnitKind::Cpu, 1, 4);
    let a = u
        .allocate(&AllocationSpec { bytes: 8, class: 1 })
        .expect("alloc");
    let _i = u
        .infer(a, &InferenceBatch { batch: 2, op: 1 })
        .expect("infer");
    u.deallocate(AllocationId(0)).expect_err("zero dealloc");
}

#[test]
fn aa_hal_all_unit_kinds_exhaustive_match() {
    // (c) totality of the closed `UnitKind` sum
    let p = |k: UnitKind| match k {
        UnitKind::Cpu => 0,
        UnitKind::Igpu => 1,
        UnitKind::Dgpu => 2,
        UnitKind::Npu => 3,
        UnitKind::Ane => 4,
        UnitKind::Ram => 5,
        UnitKind::Port => 6,
    };
    for k in UnitKind::ALL {
        assert_eq!(p(k), p(k));
    }
}

#[test]
fn aa_hal_law_associative_degenerate() {
    let a = |_: UnitKind| -> Option<UnitKind> { None };
    assert!(check_associative(a, a, a));
}
