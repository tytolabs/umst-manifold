//! Typed failures at the Warm parse boundary.

use thiserror::Error;

/// Arena load / layout validation errors (total — no panics).
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum ArenaError {
    /// Byte slice shorter than the fixed header.
    #[error("buffer too short: need at least {need} bytes, got {got}")]
    BufferTooShort {
        /// Minimum bytes required for `UmstArenaHeader`.
        need: usize,
        /// Actual byte length supplied.
        got: usize,
    },
    /// Magic field does not match [`crate::ARENA_MAGIC`].
    #[error("invalid arena magic: expected 0x{expected:08X}, got 0x{got:08X}")]
    BadMagic {
        /// Expected little-endian magic.
        expected: u32,
        /// Observed magic.
        got: u32,
    },
    /// `abi_version` is not supported by this crate revision.
    #[error("unsupported arena ABI version: {0}")]
    UnsupportedAbiVersion(u32),
    /// A section offset + length falls outside the backing buffer.
    #[error("section {section} out of bounds: offset={offset}, len={len}, buffer={buffer}")]
    SectionOutOfBounds {
        /// Human-readable section name (`state`, `proposal`, …).
        section: &'static str,
        /// Byte offset from arena start.
        offset: u64,
        /// Declared section length.
        len: u64,
        /// Total backing buffer length.
        buffer: usize,
    },
}
