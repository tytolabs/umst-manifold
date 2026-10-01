// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Public GitHub Actions posture when private UMST siblings are unavailable.

use umst_manifold::detect_whether_workflow_job;

#[test]
fn push_workflow_jobs_using_setup_ci_are_non_blocking() {
    assert!(detect_whether_workflow_job::rust_push_workflow_law_holds());
}

#[test]
fn catalog_drift_exposes_honest_public_boundary_job() {
    let catalog_yml = include_str!("../.github/workflows/umst-catalog-drift.yml");
    assert!(catalog_yml.contains("w63-catalog-public-boundary"));
    assert!(
        detect_whether_workflow_job::setup_ci_jobs_non_blocking(catalog_yml, &["verify-umst-stack"]),
        "catalog verify-umst-stack must be non-blocking when private siblings are unavailable"
    );
}

#[test]
fn checkout_siblings_script_documents_private_fence() {
    let script = include_str!("../.github/scripts/checkout-umst-siblings.sh");
    assert!(script.contains("is private"));
    assert!(script.contains("W-63"));
}
