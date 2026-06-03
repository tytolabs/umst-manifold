//! GREEN §14bis.f-S-0 — KEM round-trip property (R-3.4.x).

use umst_math::crypto::kem::ml_kem_768::{decapsulate, encapsulate};

#[test]
fn r341_roundtrip_identity_hook() {
    for _ in 0..1000 {
        let (pk, sk) = umst_math::crypto::kem::ml_kem_768::keypair_bytes().expect("keypair");
        let (ss, ct) = encapsulate(&pk, &[]).expect("encapsulate");
        let ss2 = decapsulate(&sk, &ct).expect("decapsulate");
        assert_eq!(ss, ss2);
    }
}
