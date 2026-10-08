// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Discretisation of a body on a conforming grid, aggregation of ill-posed nodes, and assembly of the stiffness and
//! consistent mass for a set of supports.
//!
//! Two stages, each a pure function of its inputs: [`discretise`] integrates every cell once (whole cells of one
//! layer and material share one set of matrices), and [`assemble`] numbers the degrees of freedom for given
//! supports and sums the element blocks, expanded through the aggregation prolongation, into profile matrices.

use std::collections::HashMap;
use std::rc::Rc;

use umst_math::profile_ldlt::{ProfilePattern, SymmetricProfile};

use super::element::{element_matrices, shape_values, ElementKind, ElementMatrices};
use super::grid::TensorGrid;
use super::quadrature::{cell_quadrature, CellQuadrature, Exactness, QuadratureSpec};
use super::{real, FiniteCellRefuse, MaterialTable, OccupancyField};

/// How to discretise.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DiscretisationSpec {
    /// Largest cell edge along each axis (m); layer interfaces add node planes.
    pub max_spacing: [f64; 3],
    /// Geometric tolerance (m): zero crossings, plane merging and the quadtree depth derive from it.
    pub tolerance: f64,
    /// Aggregation threshold: a cell with a smaller solid fraction is ill-posed. Zero disables aggregation.
    pub theta: f64,
    /// Element formulation.
    pub kind: ElementKind,
}

/// Mass, centre of mass and inertia tensor about the centre of mass, from the quadrature directly.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MassProperties {
    /// Mass (kg).
    pub mass: f64,
    /// Centre of mass (m).
    pub centroid: [f64; 3],
    /// Inertia tensor about the centre of mass (kg·m²).
    pub inertia: [[f64; 3]; 3],
}

/// Counts that describe the discretisation's faithfulness.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Diagnostics {
    /// Whole cells of one material.
    pub whole: usize,
    /// Cells cut in the plane, integrated exactly on the clipped region.
    pub clipped: usize,
    /// Cells integrated approximately (solid varying through the thickness, or mixed material in a cut leaf).
    pub approximate: usize,
    /// Cells whose incompatible-mode block was singular and fell back to Q1.
    pub condensation_fallback: usize,
    /// Occupied cells below the aggregation threshold.
    pub ill_posed: usize,
    /// Nodes expressed through a neighbouring cell.
    pub aggregated_nodes: usize,
}

/// One occupied cell.
#[derive(Clone, Debug)]
struct Cell {
    nodes: [usize; 8],
    matrices: Rc<ElementMatrices>,
}

/// A node's role in the discrete space.
#[derive(Clone, Debug, PartialEq)]
enum Role {
    /// Touched by no solid.
    Unused,
    /// Carries its own values.
    Free,
    /// The trilinear extrapolation of a well-posed cell: `(node, weight)` over that cell's eight nodes.
    Aggregated([(usize, f64); 8]),
}

/// A discretised body, independent of supports.
#[derive(Clone, Debug)]
pub struct Discretisation {
    grid: TensorGrid,
    cells: Vec<Cell>,
    cell_of: HashMap<[usize; 3], usize>,
    roles: Vec<Role>,
    mass: MassProperties,
    diagnostics: Diagnostics,
    kind: ElementKind,
}

/// Cache key of a whole cell: edges (bit patterns) and material.
type WholeKey = ([u64; 3], u16);

/// State of the integration fold over the grid's cells.
struct Acc {
    cells: Vec<Cell>,
    fractions: Vec<f64>,
    ijks: Vec<[usize; 3]>,
    cache: HashMap<WholeKey, Rc<ElementMatrices>>,
    moments: [f64; 10],
    diagnostics: Diagnostics,
}

