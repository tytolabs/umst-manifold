//! # Hardware Abstraction Layer — category **𝓗** (FORWARD-PLAN v1.2 §0.1–0.2; **§14bis.f-H-8**)
//!
//! Trait surface *only* — no RAPL, NVML, IOKit, or Core ML. Backends: H-9, H-9-mac.
//! Pairing: [`mocks`] for two reference arches on the same test host (Q6 C-strict, mocks-only in CI).

/// Linux/Intel `HardwareUnit` impls (§14bis.f-H-9; `cfg(target_os = "linux")` only).
pub mod backends;
#[cfg(all(target_os = "linux", feature = "linux-hal-sysfs"))]
pub use backends::build_linux_inventory;
#[cfg(target_os = "linux")]
pub use backends::build_linux_inventory_from_snapshot;

pub mod inventory;
pub mod kinds;
pub mod laws;
/// PAIRED-ARCH: [`mock_linux_intel_inventory`], [`mock_apple_m3_inventory`]
pub mod mocks;
pub mod permission_state;
pub mod presence;
/// Host sysfs/proc probe IO (`linux-hal-sysfs` feature).
#[cfg(all(target_os = "linux", feature = "linux-hal-sysfs"))]
pub mod probe_host;
/// Pure host probe snapshot — injected at runtime/CLI boundary (FP §4).
#[cfg(target_os = "linux")]
pub mod probe_snapshot;
/// THEOREM-BOUND: B-2.5 [`ArchitectureProfile`], [`ArchClass`]
pub mod profile;
/// ZCI-EXEMPT: the seven-method [`HardwareUnit`]
pub mod traits;

pub use kinds::UnitKind;
pub use laws::{
    check_associative, check_identity_law, compose, example_route_cpu_igpu_ram,
    example_route_ram_port, f_cpu_igpu, g_igpu_ram, h_ram_port, id_route, kcompose, route_id,
    UnitRoute,
};
pub use permission_state::{classify_read_probe, PermissionState};
pub use presence::{AbsentReason, UnitPresence};
#[cfg(all(target_os = "linux", feature = "linux-hal-sysfs"))]
pub use probe_host::probe_sysfs_snapshot;
#[cfg(target_os = "linux")]
pub use probe_snapshot::HalProbeSnapshot;
pub use profile::{ArchClass, ArchitectureProfile, ParetoReferenceLabel};
pub use traits::{
    AllocationId, AllocationSpec, ComputePrecision, HalError, HardwareUnit, InferenceBatch,
    InferenceId, PowerState, PowerStateKind, WorkloadClass, WorkloadKind, SMOKE_INFER_OP,
};

pub use inventory::{HardwareInventory, UnitSlot};
