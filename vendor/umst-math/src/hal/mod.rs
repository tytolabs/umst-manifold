//! # Hardware Abstraction Layer — category **𝓗** (FORWARD-PLAN v1.2 §0.1–0.2; **§14bis.f-H-8**)
//!
//! Trait surface *only* — no RAPL, NVML, IOKit, or Core ML. Backends: H-9, H-9-mac.
//! Pairing: [`mocks`] for two reference arches on the same test host (Q6 C-strict, mocks-only in CI).

/// Linux/Intel `HardwareUnit` impls (§14bis.f-H-9; `cfg(target_os = "linux")` only).
pub mod backends;
#[cfg(target_os = "linux")]
pub use backends::build_linux_inventory;

pub mod inventory;
pub mod kinds;
pub mod laws;
/// PAIRED-ARCH: [`mock_linux_intel_inventory`], [`mock_apple_m3_inventory`]
pub mod mocks;
pub mod presence;
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
pub use presence::{AbsentReason, UnitPresence};
pub use profile::{ArchClass, ArchitectureProfile, ParetoReferenceLabel};
pub use traits::{
    AllocationId, AllocationSpec, ComputePrecision, HalError, HardwareUnit, InferenceBatch,
    InferenceId, PowerState, PowerStateKind, WorkloadClass, WorkloadKind, SMOKE_INFER_OP,
};

pub use inventory::{HardwareInventory, UnitSlot};