/// Integrate every cell of the conforming grid and aggregate the ill-posed nodes.
///
/// # Errors
/// [`FiniteCellRefuse::InvalidGeometry`] for bad spacings or tolerance, [`FiniteCellRefuse::EmptyBody`] when no
/// cell holds solid, [`FiniteCellRefuse::UnknownMaterial`] for a material outside the table, and
/// [`FiniteCellRefuse::AggregationUnresolved`] when an ill-posed cell has no well-posed cell to join.
pub fn discretise<F: OccupancyField + ?Sized>(
    field: &F,
    table: &MaterialTable,
    spec: &DiscretisationSpec,
) -> Result<Discretisation, FiniteCellRefuse> {
    if !(spec.theta.is_finite() && spec.theta >= 0.0 && spec.theta < 1.0) {
        return Err(FiniteCellRefuse::InvalidGeometry);
    }
    let (lo, hi) = field.bounds();
    let planes = [
        field.interface_planes(0),
        field.interface_planes(1),
        field.interface_planes(2),
    ];
    let grid = TensorGrid::conforming(lo, hi, spec.max_spacing, &planes, spec.tolerance)?;
    let largest = spec.max_spacing.iter().fold(0.0_f64, |m, &s| m.max(s));
    let qspec = QuadratureSpec::from_tolerance(largest, spec.tolerance);
    let init = Acc {
        cells: Vec::new(),
        fractions: Vec::new(),
        ijks: Vec::new(),
        cache: HashMap::new(),
        moments: [0.0; 10],
        diagnostics: Diagnostics::default(),
    };
    let acc = grid.cells().try_fold(init, |mut acc, ijk| {
        let (clo, chi) = grid.cell_box(ijk);
        let quad = cell_quadrature(field, clo, chi, &qspec);
        if quad.points.is_empty() {
            return Ok(acc);
        }
        let h: [f64; 3] = std::array::from_fn(|i| chi[i] - clo[i]);
        add_moments(&mut acc.moments, &quad, clo, chi, table)?;
        let whole_material = (quad.exactness == Exactness::Whole).then(|| quad.points[0].material);
        let key = whole_material.map(|m| (h.map(f64::to_bits), m));
        let cached = key.and_then(|k| acc.cache.get(&k).cloned());
        let matrices = if let Some(hit) = cached {
            hit
        } else {
            let built = Rc::new(element_matrices(h, &quad, table, spec.kind)?);
            if let Some(k) = key {
                acc.cache.insert(k, Rc::clone(&built));
            }
            built
        };
        match quad.exactness {
            Exactness::Whole => acc.diagnostics.whole += 1,
            Exactness::Clipped => acc.diagnostics.clipped += 1,
            Exactness::Approximate => acc.diagnostics.approximate += 1,
        }
        acc.diagnostics.condensation_fallback += usize::from(matrices.condensation_fallback);
        acc.fractions.push(quad.volume / h.iter().product::<f64>());
        acc.ijks.push(ijk);
        acc.cells.push(Cell {
            nodes: grid.cell_nodes(ijk),
            matrices,
        });
        Ok::<Acc, FiniteCellRefuse>(acc)
    })?;
    if acc.cells.is_empty() {
        return Err(FiniteCellRefuse::EmptyBody);
    }
    let cell_of: HashMap<[usize; 3], usize> = acc
        .ijks
        .iter()
        .enumerate()
        .map(|(c, &ijk)| (ijk, c))
        .collect();
    let well_posed: Vec<bool> = acc.fractions.iter().map(|&f| f >= spec.theta).collect();
    let roles = aggregate(&grid, &acc.cells, &acc.ijks, &cell_of, &well_posed)?;
    let mut diagnostics = acc.diagnostics;
    diagnostics.ill_posed = well_posed.iter().filter(|&&w| !w).count();
    diagnostics.aggregated_nodes = roles
        .iter()
        .filter(|r| matches!(r, Role::Aggregated(_)))
        .count();
    Ok(Discretisation {
        grid,
        cells: acc.cells,
        cell_of,
        roles,
        mass: mass_properties(&acc.moments),
        diagnostics,
        kind: spec.kind,
    })
}

/// Accumulate `∫ρ`, `∫ρx`, `∫ρ xᵢxⱼ` (upper triangle) over the cell's solid.
fn add_moments(
    m: &mut [f64; 10],
    quad: &CellQuadrature,
    lo: [f64; 3],
    hi: [f64; 3],
    table: &MaterialTable,
) -> Result<(), FiniteCellRefuse> {
    for q in &quad.points {
        let rho = table.density(q.material)?;
        let x: [f64; 3] =
            std::array::from_fn(|i| lo[i] + (q.xi[i] + 1.0) / real(2) * (hi[i] - lo[i]));
        let w = q.weight * rho;
        m[0] += w;
        (0..3).for_each(|i| m[1 + i] += w * x[i]);
        m[4] += w * x[0] * x[0];
        m[5] += w * x[1] * x[1];
        m[6] += w * x[2] * x[2];
        m[7] += w * x[0] * x[1];
        m[8] += w * x[1] * x[2];
        m[9] += w * x[0] * x[2];
    }
    Ok(())
}

