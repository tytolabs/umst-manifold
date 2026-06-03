//! H-9 permission state — typed sum (I1: not booleans only).

/// Probe outcome for a sysfs/udev path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PermissionState {
    /// Read succeeded
    Granted,
    /// e.g. EACCES; path is not secret at compile time
    Denied {
        sysfs_path: &'static str,
        errno: i32,
    },
    /// Not probed
    Untested,
    /// Path does not apply
    NotApplicable,
}

/// Read entire small file: success → `Granted`; EACCES → `Denied(13)`.
pub fn read_probe(path: &std::path::Path, label: &'static str) -> PermissionState {
    if !path.exists() {
        return PermissionState::NotApplicable;
    }
    match std::fs::read(path) {
        Ok(_) => PermissionState::Granted,
        Err(e)
            if e.raw_os_error() == Some(13) || e.kind() == std::io::ErrorKind::PermissionDenied =>
        {
            PermissionState::Denied {
                sysfs_path: label,
                errno: e.raw_os_error().unwrap_or(13),
            }
        }
        Err(_) => PermissionState::Untested,
    }
}
