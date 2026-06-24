//! UMST runtime arena — Warm-boundary parse of versioned arena bytes.
//!
//! Cold agents hand off an owned byte buffer; [`load_arena`] validates the
//! [`UmstArenaHeader`] once and returns a borrowed [`UmstArenaView`] for
//! in-process hot loops. `mmap` and UCRS commit stamps are deferred.
//!
//! # Crate invariants
//! - **`#![forbid(unsafe_code)]`** — no `unsafe` in this crate.
//! - Total functions at the parse boundary — [`ArenaError`] instead of panics.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod error;
mod header;
mod load;

pub use error::ArenaError;
pub use header::{UmstArenaHeader, ARENA_ABI_VERSION, ARENA_HEADER_BYTES, ARENA_MAGIC};
pub use load::{load_arena, UmstArenaView};
