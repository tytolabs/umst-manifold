//! `HardwareUnit` backend implementations. Linux/Intel: §14bis.f-H-9. macOS: H-9-mac (future).

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
pub use linux::build_linux_inventory;
