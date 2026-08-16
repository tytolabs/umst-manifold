SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
# Environment variable migration (cockpit → UMST)

**Status:** Complete (2026-06-19) — env overrides and registry `name` column `umst_*` identifiers migrated.  
**Breaking:** Operators must update shell exports and deployment configs.

## Pattern (all families)

Legacy deployments used a **vendor-prefixed** environment namespace for the external cockpit runtime.  
All such variables now use the **`UMST_`** prefix with the same suffix (memory, cockpit, discovery, manifold, LLM, toolchain, etc.).

| Family | Migration rule |
|--------|----------------|
| Memory (H4a) | `…_MEMORY_*` vendor prefix → `UMST_MEMORY_*` |
| Cockpit + TUI | `…_COCKPIT_*` / `…_TUI_*` → `UMST_COCKPIT_*` / `UMST_TUI_*` |
| Discovery + tools | `…_DISCOVERY_*`, `…_TOOL_*`, `…_EMBEDDING_*` → `UMST_*` |
| Manifold + energy | `…_MANIFOLD_*`, `…_ENERGY_*`, `…_GPU_*`, `…_NPU_*` → `UMST_*` |
| LLM + misc | `…_LLM_*`, `…_MSDF_*`, `…_MCERT_*`, `…_ACTION_SHAPE_*` → `UMST_*` |
| Toolchain pin | `…_TOOLCHAIN_PIN_STRICT` → `UMST_TOOLCHAIN_PIN_STRICT` |

**Authoritative list:** `umst-math/src/constants/registry.rs` (`env_override` column).

**Verify:** `rg 'env_override: Some\("cockpit' umst-math/src/constants/registry.rs` → empty; `cargo test -p umst-math --lib`.
