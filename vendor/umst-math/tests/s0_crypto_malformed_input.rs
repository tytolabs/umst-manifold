//! GREEN §14bis.f-S-0 — Malformed-input typed errors (R-3.8.x).

use umst_math::crypto::error::CryptoError;
use umst_math::crypto::hash::sha3_256::digest;
use umst_math::crypto::kem::ml_kem_768::{decapsulate, encapsulate, KemError};
use umst_math::crypto::sig::ml_dsa_65::{verify, SigError};

fn assert_crypto_err(_: CryptoError) {}

#[test]
fn r381_malformed_kem_encap() {
    assert!(matches!(
        encapsulate(&[], &[]),
        Err(KemError::MalformedInput)
    ));
}

#[test]
fn r382_malformed_kem_decap() {
    assert!(matches!(
        decapsulate(&[], &[]),
        Err(KemError::MalformedInput)
    ));
}

#[test]
fn r383_malformed_sig() {
    assert!(matches!(
        verify(&[], &[], &[]),
        Err(SigError::MalformedInput)
    ));
}

#[test]
fn r384_malformed_hash() {
    assert!(digest(&[]).is_ok());
}

#[test]
fn r385_error_variant_kem_failed() {
    assert_crypto_err(CryptoError::KemFailed);
}

#[test]
fn r386_error_variant_sig_invalid() {
    assert_crypto_err(CryptoError::SigInvalid);
}

#[test]
fn r387_error_variant_hash_mismatch() {
    assert_crypto_err(CryptoError::HashMismatch);
}

#[test]
fn r388_error_variant_malformed_input() {
    assert_crypto_err(CryptoError::MalformedInput);
}
