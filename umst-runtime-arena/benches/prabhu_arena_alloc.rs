// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! RW-FP-PRABHU PB-3 — arena `load_arena` hot-loop timing stub.
//!
//! Prints `arena_100_loads_sec` for transcript collection (PB-S3).

use umst_runtime_arena::{load_arena, ARENA_ABI_VERSION, ARENA_HEADER_BYTES, ARENA_MAGIC};

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

fn main() {
    let iters: usize = std::env::var("UMST_ARENA_HOT_ITERS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10_000);

    let buf = fixture();
    let view = load_arena(&buf).expect("load_arena");

    // Warm-up
    for _ in 0..100 {
        let _ = view.state_bytes();
    }

    let start = std::time::Instant::now();
    for _ in 0..iters {
        let _ = view.state_bytes();
    }
    let elapsed = start.elapsed().as_secs_f64();
    let arena_100_loads_sec = elapsed * (100.0 / iters as f64);

    println!("arena_100_loads_sec={arena_100_loads_sec:.6}");
    eprintln!("prabhu_pb3_ok iters={iters} sec={elapsed:.6}");
}
