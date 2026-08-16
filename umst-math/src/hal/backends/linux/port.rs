// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! USB + net enumeration (honest count)

use std::sync::atomic::{AtomicU64, Ordering};

use super::smoke;
use crate::hal::probe_snapshot::{HalProbeSnapshot, PortProbe};
use crate::hal::traits::{
    AllocationId, AllocationSpec, ComputePrecision, HalError, HardwareUnit, InferenceBatch,
    InferenceId, PowerState, PowerStateKind, SMOKE_INFER_OP,
};

/// Port cluster (net + USB device entries)
pub struct LinuxPort {
    next: AtomicU64,
    net: usize,
    usb: usize,
}

impl Default for LinuxPort {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxPort {
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
        Self::from_port_probe(&snapshot.port)
    }

    /// Pure assembly from port probe fields.
    pub fn from_port_probe(port: &PortProbe) -> Self {
        Self {
            next: AtomicU64::new(1),
            net: port.net,
            usb: port.usb,
        }
    }

    pub fn total_ports(&self) -> usize {
        self.net.saturating_add(self.usb)
    }
}

impl HardwareUnit for LinuxPort {
    fn enumerate_models(&self) -> Vec<String> {
        if self.total_ports() == 0 {
            return vec!["port#empty".to_string()];
        }
        vec![format!("linux-port#net={} usb={}", self.net, self.usb)]
    }

    fn supported_precisions(&self) -> Vec<ComputePrecision> {
        vec![ComputePrecision::I32]
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
        Ok(PowerState {
            headroom: 0.0,
            kind: PowerStateKind::P0,
            reason: Some("port_power_ned_n_a".to_string()),
        })
    }

    fn drift_window(&self) -> u64 {
        1
    }
}
