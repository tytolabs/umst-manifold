//! In-memory `HardwareUnit` fixtures and paired-arch **inventory** patterns (H-8 §0.8 RED/GRN; Q6 C-strict mocks).
//! No RAPL, NVML, IOKit — pure Rust. Used by `tests/hal_*.rs` and law verification.

use std::string::String;
use std::sync::Arc;

use super::inventory::HardwareInventory;
use super::kinds::UnitKind;
use super::presence::UnitPresence;
use super::profile::{ArchClass, ArchitectureProfile, ParetoReferenceLabel};
use super::traits::*;

/// Generic mock unit: behavior varies only by `id` and `k`
#[derive(Debug)]
pub struct MockHardwareUnit {
    k: UnitKind,
    id: u32,
    drift: u64,
}

impl MockHardwareUnit {
    /// NED-consistent `new` (H-8)
    pub const fn new(k: UnitKind, id: u32, drift: u64) -> Self {
        Self { k, id, drift }
    }
}

impl HardwareUnit for MockHardwareUnit {
    fn enumerate_models(&self) -> Vec<String> {
        vec![format!("mock-{:?}-{}", self.k, self.id)]
    }
    fn supported_precisions(&self) -> Vec<ComputePrecision> {
        vec![ComputePrecision::Fp32, ComputePrecision::Fp16]
    }
    fn allocate(&self, spec: &AllocationSpec) -> Result<AllocationId, HalError> {
        if spec.bytes == 0 {
            return Err(HalError::NotApplicable);
        }
        Ok(AllocationId(
            self.id as u64 + spec.bytes.saturating_add(spec.class as u64),
        ))
    }
    fn infer(&self, alloc: AllocationId, batch: &InferenceBatch) -> Result<InferenceId, HalError> {
        if alloc.0 == 0 {
            return Err(HalError::NotConfigured);
        }
        Ok(InferenceId(
            self.id as u64 + alloc.0 + batch.batch as u64 + batch.op as u64 + self.drift,
        ))
    }
    fn deallocate(&self, id: AllocationId) -> Result<(), HalError> {
        if id.0 == 0 {
            Err(HalError::NotConfigured)
        } else {
            Ok(())
        }
    }
    fn power_state(&self) -> Result<PowerState, HalError> {
        Ok(PowerState {
            headroom: 0.5,
            kind: PowerStateKind::P0,
            reason: None,
        })
    }
    fn drift_window(&self) -> u64 {
        self.drift
    }
}

fn mk(kind: UnitKind) -> Arc<dyn HardwareUnit + Send + Sync> {
    Arc::new(MockHardwareUnit::new(kind, kind as u8 as u32 + 1, 3))
}

/// **Linux/Intel** mock: Present on CPU, IGPU, DGPU, NPU, RAM, PORT; `AbsentByArch` for ANE (Q6; FORWARD-PLAN Q4)
pub fn mock_linux_intel_inventory() -> HardwareInventory {
    let present = |k: UnitKind| {
        if k == UnitKind::Ane {
            UnitPresence::AbsentByArch
        } else {
            UnitPresence::Present(mk(k))
        }
    };
    HardwareInventory {
        profile: ArchitectureProfile {
            arch_class: ArchClass::LinuxIntelP14sGen5,
            canonical_fallback_chains: vec![(
                WorkloadClass::Inference,
                vec![UnitKind::Cpu, UnitKind::Igpu, UnitKind::Dgpu, UnitKind::Npu],
            )],
            architectural_constraints: "linuxintel_mock".to_string(),
            pareto_reference: Some(ParetoReferenceLabel {
                reg_token: "B2_LINUX_INTEL_STUB".to_string(),
            }),
        },
        cpu: present(UnitKind::Cpu),
        igpu: present(UnitKind::Igpu),
        dgpu: present(UnitKind::Dgpu),
        npu: present(UnitKind::Npu),
        ane: present(UnitKind::Ane),
        ram: present(UnitKind::Ram),
        port: present(UnitKind::Port),
    }
}

/// **Apple M3** mock: Present on CPU, IGPU, ANE, RAM, PORT; `AbsentByArch` for DGPU, NPU (C-strict pairing; Q4)
pub fn mock_apple_m3_inventory() -> HardwareInventory {
    let present = |k: UnitKind| {
        if matches!(k, UnitKind::Dgpu | UnitKind::Npu) {
            UnitPresence::AbsentByArch
        } else {
            UnitPresence::Present(mk(k))
        }
    };
    HardwareInventory {
        profile: ArchitectureProfile {
            arch_class: ArchClass::MacosM3Max,
            canonical_fallback_chains: vec![(
                WorkloadClass::Inference,
                vec![UnitKind::Cpu, UnitKind::Igpu, UnitKind::Ane],
            )],
            architectural_constraints: "applem3_uma_mock".to_string(),
            pareto_reference: Some(ParetoReferenceLabel {
                reg_token: "B2_MACOS_M3_MAX_STUB".to_string(),
            }),
        },
        cpu: present(UnitKind::Cpu),
        igpu: present(UnitKind::Igpu),
        dgpu: present(UnitKind::Dgpu),
        npu: present(UnitKind::Npu),
        ane: present(UnitKind::Ane),
        ram: present(UnitKind::Ram),
        port: present(UnitKind::Port),
    }
}
