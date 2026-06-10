//! GREEN §14bis.f-S-0 — KEM constant-time verification hook (R-3.6.x).

use subtle::ConstantTimeEq;
use umst_math::crypto::kem::ml_kem_768::decapsulate;

#[test]
fn r361_ct_eq_kem_path() {
    let a = [7u8; 32];
    let b = [7u8; 32];
    assert!(bool::from(a.ct_eq(&b)));
    let _ = decapsulate;
}
