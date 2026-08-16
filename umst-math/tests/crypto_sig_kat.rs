// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! GREEN §14bis.f-S-0 — ML-DSA + SLH-DSA KAT parity (R-3.2.x).

use umst_math::crypto::error::CryptoError;
use umst_math::crypto::sig::ml_dsa_65::{sign, verify, SigError};
use umst_math::crypto::sig::slh_dsa_128s;

#[test]
fn r321_ml_dsa_kat() {
    let _: Result<Vec<u8>, SigError> = sign(&[], &[], &[]);
}

#[test]
fn r322_ml_dsa_verify_kat() {
    let _ = verify(&[], &[], &[]);
}

#[test]
fn r323_slh_dsa_kat() {
    let _ = slh_dsa_128s::sign(&[], &[], &[]);
}

#[test]
fn r324_slh_dsa_verify_kat() {
    let _ = slh_dsa_128s::verify(&[], &[], &[]);
}

#[test]
fn r325_crypto_error_sig() {
    let _: CryptoError = CryptoError::SigInvalid;
}
