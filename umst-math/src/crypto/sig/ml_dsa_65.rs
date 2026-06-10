//! ML-DSA-65 engineering lane — PQClean **Dilithium3** (`pqcrypto-dilithium`).
//!
//! NIST FIPS 204 parameter class maps to Dilithium3 in PQClean (`ML-DSA-65` slice naming).

use pqcrypto_dilithium::dilithium3::{self};
use pqcrypto_traits::sign::{
    DetachedSignature as DetSigTrait, PublicKey as SignPk, SecretKey as SignSk,
    VerificationError as VErr,
};

/// ML-DSA (Dilithium3) signature-specific errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SigError {
    #[error("malformed ML-DSA input")]
    MalformedInput,
    #[error("ML-DSA verification failed")]
    Invalid,
}

fn bad_pk_sk_pair() -> SigError {
    SigError::Invalid
}

/// Detached-sign `msg` with secret key; `pk` must be the matching public key (pair-wise consistency).
pub fn sign(msg: &[u8], sk: &[u8], pk: &[u8]) -> Result<Vec<u8>, SigError> {
    let sk = SignSk::from_bytes(sk).map_err(|_| SigError::MalformedInput)?;
    let pk = SignPk::from_bytes(pk).map_err(|_| SigError::MalformedInput)?;
    let sig = dilithium3::detached_sign(msg, &sk);
    dilithium3::verify_detached_signature(&sig, msg, &pk).map_err(|e| match e {
        VErr::InvalidSignature => bad_pk_sk_pair(),
        VErr::UnknownVerificationError => SigError::Invalid,
        _ => SigError::Invalid,
    })?;
    Ok(DetSigTrait::as_bytes(&sig).to_vec())
}

/// Random ML-DSA-65 (Dilithium3) keypair bytes.
#[must_use]
pub fn keypair_bytes() -> (Vec<u8>, Vec<u8>) {
    let (pk, sk) = dilithium3::keypair();
    (
        SignPk::as_bytes(&pk).to_vec(),
        SignSk::as_bytes(&sk).to_vec(),
    )
}

pub fn verify(sig: &[u8], msg: &[u8], pk: &[u8]) -> Result<(), SigError> {
    let pk = SignPk::from_bytes(pk).map_err(|_| SigError::MalformedInput)?;
    let sig = DetSigTrait::from_bytes(sig).map_err(|_| SigError::MalformedInput)?;
    dilithium3::verify_detached_signature(&sig, msg, &pk).map_err(|e| match e {
        VErr::InvalidSignature => SigError::Invalid,
        VErr::UnknownVerificationError => SigError::Invalid,
        _ => SigError::Invalid,
    })
}
