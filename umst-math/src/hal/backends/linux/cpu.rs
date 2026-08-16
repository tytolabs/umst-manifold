// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Linux Intel CPU `HardwareUnit`

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::permissions::PermissionState;
use super::smoke;
use crate::hal::probe_snapshot::{CpuProbe, HalProbeSnapshot};
use crate::hal::traits::{
    AllocationId, AllocationSpec, ComputePrecision, HalError, HardwareUnit, InferenceBatch,
    InferenceId, PowerState, PowerStateKind, SMOKE_INFER_OP,
};

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
    /// Construct with host probes when `linux-hal-sysfs` is enabled; otherwise empty snapshot.
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
        Self::from_cpu_probe(&snapshot.cpu)
    }

    /// Pure assembly from CPU probe fields.
    pub fn from_cpu_probe(cpu: &CpuProbe) -> Self {
        let power_readable = matches!(cpu.rapl_permission, PermissionState::Granted);
        let mut models = Vec::new();
        if let Some(name) = &cpu.model_name {
            models.push(format!("intel-cpu#{name}"));
        }
        if models.is_empty() {
            models.push("intel-cpu#unknown".to_string());
        }
        if !power_readable {
            models.push("power_readable=false".to_string());
        }
        models.push(format!(
            "intel-cpu#reg#cores={}#l3_kb={}",
            cpu.logical_cores, cpu.l3_cache_kb
        ));
        Self {
            next_id: AtomicU64::new(1),
            rapl_uj: cpu.rapl_uj,
            power_readable,
            permission: cpu.rapl_permission.clone(),
            models,
            rapl_path: cpu.rapl_path.clone(),
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
