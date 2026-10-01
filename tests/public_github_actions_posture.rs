// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Public GitHub Actions posture when private UMST siblings are unavailable (W-63 fence).

fn job_uses_continue_on_error(yml: &str, job_key: &str) -> bool {
    let marker = format!("  {job_key}:");
    let start = yml
        .find(&marker)
        .unwrap_or_else(|| panic!("missing job {job_key}"));
    let window = yml[start..].chars().take(1200).collect::<String>();
    window.contains("continue-on-error: true")
}

#[test]
fn push_workflow_jobs_using_setup_ci_are_non_blocking() {
    let rust_yml = include_str!("../.github/workflows/rust.yml");
    assert!(rust_yml.contains("w63-public-sibling-boundary:"));
    for job_key in [
        "build-test",
        "verify-umst-stack",
        "kleisli-ppo-hot-bind",
        "lint",
        "arena-vs-mcp",
        "research-stack",
    ] {
        assert!(
            job_uses_continue_on_error(rust_yml, job_key),
            "job {job_key} must be non-blocking when setup-ci cannot clone private siblings"
        );
    }
}

#[test]
fn catalog_drift_exposes_honest_public_boundary_job() {
    let catalog_yml = include_str!("../.github/workflows/umst-catalog-drift.yml");
    assert!(catalog_yml.contains("w63-catalog-public-boundary"));
    assert!(
        job_uses_continue_on_error(catalog_yml, "verify-umst-stack"),
        "catalog verify-umst-stack must be non-blocking under W-63"
    );
}

#[test]
fn checkout_siblings_script_documents_private_fence() {
    let script = include_str!("../.github/scripts/checkout-umst-siblings.sh");
    assert!(script.contains("is private"));
    assert!(script.contains("W-63"));
}
