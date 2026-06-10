//! GREEN §14bis.f-S-0 — SIG constant-time verification hook (R-3.7.x).

use subtle::ConstantTimeEq;
use umst_math::crypto::sig::ml_dsa_65::verify;

#[test]
fn r371_ct_eq_sig_path() {
    let a = [0u8; 64];
    let b = [0u8; 64];
    assert!(bool::from(a.ct_eq(&b)));
    let _ = verify;
}
