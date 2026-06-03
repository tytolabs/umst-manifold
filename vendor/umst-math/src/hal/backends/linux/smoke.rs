//! WorkloadKind::Smoke — 1 KiB copy round-trip; pure CPU-side, no I/O in the body.

const SMOKE_BYTES: usize = 1024;

pub(crate) fn run_smoke() {
    let a: Vec<u8> = (0u8..=255).cycle().take(SMOKE_BYTES).collect();
    let mut b = [0u8; SMOKE_BYTES];
    b.copy_from_slice(&a);
    debug_assert_eq!(a.as_slice(), b.as_slice());
}
