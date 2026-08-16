// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Pure **category-𝓗** *routing* laws on [`super::kinds::UnitKind`] (FORWARD-PLAN §0.2).
//! Routes are Kleisli arrows `α → Option<UnitKind>`; composition is the `Option` monad `and_then` chain.
//!
//! # Axioms (H-8 §0.8)
//! - (a) **left/right identity** with `id = |k| Some(k)`.
//! - (b) **associativity** of Kleisli composition: `(a >=> b) >=> c` vs `a >=> (b >=> c)`.
//! - (c) [`UnitKind`] is a *finite* object set; totality of pattern-matching is enforced in tests.

use super::kinds::UnitKind;

/// A routing morphism: partial map on [`UnitKind`]
pub type UnitRoute = fn(UnitKind) -> Option<UnitKind>;

/// The identity route `k ↦ k`
pub fn route_id() -> impl Fn(UnitKind) -> Option<UnitKind> {
    |k: UnitKind| Some(k)
}

/// `fn` item form for `check_identity_law` (Kleisli `id` morphism)
pub fn id_route(k: UnitKind) -> Option<UnitKind> {
    Some(k)
}

/// Post-compose two Kleisli arrows: `(g >=> f)(a) = f(a).and_then(g)`  — **first** `f`, then `g` (read: land on `f`, then go `g`).
pub fn kcompose(
    f: fn(UnitKind) -> Option<UnitKind>,
    g: fn(UnitKind) -> Option<UnitKind>,
) -> impl Fn(UnitKind) -> Option<UnitKind> {
    move |a: UnitKind| f(a).and_then(g)
}

/// (a) `id >=> f = f` and `f >=> id = f` on the domain of `f`
pub fn check_identity_law(f: fn(UnitKind) -> Option<UnitKind>) -> (bool, bool) {
    let id: fn(UnitKind) -> Option<UnitKind> = |k: UnitKind| Some(k);
    let id_left = |a: fn(UnitKind) -> Option<UnitKind>| {
        UnitKind::ALL
            .iter()
            .copied()
            .all(|k: UnitKind| kcompose(id, a)(k) == a(k))
    };
    let id_right = |a: fn(UnitKind) -> Option<UnitKind>| {
        UnitKind::ALL
            .iter()
            .copied()
            .all(|k: UnitKind| kcompose(a, id)(k) == a(k))
    };
    (id_left(f), id_right(f))
}

/// (b) `(h >=> g) >=> f` equals `h >=> (g >=> f)` pointwise
pub fn check_associative(
    f: fn(UnitKind) -> Option<UnitKind>,
    g: fn(UnitKind) -> Option<UnitKind>,
    h: fn(UnitKind) -> Option<UnitKind>,
) -> bool {
    UnitKind::ALL.iter().copied().all(|k: UnitKind| {
        let l = f(k).and_then(g).and_then(h);
        let r = f(k).and_then(|b| g(b).and_then(h));
        l == r
    })
}

/// Example: `f : Cpu -> Igpu`
pub fn f_cpu_igpu(k: UnitKind) -> Option<UnitKind> {
    if k == UnitKind::Cpu {
        Some(UnitKind::Igpu)
    } else {
        None
    }
}
/// `g : Igpu -> Ram`
pub fn g_igpu_ram(k: UnitKind) -> Option<UnitKind> {
    if k == UnitKind::Igpu {
        Some(UnitKind::Ram)
    } else {
        None
    }
}
/// `h : Ram -> Port`
pub fn h_ram_port(k: UnitKind) -> Option<UnitKind> {
    if k == UnitKind::Ram {
        Some(UnitKind::Port)
    } else {
        None
    }
}

/// (legacy) tuple form — kept for tests that import the names from the planning text
pub fn example_route_cpu_igpu_ram() -> (UnitRoute, UnitRoute) {
    (f_cpu_igpu, g_igpu_ram)
}

pub fn example_route_ram_port() -> UnitRoute {
    h_ram_port
}

/// Legacy: binary `compose` re-exported as kcompose on fns
pub fn compose(
    f: fn(UnitKind) -> Option<UnitKind>,
    g: fn(UnitKind) -> Option<UnitKind>,
) -> impl Fn(UnitKind) -> Option<UnitKind> {
    kcompose(f, g)
}
