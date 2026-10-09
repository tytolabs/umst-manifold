// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Track K8 — the posture of the research-tier CI job, read from the workflow file.

/// Whether the `research-stack` job of `.github/workflows/rust.yml` fails the workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResearchCiPosture {
    /// `research-stack` fails the workflow on a failing research test.
    Required,
    /// `research-stack` carries `continue-on-error: true` (W-63, 2026-10-01): its result is a signal,
    /// and the job is skipped while the path siblings are private.
    NonBlockingW63,
}

impl ResearchCiPosture {
    /// Current posture, as the workflow file states it (checked by the test below).
    pub const CURRENT: Self = Self::NonBlockingW63;

    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::NonBlockingW63 => "non_blocking_w63",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn research_ci_posture_matches_the_workflow() {
        let src = include_str!("../.github/workflows/rust.yml");
        let start = match src.find("  research-stack:") {
            Some(i) => i,
            None => panic!("research-stack job missing"),
        };
        let rest = &src[start..];
        let end = rest
            .find("  verify-umst-stack:")
            .map(|o| start + o)
            .unwrap_or(src.len());
        let block = &src[start..end];
        assert!(block.contains("solver-experimental"));
        let non_blocking = block.contains("continue-on-error: true");
        assert_eq!(
            ResearchCiPosture::CURRENT == ResearchCiPosture::NonBlockingW63,
            non_blocking,
            "ResearchCiPosture::CURRENT must state what rust.yml research-stack does"
        );
    }
}
