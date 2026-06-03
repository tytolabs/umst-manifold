//! Linux Intel CPU `HardwareUnit`

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::permissions::PermissionState;
use super::rapl;
use super::smoke;
use super::sysfs;
use crate::hal::traits::{
    AllocationId, AllocationSpec, ComputePrecision, HalError, HardwareUnit, InferenceBatch,
    InferenceId, PowerState, PowerStateKind, SMOKE_INFER_OP,
};

const RAPL_PATH: &str = "/sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj";

/// Linux Intel host CPU
pub struct IntelCpu {
    next_id: AtomicU64,
    pub rapl_uj: Option<u64>,
    pub power_readable: bool,
    pub permission: PermissionState,
    pub models: Vec<String>,
    rapl_path: PathBuf,
}

impl Default for IntelCpu {
    fn default() -> Self {
        Self::new()
    }
}

impl IntelCpu {
    /// Construct with host probes
    pub fn new() -> Self {
        let path = super::sysfs::default_rapl_package_energy();
        let perm = super::permissions::read_probe(&path, RAPL_PATH);
        let power_readable = matches!(perm, PermissionState::Granted);
        let rapl_uj = if power_readable {
            rapl::read_package_energy_uj(&path)
        } else {
            None
        };
        let logical = sysfs::cpuinfo_logical_cores().unwrap_or(1);
        let l3 = sysfs::l3_cache_kb();
        let mut models = Vec::new();
        if let Ok(c) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in c.lines() {
                if line.to_lowercase().starts_with("model name") {
                    if let Some(n) = line.split(':').nth(1) {
                        models.push(format!("intel-cpu#{}", n.trim()));
                        break;
                    }
                }
            }
        }
        if models.is_empty() {
            models.push("intel-cpu#unknown".to_string());
        }
        if !power_readable {
            models.push("power_readable=false".to_string());
        }
        models.push(format!("intel-cpu#reg#cores={logical}#l3_kb={l3}"));
        Self {
            next_id: AtomicU64::new(1),
            rapl_uj,
            power_readable,
            permission: perm,
            models,
            rapl_path: path,
        }
    }
}

impl HardwareUnit for IntelCpu {
    fn enumerate_models(&self) -> Vec<String> {
        self.models.clone()
    }

    fn supported_precisions(&self) -> Vec<ComputePrecision> {
        vec![
            ComputePrecision::Fp32,
            ComputePrecision::Fp64,
            ComputePrecision::I32,
            ComputePrecision::I64,
        ]
    }

    fn allocate(&self, spec: &AllocationSpec) -> Result<AllocationId, HalError> {
        if spec.bytes == 0 {
            return Err(HalError::NotApplicable);
        }
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        Ok(AllocationId(id))
    }

    fn infer(&self, _alloc: AllocationId, batch: &InferenceBatch) -> Result<InferenceId, HalError> {
        if batch.op != SMOKE_INFER_OP {
            return Err(HalError::NotConfigured);
        }
        smoke::run_smoke();
        Ok(InferenceId(batch.batch as u64 + 1))
    }

    fn deallocate(&self, id: AllocationId) -> Result<(), HalError> {
        if id.0 == 0 {
            return Err(HalError::NotConfigured);
        }
        Ok(())
    }

    fn power_state(&self) -> Result<PowerState, HalError> {
        if let PermissionState::Denied { sysfs_path, .. } = &self.permission {
            return Ok(PowerState {
                headroom: 0.0,
                kind: PowerStateKind::P3,
                reason: Some(format!("permission_denied: {sysfs_path}")),
            });
        }
        if !self.power_readable {
            return Ok(PowerState {
                headroom: 0.0,
                kind: PowerStateKind::P2,
                reason: Some("power_unreadable: rapl".to_string()),
            });
        }
        Ok(PowerState {
            headroom: 0.75,
            kind: PowerStateKind::P0,
            reason: self.rapl_uj.map(|j| format!("rapl_uj={j}")),
        })
    }

    fn drift_window(&self) -> u64 {
        16
    }
}

impl IntelCpu {
    /// For permission warning at startup
    pub fn rapl_path_display(&self) -> String {
        self.rapl_path.to_string_lossy().into_owned()
    }
}
