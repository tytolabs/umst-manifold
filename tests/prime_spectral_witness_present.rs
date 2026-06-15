// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! CI durability guard: declared prime-spectral modules must exist on disk.

#[test]
fn prime_spectral_witness_files_present() {
    let root = env!("CARGO_MANIFEST_DIR");
    let filter = format!("{root}/src/physics/prime_spectral_filter.rs");
    let research = format!("{root}/src/physics/prime_spectral_research.rs");
    let ntt = format!("{root}/src/physics/prime_spectral_ntt.rs");
    let qmc = format!("{root}/src/physics/prime_spectral_qmc.rs");
    assert!(
        std::path::Path::new(&filter).is_file(),
        "missing {filter}"
    );
    assert!(
        std::path::Path::new(&research).is_file(),
        "missing {research}"
    );
    assert!(std::path::Path::new(&ntt).is_file(), "missing {ntt}");
    assert!(std::path::Path::new(&qmc).is_file(), "missing {qmc}");
}