fn mass_properties(m: &[f64; 10]) -> MassProperties {
    let mass = m[0];
    let c: [f64; 3] = std::array::from_fn(|i| m[1 + i] / mass);
    // Second moments about the centre of mass, then I = tr(S) 1 − S.
    let s = [
        [
            m[4] - mass * c[0] * c[0],
            m[7] - mass * c[0] * c[1],
            m[9] - mass * c[0] * c[2],
        ],
        [
            m[7] - mass * c[0] * c[1],
            m[5] - mass * c[1] * c[1],
            m[8] - mass * c[1] * c[2],
        ],
        [
            m[9] - mass * c[0] * c[2],
            m[8] - mass * c[1] * c[2],
            m[6] - mass * c[2] * c[2],
        ],
    ];
    let trace = s[0][0] + s[1][1] + s[2][2];
    let inertia = std::array::from_fn(|i| {
        std::array::from_fn(|j| if i == j { trace - s[i][j] } else { -s[i][j] })
    });
    MassProperties {
        mass,
        centroid: c,
        inertia,
    }
}

/// Free nodes are those of well-posed cells; a node touched only by ill-posed cells takes the trilinear
/// extrapolation of the nearest well-posed cell (centre distance, searched in growing Chebyshev rings).
fn aggregate(
    grid: &TensorGrid,
    cells: &[Cell],
    ijks: &[[usize; 3]],
    cell_of: &HashMap<[usize; 3], usize>,
    well_posed: &[bool],
) -> Result<Vec<Role>, FiniteCellRefuse> {
    let mut roles = vec![Role::Unused; grid.node_total()];
    cells
        .iter()
        .zip(well_posed)
        .filter(|(_, &w)| w)
        .for_each(|(c, _)| c.nodes.iter().for_each(|&n| roles[n] = Role::Free));
    let counts = grid.cell_counts();
    let reach = counts.iter().copied().max().unwrap_or(0);
    let centre = |ijk: [usize; 3]| -> [f64; 3] {
        let (lo, hi) = grid.cell_box(ijk);
        std::array::from_fn(|i| (lo[i] + hi[i]) / real(2))
    };
    for (c, cell) in cells.iter().enumerate() {
        if well_posed[c] {
            continue;
        }
        let pending: Vec<usize> = cell
            .nodes
            .iter()
            .copied()
            .filter(|&n| roles[n] == Role::Unused)
            .collect();
        if pending.is_empty() {
            continue;
        }
        let here = ijks[c];
        let root = (1..=reach)
            .find_map(|r| {
                let ring = ring_cells(here, r, counts);
                ring.into_iter()
                    .filter_map(|ijk| {
                        cell_of
                            .get(&ijk)
                            .copied()
                            .filter(|&k| well_posed[k])
                            .map(|k| (k, ijk))
                    })
                    .min_by(|a, b| {
                        let (ca, cb, h) = (centre(a.1), centre(b.1), centre(here));
                        dist2(ca, h).total_cmp(&dist2(cb, h))
                    })
            })
            .ok_or(FiniteCellRefuse::AggregationUnresolved)?;
        let (rlo, rhi) = grid.cell_box(root.1);
        let root_nodes = cells[root.0].nodes;
        for n in pending {
            let p = grid.node_point(grid.node_ijk(n));
            let xi: [f64; 3] =
                std::array::from_fn(|i| (p[i] - rlo[i]) / (rhi[i] - rlo[i]) * real(2) - 1.0);
            let w = shape_values(xi);
            roles[n] = Role::Aggregated(std::array::from_fn(|a| (root_nodes[a], w[a])));
        }
    }
    Ok(roles)
}

fn dist2(a: [f64; 3], b: [f64; 3]) -> f64 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum()
}

