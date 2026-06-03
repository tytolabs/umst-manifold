//! System RAM + best-effort DRAM RAPL (package used as package path)

use std::sync::atomic::{AtomicU64, Ordering};

use super::permissions::PermissionState;
use super::rapl;
use super::smoke;
use super::sysfs;
use super::sysfs::default_rapl_package_energy;
use crate::hal::traits::{
    AllocationId, AllocationSpec, ComputePrecision, HalError, HardwareUnit, InferenceBatch,
    InferenceId, PowerState, PowerStateKind, SMOKE_INFER_OP,
};

const RAPL: &str = "/sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj";

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
        let p = default_rapl_package_energy();
        let perm = super::permissions::read_probe(&p, RAPL);
        let rapl_ok = matches!(perm, PermissionState::Granted);
        let uj = if rapl_ok {
            rapl::read_package_energy_uj(&p)
        } else {
            None
        };
        let total = sysfs::mem_total_kb().unwrap_or(0);
        Self {
            next: AtomicU64::new(1),
            total_kb: total,
            rapl_ok,
            uj,
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
