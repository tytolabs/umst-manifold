// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Track K8 — research-tier CI must fail the job when a research test fails.

/// Honest posture: research CI is a merge/push gate, not an optional signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResearchCiPosture {
    /// `research-stack` fails the workflow on a failing research test.
    Required,
}

impl ResearchCiPosture {
    /// Current posture.
    pub const CURRENT: Self = Self::Required;

    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_audit_research_ci_fails_the_job() {
        assert_eq!(ResearchCiPosture::CURRENT, ResearchCiPosture::Required);
        let src = include_str!("../.github/workflows/rust.yml");
        let start = match src.find("  research-stack:") {
            Some(i) => i,
            None => {
                assert!(false, "research-stack job missing");
                return;
            }
        };
        let rest = &src[start..];
        let end = rest
            .find("  verify-umst-stack:")
            .map(|o| start + o)
            .unwrap_or(src.len());
        let block = &src[start..end];
        assert!(block.contains("solver-experimental"));
        assert!(!block.contains("continue-on-error"));
    }
}
