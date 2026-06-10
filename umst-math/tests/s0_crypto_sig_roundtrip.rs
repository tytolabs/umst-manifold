//! GREEN §14bis.f-S-0 — SIG round-trip property (R-3.5.x).

use umst_math::crypto::sig::ml_dsa_65::{sign, verify};

#[test]
fn r351_sig_roundtrip_hook() {
    let msg = b"S-0 GREEN umst-math/ml-dsa-65 round-trip witness";
    for _ in 0..1000 {
        let (pk, sk) = umst_math::crypto::sig::ml_dsa_65::keypair_bytes();
        let sig = sign(msg, &sk, &pk).expect("sign");
        verify(&sig, msg, &pk).expect("verify");
    }
}
