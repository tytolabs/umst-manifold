// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Const-generic catalog grid witnesses aligned with vault [`VaultGeometry`] names.
//!
//! Compile-time `(NX, NY, NZ)` and `N_NODES` for stack-sized buffers on verification grids.
//! Striatus witness is dimension-only (no full tensor materialization in unit tests).

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
    fn n_nodes() -> usize {
        Self::N_NODES
    }
}

/// 6×4×1 — matches `VaultGeometry::symmetry_quick`.
pub struct SymmetryQuickGrid;

impl GridWitness for SymmetryQuickGrid {
    const NX: usize = 6;
    const NY: usize = 4;
    const NZ: usize = 1;
    const N_NODES: usize = (Self::NX + 1) * (Self::NY + 1) * (Self::NZ + 1);
}

/// 12×8×1 — matches `VaultGeometry::demo`.
pub struct DemoGrid;

impl GridWitness for DemoGrid {
    const NX: usize = 12;
    const NY: usize = 8;
    const NZ: usize = 1;
    const N_NODES: usize = (Self::NX + 1) * (Self::NY + 1) * (Self::NZ + 1);
}

/// 40×40×4 — Striatus-scale witness (dimension catalog only; avoids bloating test compile).
pub struct StriatusWitnessGrid;

impl GridWitness for StriatusWitnessGrid {
    const NX: usize = 40;
    const NY: usize = 40;
    const NZ: usize = 4;
    const N_NODES: usize = (Self::NX + 1) * (Self::NY + 1) * (Self::NZ + 1);
}

/// Named catalog entry for runtime dispatch (3–5 grids).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CatalogGrid {
    SymmetryQuick,
    Demo,
    StriatusWitness,
}

impl CatalogGrid {
    #[must_use]
    pub fn dimensions(self) -> (usize, usize, usize) {
        match self {
            Self::SymmetryQuick => SymmetryQuickGrid::dimensions(),
            Self::Demo => DemoGrid::dimensions(),
            Self::StriatusWitness => StriatusWitnessGrid::dimensions(),
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

/// Build [`ExtrudedPlateMechanics`] from a const-generic witness (unit spacing).
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
#[must_use]
pub fn extruded_plate_from_witness<W: GridWitness>(dx: f32, dy: f32, dz: f32) -> ExtrudedPlateMechanics {
    ExtrudedPlateMechanics {
        nx: W::NX,
        ny: W::NY,
        nz: W::NZ,
        dx,
        dy,
        dz,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symmetry_quick_node_count() {
        assert_eq!(SymmetryQuickGrid::N_NODES, 70);
        assert_eq!(CatalogGrid::SymmetryQuick.n_nodes(), 70);
    }

    #[test]
    fn demo_node_count() {
        assert_eq!(DemoGrid::N_NODES, 13 * 9 * 2);
    }

    #[test]
    fn striatus_witness_dimensions_only() {
        let (nx, ny, nz) = StriatusWitnessGrid::dimensions();
        assert_eq!((nx, ny, nz), (40, 40, 4));
        assert_eq!(StriatusWitnessGrid::N_NODES, 41 * 41 * 5);
    }

    #[test]
    fn catalog_from_name() {
        assert_eq!(
            CatalogGrid::from_name("symmetry_quick"),
            Some(CatalogGrid::SymmetryQuick)
        );
        assert!(CatalogGrid::from_name("unknown").is_none());
    }
}
