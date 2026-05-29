// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Thin gate-layer alias around [`crate::ai::cbf::ThermodynamicCBF`] (no duplicated physics).

pub use crate::ai::cbf::ThermodynamicCBF as ThermodynamicCBFInner;

/// Newtype forwarding to [`ThermodynamicCBFInner`] via [`std::ops::Deref`].
pub struct GateThermodynamicCBF(pub ThermodynamicCBFInner);

impl GateThermodynamicCBF {
    pub fn new(temperature_k: f64, initial_credit_joules: f64) -> Self {
        Self(ThermodynamicCBFInner::new(
            temperature_k,
            initial_credit_joules,
        ))
    }

    pub fn into_inner(self) -> ThermodynamicCBFInner {
        self.0
    }
}

impl std::ops::Deref for GateThermodynamicCBF {
    type Target = ThermodynamicCBFInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for GateThermodynamicCBF {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
