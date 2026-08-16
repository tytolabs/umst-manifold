// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! G4: cross-hardware determinism of the gate verdict computation.
//!
//! The same deterministic inputs are evaluated on the CPU backend (NdArray) and one
//! GPU backend (Wgpu — Metal on macOS, Vulkan on Linux/Windows). The gate verdict
//! quantities — Clausius–Duhem violation slack and Landauer slack — must agree within
//! `f32` tolerance across backends. This is the honest "hardware-agnostic" test for the
//! gate margin: identical admissibility decision regardless of silicon.
//!
//! Scope note: this covers the gate/constraint computation, which is backend-generic.
//! The full Q1-hex compliance *design* solve is NOT exercised on Wgpu here — the custom
//! matrix-free PCG path hits a documented wgpu/Metal minimum-buffer-alignment limitation
//! (see `tests/rejection_witness_gpu.rs::kleisli_ppo_gpu_autodiff_smoke`), so
//! cross-hardware parity for the *full design* remains UNVERIFIED and is not claimed.

#![cfg(feature = "wgpu")]

use burn::backend::wgpu::{Wgpu, WgpuDevice};
use burn::tensor::{backend::Backend, Data, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::ai::constraint_loss::{clausius_duhem_violation, landauer_slack_violation};

/// Relative tolerance appropriate for `f32` across heterogeneous silicon. The gate
/// verdict values live on a large joule-rate scale (~1e5); absolute differences at the
/// f32 rounding floor (~1e-2 there) are expected from CPU/GPU fma + reduction-order
/// differences. What must be hardware-agnostic is (a) the admissibility *decision* and
/// (b) the value to f32 *relative* precision.
const REL_TOL: f32 = 1e-5;
const N: usize = 64;

/// Deterministic sweep mixing admissible (`ψ_new ≤ ψ_old`) and inadmissible
/// (`ψ_new > ψ_old`) transitions so the violation slack is non-trivial.
fn sweep() -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut od = Vec::with_capacity(N);
    let mut nd = Vec::with_capacity(N);
    let mut ofe = Vec::with_capacity(N);
    let mut nfe = Vec::with_capacity(N);
    let mut dt = Vec::with_capacity(N);
    for i in 0..N {
        let t = i as f32;
        od.push(2400.0 + t);
        nd.push(2400.0 + t + if i % 3 == 0 { 5.0 } else { -2.0 });
        let psi_old = -1.0e5 - t * 10.0;
        ofe.push(psi_old);
        // Alternate inadmissible transitions (ψ_new > ψ_old ⇒ positive violation).
        nfe.push(if i % 2 == 0 {
            psi_old - 50.0
        } else {
            psi_old + 80.0
        });
        dt.push(1.0 + 0.01 * t);
    }
    (od, nd, ofe, nfe, dt)
}

fn t1<B: Backend<FloatElem = f32>>(v: &[f32], dev: &B::Device) -> Tensor<B, 1> {
    Tensor::<B, 1>::from_data(Data::new(v.to_vec(), Shape::new([v.len()])), dev)
}

fn cd_violation_on<B: Backend<FloatElem = f32>>(dev: &B::Device) -> Vec<f32> {
    let (od, nd, ofe, nfe, dt) = sweep();
    clausius_duhem_violation::<B>(
        t1::<B>(&od, dev),
        t1::<B>(&nd, dev),
        t1::<B>(&ofe, dev),
        t1::<B>(&nfe, dev),
        t1::<B>(&dt, dev),
    )
    .into_data()
    .value
}

fn landauer_on<B: Backend<FloatElem = f32>>(dev: &B::Device) -> Vec<f32> {
    let bits: Vec<f32> = (0..N).map(|i| 0.1 * i as f32).collect();
    landauer_slack_violation::<B>(t1::<B>(&bits, dev), 293.15, 1.0e-21)
        .into_data()
        .value
}

fn max_abs_diff(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0.0_f32, f32::max)
}

/// Worst-case relative difference `|a−b| / max(|a|,|b|,floor)`.
fn max_rel_diff(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| {
            let denom = x.abs().max(y.abs()).max(1.0_f32);
            (x - y).abs() / denom
        })
        .fold(0.0_f32, f32::max)
}

/// The admissibility decision is `violation > 0` (inadmissible) vs `== 0` (admissible).
fn decisions(v: &[f32]) -> Vec<bool> {
    v.iter().map(|&x| x > 0.0).collect()
}

#[test]
fn cross_backend_gate_verdict_determinism() {
    let cpu = NdArrayDevice::default();
    let gpu = WgpuDevice::default();

    let cd_cpu = cd_violation_on::<NdArray<f32>>(&cpu);
    let cd_gpu = cd_violation_on::<Wgpu>(&gpu);
    let l_cpu = landauer_on::<NdArray<f32>>(&cpu);
    let l_gpu = landauer_on::<Wgpu>(&gpu);

    let d_cd_abs = max_abs_diff(&cd_cpu, &cd_gpu);
    let d_cd_rel = max_rel_diff(&cd_cpu, &cd_gpu);
    let d_l_rel = max_rel_diff(&l_cpu, &l_gpu);
    let nonzero = cd_cpu.iter().filter(|&&x| x > 0.0).count();

    // The admissibility DECISION must be bit-identical across silicon.
    let dec_cpu = decisions(&cd_cpu);
    let dec_gpu = decisions(&cd_gpu);
    let decision_mismatch = dec_cpu
        .iter()
        .zip(dec_gpu.iter())
        .filter(|(a, b)| a != b)
        .count();

    eprintln!(
        "cross_backend_gate: decision_mismatch={decision_mismatch}/{} | CD |Δ|∞_abs={d_cd_abs:.3e} rel={d_cd_rel:.3e} (nonzero={nonzero}/{}) | Landauer rel={d_l_rel:.3e} | rel_tol={REL_TOL:.0e}",
        cd_cpu.len(),
        cd_cpu.len()
    );

    assert!(
        nonzero > 0,
        "sweep must exercise inadmissible (nonzero violation) cases, got {nonzero}"
    );
    assert_eq!(
        decision_mismatch, 0,
        "admissibility DECISION must match across CPU/GPU; {decision_mismatch} disagreements"
    );
    assert!(
        d_cd_rel < REL_TOL,
        "Clausius–Duhem violation diverges (relative) across CPU/GPU: {d_cd_rel:.3e} ≥ {REL_TOL:.0e}"
    );
    assert!(
        d_l_rel < REL_TOL,
        "Landauer slack diverges (relative) across CPU/GPU: {d_l_rel:.3e} ≥ {REL_TOL:.0e}"
    );
}
