// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! `HardwareUnit` backend implementations. Linux/Intel: §14bis.f-H-9. macOS: H-9-mac (future).

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(all(target_os = "linux", feature = "linux-hal-sysfs"))]
pub use linux::build_linux_inventory;

#[cfg(target_os = "linux")]
pub use linux::build_linux_inventory_from_snapshot;
