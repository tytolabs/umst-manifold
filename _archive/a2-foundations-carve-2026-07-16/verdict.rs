// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! REST-stable admissibility verdict strings.

/// REST / SSOT string tokens for admissibility (aligned with thermodynamic gate binaries).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdmissibilityVerdict {
    Accepted,
    MassViolation,
    NegativeDissipation,
    Unknown,
}

impl AdmissibilityVerdict {
    pub const ACCEPTED: &'static str = "ACCEPTED";
    pub const MASS_VIOLATION: &'static str = "MASS_VIOLATION";
    pub const NEGATIVE_DISSIPATION: &'static str = "NEGATIVE_DISSIPATION";
    pub const UNKNOWN: &'static str = "UNKNOWN";

    pub fn as_str(&self) -> &'static str {
        match self {
            AdmissibilityVerdict::Accepted => Self::ACCEPTED,
            AdmissibilityVerdict::MassViolation => Self::MASS_VIOLATION,
            AdmissibilityVerdict::NegativeDissipation => Self::NEGATIVE_DISSIPATION,
            AdmissibilityVerdict::Unknown => Self::UNKNOWN,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            Self::ACCEPTED => Some(Self::Accepted),
            Self::MASS_VIOLATION => Some(Self::MassViolation),
            Self::NEGATIVE_DISSIPATION => Some(Self::NegativeDissipation),
            Self::UNKNOWN => Some(Self::Unknown),
            _ => None,
        }
    }

    pub(crate) fn from_thermo_flags(
        accepted: bool,
        mass_conserved: bool,
        energy_positive: bool,
    ) -> Self {
        if accepted {
            AdmissibilityVerdict::Accepted
        } else if !mass_conserved {
            AdmissibilityVerdict::MassViolation
        } else if !energy_positive {
            AdmissibilityVerdict::NegativeDissipation
        } else {
            AdmissibilityVerdict::Unknown
        }
    }
}

impl std::fmt::Display for AdmissibilityVerdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
