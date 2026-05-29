// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! W8 publish prep script: must exist and exit 0 on the current MaOS workspace.

use std::path::PathBuf;
use std::process::Command;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn w8_publish_readiness_script_is_present_and_documents_prep_vs_publish() {
    let script = manifest_dir().join("scripts/w8_publish_readiness.sh");
    let body = std::fs::read_to_string(&script).expect("w8_publish_readiness.sh");
    assert!(
        body.contains("module_count=119") || body.contains("module_count\": 119"),
        "script must pin module_count 119"
    );
    assert!(
        body.contains("0697014f"),
        "script must pin digest prefix 0697014f"
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
        .expect("verify_umst_stack.sh");
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
    let script = manifest.join("scripts/w8_publish_readiness.sh");
    let workspace = manifest.parent().expect("umst-manifold parent = MaOS-Workspace");
    let concrete = workspace.join("umst-concrete-cartridge");
    if !concrete.join("Cargo.toml").is_file() {
        panic!(
            "umst-concrete-cartridge missing at {}; cannot verify manifest-bridge",
            concrete.display()
        );
    }

    let out = Command::new("bash")
        .arg(&script)
        .current_dir(&manifest)
        .output()
        .expect("run w8_publish_readiness.sh");

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
