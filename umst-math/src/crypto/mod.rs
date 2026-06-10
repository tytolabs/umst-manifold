//! §14bis.f-S-0 — Post-quantum cryptographic primitives (Rust engineering mirrors of `umst-formal` L-S0..L-S5 statements).
//!
//! Concrete bindings: PQClean via `pqcrypto-*` (`kyber768`, `dilithium3`, `sphincssha2128ssimple`) + FIPS 202 SHA3-256.

pub mod error;
pub mod hash;
pub mod kem;
pub mod sig;

pub use error::CryptoError;
