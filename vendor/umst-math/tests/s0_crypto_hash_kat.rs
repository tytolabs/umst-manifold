//! GREEN §14bis.f-S-0 — SHA3-256 KAT (R-3.3.x).

use umst_math::crypto::error::CryptoError;
use umst_math::crypto::hash::sha3_256::{digest, HashError};

/// NIST SHA3-256("") — empty message (FIPS 202).
const EMPTY_SHA3_256: [u8; 32] = [
    0xa7, 0xff, 0xc6, 0xf8, 0xbf, 0x1e, 0xd7, 0x66, 0x51, 0xc1, 0x47, 0x56, 0xa0, 0x61, 0xd6, 0x62,
    0xf5, 0x80, 0xff, 0x4d, 0xe4, 0x3b, 0x49, 0xfa, 0x82, 0xd8, 0x0a, 0x4b, 0x80, 0xf8, 0x43, 0x4a,
];

#[test]
fn r331_digest_kat_a() {
    let _: Result<[u8; 32], HashError> = digest(&[]);
}

#[test]
fn r332_digest_kat_empty_matches_sha3_256_vector() {
    assert_eq!(digest(&[]).expect("sha3-256"), EMPTY_SHA3_256);
}

#[test]
fn r333_crypto_error_hash() {
    let _: CryptoError = CryptoError::HashMismatch;
}
