//! System RAM + best-effort DRAM RAPL (package used as package path)

use std::sync::atomic::{AtomicU64, Ordering};

use super::smoke;
use crate::hal::probe_snapshot::{HalProbeSnapshot, RamProbe};
use crate::hal::traits::{
    AllocationId, AllocationSpec, ComputePrecision, HalError, HardwareUnit, InferenceBatch,
    InferenceId, PowerState, PowerStateKind, SMOKE_INFER_OP,
};

/// Linux main memory lane (DRAM; RAPL vs MemTotal honest)
pub struct LinuxRam {
    next: AtomicU64,
    total_kb: u64,
    rapl_ok: bool,
    uj: Option<u64>,
}

impl Default for LinuxRam {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxRam {
    pub fn new() -> Self {
        #[cfg(feature = "linux-hal-sysfs")]
        {
            return Self::from_snapshot(&crate::hal::probe_host::probe_sysfs_snapshot());
        }
        #[cfg(not(feature = "linux-hal-sysfs"))]
        {
            Self::from_snapshot(&HalProbeSnapshot::default())
        }
    }

    /// Pure assembly from an injected probe snapshot (FP §4).
    pub fn from_snapshot(snapshot: &HalProbeSnapshot) -> Self {
        Self::from_ram_probe(&snapshot.ram)
    }

    /// Pure assembly from RAM probe fields.
    pub fn from_ram_probe(ram: &RamProbe) -> Self {
        Self {
            next: AtomicU64::new(1),
            total_kb: ram.total_kb,
            rapl_ok: ram.rapl_ok,
            uj: ram.rapl_uj,
        }
    }
}

impl HardwareUnit for LinuxRam {
    fn enumerate_models(&self) -> Vec<String> {
        vec![format!("linux-ram#MemTotal_kb={}", self.total_kb)]
    }

    fn supported_precisions(&self) -> Vec<ComputePrecision> {
        vec![ComputePrecision::Fp32]
    }

    fn allocate(&self, spec: &AllocationSpec) -> Result<AllocationId, HalError> {
        if spec.bytes == 0 {
            return Err(HalError::NotApplicable);
        }
        Ok(AllocationId(self.next.fetch_add(1, Ordering::Relaxed)))
    }

    fn infer(&self, _a: AllocationId, b: &InferenceBatch) -> Result<InferenceId, HalError> {
        if b.op != SMOKE_INFER_OP {
            return Err(HalError::NotConfigured);
        }
        smoke::run_smoke();
        Ok(InferenceId(1))
    }

    fn deallocate(&self, _i: AllocationId) -> Result<(), HalError> {
        Ok(())
    }

    fn power_state(&self) -> Result<PowerState, HalError> {
        if !self.rapl_ok {
            return Ok(PowerState {
                headroom: 0.0,
                kind: PowerStateKind::P2,
                reason: Some("dram_rapl_unread_use_package_policy_h9".to_string()),
            });
        }
        Ok(PowerState {
            headroom: 0.2,
            kind: PowerStateKind::P0,
            reason: self.uj.map(|e| format!("rapl_uj(fallback)={e}")),
        })
    }

    fn drift_window(&self) -> u64 {
        2
    }
}
