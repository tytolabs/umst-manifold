// SPDX-License-Identifier: MIT
//! W9 cartridge migration honest stub — production inject refused; posture census only.

pub const CARTRIDGE_MIGRATION_MIGRATION_POSTURE_TAG: &str = "honest-w9-migration-stub";
pub const CARTRIDGE_MIGRATION_CARTRIDGE_INJECT_WIRED: bool = false;
pub const CARTRIDGE_MIGRATION_MIGRATION_PHYSICS_GREEN: bool = false;
pub const CARTRIDGE_MIGRATION_MIGRATION_PRODUCTION_WIRED: bool = false;
pub const CARTRIDGE_MIGRATION_MIGRATION_MASTER: bool = false;
pub const CARTRIDGE_MIGRATION_MIGRATION_OP5: bool = false;
pub const CARTRIDGE_MIGRATION_MIGRATION_HONEST_FENCE: &str =
    "cartridge_inject_wired=false|physics_green=false|production_wired=false|master=false|op5=false";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct W9MigrationPostureProbe {
    pub cartridge_inject_wired: bool,
    pub physics_green: bool,
    pub production_wired: bool,
}

#[must_use]
pub const fn cartridge_migration_posture_probe() -> W9MigrationPostureProbe {
    W9MigrationPostureProbe {
        cartridge_inject_wired: CARTRIDGE_MIGRATION_CARTRIDGE_INJECT_WIRED,
        physics_green: CARTRIDGE_MIGRATION_MIGRATION_PHYSICS_GREEN,
        production_wired: CARTRIDGE_MIGRATION_MIGRATION_PRODUCTION_WIRED,
    }
}

#[must_use]
pub const fn cartridge_migration_posture_honest(probe: &W9MigrationPostureProbe) -> bool {
    !probe.cartridge_inject_wired && !probe.physics_green && !probe.production_wired
}

pub fn validate_cartridge_migration_posture_honesty() -> Result<(), &'static str> {
    let p = cartridge_migration_posture_probe();
    if cartridge_migration_posture_honest(&p) {
        Ok(())
    } else {
        Err("cartridge migration posture invented GREEN/production")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cartridge_migration_honesty_fence_refuses_green_production_master_op5() {
        validate_cartridge_migration_posture_honesty().expect("honest stub");
        assert!(!CARTRIDGE_MIGRATION_MIGRATION_PHYSICS_GREEN);
        assert!(!CARTRIDGE_MIGRATION_MIGRATION_PRODUCTION_WIRED);
    }
}
