//! UMST runtime arena — Warm-boundary parse of versioned arena bytes.
//!
//! Cold agents hand off an owned byte buffer; [`load_arena`] validates the
//! [`UmstArenaHeader`] once and returns a borrowed [`UmstArenaView`] for
//! in-process hot loops. Optional [`mmap_arena_path`](crate::mmap_arena_path) (`feature = "mmap"`)
//! maps a file read-only at the Warm boundary; UCRS commit stamps live in header bytes 12..20.
//!
//! # Crate invariants
//! - **`#![forbid(unsafe_code)]`** on default build; `feature = "mmap"` permits bounded `unsafe` for `memmap2`.
//! - Total functions at the parse boundary — [`ArenaError`] instead of panics.

#![cfg_attr(not(feature = "mmap"), forbid(unsafe_code))]
#![warn(missing_docs)]

mod error;
mod header;
mod load;
#[cfg(feature = "mmap")]
mod mmap;
mod stamp;

pub use error::ArenaError;
pub use header::{UmstArenaHeader, ARENA_ABI_VERSION, ARENA_HEADER_BYTES, ARENA_MAGIC};
pub use load::{load_arena, UmstArenaView};
#[cfg(feature = "mmap")]
pub use mmap::{mmap_arena_path, MmappedArena};
pub use stamp::{read_commit_stamp, seal_arena_commit, write_commit_stamp};
