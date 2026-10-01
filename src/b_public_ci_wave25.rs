// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
//! B_public_ci_red pool receipt pins — tytolabs/umst-manifold workflow posture (audit only).

/// Steer receipt for wave-25 `LANE-POOL-B_public_ci_red-umst-manifold`.
pub const B_PUBLIC_CI_STEER_RECEIPT: &str = "STEER_20261001T0509";

/// Monotone steer-wave counter (audit only).
pub const B_PUBLIC_CI_STEER_WAVE_SEQ: u32 = 25;

/// W-63 boundary: private-sibling setup jobs are non-blocking on public default branch.
pub const W63_PRIVATE_SIBLING_SETUP_CONTINUE_ON_ERROR: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn b_public_ci_steer_wave_seq_twenty_five() {
        assert_eq!(B_PUBLIC_CI_STEER_RECEIPT, "STEER_20261001T0509");
        assert_eq!(B_PUBLIC_CI_STEER_WAVE_SEQ, 25);
        assert!(W63_PRIVATE_SIBLING_SETUP_CONTINUE_ON_ERROR);
        let rust_yml = include_str!("../.github/workflows/rust.yml");
        assert!(rust_yml.contains("continue-on-error: true"));
    }
}
