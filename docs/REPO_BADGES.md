# Repository badge SSOT

**Slug:** `repo-badges`  
**Purpose:** Single source for README shields and `studiotyto-website` `/umst` cards.

| Repository | Role | Primary CI workflow | Maturity chip | Website card |
|------------|------|---------------------|---------------|--------------|
| `tytolabs/umst-manifold` | runtime engine | `rust.yml` | physics substrate | Manifold + catalog drift (stack hub only) |
| `tytolabs/umst-concrete-cartridge` | applied cartridge | `rust.yml` | PRL agent-layer | Concrete MCP + schemas |
| `tytolabs/umst-ucrs` | constitutional time | `rust.yml` | witness: library+live; p2p: deferred | UCRS Tier-2 stamps |
| `tytolabs/umst-formal` | formal proof | `ci.yml` | Lean 0 sorry (scoped roots) | Meso-layer formal |
| `tytolabs/umst-formal-double-slit` | formal proof | `lean.yml` | quantum RCC verified | Knowing paper |
| `tytolabs/umst-supercap-cartridge` | applied cartridge | `rust.yml` | electrochemistry | Supercap cartridge |
| `tytolabs/MaOS-Workspace` | stack meta | `docs-integrity.yml` | catalog drift link | — |

**Rules**

1. Stack hub (`MaOS-Workspace`, manifold README) shows **catalog drift** badge only — not duplicated on every repo.
2. Formal repos: umbrella **CI** on website; README may keep Lean/Haskell/Formal trio (≤5 shields).
3. UCRS: Rust CI + static maturity labels — no Lean badge until `Lean/` ships.
4. Website cards: one CI badge + one maturity chip + verify link per repo.
