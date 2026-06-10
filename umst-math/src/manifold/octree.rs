//! AABB octree — directed tree, **no self-ancestor** (I4).

// SPDX-License-Identifier: MIT
use super::error::ManifoldError;

/// One octree node (8-way branching; M-0 uses linear chains for tests).
pub struct OctreeNode {
    pub id: usize,
    pub parent: Option<usize>,
    pub child: [Option<usize>; 8],
    pub depth: u8,
    pub is_leaf: bool,
}

/// Build a **linear** chain of `n_levels` splits (depth `0..=n_levels`), one child per level.
pub fn build_linear_octree_with_depth(
    n_levels: u8,
) -> Result<(Vec<OctreeNode>, usize, usize), ManifoldError> {
    if n_levels == 0 || n_levels as u32 > 16u32 {
        return Err(ManifoldError::OctreeLayout);
    }
    let mut v = Vec::new();
    v.push(OctreeNode {
        id: 0,
        parent: None,
        child: [None; 8],
        depth: 0,
        is_leaf: false,
    });
    for d in 1u8..=n_levels {
        let id = v.len();
        let p = d as usize - 1;
        v.push(OctreeNode {
            id,
            parent: Some(p),
            child: [None; 8],
            depth: d,
            is_leaf: d == n_levels,
        });
        if let Some(par) = v.get_mut(p) {
            par.child[0] = Some(id);
        }
    }
    let nleaf = v.iter().filter(|n| n.is_leaf).count();
    for n in &v {
        if let Some(p) = n.parent {
            if p == n.id {
                return Err(ManifoldError::OctreeLayout);
            }
        }
    }
    let n = v.len();
    Ok((v, n, nleaf))
}

/// Parent-walk from `id` — must always move to a **smaller** index until root.
pub fn no_ancestor_loop(nodes: &[OctreeNode], mut id: usize) -> bool {
    let nmax = nodes.len() + 2;
    for _ in 0..nmax {
        if id >= nodes.len() {
            return true;
        }
        let p = match nodes[id].parent {
            Some(p) if p < id => p,
            None => return true,
            _ => return false,
        };
        if p == id {
            return false;
        }
        id = p;
    }
    false
}
