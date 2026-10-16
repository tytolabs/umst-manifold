// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! A [`StepEnergyMeter`] over the host package energy counter.
//!
//! Each step is bracketed by two readings of `umst_ucrs::rapl::read_package_energy`; their
//! difference is a `Measured` spend. Where the host exposes no counter (no RAPL, as on macOS,
//! or a counter the process may not read), the step still runs and its spend is the typed
//! absence [`EnergySpent::absent`], never a number. `unfold` debits an absent step at the
//! certified Landauer floor, so the step count stays bounded by `UnfoldStop::max_steps`
//! (law L-E), and the grade of every sum it enters is the meet, `Absent`.

use umst_math::solve_combinator::{CombinatorRefuse, EnergySpent, StepEnergyMeter};
use umst_ucrs::rapl::{read_package_energy, EnergyReading};

/// The host package energy counter as a step meter.
#[derive(Clone, Copy, Debug, Default)]
pub struct PackageEnergyMeter;

/// One counter reading, or `None` when the host gives none to this process.
fn reading() -> Option<EnergyReading> {
    read_package_energy().ok()
}

impl StepEnergyMeter for PackageEnergyMeter {
    fn measure<F, T>(&self, step: F) -> Result<(T, EnergySpent), CombinatorRefuse>
    where
        F: FnOnce() -> Result<T, CombinatorRefuse>,
    {
        let before = reading();
        let value = step()?;
        let spent = match (before, reading()) {
            (Some(a), Some(b)) => EnergySpent::measured(b.delta(&a))?,
            _ => EnergySpent::absent(),
        };
        Ok((value, spent))
    }
}
