//! SHA3-256 (NIST FIPS 202) — Keccak via `sha3` crate.

use sha3::{Digest, Sha3_256};

/// Hash verification failures at typed digest boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum HashError {
    /// Compare-by-hash ε-bisim mismatch (`Crypto.Hash` parity hooks).
    #[error("SHA3-256 digest mismatch")]
    Mismatch,
}

/// Compute SHA3-256 digest (256 bits).
pub fn digest(msg: &[u8]) -> Result<[u8; 32], HashError> {
    let mut hasher = Sha3_256::new();
    hasher.update(msg);
    Ok(hasher.finalize().into())
}
