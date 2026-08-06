// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Const-generic catalog grid witnesses aligned with vault geometry names
//! (`symmetry_quick`, `demo`, `striatus_witness`).
//!
//! Compile-time `(NX, NY, NZ)` and `N_NODES` for stack-sized buffers on verification grids.
//! Striatus witness is dimension-only (no full tensor materialization in unit tests).
//!
//! # Honest boundary (W29-055)
//!
//! Catalog dimensions + node-count formula are landed for verification grids. Striatus remains
//! dimension-catalog only. Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER`.

/// W29 deepen cell — grid witness honest fence bundle.
pub const W29_GRID_WITNESS_DEEPEN_CELL: &str = "W29-055-GRID_WITNESS";

/// Honest posture tag — catalog grid witnesses research / verification lane.
pub const GRID_WITNESS_POSTURE_TAG: &str = "honest-grid-witness-catalog-research-lane";

/// Honest physics posture — catalog contracts pass unit tests; does not certify fleet physics GREEN.
pub const GRID_WITNESS_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by catalog dimensions alone.
pub const GRID_WITNESS_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const GRID_WITNESS_MASTER: bool = false;

/// Whether catalog grids + node formula are landed in this module.
pub const GRID_WITNESS_CATALOG_LANDED: bool = true;

/// Striatus remains dimension-only (no full tensor materialization in unit tests).
pub const GRID_WITNESS_STRIATUS_DIMENSION_ONLY: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const GRID_WITNESS_HONEST_FENCE: &str = concat!(
    "catalog_grids_landed=true ",
    "node_formula_wired=true ",
    "striatus_dimension_only=true ",
    "production_wired=false ",
    "master_composition_wired=false ",
    "physics_green=false"
);

/// Typed probe for grid-witness posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridWitnessPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub catalog_landed: bool,
    pub striatus_dimension_only: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for catalog grid witnesses.
#[must_use]
pub fn grid_witness_honest_posture_bundle() -> GridWitnessPostureProbe {
    GridWitnessPostureProbe {
        physics_green: GRID_WITNESS_PHYSICS_GREEN,
        production_wired: GRID_WITNESS_PRODUCTION_WIRED,
        master: GRID_WITNESS_MASTER,
        catalog_landed: GRID_WITNESS_CATALOG_LANDED,
        striatus_dimension_only: GRID_WITNESS_STRIATUS_DIMENSION_ONLY,
        honest_fence: GRID_WITNESS_HONEST_FENCE,
        posture_tag: GRID_WITNESS_POSTURE_TAG,
        deepen_cell: W29_GRID_WITNESS_DEEPEN_CELL,
    }
}

/// Catalog SSOT landed with production/master/physics-green honestly open.
#[must_use]
pub fn grid_witness_posture_honest(probe: &GridWitnessPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.catalog_landed
        && probe.striatus_dimension_only
        && probe.honest_fence.contains("catalog_grids_landed=true")
        && probe.honest_fence.contains("node_formula_wired=true")
        && probe.honest_fence.contains("striatus_dimension_only=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// Node count from cell counts: `(nx+1)(ny+1)(nz+1)`.
#[must_use]
pub const fn nodes_from_cells(nx: usize, ny: usize, nz: usize) -> usize {
    (nx + 1) * (ny + 1) * (nz + 1)
}

/// Catalog grid dimensions and node count at compile time.
pub trait GridWitness {
    const NX: usize;
    const NY: usize;
    const NZ: usize;
    const N_NODES: usize;

    #[must_use]
    fn dimensions() -> (usize, usize, usize) {
        (Self::NX, Self::NY, Self::NZ)
    }

    #[must_use]
    fn n_cells() -> usize {
        Self::NX * Self::NY * Self::NZ
    }

    #[must_use]
    fn n_nodes() -> usize {
        Self::N_NODES
    }

    /// True when `N_NODES` matches the extruded-plate node formula.
    #[must_use]
    fn node_formula_holds() -> bool {
        Self::N_NODES == nodes_from_cells(Self::NX, Self::NY, Self::NZ)
    }
}

/// 6×4×1 — matches vault geometry `symmetry_quick`.
pub struct SymmetryQuickGrid;

impl GridWitness for SymmetryQuickGrid {
    const NX: usize = 6;
    const NY: usize = 4;
    const NZ: usize = 1;
    const N_NODES: usize = nodes_from_cells(Self::NX, Self::NY, Self::NZ);
}

/// 12×8×1 — matches vault geometry `demo`.
pub struct DemoGrid;

impl GridWitness for DemoGrid {
    const NX: usize = 12;
    const NY: usize = 8;
    const NZ: usize = 1;
    const N_NODES: usize = nodes_from_cells(Self::NX, Self::NY, Self::NZ);
}

/// 40×40×4 — Striatus-scale witness (dimension catalog only; avoids bloating test compile).
pub struct StriatusWitnessGrid;

impl GridWitness for StriatusWitnessGrid {
    const NX: usize = 40;
    const NY: usize = 40;
    const NZ: usize = 4;
    const N_NODES: usize = nodes_from_cells(Self::NX, Self::NY, Self::NZ);
}

/// Named catalog entry for runtime dispatch (3–5 grids).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CatalogGrid {
    SymmetryQuick,
    Demo,
    StriatusWitness,
}

impl CatalogGrid {
    /// Closed catalog (exact three named verification grids).
    pub const ALL: [Self; 3] = [Self::SymmetryQuick, Self::Demo, Self::StriatusWitness];

    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::SymmetryQuick => "symmetry_quick",
            Self::Demo => "demo",
            Self::StriatusWitness => "striatus_witness",
        }
    }

    #[must_use]
    pub fn dimensions(self) -> (usize, usize, usize) {
        match self {
            Self::SymmetryQuick => SymmetryQuickGrid::dimensions(),
            Self::Demo => DemoGrid::dimensions(),
            Self::StriatusWitness => StriatusWitnessGrid::dimensions(),
        }
    }

    #[must_use]
    pub fn n_cells(self) -> usize {
        match self {
            Self::SymmetryQuick => SymmetryQuickGrid::n_cells(),
            Self::Demo => DemoGrid::n_cells(),
            Self::StriatusWitness => StriatusWitnessGrid::n_cells(),
        }
    }

    #[must_use]
    pub fn n_nodes(self) -> usize {
        match self {
            Self::SymmetryQuick => SymmetryQuickGrid::n_nodes(),
            Self::Demo => DemoGrid::n_nodes(),
            Self::StriatusWitness => StriatusWitnessGrid::n_nodes(),
        }
    }

    /// True when this entry is Striatus dimension-catalog only (no test tensor materialization).
    #[must_use]
    pub fn is_dimension_only(self) -> bool {
        matches!(self, Self::StriatusWitness)
    }

    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "symmetry_quick" => Some(Self::SymmetryQuick),
            "demo" => Some(Self::Demo),
            "striatus_witness" | "striatus" => Some(Self::StriatusWitness),
            _ => None,
        }
    }
}

