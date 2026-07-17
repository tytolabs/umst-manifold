//! Linux H-9 permission probes — re-export shared ADT.

pub use crate::hal::permission_state::{classify_read_probe, PermissionState};

#[cfg(feature = "linux-hal-sysfs")]
pub use crate::hal::probe_host::read_probe;
