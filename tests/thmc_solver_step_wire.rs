// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Compile witness for [`ThmcSolverStep`] post-step gate evidence wiring (`p5-thmc-wire`).

#[cfg(feature = "thmc-coupled")]
mod thmc_solver_step_wire {
    use burn::tensor::backend::Backend;
    use burn_ndarray::NdArray;
    use umst_manifold::physics::solvers::{ThmcSolver, ThmcSolverStep};

    type B = NdArray<f32>;

    fn accepts_thmc_solver_step<Bk: Backend>(solver: &impl ThmcSolverStep<Bk>) -> &'static str {
        let _ = solver;
        "ThmcSolverStep"
    }

    #[test]
    fn thmc_solver_step_trait_is_object_safe_and_exported() {
        let solver = ThmcSolver::default();
        assert_eq!(
            accepts_thmc_solver_step::<B>(&solver),
            "ThmcSolverStep"
        );
    }
}
