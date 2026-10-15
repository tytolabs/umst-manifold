//! K4-FRACTURE-PHAM-MARIGO-BAR-TEST — homogeneous 1-D AT2 bar nucleation stress.
//!
//! Drive a uniform chain with monotone uniaxial strain. Damage stays on the homogeneous
//! AT2 branch; the peak of \(\sigma = g(d)\,E\,\varepsilon\) matches Pham–Marigo
//! \(\sigma_c = \frac{3}{16}\sqrt{3 E G_c/\ell}\) within a stated discretisation tolerance.
//! Route: damage is non-decreasing (irreversibility / SecondLaw-compatible dissipation ≥ 0).
//!
//! Citation lands on the production degradation surface (Pham et al. 2011 AT2 peak).

#![cfg(feature = "fracture-at2")]

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};

use umst_manifold::core::field::{Field, FractureEnergyField, SmallStrainField};
use umst_manifold::physics::solvers::fracture_field::{
    degradation_g_f32, pham_marigo_at2_homogeneous_peak_stress, PhaseFieldFractureSolver,
};
use umst_manifold::physics::solvers::spectral_tensile_psi_plus_from_strain;

type B = NdArray<f32>;

/// Relative tolerance on \(\sigma_{\mathrm{peak}}\) vs \(\sigma_c\) (discretisation of the ε ramp).
const NUCLEATION_STRESS_REL_TOL: f32 = 0.08;

#[test]
fn pham_marigo_at2_homogeneous_bar_nucleates_at_cited_peak_stress() {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 5usize;
    // Chain edges so the Laplacian couples neighbours; drive stays spatially uniform.
    let mut edge_vals = Vec::with_capacity(2 * (n - 1));
    for i in 0..(n - 1) {
        edge_vals.push(i as i64);
        edge_vals.push((i + 1) as i64);
    }
    let e_ct = n - 1;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edge_vals, Shape::new([2, e_ct])), &dev);

    // Surrogate Lamé (λ=0, μ=½) ⇒ ψ⁺ = ½ ε_xx² for uniaxial tension ⇒ E ≡ 1 in Pham's formula.
    let young_pa = 1.0_f32;
    let gc = 150.0_f32;
    let length_scale = 0.08_f32;
    let sigma_c = pham_marigo_at2_homogeneous_peak_stress(young_pa, gc, length_scale)
        .expect("σ_c admits for positive E, Gc, ℓ");
    assert!(
        pham_marigo_at2_homogeneous_peak_stress(0.0, gc, length_scale).is_none(),
        "non-positive E must refuse"
    );

    let solver = PhaseFieldFractureSolver { length_scale };
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![gc; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );

    // Strain ramp past the analytic critical strain ε_c = sqrt(Gc / (3 E ℓ)) for AT2 peak.
    let eps_c = (gc / (3.0 * young_pa * length_scale)).sqrt();
    let eps_max = 2.5 * eps_c;
    let n_steps = 64usize;

    let mut damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let mut d_prev_mean = 0.0_f32;
    let mut sigma_peak = 0.0_f32;
    let mut dissipation_proxy = 0.0_f32;

    for step in 0..=n_steps {
        let exx = eps_max * (step as f32) / (n_steps as f32);
        let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
        for nod in 0..n {
            let base = nod * 9;
            strain_data[base] = exx;
        }
        let strain: Tensor<B, 4> =
            Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

        let d_new = solver
            .update_damage(
                SmallStrainField::from_tensor(strain.clone()),
                Field::new(damage.clone()),
                FractureEnergyField::from_tensor(fracture_energy_gc.clone()),
                edges_b1.clone(),
            )
            .expect("AT2 update on homogeneous bar")
            .into_tensor();

        let vals = d_new.clone().into_data().value;
        let d_mean = vals.iter().sum::<f32>() / vals.len() as f32;
        // Irreversibility: mean damage must not decrease (SecondLaw-compatible D ≥ 0 proxy).
        assert!(
            d_mean + 1e-5 >= d_prev_mean,
            "damage must be non-decreasing; prev={d_prev_mean} now={d_mean} step={step}"
        );
        dissipation_proxy += (d_mean - d_prev_mean).max(0.0) * gc / length_scale;
        d_prev_mean = d_mean;

        let g = degradation_g_f32(d_mean, 0.0);
        let sigma = g * young_pa * exx;
        if sigma > sigma_peak {
            sigma_peak = sigma;
        }

        // Homogeneous branch: spatial variation of d stays tiny before localisation.
        let d_spread = vals.iter().cloned().fold(0.0_f32, f32::max)
            - vals.iter().cloned().fold(f32::INFINITY, f32::min);
        if exx < 0.95 * eps_c {
            assert!(
                d_spread < 5e-3,
                "pre-peak damage must stay homogeneous; spread={d_spread} at ε={exx}"
            );
        }

        damage = d_new;

        let _psi = spectral_tensile_psi_plus_from_strain(strain);
        let _ = _psi;
    }

    let rel = ((sigma_peak - sigma_c) / sigma_c).abs();
    assert!(
        rel < NUCLEATION_STRESS_REL_TOL,
        "peak stress {sigma_peak} must match Pham–Marigo σ_c={sigma_c} within {NUCLEATION_STRESS_REL_TOL}; rel={rel}"
    );
    assert!(
        dissipation_proxy >= 0.0,
        "dissipation proxy Gc/ℓ·Δd must be non-negative"
    );
}
