//! Warm-boundary `load_arena` — parse once, borrow for hot loops.

use crate::header::UmstArenaHeader;
use crate::ArenaError;

/// Borrowed view into a loaded arena backing buffer.
///
/// `Send + Sync` and zero-allocation on hot paths: holds only references and
/// the parsed header; no per-step heap traffic.
#[derive(Debug, Clone, Copy)]
pub struct UmstArenaView<'a> {
    bytes: &'a [u8],
    header: UmstArenaHeader,
}

impl<'a> UmstArenaView<'a> {
    /// Full backing slice (header + payload sections).
    #[inline]
    pub fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Parsed header (catalog digest + section offsets).
    #[inline]
    pub fn header(&self) -> &UmstArenaHeader {
        &self.header
    }

    /// UMST state section as a sub-slice (empty when `state_bytes == 0`).
    #[inline]
    pub fn state_bytes(&self) -> &'a [u8] {
        let start = self.header.state_offset as usize;
        let end = start + self.header.state_bytes as usize;
        &self.bytes[start..end]
    }
}

/// Parse `bytes` at the Warm boundary into a borrowed [`UmstArenaView`].
///
/// Pure parse stub: no mmap, no I/O, no allocation beyond the caller-owned buffer.
pub fn load_arena(bytes: &[u8]) -> Result<UmstArenaView<'_>, ArenaError> {
    let header = UmstArenaHeader::parse(bytes)?;
    header.validate_sections(bytes.len())?;
    Ok(UmstArenaView { bytes, header })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::header::{ARENA_ABI_VERSION, ARENA_HEADER_BYTES, ARENA_MAGIC};

    fn fixture() -> Vec<u8> {
        let mut buf = vec![0u8; ARENA_HEADER_BYTES + 8];
        buf[0..4].copy_from_slice(&ARENA_MAGIC.to_le_bytes());
        buf[4..8].copy_from_slice(&ARENA_ABI_VERSION.to_le_bytes());
        buf[8..12].copy_from_slice(&(ARENA_HEADER_BYTES as u32).to_le_bytes());
        buf[48..56].copy_from_slice(&(ARENA_HEADER_BYTES as u64).to_le_bytes());
        buf[56..64].copy_from_slice(&8u64.to_le_bytes());
        buf[ARENA_HEADER_BYTES..].fill(0x42);
        buf
    }

    #[test]
    fn load_returns_state_slice() {
        let buf = fixture();
        let view = load_arena(&buf).expect(
            "load_arena on synthetic v1 fixture with state section (FP §6 load_returns_state_slice witness)",
        );
        assert_eq!(view.state_bytes(), &[0x42; 8]);
    }

    #[test]
    fn seal_arena_commit_roundtrip_on_load_path() {
        use crate::stamp::{read_commit_stamp, seal_arena_commit};

        let mut buf = fixture();
        seal_arena_commit(&mut buf, 0xCAFE_BABE_0000_0001).expect(
            "seal_arena_commit on synthetic v1 fixture (FP §6 seal_arena_commit_roundtrip witness)",
        );
        assert_eq!(read_commit_stamp(&buf), 0xCAFE_BABE_0000_0001);
        let view = load_arena(&buf).expect(
            "load_arena after seal_arena_commit on synthetic fixture (FP §6 seal_arena_commit_roundtrip witness)",
        );
        assert_eq!(view.state_bytes(), &[0x42; 8]);
    }

    #[test]
    fn bench_load_arena_hot_loop() {
        let iters: usize = std::env::var("UMST_ARENA_HOT_ITERS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(100);
        let buf = fixture();
        let view = load_arena(&buf).expect(
            "load_arena on synthetic v1 fixture for hot-loop timing (FP §6 bench_load_arena_hot_loop witness)",
        );
        let start = std::time::Instant::now();
        for _ in 0..iters {
            let _ = view.state_bytes();
        }
        let elapsed = start.elapsed().as_secs_f64();
        eprintln!("arena_hot_loop_ok iters={iters} sec={elapsed:.6}");
        println!(
            "arena_100_loads_sec {:.6}",
            elapsed * (100.0 / iters as f64)
        );
    }
}
