// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! W8 publish prep script: must exist and exit 0 on the current local workspace.

use std::path::PathBuf;
use std::process::Command;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn w8_publish_readiness_script_is_present_and_documents_prep_vs_publish() {
    let script = manifest_dir().join("scripts/publish_readiness.sh");
    let body = std::fs::read_to_string(&script).expect(
        "scripts/publish_readiness.sh readable for module_count/digest pin + prep-vs-publish contract scan (FP §6 W8 publish readiness)",
    );
    assert!(
        body.contains("module_count=129") || body.contains("module_count=122"),
        "script must pin module_count 129 (or legacy 122)"
    );
    assert!(
        body.contains("17a6d8e1")
            || body.contains("c61b1bef")
            || body.contains("2f17cdf1")
            || body.contains("ef0ed071")
            || body.contains("37bf5a18")
            || body.contains("4524ed21")
            || body.contains("0697014f"),
        "script must pin catalog digest prefix from lock"
    );
    assert!(
        body.contains("manifest-bridge"),
        "script must run manifest-bridge tests"
    );
    assert!(
        body.contains("secrets hygiene") || body.contains("dirty secrets"),
        "script must scan for secrets"
    );
    assert!(
        body.contains("staged secret-like") || body.contains("staged files"),
        "script must scan staged files for .env / credentials"
    );
    assert!(
        body.contains("verify_umst_stack.sh"),
        "script must reference verify_umst_stack.sh for 16/16 evidence"
    );
    let verify = std::fs::read_to_string(manifest_dir().join("scripts/verify_umst_stack.sh"))
        .expect(
            "scripts/verify_umst_stack.sh readable — must invoke w8_publish_readiness.sh for 16/16 evidence (FP §6 W8 publish readiness)",
        );
    assert!(
        verify.contains("w8_publish_readiness.sh"),
        "verify_umst_stack.sh must invoke w8_publish_readiness.sh"
    );
    assert!(
        body.contains("16/16"),
        "script must assert 16/16 checklist evidence"
    );
    assert!(
        body.contains("publish remains human") || body.contains("operator publish"),
        "script must label publish as human-only"
    );
}

#[test]
fn w8_publish_readiness_exits_zero_on_current_workspace() {
    let manifest = manifest_dir();
    let script = manifest.join("scripts/publish_readiness.sh");
    let workspace = manifest
        .parent()
        .expect(
            "umst-manifold CARGO_MANIFEST_DIR parent is multi-repo workspace root with umst-concrete-cartridge sibling (FP §6 W8 publish readiness)",
        );
    let concrete = workspace.join("umst-concrete-cartridge");
    if !concrete.join("Cargo.toml").is_file() {
        eprintln!(
            "SKIP: umst-concrete-cartridge absent at {} (CI / standalone clone); prep script not run",
            concrete.display()
        );
        return;
    }

    let out = Command::new("bash")
        .arg(&script)
        .current_dir(&manifest)
        .output()
        .expect(
            "bash scripts/publish_readiness.sh exits 0 on current workspace when umst-concrete-cartridge present (FP §6 W8 publish readiness)",
        );

    if !out.status.success() {
        eprintln!(
            "stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    assert!(
        out.status.success(),
        "w8_publish_readiness.sh must exit 0 (status={:?})",
        out.status.code()
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("w8_publish_readiness: READY"),
        "expected READY banner in stdout"
    );
}
