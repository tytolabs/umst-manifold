// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! H-9 IntelNpu: Smoke → `NotYetImplemented` on present lane until B-2
#![cfg(target_os = "linux")]

use umst_math::hal::backends::linux::IntelNpu;
use umst_math::hal::traits::{
    AllocationSpec, HalError, HardwareUnit, InferenceBatch, WorkloadClass, SMOKE_INFER_OP,
};

#[test]
fn aa_hal_intel_npu_not_yet_when_present() {
    let n = IntelNpu::new();
    if !n.is_present() {
        let err = n
            .allocate(&AllocationSpec {
                bytes: 64,
                class: WorkloadClass::Inference as u32,
            })
            .err();
        assert_eq!(err, Some(HalError::NotConfigured));
        return;
    }
    let id = n
        .allocate(&AllocationSpec {
            bytes: 1024,
            class: WorkloadClass::Inference as u32,
        })
        .expect("a");
    let b = InferenceBatch {
        batch: 0,
        op: SMOKE_INFER_OP,
    };
    let e = n.infer(id, &b);
    match e {
        Err(HalError::NotYetImplemented {
            unit,
            workload,
            reason,
        }) => {
            assert!(unit.contains("Intel") || unit == "IntelNpu");
            assert!(!workload.is_empty() && !reason.is_empty());
        }
        o => panic!("expected NotYetImplemented, got {o:?}"),
    }
}

#[test]
fn aa_hal_intel_npu_models_empty_or_placeholder() {
    let n = IntelNpu::new();
    if n.is_present() {
        assert!(!n.enumerate_models().is_empty());
    } else {
        assert!(n.enumerate_models().is_empty());
    }
}

#[test]
fn aa_hal_intel_npu_drift_positive() {
    assert!(IntelNpu::new().drift_window() > 0);
}

#[test]
fn aa_hal_intel_npu_e_bisim_twin() {
    let a = (
        IntelNpu::new().is_present(),
        IntelNpu::new().supported_precisions().len(),
    );
    let b = (
        IntelNpu::new().is_present(),
        IntelNpu::new().supported_precisions().len(),
    );
    assert_eq!(a, b);
}

#[test]
fn aa_hal_intel_npu_infer_non_smoke_fails() {
    let n = IntelNpu::new();
    if !n.is_present() {
        return;
    }
    let id = n
        .allocate(&AllocationSpec {
            bytes: 4,
            class: WorkloadClass::Inference as u32,
        })
        .expect("a");
    let b = InferenceBatch {
        batch: 0,
        op: 0xDEAD_BEEF,
    };
    assert_eq!(n.infer(id, &b).err(), Some(HalError::NotConfigured));
}
