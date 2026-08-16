// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! UCRS commit stamp read/write (header bytes 12..20, little-endian u64).

use crate::load::load_arena;
use crate::ArenaError;

/// Read optional commit stamp from header reserved field (offset 12, 8 bytes).
pub fn read_commit_stamp(bytes: &[u8]) -> u64 {
    if bytes.len() < 20 {
        return 0;
    }
    u64::from_le_bytes([
        bytes[12], bytes[13], bytes[14], bytes[15], bytes[16], bytes[17], bytes[18], bytes[19],
    ])
}

/// Write commit stamp into arena bytes in-place (Warm commit hook; caller owns buffer).
pub fn write_commit_stamp(bytes: &mut [u8], stamp: u64) -> Result<(), ArenaError> {
    if bytes.len() < 20 {
        return Err(ArenaError::BufferTooShort {
            need: 20,
            got: bytes.len(),
        });
    }
    bytes[12..20].copy_from_slice(&stamp.to_le_bytes());
    Ok(())
}

/// Seal arena commit at egress: write stamp then re-validate via [`load_arena`].
pub fn seal_arena_commit(bytes: &mut [u8], stamp: u64) -> Result<(), ArenaError> {
    write_commit_stamp(bytes, stamp)?;
    load_arena(bytes)?;
    Ok(())
}
