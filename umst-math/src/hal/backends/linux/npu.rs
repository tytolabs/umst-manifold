//! Intel AI Boost (accel class) — infer Smoke → NotYet until B-2 OpenVINO binding

use std::sync::atomic::{AtomicU64, Ordering};

use super::sysfs;
use crate::hal::traits::{
    AllocationId, AllocationSpec, ComputePrecision, HalError, HardwareUnit, InferenceBatch,
    InferenceId, PowerState, PowerStateKind, WorkloadKind, SMOKE_INFER_OP,
};

/// Intel NPU (accelerator) — `present` = `/sys/class/accel/accel0` exists
pub struct IntelNpu {
    next: AtomicU64,
    present: bool,
}

impl Default for IntelNpu {
    fn default() -> Self {
        Self::new()
    }
}

impl IntelNpu {
    /// Detect accelerator device
    pub fn new() -> Self {
        let present = sysfs::class_accel0().exists();
        Self {
            next: AtomicU64::new(1),
            present,
        }
    }

    /// Device node exists
    pub fn is_present(&self) -> bool {
        self.present
    }
}

impl HardwareUnit for IntelNpu {
    fn enumerate_models(&self) -> Vec<String> {
        if !self.present {
            return vec![];
        }
        vec!["intel-npu#not_yet_instrumented".to_string()]
    }

    fn supported_precisions(&self) -> Vec<ComputePrecision> {
        vec![ComputePrecision::Fp16, ComputePrecision::Int8]
    }

    fn allocate(&self, spec: &AllocationSpec) -> Result<AllocationId, HalError> {
        if !self.present {
            return Err(HalError::NotConfigured);
        }
        if spec.bytes == 0 {
            return Err(HalError::NotApplicable);
        }
        Ok(AllocationId(self.next.fetch_add(1, Ordering::Relaxed)))
    }

    fn infer(&self, _a: AllocationId, b: &InferenceBatch) -> Result<InferenceId, HalError> {
        if b.op == SMOKE_INFER_OP {
            // Honest: host-side memcpy only; real NPU kernel in B-2
            if self.present {
                return Err(HalError::NotYetImplemented {
                    unit: "IntelNpu".to_string(),
                    workload: format!("{:?}", WorkloadKind::Smoke),
                    reason: "openvino_kernel_pending_B2".to_string(),
                });
            }
        }
        Err(HalError::NotConfigured)
    }

    fn deallocate(&self, _id: AllocationId) -> Result<(), HalError> {
        Ok(())
    }

    fn power_state(&self) -> Result<PowerState, HalError> {
        if !self.present {
            return Err(HalError::NotConfigured);
        }
        // sysfs power not standardized here — do not guess joules
        Ok(PowerState {
            headroom: 0.0,
            kind: PowerStateKind::P0,
            reason: Some("npu_power_unmapped_h9".to_string()),
        })
    }

    fn drift_window(&self) -> u64 {
        8
    }
}
