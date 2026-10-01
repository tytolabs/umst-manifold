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
    setup_ci_jobs_non_blocking(
        rust_yml,
        &[
            "build-test",
            "verify-umst-stack",
            "kleisli-ppo-hot-bind",
            "lint",
            "arena-vs-mcp",
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::rust_push_workflow_law_holds;

    #[test]
    fn rust_push_workflow_setup_ci_jobs_non_blocking() {
        assert!(rust_push_workflow_law_holds());
    }
}
