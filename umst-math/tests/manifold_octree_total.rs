//! Octree: DAG, depth cap, leaves.
use umst_math::manifold::octree::{build_linear_octree_with_depth, no_ancestor_loop};

#[test]
fn aa_manifold_octree_no_self_ancestor() {
    let (nodes, n, _lf) = build_linear_octree_with_depth(4).expect("b");
    assert_eq!(n, 5);
    for i in 0..nodes.len() {
        assert!(no_ancestor_loop(&nodes, i));
    }
}

#[test]
fn aa_manifold_octree_max_depth() {
    let (nodes, _, _) = build_linear_octree_with_depth(12).expect("b");
    let mx = nodes.iter().map(|n| n.depth).max().unwrap();
    assert_eq!(mx, 12);
    assert!(mx <= 16);
}

#[test]
fn aa_manifold_octree_single_leaf() {
    let (nodes, _, nleaf) = build_linear_octree_with_depth(5).expect("b");
    assert_eq!(nleaf, 1);
    assert_eq!(nodes.len(), 6);
}

#[test]
fn aa_manifold_octree_rejects_too_deep() {
    assert!(build_linear_octree_with_depth(0).is_err());
    assert!(build_linear_octree_with_depth(20).is_err());
}

#[test]
fn aa_manifold_octree_parent_chain() {
    let (nodes, _, _) = build_linear_octree_with_depth(3).expect("b");
    let last = nodes.len() - 1;
    assert_eq!(nodes[last].depth, 3);
    assert_eq!(nodes[last].parent, Some(last - 1));
}
