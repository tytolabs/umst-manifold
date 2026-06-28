// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Catalog grid witnesses match vault geometry dimensions.

use umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics;
use umst_manifold::physics::grid_witness::{
    CatalogGrid, DemoGrid, GridWitness, StriatusWitnessGrid, SymmetryQuickGrid,
};

fn assert_witness_matches_plate<W: GridWitness>(dx: f32, dy: f32, dz: f32) {
    let plate = umst_manifold::physics::grid_witness::extruded_plate_from_witness::<W>(dx, dy, dz);
    assert_eq!((plate.nx, plate.ny, plate.nz), W::dimensions());
    assert_eq!(plate.n_nodes(), W::n_nodes());
}

#[test]
fn symmetry_quick_matches_extruded_plate() {
    assert_witness_matches_plate::<SymmetryQuickGrid>(1.0, 1.0, 0.15);
    let (nx, ny, nz) = CatalogGrid::SymmetryQuick.dimensions();
    assert_eq!((nx, ny, nz), (6, 4, 1));
    assert_eq!(CatalogGrid::SymmetryQuick.n_nodes(), 70);
}

#[test]
fn demo_matches_extruded_plate() {
    assert_witness_matches_plate::<DemoGrid>(1.0, 1.0, 0.15);
    let plate = ExtrudedPlateMechanics {
        nx: DemoGrid::NX,
        ny: DemoGrid::NY,
        nz: DemoGrid::NZ,
        dx: 1.0,
        dy: 1.0,
        dz: 0.15,
    };
    assert_eq!(plate.n_nodes(), DemoGrid::N_NODES);
}

#[test]
fn striatus_witness_dimensions() {
    let (nx, ny, nz) = StriatusWitnessGrid::dimensions();
    assert_eq!((nx, ny, nz), (40, 40, 4));
    assert_eq!(StriatusWitnessGrid::N_NODES, 41 * 41 * 5);
}

#[test]
fn catalog_names_align_with_vault() {
    assert_eq!(
        CatalogGrid::from_name("symmetry_quick"),
        Some(CatalogGrid::SymmetryQuick)
    );
    assert_eq!(CatalogGrid::from_name("demo"), Some(CatalogGrid::Demo));
}
