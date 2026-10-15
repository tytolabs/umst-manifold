// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Operator probe: Bruyneel–Duysinx self-weight body force on a 2-node hanging chain.
//! Exit 2 if the mass-penalty force is non-finite or non-negative in z (expects gravity −z).

use umst_manifold::physics::mechanics::SelfWeightConfig;

fn main() {
    let sw = SelfWeightConfig {
        gravity_m_s2: 9.81,
        voxel_volume_m3: 1e-3,
        mass_penalty_q: 1.5,
        direction: [0.0, 0.0, -1.0],
    };
    let f = sw.body_force_flat(&[0.5, 1.0], 2);
    if f.len() != 6 || !f.iter().all(|x| x.is_finite()) {
        eprintln!("refuse: non-finite or wrong-length body force");
        std::process::exit(2);
    }
    let m0 = 0.5_f32.powf(1.5) * 1e-3 * 9.81;
    let m1 = 1.0_f32 * 1e-3 * 9.81;
    if (f[2] + m0).abs() >= 1e-5 || (f[5] + m1).abs() >= 1e-5 {
        eprintln!("refuse: fz mismatch fz0={} fz1={} exp0={} exp1={}", f[2], f[5], -m0, -m1);
        std::process::exit(2);
    }
    println!("ok fz0={} fz1={}", f[2], f[5]);
}
