// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MPL-2.0

//! The P2 chair: the action tree of the cross-language chair fixture (`chair_cross_lang_p2.json`, M6-C04 §3).
//!
//! The dimensions are the fixture's design definition (metres at `scale = 1`): a seat slab, an optional back
//! slab, four capped legs and a clearance box removed from the hull. Every consumer that names the chair
//! (egoff credit, umst-semantics) builds it here, so one tree fixes its quotient (lifted from egoff-credit, E-2).

use super::{Action, ComposeOp};

/// Leg base points `(x, 0, z)` in the order the legs union lists them: (−,−), (−,+), (+,−), (+,+).
pub const CHAIR_LEG_BASES: [[f64; 3]; 4] = [
    [-0.20, 0.0, -0.20],
    [-0.20, 0.0, 0.20],
    [0.20, 0.0, -0.20],
    [0.20, 0.0, 0.20],
];

/// The chair action at `scale`; `include_back = false` omits the back slab everywhere.
#[must_use]
pub fn compose_chair_action(include_back: bool, scale: f64) -> Action {
    let s = scale;
    let seat = Action::Box {
        center: scale_point([0.0, 0.45, 0.0], s),
        half_extents: scale_point([0.25, 0.04, 0.25], s),
    };
    let back = Action::Box {
        center: scale_point([0.0, 0.75, -0.22], s),
        half_extents: scale_point([0.25, 0.30, 0.03], s),
    };
    let clearance = Action::Box {
        center: scale_point([0.0, 0.25, 0.0], s),
        half_extents: scale_point([0.30, 0.30, 0.30], s),
    };
    let legs: Vec<Action> = CHAIR_LEG_BASES
        .iter()
        .map(|base| Action::Cylinder {
            base: scale_point(*base, s),
            axis: [0.0, 1.0, 0.0],
            radius: 0.03 * s,
            height: 0.45 * s,
        })
        .collect();
    let legs_union = Action::Compose {
        op: ComposeOp::Union,
        children: legs,
    };

    let mut hull_children = vec![seat.clone(), legs_union.clone()];
    if include_back {
        hull_children.insert(1, back.clone());
    }
    let hull_union = Action::Compose {
        op: ComposeOp::Union,
        children: hull_children,
    };
    let clearance_diff = Action::Compose {
        op: ComposeOp::Difference,
        children: vec![hull_union, clearance],
    };

    let mut outer = vec![seat, legs_union, clearance_diff];
    if include_back {
        outer.insert(1, back);
    }
    Action::Compose {
        op: ComposeOp::Union,
        children: outer,
    }
}

#[inline]
fn scale_point(p: [f64; 3], scale: f64) -> [f64; 3] {
    [p[0] * scale, p[1] * scale, p[2] * scale]
}

#[cfg(test)]
mod tests {
    use super::super::{action_geometry_quotient_key, action_quotient_id, action_sdf_canonicalize};
    use super::*;

    fn hex(b: &[u8; 32]) -> String {
        b.iter().map(|x| format!("{x:02x}")).collect()
    }

    /// The quotient recorded at the V-L10-CHAIR-01 ceremony (bits = 6) from the egoff-credit tree this
    /// module replaced; a change to the tree or to the canonicaliser moves it.
    #[test]
    fn chair_quotient_matches_the_ceremony_record() {
        let shape = action_sdf_canonicalize(&compose_chair_action(true, 1.0), 6)
            .expect("canonicalize chair");
        assert_eq!(
            hex(&action_quotient_id(&shape).0),
            "77eb457222fbe151d356842ee93540f5ee0b6d8dea560642aa058354779b8f81"
        );
    }

    #[test]
    fn chair_without_back_has_another_quotient() {
        let full = compose_chair_action(true, 1.0);
        let no_back = compose_chair_action(false, 1.0);
        assert_ne!(
            action_geometry_quotient_key(&full),
            action_geometry_quotient_key(&no_back)
        );
        let a = action_sdf_canonicalize(&full, 6).expect("full");
        let b = action_sdf_canonicalize(&no_back, 6).expect("no back");
        assert_ne!(action_quotient_id(&a), action_quotient_id(&b));
    }

    #[test]
    fn legs_keep_their_order_and_scale() {
        let Action::Compose { children, .. } = compose_chair_action(true, 2.0) else {
            panic!("chair root is a composition");
        };
        let legs = children
            .iter()
            .find_map(|c| match c {
                Action::Compose { children: cs, .. } if cs.len() == 4 => Some(cs),
                _ => None,
            })
            .expect("legs union");
        for (leg, base) in legs.iter().zip(CHAIR_LEG_BASES) {
            let Action::Cylinder {
                base: b,
                radius,
                height,
                ..
            } = leg
            else {
                panic!("a leg is a cylinder");
            };
            assert_eq!(*b, [base[0] * 2.0, 0.0, base[2] * 2.0]);
            assert!((radius - 0.06).abs() < 1e-12 && (height - 0.9).abs() < 1e-12);
        }
    }
}
