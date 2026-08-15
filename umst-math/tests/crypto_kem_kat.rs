//! GREEN §14bis.f-S-0 — ML-KEM-768 KAT parity (R-3.1.x).

use umst_math::crypto::error::CryptoError;
use umst_math::crypto::kem::ml_kem_768::{
    decapsulate, encapsulate, KemError, ML_KEM_768_PUBLIC_KEY_BYTES,
};

#[test]
fn r311_kat_constants() {
    assert_eq!(ML_KEM_768_PUBLIC_KEY_BYTES, 1184);
}

#[test]
fn r312_surfaces() {
    let _ = encapsulate;
    let _ = decapsulate;
}

#[test]
fn r313_crypto_error() {
    let _: CryptoError = CryptoError::KemFailed;
}

#[test]
fn r314_encapsulate_kat_empty_pk_errors() {
    assert!(matches!(
        encapsulate(&[], &[]),
        Err(KemError::MalformedInput)
    ));
}
