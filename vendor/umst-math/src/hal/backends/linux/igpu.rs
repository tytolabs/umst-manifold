//! Intel integrated GPU (i915 sysfs / DRM)

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::smoke;
use super::sysfs;
use crate::hal::traits::{
    AllocationId, AllocationSpec, ComputePrecision, HalError, HardwareUnit, InferenceBatch,
    InferenceId, PowerState, PowerStateKind, SMOKE_INFER_OP,
};

/// Intel i915 backed unit (H-9)
pub struct IntelIgpu {
    next: AtomicU64,
    i915_path: PathBuf,
    has_intel_render: bool,
    debug_node_readable: bool,
}

impl Default for IntelIgpu {
    fn default() -> Self {
        Self::new()
    }
}

impl IntelIgpu {
    /// `present` = Intel 0x8086 DRM `render` or `card` node
    pub fn new() -> Self {
        let mut has_intel = false;
        for card in sysfs::list_drm_cards().unwrap_or_default() {
            let v = card.join("device/vendor");
            if let Some(s) = sysfs::pci_vendor_hex(&v) {
                if s.trim() == "0x8086" && card.join("device/uevent").exists() {
                    has_intel = true;
                    break;
                }
            }
        }
        let p = super::sysfs::i915_gpu_info();
        let d = p.exists() && std::fs::read(&p).is_ok();
        Self {
            next: AtomicU64::new(1),
            i915_path: p,
            has_intel_render: has_intel,
            debug_node_readable: d,
        }
    }

    /// Whether iGPU is present (Intel on DRM)
    pub fn is_present(&self) -> bool {
        self.has_intel_render
    }

    /// Whether i915 debug node is readable
    pub fn debug_readable(&self) -> bool {
        self.debug_node_readable
    }
}

impl HardwareUnit for IntelIgpu {
    fn enumerate_models(&self) -> Vec<String> {
        if !self.has_intel_render {
            return Vec::new();
        }
        let mut m = vec![
            "intel-igpu#xe-lpg".to_string(),
            format!("debug_readable={}", self.debug_node_readable),
        ];
        if !self.debug_node_readable {
            m.push("power_probe_note=i915_debugfs_may_require_root".to_string());
        }
        m
    }

    fn supported_precisions(&self) -> Vec<ComputePrecision> {
        vec![
            ComputePrecision::Fp32,
            ComputePrecision::Fp16,
            ComputePrecision::I32,
        ]
    }

    fn allocate(&self, spec: &AllocationSpec) -> Result<AllocationId, HalError> {
        if spec.bytes == 0 {
            return Err(HalError::NotApplicable);
        }
        if !self.has_intel_render {
            return Err(HalError::NotApplicable);
        }
        Ok(AllocationId(self.next.fetch_add(1, Ordering::Relaxed)))
    }

    fn infer(&self, _a: AllocationId, b: &InferenceBatch) -> Result<InferenceId, HalError> {
        if b.op != SMOKE_INFER_OP {
            return Err(HalError::NotConfigured);
        }
        if !self.has_intel_render {
            return Err(HalError::NotApplicable);
        }
        smoke::run_smoke();
        Ok(InferenceId(1))
    }

    fn deallocate(&self, _id: AllocationId) -> Result<(), HalError> {
        Ok(())
    }

    fn power_state(&self) -> Result<PowerState, HalError> {
        if !self.has_intel_render {
            return Err(HalError::NotApplicable);
        }
        if !self.debug_node_readable {
            return Ok(PowerState {
                headroom: 0.0,
                kind: PowerStateKind::P2,
                reason: Some(
                    "permission_or_missing: /sys/kernel/debug/.../i915_gpu_info".to_string(),
                ),
            });
        }
        Ok(PowerState {
            headroom: 0.4,
            kind: PowerStateKind::P0,
            reason: None,
        })
    }

    fn drift_window(&self) -> u64 {
        4
    }
}

impl IntelIgpu {
    pub fn i915_path_str(&self) -> String {
        self.i915_path.to_string_lossy().into_owned()
    }
}
