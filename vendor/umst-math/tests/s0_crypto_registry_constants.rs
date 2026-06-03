//! §14bis.f-S-0 — REGISTRY mirrors byte-width anchors for ε-bisim + §24a.

use pqcrypto_dilithium::dilithium3;
use pqcrypto_sphincsplus::sphincssha2128ssimple;
use umst_math::constants::registry::REGISTRY;
use umst_math::crypto::kem::ml_kem_768::{
    ML_KEM_768_CIPHERTEXT_BYTES, ML_KEM_768_PUBLIC_KEY_BYTES, ML_KEM_768_SECRET_KEY_BYTES,
};
use umst_math::crypto::sig::ml_dsa_65;
use umst_math::crypto::sig::slh_dsa_128s;

#[test]
fn r391_registry_matches_ml_kem_constants() {
    let pk = REGISTRY
        .iter()
        .find(|e| e.name == "crypto_ml_kem_768_public_key_bytes")
        .expect("registry row");
    assert!(pk.expression.contains("1184"));
    assert_eq!(ML_KEM_768_PUBLIC_KEY_BYTES, 1184);
    assert_eq!(ML_KEM_768_SECRET_KEY_BYTES, 2400);
    assert_eq!(ML_KEM_768_CIPHERTEXT_BYTES, 1088);
}

#[test]
fn r392_registry_matches_ml_dsa_slh_and_sha3() {
    assert_eq!(
        dilithium3::public_key_bytes(),
        REGISTRY
            .iter()
            .find(|e| e.name == "crypto_ml_dsa_65_public_key_bytes")
            .expect("row")
            .expression
            .split_whitespace()
            .next()
            .expect("token")
            .parse::<usize>()
            .expect("pk bytes")
    );
    assert_eq!(
        dilithium3::secret_key_bytes(),
        REGISTRY
            .iter()
            .find(|e| e.name == "crypto_ml_dsa_65_secret_key_bytes")
            .expect("row")
            .expression
            .split_whitespace()
            .next()
            .expect("token")
            .parse::<usize>()
            .expect("sk bytes")
    );
    assert_eq!(
        sphincssha2128ssimple::public_key_bytes(),
        REGISTRY
            .iter()
            .find(|e| e.name == "crypto_slh_dsa_128s_public_key_bytes")
            .expect("row")
            .expression
            .split_whitespace()
            .next()
            .expect("token")
            .parse::<usize>()
            .expect("pk bytes")
    );
    let d = REGISTRY
        .iter()
        .find(|e| e.name == "crypto_sha3_256_digest_bytes")
        .expect("row");
    assert!(d.expression.starts_with("32"));
}

#[test]
fn r393_sig_surface_keypair_smoke() {
    let _ = ml_dsa_65::keypair_bytes;
    let _ = slh_dsa_128s::keypair_bytes;
}