/// Cells at Chebyshev distance exactly `r` from `c`, inside the grid.
fn ring_cells(c: [usize; 3], r: usize, counts: [usize; 3]) -> Vec<[usize; 3]> {
    let range = |i: usize| c[i].saturating_sub(r)..=(c[i] + r).min(counts[i] - 1);
    range(0)
        .flat_map(|i| range(1).flat_map(move |j| range(2).map(move |k| [i, j, k])))
        .filter(|ijk| (0..3).map(|a| ijk[a].abs_diff(c[a])).max() == Some(r))
        .collect()
}

/// Which displacement components are fixed at a node, from its coordinates.
pub type Supports<'a> = &'a dyn Fn([f64; 3]) -> [bool; 3];

/// Stiffness and consistent mass for one set of supports, with the map from nodes to degrees of freedom.
#[derive(Clone, Debug)]
pub struct System {
    dof: Vec<[Option<usize>; 3]>,
    n: usize,
    k: SymmetricProfile,
    m: SymmetricProfile,
    supported: bool,
}

impl System {
    /// Number of degrees of freedom.
    #[must_use]
    pub fn n(&self) -> usize {
        self.n
    }

    /// Stiffness.
    #[must_use]
    pub fn stiffness(&self) -> &SymmetricProfile {
        &self.k
    }

    /// Consistent mass.
    #[must_use]
    pub fn mass(&self) -> &SymmetricProfile {
        &self.m
    }

    /// Whether any component is fixed (otherwise the body is free and has six rigid-body modes).
    #[must_use]
    pub fn is_supported(&self) -> bool {
        self.supported
    }

    /// Degree of freedom of component `c` of a free node, if not fixed.
    #[must_use]
    pub fn dof(&self, node: usize, c: usize) -> Option<usize> {
        self.dof.get(node).and_then(|d| d[c])
    }

    /// Row sums of the consistent mass: the lumped mass, which keeps total mass and first moments exactly.
    #[must_use]
    pub fn lumped_mass(&self) -> Vec<f64> {
        self.m.mul(&vec![1.0; self.n]).unwrap_or_default()
    }
}

impl Discretisation {
    /// The grid.
    #[must_use]
    pub fn grid(&self) -> &TensorGrid {
        &self.grid
    }

    /// Mass properties from the quadrature.
    #[must_use]
    pub fn mass_properties(&self) -> MassProperties {
        self.mass
    }

    /// Faithfulness counts.
    #[must_use]
    pub fn diagnostics(&self) -> Diagnostics {
        self.diagnostics
    }

    /// Element formulation.
    #[must_use]
    pub fn kind(&self) -> ElementKind {
        self.kind
    }

