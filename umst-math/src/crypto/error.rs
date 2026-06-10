//! Unified cryptographic surface errors (`§14bis.f-S-0`).

/// Crate-wide cryptographic failures surfaced to ε-bisim witnesses (`egoff` §24).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum CryptoError {
    /// ML-KEM encapsulation or decapsulation failed after parsing inputs.
    #[error("post-quantum KEM failed")]
    KemFailed,
    /// ML-DSA / SLH-DSA verification rejected the signature.
    #[error("post-quantum signature invalid")]
    SigInvalid,
    /// Compare-by-hash gate rejected a digest (`CryptoHash` parity hooks).
    #[error("cryptographic hash mismatch")]
    HashMismatch,
    /// Length or structural parse failure prior to invoking PQClean.
    #[error("malformed cryptographic input")]
    MalformedInput,
}
