#!/usr/bin/env python3
"""RW-FP-TOT-3C3: migrate electrochemistry Option/String paths to PhysicsError."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
EC = ROOT / "src/physics/solvers/electrochemistry.rs"
PNP_TEST = ROOT / "tests/verification/pnp_debye_layer.rs"


def ensure_import(text: str) -> str:
    needle = "use burn::tensor::{backend::Backend, Int, Tensor};"
    insert = needle + "\nuse crate::physics::PhysicsError;"
    if "use crate::physics::PhysicsError;" not in text:
        text = text.replace(needle, insert, 1)
    return text


def replace_once(text: str, old: str, new: str, label: str) -> str:
    if old not in text:
        raise SystemExit(f"missing block: {label}")
    return text.replace(old, new, 1)


def migrate_electrochemistry(text: str) -> str:
    text = ensure_import(text)

    text = replace_once(
        text,
        """    ) -> (Tensor<B, 3>, Tensor<B, 3>) {
        #[cfg(not(feature = "electrochemistry-mvp"))]
        {
            let _ = (dt, edges_b1, permittivity, diffusivity);
            (electric_potential, ion_concentration)
        }

        #[cfg(feature = "electrochemistry-mvp")]
        {
            solve_pnp_step_experimental(
                self,
                dt,
                electric_potential,
                ion_concentration,
                edges_b1,
                permittivity,
                diffusivity,
            )
        }
    }

    /// Single entry point that respects [`Self::pnp_implicit_newton_chain`]: when it is **`Some`**""",
        """    ) -> Result<(Tensor<B, 3>, Tensor<B, 3>), PhysicsError> {
        #[cfg(not(feature = "electrochemistry-mvp"))]
        {
            let _ = (dt, edges_b1, permittivity, diffusivity);
            Ok((electric_potential, ion_concentration))
        }

        #[cfg(feature = "electrochemistry-mvp")]
        {
            solve_pnp_step_experimental(
                self,
                dt,
                electric_potential,
                ion_concentration,
                edges_b1,
                permittivity,
                diffusivity,
            )
        }
    }

    /// Single entry point that respects [`Self::pnp_implicit_newton_chain`]: when it is **`Some`**""",
        "solve_pnp_step",
    )

    text = replace_once(
        text,
        """    ) -> (Tensor<B, 3>, Tensor<B, 3>) {
        #[cfg(not(feature = "electrochemistry-mvp"))]
        {
            let _ = (dt, edges_b1, permittivity, diffusivity);
            (electric_potential, ion_concentration)
        }
        #[cfg(feature = "electrochemistry-mvp")]
        {
            if let Some(newton) = self.pnp_implicit_newton_chain {
                if let Some(out) = self.try_solve_pnp_backward_euler_newton_chain(
                    &newton,
                    dt,
                    electric_potential.clone(),
                    ion_concentration.clone(),
                    edges_b1.clone(),
                    permittivity.clone(),
                    diffusivity.clone(),
                ) {
                    return out;
                }
            }
            self.solve_pnp_step(
                dt,
                electric_potential,
                ion_concentration,
                edges_b1,
                permittivity,
                diffusivity,
            )
        }
    }

    /// Fully implicit **backward Euler** step""",
        """    ) -> Result<(Tensor<B, 3>, Tensor<B, 3>), PhysicsError> {
        #[cfg(not(feature = "electrochemistry-mvp"))]
        {
            let _ = (dt, edges_b1, permittivity, diffusivity);
            Ok((electric_potential, ion_concentration))
        }
        #[cfg(feature = "electrochemistry-mvp")]
        {
            if let Some(newton) = self.pnp_implicit_newton_chain {
                if let Ok(out) = self.try_solve_pnp_backward_euler_newton_chain(
                    &newton,
                    dt,
                    electric_potential.clone(),
                    ion_concentration.clone(),
                    edges_b1.clone(),
                    permittivity.clone(),
                    diffusivity.clone(),
                ) {
                    return Ok(out);
                }
            }
            self.solve_pnp_step(
                dt,
                electric_potential,
                ion_concentration,
                edges_b1,
                permittivity,
                diffusivity,
            )
        }
    }

    /// Fully implicit **backward Euler** step""",
        "solve_pnp_step_dispatch",
    )

    text = replace_once(
        text,
        """    ) -> Option<(Tensor<B, 3>, Tensor<B, 3>)> {
        #[cfg(not(feature = "electrochemistry-mvp"))]
        {
            None
        }
        #[cfg(feature = "electrochemistry-mvp")]
        {
            try_solve_pnp_be_newton_chain_host(
                self,
                newton,
                dt,
                electric_potential_n,
                ion_concentration_n,
                edges_b1,
                permittivity,
                diffusivity,
            )
        }
    }
}""",
        """    ) -> Result<(Tensor<B, 3>, Tensor<B, 3>), PhysicsError> {
        #[cfg(not(feature = "electrochemistry-mvp"))]
        {
            let _ = (
                newton,
                dt,
                electric_potential_n,
                ion_concentration_n,
                edges_b1,
                permittivity,
                diffusivity,
            );
            Err(PhysicsError::UnsupportedLayout {
                context: "try_solve_pnp_backward_euler_newton_chain",
            })
        }
        #[cfg(feature = "electrochemistry-mvp")]
        {
            try_solve_pnp_be_newton_chain_host(
                self,
                newton,
                dt,
                electric_potential_n,
                ion_concentration_n,
                edges_b1,
                permittivity,
                diffusivity,
            )
        }
    }
}""",
        "try_solve_pnp_backward_euler_newton_chain",
    )

    text = text.replace(
        ") -> Option<Tensor<B, 3>> {",
        ") -> Result<Tensor<B, 3>, PhysicsError> {",
        1,
    )
    text = text.replace(
        """    if fc != 1 {
        return None;
    }
    let ed = edges_b1.dims();
    if ed[0] != 2 {
        return None;
    }
    let e_ct = ed[1];
    if n < 2 || e_ct != n - 1 {
        return None;
    }
    let layout_raw = edges_b1.clone().float().into_data().value;
    let layout: Vec<i64> = layout_raw.iter().map(|&x| x as i64).collect();
    if !is_contiguous_unit_path(n, &layout) {
        return None;
    }""",
        """    if fc != 1 {
        return Err(PhysicsError::ShapeMismatch {
            context: "try_solve_poisson_chain_thomas",
            detail: "electric_potential fc must be 1",
        });
    }
    let ed = edges_b1.dims();
    if ed[0] != 2 {
        return Err(PhysicsError::ShapeMismatch {
            context: "try_solve_poisson_chain_thomas",
            detail: "edges_b1 rank-0 must be 2",
        });
    }
    let e_ct = ed[1];
    if n < 2 || e_ct != n - 1 {
        return Err(PhysicsError::UnsupportedLayout {
            context: "try_solve_poisson_chain_thomas",
        });
    }
    let layout_raw = edges_b1.clone().float().into_data().value;
    let layout: Vec<i64> = layout_raw.iter().map(|&x| x as i64).collect();
    if !is_contiguous_unit_path(n, &layout) {
        return Err(PhysicsError::UnsupportedLayout {
            context: "try_solve_poisson_chain_thomas",
        });
    }""",
        1,
    )

    text = text.replace(
        """    Some(Tensor::from_data(
        Data::new(out, Shape::new([batch, n, fc])),
        &device,
    ))
}""",
        """    Ok(Tensor::from_data(
        Data::new(out, Shape::new([batch, n, fc])),
        &device,
    ))
}""",
        1,
    )

    text = text.replace(
        """) -> (Tensor<B, 3>, Tensor<B, 3>) {
    let iters = solver.coupling_picard_iters.max(1);""",
        """) -> Result<(Tensor<B, 3>, Tensor<B, 3>), PhysicsError> {
    let iters = solver.coupling_picard_iters.max(1);""",
        1,
    )
    text = text.replace(
        """        }
    }
    (phi, c_work)
}

/// Picard L∞ gate:""",
        """        }
    }
    Ok((phi, c_work))
}

/// Picard L∞ gate:""",
        1,
    )

    text = text.replace(
        """    let phi_next = if let Some(phi_t) = try_solve_poisson_chain_thomas(
        electric_potential.clone(),
        rho_over_eps.clone(),
        permittivity.clone(),
        edges_b1.clone(),
        solver.mesh_spacing,
    ) {
        phi_t
    } else {""",
        """    let phi_next = match try_solve_poisson_chain_thomas(
        electric_potential.clone(),
        rho_over_eps.clone(),
        permittivity.clone(),
        edges_b1.clone(),
        solver.mesh_spacing,
    ) {
        Ok(phi_t) => phi_t,
        Err(_) => {""",
        1,
    )
    text = text.replace(
        """            n,
        )
    };

    // Scharfetter–Gummel drift–diffusion flux""",
        """            n,
        )
        }
    };

    // Scharfetter–Gummel drift–diffusion flux""",
        1,
    )

    text = text.replace(
        """        _ => unreachable!(),""",
        """        rem => {
            debug_assert!(rem < 3, "pnp_nm_index_to_fm: nm % 3 must be 0..2");
            node
        }""",
        1,
    )

    text = text.replace(") -> Option<Vec<f64>> {", ") -> Result<Vec<f64>, PhysicsError> {", 1)
    text = text.replace(
        "        return Some(vec![0.0_f64; dim]);",
        "        return Ok(vec![0.0_f64; dim]);",
        1,
    )
    text = text.replace(
        "    let x_g = gmres_f32_try(matvec, &b_f32, dim, max_iter, GMRES_REL_TOL).ok()?;\n    Some(x_g.into_iter().map(|x| x as f64).collect())",
        "    let x_g = gmres_f32_try(matvec, &b_f32, dim, max_iter, GMRES_REL_TOL)?;\n    Ok(x_g.into_iter().map(|x| x as f64).collect())",
        1,
    )

    text = text.replace(
        ") -> Option<(Tensor<B, 3>, Tensor<B, 3>)> {\n    let pd = electric_potential_n.dims();\n    if pd[0] != 1 || pd[2] != 1 {\n        return None;\n    }\n    let n = pd[1];\n    if n < 2 || n > newton.max_chain_nodes {\n        return None;\n    }\n    let ed = edges_b1.dims();\n    if ed[0] != 2 || ed[1] != n - 1 {\n        return None;\n    }\n    let layout_raw = edges_b1.clone().float().into_data().value;\n    let layout: Vec<i64> = layout_raw.iter().map(|&x| x as i64).collect();\n    if !is_contiguous_unit_path(n, &layout) {\n        return None;\n    }\n    let dt64 = dt as f64;\n    if !dt64.is_finite() || dt64 <= 0.0 {\n        return None;\n    }",
        """) -> Result<(Tensor<B, 3>, Tensor<B, 3>), PhysicsError> {
    const CTX: &str = "try_solve_pnp_be_newton_chain_host";
    let pd = electric_potential_n.dims();
    if pd[0] != 1 || pd[2] != 1 {
        return Err(PhysicsError::ShapeMismatch {
            context: CTX,
            detail: "batch must be 1 and fc must be 1",
        });
    }
    let n = pd[1];
    if n < 2 || n > newton.max_chain_nodes {
        return Err(PhysicsError::UnsupportedLayout { context: CTX });
    }
    let ed = edges_b1.dims();
    if ed[0] != 2 || ed[1] != n - 1 {
        return Err(PhysicsError::ShapeMismatch {
            context: CTX,
            detail: "edges_b1 must be [2, N-1]",
        });
    }
    let layout_raw = edges_b1.clone().float().into_data().value;
    let layout: Vec<i64> = layout_raw.iter().map(|&x| x as i64).collect();
    if !is_contiguous_unit_path(n, &layout) {
        return Err(PhysicsError::UnsupportedLayout { context: CTX });
    }
    let dt64 = dt as f64;
    if !dt64.is_finite() || dt64 <= 0.0 {
        return Err(PhysicsError::InvariantViolation {
            context: "try_solve_pnp_be_newton_chain_host: dt must be positive finite",
        });
    }""",
        1,
    )

    text = text.replace(
        "                let ok = if let Some(dn) = delta_nm {",
        "                let ok = if let Ok(dn) = delta_nm {",
        1,
    )

    for old, new in [
        (
            """                let Some(jac) = jac_band.as_mut() else {
                    return None;
                };
                newton_fd_jacobian_full_sg_node_major_row_band(
                    solver, newton, dt64, &u, &c_plus_n, &c_minus_n, &eps, &d_plus, &d_minus, g0,
                    g1, &r, jac,
                );
                pnp_residual_fm_to_nm(&r, n, &mut rhs_nm);
                for v in rhs_nm.iter_mut() {
                    *v = -*v;
                }
                let Some(lu_buf) = jac_lu_scratch.as_mut() else {
                    return None;
                };
                let Some(dense_buf) = jac_dense_scratch.as_mut() else {
                    return None;
                };
                let Some(swaps) = band_lu_swaps.as_mut() else {
                    return None;
                };""",
            """                let Some(jac) = jac_band.as_mut() else {
                    return Err(PhysicsError::BufferExhausted {
                        context: "try_solve_pnp_be_newton_chain_host: jac_band",
                    });
                };
                newton_fd_jacobian_full_sg_node_major_row_band(
                    solver, newton, dt64, &u, &c_plus_n, &c_minus_n, &eps, &d_plus, &d_minus, g0,
                    g1, &r, jac,
                );
                pnp_residual_fm_to_nm(&r, n, &mut rhs_nm);
                for v in rhs_nm.iter_mut() {
                    *v = -*v;
                }
                let Some(lu_buf) = jac_lu_scratch.as_mut() else {
                    return Err(PhysicsError::BufferExhausted {
                        context: "try_solve_pnp_be_newton_chain_host: jac_lu_scratch",
                    });
                };
                let Some(dense_buf) = jac_dense_scratch.as_mut() else {
                    return Err(PhysicsError::BufferExhausted {
                        context: "try_solve_pnp_be_newton_chain_host: jac_dense_scratch",
                    });
                };
                let Some(swaps) = band_lu_swaps.as_mut() else {
                    return Err(PhysicsError::BufferExhausted {
                        context: "try_solve_pnp_be_newton_chain_host: band_lu_swaps",
                    });
                };""",
        ),
        (
            """            let Some(jac) = jac_band.as_mut() else {
                return None;
            };
            let Some(lu_buf) = jac_lu_scratch.as_mut() else {
                return None;
            };
            let Some(dense_buf) = jac_dense_scratch.as_mut() else {
                return None;
            };
            let Some(swaps) = band_lu_swaps.as_mut() else {
                return None;
            };""",
            """            let Some(jac) = jac_band.as_mut() else {
                return Err(PhysicsError::BufferExhausted {
                    context: "try_solve_pnp_be_newton_chain_host: jac_band",
                });
            };
            let Some(lu_buf) = jac_lu_scratch.as_mut() else {
                return Err(PhysicsError::BufferExhausted {
                    context: "try_solve_pnp_be_newton_chain_host: jac_lu_scratch",
                });
            };
            let Some(dense_buf) = jac_dense_scratch.as_mut() else {
                return Err(PhysicsError::BufferExhausted {
                    context: "try_solve_pnp_be_newton_chain_host: jac_dense_scratch",
                });
            };
            let Some(swaps) = band_lu_swaps.as_mut() else {
                return Err(PhysicsError::BufferExhausted {
                    context: "try_solve_pnp_be_newton_chain_host: band_lu_swaps",
                });
            };""",
        ),
        (
            """        if !ok {
            return None;
        }""",
            """        if !ok {
            return Err(PhysicsError::KrylovDiverged {
                context: "try_solve_pnp_be_newton_chain_host: Newton correction failed",
            });
        }""",
        ),
        (
            """    if !n_pre.is_finite() || n_pre > 1e-6_f64 {
        return None;
    }
    let phi_out: Vec<f32> = u[0..n].iter().map(|&x| x as f32).collect();
    let mut c_out = vec![0.0_f32; n * 2];
    for i in 0..n {
        c_out[i * 2] = u[n + i] as f32;
        c_out[i * 2 + 1] = u[2 * n + i] as f32;
    }
    let phi_t = Tensor::from_data(Data::new(phi_out, Shape::new([1, n, 1])), &device);
    let c_t = Tensor::from_data(Data::new(c_out, Shape::new([1, n, 2])), &device);
    Some((phi_t, c_t))
}""",
            """    if !n_pre.is_finite() || n_pre > 1e-6_f64 {
        return Err(PhysicsError::Diverged {
            eq_rel: n_pre as f32,
            pcg_iterations: 0,
        });
    }
    let phi_out: Vec<f32> = u[0..n].iter().map(|&x| x as f32).collect();
    let mut c_out = vec![0.0_f32; n * 2];
    for i in 0..n {
        c_out[i * 2] = u[n + i] as f32;
        c_out[i * 2 + 1] = u[2 * n + i] as f32;
    }
    let phi_t = Tensor::from_data(Data::new(phi_out, Shape::new([1, n, 1])), &device);
    let c_t = Tensor::from_data(Data::new(c_out, Shape::new([1, n, 2])), &device);
    Ok((phi_t, c_t))
}""",
        ),
    ]:
        if old not in text:
            raise SystemExit(f"missing block for replacement: {old[:60]}...")
        text = text.replace(old, new, 1)

    text = text.replace(") -> Option<f64> {", ") -> Result<f64, PhysicsError> {", 1)
    text = text.replace(
        """    let pd = phi.dims();
    if pd[0] != 1 || pd[2] != 1 {
        return None;
    }
    let n = pd[1];
    let ed = edges_b1.dims();
    if ed[0] != 2 || ed[1] != n - 1 {
        return None;
    }
    let layout_raw = edges_b1.clone().float().into_data().value;
    let layout: Vec<i64> = layout_raw.iter().map(|&x| x as i64).collect();
    if !is_contiguous_unit_path(n, &layout) {
        return None;
    }""",
        """    const CTX: &str = "pnp_backward_euler_residual_l2_chain_host_f64";
    let pd = phi.dims();
    if pd[0] != 1 || pd[2] != 1 {
        return Err(PhysicsError::ShapeMismatch {
            context: CTX,
            detail: "batch must be 1 and fc must be 1",
        });
    }
    let n = pd[1];
    let ed = edges_b1.dims();
    if ed[0] != 2 || ed[1] != n - 1 {
        return Err(PhysicsError::ShapeMismatch {
            context: CTX,
            detail: "edges_b1 must be [2, N-1]",
        });
    }
    let layout_raw = edges_b1.clone().float().into_data().value;
    let layout: Vec<i64> = layout_raw.iter().map(|&x| x as i64).collect();
    if !is_contiguous_unit_path(n, &layout) {
        return Err(PhysicsError::UnsupportedLayout { context: CTX });
    }""",
        1,
    )
    text = text.replace("    Some(vec_l2(&r))", "    Ok(vec_l2(&r))", 1)

    return text


def add_expects(path: Path, methods: list[str]) -> None:
    text = path.read_text()
    for method in methods:
        pattern = rf"(\.(?:{method})\([^;]*?\));(?!\s*\.(?:expect|unwrap))"
        text = re.sub(
            pattern,
            lambda m: f'{m.group(1)}.expect("{method}");',
            text,
            flags=re.DOTALL,
        )
    path.write_text(text)


def main() -> None:
    ec = EC.read_text()
    migrated = migrate_electrochemistry(ec)
    EC.write_text(migrated)
    methods = [
        "solve_pnp_step",
        "solve_pnp_step_dispatch",
        "try_solve_pnp_backward_euler_newton_chain",
        "try_solve_pnp_be_newton_chain_host",
    ]
    add_expects(EC, methods)
    if PNP_TEST.exists():
        add_expects(PNP_TEST, methods)
    print("migration complete")


if __name__ == "__main__":
    main()