    /// Nodes that carry their own values.
    pub fn free_nodes(&self) -> impl Iterator<Item = usize> + '_ {
        self.roles
            .iter()
            .enumerate()
            .filter(|(_, r)| matches!(r, Role::Free))
            .map(|(n, _)| n)
    }

    /// The local corners of a cell expanded into free nodes: `(free node, weight)` per corner.
    fn expansion(&self, cell: &Cell) -> Vec<Vec<(usize, f64)>> {
        cell.nodes
            .iter()
            .map(|&n| match &self.roles[n] {
                Role::Aggregated(w) => w.iter().copied().filter(|&(_, x)| x != 0.0).collect(),
                _ => vec![(n, 1.0)],
            })
            .collect()
    }

    /// Displacement at a point from a vector of degrees of freedom of `system`.
    ///
    /// # Errors
    /// [`FiniteCellRefuse::PointOutsideBody`] outside every occupied cell; [`FiniteCellRefuse::DimMismatch`] for a
    /// vector of the wrong length.
    pub fn value_at(
        &self,
        system: &System,
        u: &[f64],
        p: [f64; 3],
    ) -> Result<[f64; 3], FiniteCellRefuse> {
        let (cell, w) = self.weights_at(p)?;
        if u.len() != system.n {
            return Err(FiniteCellRefuse::DimMismatch);
        }
        let exp = self.expansion(&self.cells[cell]);
        Ok(std::array::from_fn(|c| {
            exp.iter()
                .zip(&w)
                .flat_map(|(terms, wa)| terms.iter().map(move |&(node, t)| (node, wa * t)))
                .map(|(node, wt)| system.dof(node, c).map_or(0.0, |d| wt * u[d]))
                .sum()
        }))
    }

    /// `(free node, weight)` pairs whose combination is the trilinear value at `p`.
    ///
    /// # Errors
    /// [`FiniteCellRefuse::PointOutsideBody`] outside every occupied cell.
    pub fn interpolation_at(&self, p: [f64; 3]) -> Result<Vec<(usize, f64)>, FiniteCellRefuse> {
        let (cell, w) = self.weights_at(p)?;
        let exp = self.expansion(&self.cells[cell]);
        Ok(exp
            .iter()
            .zip(&w)
            .flat_map(|(terms, wa)| terms.iter().map(move |&(n, t)| (n, wa * t)))
            .collect())
    }

    /// `(free node, weight)` pairs of the trilinear value at local coordinates `xi` of cell `ijk` (a point on a
    /// face shared with an empty cell is read from the occupied side this way).
    ///
    /// # Errors
    /// [`FiniteCellRefuse::PointOutsideBody`] when the cell holds no solid.
    pub fn interpolation_in(
        &self,
        ijk: [usize; 3],
        xi: [f64; 3],
    ) -> Result<Vec<(usize, f64)>, FiniteCellRefuse> {
        let cell = *self
            .cell_of
            .get(&ijk)
            .ok_or(FiniteCellRefuse::PointOutsideBody)?;
        let w = shape_values(xi);
        let exp = self.expansion(&self.cells[cell]);
        Ok(exp
            .iter()
            .zip(&w)
            .flat_map(|(terms, wa)| terms.iter().map(move |&(n, t)| (n, wa * t)))
            .collect())
    }

    /// Whether cell `ijk` holds solid.
    #[must_use]
    pub fn is_occupied(&self, ijk: [usize; 3]) -> bool {
        self.cell_of.contains_key(&ijk)
    }

    fn weights_at(&self, p: [f64; 3]) -> Result<(usize, [f64; 8]), FiniteCellRefuse> {
        let ijk = self
            .grid
            .locate(p)
            .ok_or(FiniteCellRefuse::PointOutsideBody)?;
        let cell = *self
            .cell_of
            .get(&ijk)
            .ok_or(FiniteCellRefuse::PointOutsideBody)?;
        let (lo, hi) = self.grid.cell_box(ijk);
        let xi: [f64; 3] =
            std::array::from_fn(|i| (p[i] - lo[i]) / (hi[i] - lo[i]) * real(2) - 1.0);
        Ok((cell, shape_values(xi)))
    }

    /// Coordinates of a node.
    #[must_use]
    pub fn node_point(&self, node: usize) -> [f64; 3] {
        self.grid.node_point(self.grid.node_ijk(node))
    }
}

/// Number the free degrees of freedom for `supports` and assemble stiffness and consistent mass.
///
/// # Errors
/// [`FiniteCellRefuse::SupportOnAggregatedNode`] when a support selects an aggregated node; profile refusals.
pub fn assemble(disc: &Discretisation, supports: Supports<'_>) -> Result<System, FiniteCellRefuse> {
    let mut supported = false;
    let mut next = 0_usize;
    let mut dof = vec![[None; 3]; disc.roles.len()];
    for (node, role) in disc.roles.iter().enumerate() {
        let fixed = supports(disc.node_point(node));
        match role {
            Role::Free => {
                for (c, &is_fixed) in fixed.iter().enumerate() {
                    if is_fixed {
                        supported = true;
                    } else {
                        dof[node][c] = Some(next);
                        next += 1;
                    }
                }
            }
            Role::Aggregated(_) if fixed.iter().any(|&f| f) => {
                return Err(FiniteCellRefuse::SupportOnAggregatedNode)
            }
            _ => {}
        }
    }
    if next == 0 {
        return Err(FiniteCellRefuse::EmptyBody);
    }
    let dof_of = &dof;
    let blocks =
        |cell: &Cell| -> (Vec<usize>, Vec<f64>, Vec<f64>) { cell_blocks(disc, cell, dof_of) };
    let indices = |cell: &Cell| -> Vec<usize> {
        disc.expansion(cell)
            .iter()
            .flatten()
            .flat_map(|&(n, _)| (0..3).filter_map(move |c| dof_of[n][c]))
            .collect()
    };
    let pattern = ProfilePattern::from_cliques(next, disc.cells.iter().map(indices))?;
    let k = pattern.assemble_owned(disc.cells.iter().map(|c| {
        let (d, k, _) = blocks(c);
        (d, k)
    }))?;
    let m = pattern.assemble_owned(disc.cells.iter().map(|c| {
        let (d, _, m) = blocks(c);
        (d, m)
    }))?;
    Ok(System {
        dof,
        n: next,
        k,
        m,
        supported,
    })
}

