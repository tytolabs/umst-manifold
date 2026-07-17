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

/// Classify a sysfs read attempt (pure; no I/O).
pub fn classify_read_probe(
    read_result: Result<Vec<u8>, std::io::Error>,
    label: &'static str,
) -> PermissionState {
    match read_result {
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
