// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Order-statistics band classifier — mirror of `UMST.Formal.OrderStatisticsBand`.
//!
//! The analytic sample-size budget reuses the median-convergence closed form (envelope pattern);
//! the runtime surface adds NIST-style linear-interpolated percentiles for P25 / P75 banding.

pub mod core;

pub use crate::kernels::{
    classify_band, sample_percentile, sample_percentile_presorted, BandLabel,
};
pub use core::{n_quantile, NQuantileError};
