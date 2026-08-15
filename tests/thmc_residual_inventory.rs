// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! THMC residual witness inventory probe @ 22:52.

use umst_manifold::physics::solvers::{
    residual_leg_close_authorized, thmc_residual_honesty_holds, thmc_residual_leg_by_id,
    ThmcResidualLegGateStatus, DRIP_INTEGRATION_TESTS, IDEM_INTEGRATION_TESTS,
    RESIDUAL_WITNESS_TEST_TOTAL, THMC_RESIDUAL_INVENTORY, THMC_RESIDUAL_RECEIPT,
    THMC_RESIDUAL_STRING_ERROR_SITES,
};

#[test]
fn thmc_residual_inventory_public_api_honest_at_2252() {
    assert_eq!(THMC_RESIDUAL_RECEIPT, "g_spawn_i_thmc_2252");
    assert_eq!(THMC_RESIDUAL_INVENTORY.len(), 4);
    assert_eq!(RESIDUAL_WITNESS_TEST_TOTAL, 56);
    assert_eq!(IDEM_INTEGRATION_TESTS, 13);
    assert_eq!(DRIP_INTEGRATION_TESTS, 33);
    assert_eq!(THMC_RESIDUAL_STRING_ERROR_SITES, 0);
    assert!(residual_leg_close_authorized());
    assert!(thmc_residual_honesty_holds());

    for leg in ["TOT-3B4", "IDEM", "DRIP", "WIRE"] {
        let row = thmc_residual_leg_by_id(leg).expect("leg row");
        assert_eq!(row.status, ThmcResidualLegGateStatus::Closed);
    }
}
