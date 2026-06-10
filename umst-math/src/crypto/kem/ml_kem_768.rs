//! ML-KEM-768 — PQClean `kyber768` (`pqcrypto-kyber`).
//!
//! THEOREM-BOUND hooks cite `Crypto/KEM.lean` upstream (`umst-formal` L-S0 stubs).

use pqcrypto_kyber::kyber768::{self};
use pqcrypto_traits::kem::{
    Ciphertext as KemCt, PublicKey as KemPk, SecretKey as KemSk, SharedSecret as KemSs,
};
use pqcrypto_traits::Error as PqError;

/// NIST ML-KEM-768 public key byte length (`CRYPTO_PUBLICKEYBYTES`).
pub const ML_KEM_768_PUBLIC_KEY_BYTES: usize = kyber768::public_key_bytes();
/// Secret key byte length.
pub const ML_KEM_768_SECRET_KEY_BYTES: usize = kyber768::secret_key_bytes();
/// Ciphertext byte length.
pub const ML_KEM_768_CIPHERTEXT_BYTES: usize = kyber768::ciphertext_bytes();
/// Shared-secret byte length.
pub const ML_KEM_768_SHARED_SECRET_BYTES: usize = kyber768::shared_secret_bytes();

/// ML-KEM-specific error lane (maps into [`crate::crypto::error::CryptoError`] at ε-bisim borders).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum KemError {
    /// PQClean object sizes disagree with caller-supplied buffers.
    #[error("malformed ML-KEM input")]
    MalformedInput,
    /// Decapsulation rejected (non-canonical ciphertext wiring).
    #[error("ML-KEM encapsulation or decapsulation failed")]
    KemFailed,
}

fn map_parse<T>(r: Result<T, PqError>) -> Result<T, KemError> {
    r.map_err(|_| KemError::MalformedInput)
}

/// Encapsulate to a peer ML-KEM-768 public key using PQClean RNG (`randombytes`).
///
/// `entropy` is reserved for harness-visible deterministic modes; production callers pass `&[]`.
/// Returns `(shared_secret, ciphertext)` — matching PQClean `(ss, ct)` ordering.
pub fn encapsulate(pk: &[u8], _entropy: &[u8]) -> Result<(Vec<u8>, Vec<u8>), KemError> {
    let pk = map_parse(KemPk::from_bytes(pk))?;
    let (ss, ct) = kyber768::encapsulate(&pk);
    Ok((KemSs::as_bytes(&ss).to_vec(), KemCt::as_bytes(&ct).to_vec()))
}

pub fn decapsulate(sk: &[u8], ct: &[u8]) -> Result<Vec<u8>, KemError> {
    let sk = map_parse(KemSk::from_bytes(sk))?;
    let ct = map_parse(KemCt::from_bytes(ct))?;
    let ss = kyber768::decapsulate(&ct, &sk);
    Ok(KemSs::as_bytes(&ss).to_vec())
}

/// Random keypair bytes (`crypto_kem_keypair`).
pub fn keypair_bytes() -> Result<(Vec<u8>, Vec<u8>), KemError> {
    let (pk, sk) = kyber768::keypair();
    Ok((KemPk::as_bytes(&pk).to_vec(), KemSk::as_bytes(&sk).to_vec()))
}
