// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
//! Detect whether workflow job headers mark setup-ci lanes as non-blocking on public runners.

/// True when every named job header in `workflow_yml` sets `continue-on-error: true`.
pub fn setup_ci_jobs_non_blocking(workflow_yml: &str, job_keys: &[&str]) -> bool {
    job_keys.iter().all(|job_key| {
        let marker = format!("  {job_key}:");
        workflow_yml
            .find(&marker)
            .map(|start| {
                workflow_yml[start..]
                    .chars()
                    .take(1200)
                    .collect::<String>()
                    .contains("continue-on-error: true")
            })
            .unwrap_or(false)
    })
}

/// Embedded CI workflow law for push lanes that call `./.github/actions/setup-ci`.
pub fn rust_push_workflow_law_holds() -> bool {
    let rust_yml = include_str!("../.github/workflows/rust.yml");
    rust_yml.contains("w63-public-sibling-boundary:")
        && setup_ci_jobs_non_blocking(
            rust_yml,
            &[
                "build-test",
                "verify-umst-stack",
                "kleisli-ppo-hot-bind",
                "lint",
                "arena-vs-mcp",
                "research-stack",
                "p4-gpu-witness",
            ],
        )
}

/// Both push workflows: every setup-ci lane that needs private siblings is non-blocking.
pub fn push_and_catalog_workflow_setup_ci_laws_hold() -> bool {
    rust_push_workflow_law_holds() && catalog_drift_workflow_law_holds()
}

/// Catalog drift workflow: honest W-63 boundary job + non-blocking verify lane.
pub fn catalog_drift_workflow_law_holds() -> bool {
    let catalog_yml = include_str!("../.github/workflows/umst-catalog-drift.yml");
    catalog_yml.contains("w63-catalog-public-boundary:")
        && catalog_yml.contains("checkout_private_siblings: \"false\"")
        && setup_ci_jobs_non_blocking(catalog_yml, &["verify-umst-stack"])
}

#[cfg(test)]
mod tests {
    use super::{
        catalog_drift_workflow_law_holds, push_and_catalog_workflow_setup_ci_laws_hold,
        rust_push_workflow_law_holds,
    };

    #[test]
    fn rust_push_workflow_setup_ci_jobs_non_blocking() {
        assert!(rust_push_workflow_law_holds());
    }

    #[test]
    fn catalog_drift_workflow_public_boundary_and_verify_lane() {
        assert!(catalog_drift_workflow_law_holds());
    }

    #[test]
    fn push_and_catalog_workflow_setup_ci_laws() {
        assert!(push_and_catalog_workflow_setup_ci_laws_hold());
    }
}
