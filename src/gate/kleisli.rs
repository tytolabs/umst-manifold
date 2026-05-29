// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Kleisli arrows over the admissibility monad (prototype port, **generic carriers**).
//!
//! [`KleisliUnitEvaluator`] implements [`super::evaluator::GateEvaluator`] for catalog id
//! `umst.gate.kleisli_unit`, hand-aligned to `Gate.lean` (`admissibleNRefl`, `kleisliAdmissibility`).

use super::evaluator::GateEvaluator;
use super::mix_proposal::ThermodynamicStateSnapshot;
use super::verdict::AdmissibilityVerdict;

/// Result of a thermodynamic gate check on a wrapped value.
#[derive(Debug, Clone)]
pub struct AdmissibilityResult {
    pub admissible: bool,
    pub dissipation: f32,
    pub violation: Option<String>,
}

/// The admissibility monad wraps a value with its gate status: `M(A) = (A, AdmissibilityResult)`.
#[derive(Debug, Clone)]
pub struct Admissible<A: Clone> {
    pub value: A,
    pub result: AdmissibilityResult,
}

impl<A: Clone> Admissible<A> {
    /// Monadic unit (η): lift a value — trivial self-transition is admissible.
    #[must_use]
    pub fn pure(value: A) -> Self {
        Admissible {
            value,
            result: AdmissibilityResult {
                admissible: true,
                dissipation: 0.0,
                violation: None,
            },
        }
    }

    /// Bind (short-circuit on inadmissible intermediate carriers).
    pub fn bind<B: Clone, F>(self, f: F) -> Admissible<B>
    where
        F: FnOnce(A) -> Admissible<B>,
    {
        if !self.result.admissible {
            return Admissible {
                value: f(self.value).value,
                result: self.result,
            };
        }
        f(self.value)
    }

    #[must_use]
    pub fn join(nested: Admissible<Admissible<A>>) -> Admissible<A> {
        if !nested.result.admissible {
            Admissible {
                value: nested.value.value,
                result: nested.result,
            }
        } else {
            nested.value
        }
    }
}

/// Kleisli arrow `A → M(B)`.
pub struct KleisliArrow<A: Clone, B: Clone> {
    pub name: String,
    arrow: Box<dyn Fn(A) -> Admissible<B> + Send + Sync>,
}

impl<A: Clone, B: Clone> KleisliArrow<A, B> {
    pub fn new<F>(name: impl Into<String>, f: F) -> Self
    where
        F: Fn(A) -> Admissible<B> + Send + Sync + 'static,
    {
        KleisliArrow {
            name: name.into(),
            arrow: Box::new(f),
        }
    }

    pub fn run(&self, input: A) -> Admissible<B> {
        (self.arrow)(input)
    }
}

/// Compose sequentially: `(f ● g)(x) = f(x) >>= g`.
#[must_use]
pub fn kleisli_compose_pair<A, B, C>(
    f: impl Fn(A) -> Admissible<B> + Send + Sync + 'static,
    g: impl Fn(B) -> Admissible<C> + Send + Sync + 'static,
    name: impl Into<String>,
) -> KleisliArrow<A, C>
where
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
{
    KleisliArrow::new(name, move |a: A| {
        let mb = f(a);
        mb.bind(&g)
    })
}

#[derive(Clone, Debug)]
pub struct KleisliPipeline {
    pub name: String,
    pub steps: Vec<String>,
}

impl KleisliPipeline {
    pub fn new(name: impl Into<String>) -> Self {
        KleisliPipeline {
            name: name.into(),
            steps: Vec::new(),
        }
    }

    /// Run sequential Kleisli arrows (short-circuits once inadmissible).
    #[must_use]
    pub fn run_sequence<A>(&self, initial: A, arrows: &[&KleisliArrow<A, A>]) -> Admissible<A>
    where
        A: Clone,
    {
        let mut current = Admissible::pure(initial);
        for arrow in arrows {
            if !current.result.admissible {
                break;
            }
            current = current.bind(|state| arrow.run(state));
        }
        current
    }
}

/// Registry-facing evaluator for the admissibility monad unit η ([`Admissible::pure`]).
///
/// Aligns with `Gate.lean` reflexivity (`admissibleNRefl`): identity carriers lift with zero dissipation.
#[derive(Debug, Clone, Copy, Default)]
pub struct KleisliUnitEvaluator;

impl KleisliUnitEvaluator {
    pub const CATALOG_ID: &'static str = "umst.gate.kleisli_unit";

    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Monadic unit η: lift any cloneable carrier (trivial self-transition is admissible).
    #[must_use]
    pub fn lift<A: Clone>(&self, value: A) -> Admissible<A> {
        Admissible::pure(value)
    }

    /// Map a lifted carrier to REST-stable [`AdmissibilityVerdict`].
    #[must_use]
    pub fn verdict_for_lift<A: Clone>(&self, value: A) -> AdmissibilityVerdict {
        if self.lift(value).result.admissible {
            AdmissibilityVerdict::Accepted
        } else {
            AdmissibilityVerdict::Unknown
        }
    }

    /// Reflexive thermodynamic snapshot step (`AdmissibleN n s s` / `admissibleNRefl`).
    #[must_use]
    pub fn evaluate_reflexive_step(
        &self,
        state: &ThermodynamicStateSnapshot,
    ) -> AdmissibilityVerdict {
        self.verdict_for_lift(*state)
    }
}

impl GateEvaluator for KleisliUnitEvaluator {
    fn catalog_id(&self) -> &'static str {
        Self::CATALOG_ID
    }

    fn gate_family(&self) -> &'static str {
        "kleisli_admissibility_unit"
    }
}

#[must_use]
pub fn gate_arrow_generic<A: Clone>(
    name: impl Into<String>,
    check: impl Fn(&A) -> (bool, f32, Option<String>) + Send + Sync + 'static,
) -> KleisliArrow<A, A> {
    KleisliArrow::new(name, move |state: A| {
        let (ok, dissipation, violation) = check(&state);
        Admissible {
            value: state,
            result: AdmissibilityResult {
                admissible: ok,
                dissipation,
                violation,
            },
        }
    })
}