#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
use super::extruded_plate::ExtrudedPlateMechanics;

/// Build [`ExtrudedPlateMechanics`] from a const-generic witness (caller supplies spacing).
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
#[must_use]
pub fn extruded_plate_from_witness<W: GridWitness>(
    dx: f32,
    dy: f32,
    dz: f32,
) -> ExtrudedPlateMechanics {
    ExtrudedPlateMechanics {
        nx: W::NX,
        ny: W::NY,
        nz: W::NZ,
        dx,
        dy,
        dz,
    }
}

/// Honest spacing fence — refuses non-positive `dx`/`dy`/`dz` (no silent degenerate plate).
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
#[must_use]
pub fn try_extruded_plate_from_witness<W: GridWitness>(
    dx: f32,
    dy: f32,
    dz: f32,
) -> Option<ExtrudedPlateMechanics> {
    if !(dx > 0.0 && dy > 0.0 && dz > 0.0) {
        return None;
    }
    if !W::node_formula_holds() {
        return None;
    }
    Some(extruded_plate_from_witness::<W>(dx, dy, dz))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_witness_honest_posture_refuses_green_production_master() {
        let probe = grid_witness_honest_posture_bundle();
        assert!(grid_witness_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(probe.catalog_landed);
        assert!(probe.striatus_dimension_only);
        assert_eq!(probe.deepen_cell, W29_GRID_WITNESS_DEEPEN_CELL);
        assert_eq!(probe.posture_tag, GRID_WITNESS_POSTURE_TAG);
    }

    #[test]
    fn symmetry_quick_node_count() {
        assert_eq!(SymmetryQuickGrid::N_NODES, 70);
        assert_eq!(CatalogGrid::SymmetryQuick.n_nodes(), 70);
        assert!(SymmetryQuickGrid::node_formula_holds());
        assert_eq!(SymmetryQuickGrid::n_cells(), 24);
    }

    #[test]
    fn demo_node_count() {
        assert_eq!(DemoGrid::N_NODES, 13 * 9 * 2);
        assert!(DemoGrid::node_formula_holds());
        assert_eq!(DemoGrid::n_cells(), 12 * 8 * 1);
    }

    #[test]
    fn striatus_witness_dimensions_only() {
        let (nx, ny, nz) = StriatusWitnessGrid::dimensions();
        assert_eq!((nx, ny, nz), (40, 40, 4));
        assert_eq!(StriatusWitnessGrid::N_NODES, 41 * 41 * 5);
        assert!(StriatusWitnessGrid::node_formula_holds());
        assert!(CatalogGrid::StriatusWitness.is_dimension_only());
        assert!(!CatalogGrid::Demo.is_dimension_only());
    }

    #[test]
    fn catalog_from_name_and_roundtrip() {
        assert_eq!(
            CatalogGrid::from_name("symmetry_quick"),
            Some(CatalogGrid::SymmetryQuick)
        );
        assert_eq!(CatalogGrid::from_name("demo"), Some(CatalogGrid::Demo));
        assert_eq!(
            CatalogGrid::from_name("striatus"),
            Some(CatalogGrid::StriatusWitness)
        );
        assert!(CatalogGrid::from_name("unknown").is_none());
        for g in CatalogGrid::ALL {
            assert_eq!(CatalogGrid::from_name(g.name()), Some(g));
            let (nx, ny, nz) = g.dimensions();
            assert_eq!(g.n_nodes(), nodes_from_cells(nx, ny, nz));
        }
        assert_eq!(CatalogGrid::ALL.len(), 3);
    }

    #[test]
    fn nodes_from_cells_formula() {
        assert_eq!(nodes_from_cells(6, 4, 1), 70);
        assert_eq!(nodes_from_cells(0, 0, 0), 1);
    }
}
