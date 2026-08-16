// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Fixed-layout arena header (ABI v1).

/// Little-endian magic: ASCII `UMST` (`0x54_53_4D_55`).
pub const ARENA_MAGIC: u32 = 0x5453_4D55;

/// Supported arena ABI revision parsed from the header.
pub const ARENA_ABI_VERSION: u32 = 1;

/// On-wire header size in bytes (fixed for ABI v1).
pub const ARENA_HEADER_BYTES: usize = 64;

/// Versioned arena header — cold/warm boundary only; no mmap in this crate yet.
///
/// Layout (little-endian, 64 bytes total):
///
/// | Offset | Field            | Size |
/// | ------ | ---------------- | ---- |
/// | 0      | `magic`          | 4    |
/// | 4      | `abi_version`    | 4    |
/// | 8      | `header_bytes`   | 4    |
/// | 12     | `_reserved`      | 4    |
/// | 16     | `catalog_digest` | 32   |
/// | 48     | `state_offset`   | 8    |
/// | 56     | `state_bytes`    | 8    |
///
/// Proposal and witness sections follow in a later ABI revision; offsets beyond
/// `state` are reserved for forward-compatible growth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UmstArenaHeader {
    /// Must equal [`ARENA_MAGIC`].
    pub magic: u32,
    /// Must equal [`ARENA_ABI_VERSION`] for this crate revision.
    pub abi_version: u32,
    /// Declared header size on wire (must be [`ARENA_HEADER_BYTES`] for v1).
    pub header_bytes: u32,
    /// SHA-256 of `artifacts/catalog.lock.json` at arena creation time.
    pub catalog_digest: [u8; 32],
    /// Byte offset from arena start to the UMST state blob.
    pub state_offset: u64,
    /// Length of the UMST state blob in bytes.
    pub state_bytes: u64,
}

impl UmstArenaHeader {
    /// Parse a header from the first [`ARENA_HEADER_BYTES`] of `bytes`.
    pub fn parse(bytes: &[u8]) -> Result<Self, crate::ArenaError> {
        if bytes.len() < ARENA_HEADER_BYTES {
            return Err(crate::ArenaError::BufferTooShort {
                need: ARENA_HEADER_BYTES,
                got: bytes.len(),
            });
        }

        let magic = read_u32_le(bytes, 0);
        if magic != ARENA_MAGIC {
            return Err(crate::ArenaError::BadMagic {
                expected: ARENA_MAGIC,
                got: magic,
            });
        }

        let abi_version = read_u32_le(bytes, 4);
        if abi_version != ARENA_ABI_VERSION {
            return Err(crate::ArenaError::UnsupportedAbiVersion(abi_version));
        }

        let header_bytes = read_u32_le(bytes, 8);
        if header_bytes as usize != ARENA_HEADER_BYTES {
            return Err(crate::ArenaError::UnsupportedAbiVersion(abi_version));
        }

        let mut catalog_digest = [0u8; 32];
        catalog_digest.copy_from_slice(&bytes[16..48]);

        Ok(Self {
            magic,
            abi_version,
            header_bytes,
            catalog_digest,
            state_offset: read_u64_le(bytes, 48),
            state_bytes: read_u64_le(bytes, 56),
        })
    }

    /// Validate that the declared state section lies within `buffer_len`.
    pub fn validate_sections(&self, buffer_len: usize) -> Result<(), crate::ArenaError> {
        validate_section("state", self.state_offset, self.state_bytes, buffer_len)
    }
}

fn validate_section(
    section: &'static str,
    offset: u64,
    len: u64,
    buffer_len: usize,
) -> Result<(), crate::ArenaError> {
    let end = offset
        .checked_add(len)
        .ok_or(crate::ArenaError::SectionOutOfBounds {
            section,
            offset,
            len,
            buffer: buffer_len,
        })?;
    if end > buffer_len as u64 {
        return Err(crate::ArenaError::SectionOutOfBounds {
            section,
            offset,
            len,
            buffer: buffer_len,
        });
    }
    Ok(())
}

fn read_u32_le(bytes: &[u8], offset: usize) -> u32 {
    let b = &bytes[offset..offset + 4];
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

fn read_u64_le(bytes: &[u8], offset: usize) -> u64 {
    let b = &bytes[offset..offset + 8];
    u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_header_bytes() -> Vec<u8> {
        let mut buf = vec![0u8; ARENA_HEADER_BYTES + 16];
        buf[0..4].copy_from_slice(&ARENA_MAGIC.to_le_bytes());
        buf[4..8].copy_from_slice(&ARENA_ABI_VERSION.to_le_bytes());
        buf[8..12].copy_from_slice(&(ARENA_HEADER_BYTES as u32).to_le_bytes());
        buf[16..48].fill(0xAB);
        buf[48..56].copy_from_slice(&(ARENA_HEADER_BYTES as u64).to_le_bytes());
        buf[56..64].copy_from_slice(&16u64.to_le_bytes());
        buf
    }

    #[test]
    fn parse_valid_header() {
        let bytes = sample_header_bytes();
        let header = UmstArenaHeader::parse(&bytes).expect(
            "UmstArenaHeader::parse on valid v1 sample bytes (FP §6 parse_valid_header witness)",
        );
        assert_eq!(header.magic, ARENA_MAGIC);
        assert_eq!(header.catalog_digest, [0xAB; 32]);
        assert_eq!(header.state_offset, ARENA_HEADER_BYTES as u64);
        assert_eq!(header.state_bytes, 16);
        header.validate_sections(bytes.len()).expect(
            "validate_sections on valid v1 sample header + payload (FP §6 parse_valid_header witness)",
        );
    }

    #[test]
    fn rejects_bad_magic() {
        let mut bytes = sample_header_bytes();
        bytes[0] = 0;
        assert!(matches!(
            UmstArenaHeader::parse(&bytes),
            Err(crate::ArenaError::BadMagic { .. })
        ));
    }
}
