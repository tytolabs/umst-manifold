//! Optional mmap-backed arena load (`feature = "mmap"`).

use std::fs::File;
use std::path::Path;

use memmap2::Mmap;

use crate::load::{load_arena, UmstArenaView};
use crate::ArenaError;

/// Owned mmap mapping with parsed arena view (Warm boundary — parse once).
pub struct MmappedArena {
    _file: File,
    map: Mmap,
}

impl MmappedArena {
    /// Parsed borrowed view into the mmap backing store.
    pub fn view(&self) -> Result<UmstArenaView<'_>, ArenaError> {
        load_arena(self.map.as_ref())
    }

    /// UCRS commit stamp witness (ABI v1 reserved bytes 12..20, little-endian).
    ///
    /// Zero means unset (synthetic / pre-commit). Non-zero stamps are monotonic HLC
    /// witnesses when written by a UCRS sidecar at arena commit time.
    #[inline]
    pub fn commit_stamp(&self) -> u64 {
        read_commit_stamp(self.map.as_ref())
    }
}

/// Map `path` read-only and parse the arena header + sections once.
pub fn mmap_arena_path(path: &Path) -> Result<MmappedArena, ArenaError> {
    let file = File::open(path).map_err(|source| ArenaError::Io {
        path: path.display().to_string(),
        source,
    })?;
    let map = unsafe {
        Mmap::map(&file).map_err(|source| ArenaError::Io {
            path: path.display().to_string(),
            source,
        })?
    };
    // Validate layout before returning handle.
    load_arena(map.as_ref())?;
    Ok(MmappedArena { _file: file, map })
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::header::{ARENA_ABI_VERSION, ARENA_HEADER_BYTES, ARENA_MAGIC};
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn fixture_with_stamp(stamp: u64) -> Vec<u8> {
        let mut buf = vec![0u8; ARENA_HEADER_BYTES + 8];
        buf[0..4].copy_from_slice(&ARENA_MAGIC.to_le_bytes());
        buf[4..8].copy_from_slice(&ARENA_ABI_VERSION.to_le_bytes());
        buf[8..12].copy_from_slice(&(ARENA_HEADER_BYTES as u32).to_le_bytes());
        write_commit_stamp(&mut buf, stamp).expect("stamp");
        buf[48..56].copy_from_slice(&(ARENA_HEADER_BYTES as u64).to_le_bytes());
        buf[56..64].copy_from_slice(&8u64.to_le_bytes());
        buf[ARENA_HEADER_BYTES..].fill(0x42);
        buf
    }

    #[test]
    fn commit_stamp_roundtrip() {
        let mut buf = fixture_with_stamp(0);
        write_commit_stamp(&mut buf, 0xDEAD_BEEF_CAFE_0001).expect("write");
        assert_eq!(read_commit_stamp(&buf), 0xDEAD_BEEF_CAFE_0001);
    }

    #[test]
    fn mmap_arena_path_loads_view() {
        let buf = fixture_with_stamp(42);
        let mut tmp = NamedTempFile::new().expect("tmp");
        tmp.write_all(&buf).expect("write");
        tmp.flush().expect("flush");
        let arena = mmap_arena_path(tmp.path()).expect("mmap");
        assert_eq!(arena.commit_stamp(), 42);
        let view = arena.view().expect("view");
        assert_eq!(view.state_bytes(), &[0x42; 8]);
    }
}
