//! SLH-DSA-128s hook — PQClean **SPHINCS+-SHA2-128s-simple** (`pqcrypto-sphincsplus`).

use pqcrypto_sphincsplus::sphincssha2128ssimple::{self};
use pqcrypto_traits::sign::{
    DetachedSignature as DetSigTrait, PublicKey as SignPk, SecretKey as SignSk,
    VerificationError as VErr,
};

/// SLH-DSA (SPHINCS+ SHA2-128s) signature-specific errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SigError {
    #[error("malformed SLH-DSA input")]
    MalformedInput,
    #[error("SLH-DSA verification failed")]
    Invalid,
}

pub fn sign(msg: &[u8], sk: &[u8], pk: &[u8]) -> Result<Vec<u8>, SigError> {
    let sk = SignSk::from_bytes(sk).map_err(|_| SigError::MalformedInput)?;
    let pk = SignPk::from_bytes(pk).map_err(|_| SigError::MalformedInput)?;
    let sig = sphincssha2128ssimple::detached_sign(msg, &sk);
    sphincssha2128ssimple::verify_detached_signature(&sig, msg, &pk).map_err(|e| match e {
        VErr::InvalidSignature => SigError::Invalid,
        VErr::UnknownVerificationError => SigError::Invalid,
        _ => SigError::Invalid,
    })?;
    Ok(DetSigTrait::as_bytes(&sig).to_vec())
}

/// Random SLH-DSA-128s (SPHINCS+ SHA2-128s-simple) keypair bytes.
pub fn keypair_bytes() -> (Vec<u8>, Vec<u8>) {
    let (pk, sk) = sphincssha2128ssimple::keypair();
    (
        SignPk::as_bytes(&pk).to_vec(),
        SignSk::as_bytes(&sk).to_vec(),
    )
}

pub fn verify(sig: &[u8], msg: &[u8], pk: &[u8]) -> Result<(), SigError> {
    let pk = SignPk::from_bytes(pk).map_err(|_| SigError::MalformedInput)?;
    let sig = DetSigTrait::from_bytes(sig).map_err(|_| SigError::MalformedInput)?;
    sphincssha2128ssimple::verify_detached_signature(&sig, msg, &pk).map_err(|e| match e {
        VErr::InvalidSignature => SigError::Invalid,
        VErr::UnknownVerificationError => SigError::Invalid,
        _ => SigError::Invalid,
    })
}