/// A cell's global dofs and its stiffness and mass blocks after expansion through the aggregation weights and
/// removal of fixed components.
fn cell_blocks(
    disc: &Discretisation,
    cell: &Cell,
    dof: &[[Option<usize>; 3]],
) -> (Vec<usize>, Vec<f64>, Vec<f64>) {
    if cell.nodes.iter().all(|&n| disc.roles[n] == Role::Free) {
        // Identity expansion: the element blocks restricted to the components that are not fixed.
        let local: Vec<(usize, usize)> = (0..24)
            .filter_map(|l| dof[cell.nodes[l / 3]][l % 3].map(|d| (l, d)))
            .collect();
        let ke = &cell.matrices.k;
        let me = &cell.matrices.m;
        let k: Vec<f64> = local
            .iter()
            .flat_map(|&(i, _)| local.iter().map(move |&(j, _)| ke[i * 24 + j]))
            .collect();
        let m: Vec<f64> = local
            .iter()
            .flat_map(|&(i, _)| {
                local.iter().map(move |&(j, _)| {
                    if i % 3 == j % 3 {
                        me[(i / 3) * 8 + j / 3]
                    } else {
                        0.0
                    }
                })
            })
            .collect();
        return (local.iter().map(|&(_, d)| d).collect(), k, m);
    }
    let exp = disc.expansion(cell);
    let mut masters: Vec<usize> = exp.iter().flatten().map(|&(n, _)| n).collect();
    masters.sort_unstable();
    masters.dedup();
    let nm = masters.len();
    // T[a][j]: weight of master j in corner a.
    let t: Vec<Vec<f64>> = exp
        .iter()
        .map(|terms| {
            let mut row = vec![0.0; nm];
            terms.iter().for_each(|&(n, w)| {
                if let Ok(j) = masters.binary_search(&n) {
                    row[j] += w;
                }
            });
            row
        })
        .collect();
    let t = &t;
    let slots: Vec<(usize, usize, usize)> = masters
        .iter()
        .enumerate()
        .flat_map(|(j, &n)| (0..3).filter_map(move |c| dof[n][c].map(|d| (j, c, d))))
        .collect();
    let ke = &cell.matrices.k;
    let me = &cell.matrices.m;
    // Tᵀ K T over corners, per component pair.
    let k_block: Vec<f64> = slots
        .iter()
        .flat_map(|&(ja, ca, _)| {
            slots.iter().map(move |&(jb, cb, _)| {
                (0..8)
                    .flat_map(|a| (0..8).map(move |b| (a, b)))
                    .filter(|&(a, b)| t[a][ja] != 0.0 && t[b][jb] != 0.0)
                    .map(|(a, b)| t[a][ja] * t[b][jb] * ke[(3 * a + ca) * 24 + 3 * b + cb])
                    .sum::<f64>()
            })
        })
        .collect();
    let m_block: Vec<f64> = slots
        .iter()
        .flat_map(|&(ja, ca, _)| {
            slots.iter().map(move |&(jb, cb, _)| {
                if ca == cb {
                    (0..8)
                        .flat_map(|a| (0..8).map(move |b| (a, b)))
                        .filter(|&(a, b)| t[a][ja] != 0.0 && t[b][jb] != 0.0)
                        .map(|(a, b)| t[a][ja] * t[b][jb] * me[a * 8 + b])
                        .sum::<f64>()
                } else {
                    0.0
                }
            })
        })
        .collect();
    (slots.iter().map(|&(_, _, d)| d).collect(), k_block, m_block)
}
