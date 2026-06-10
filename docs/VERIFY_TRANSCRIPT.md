# VERIFY_TRANSCRIPT — machine run record

**Date:** 2026-06-10  
**Host:** MaOS-Workspace monorepo (local)  
**Exit code:** 0

## Toolchain

| Item | Value |
|------|-------|
| `rustc` | 1.88.0 (6b00bc388 2025-06-23) |
| `PATH` | `$HOME/.cargo/bin` prepended (rustup 1.88; Homebrew 1.86 shadowed) |
| `umst-manifold` | `a8a693af121199c2b0acbf291723816ed7e58c6d` |
| `umst-formal-double-slit` | `0b049b0e3ee190c65e587b1ba47dd81c2b2c4d58` |
| `umst-formal` | `44a8b7bd1e4e793ee5d804001e38dc10fd8e4dc1` |

## Command (exact)

```bash
cd umst-manifold
export PATH="$HOME/.cargo/bin:$PATH"
UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh 2>&1 | tee /tmp/verify_transcript.txt
```

## Catalog pin at run time (R0)

| Field | Value |
|-------|-------|
| `module_count` | 119 |
| `upstream_catalog_digest_hex` | `ef0ed071fc82bf8ebc8971aeee8d142b4b54e15583f0c575d942cb237474d1dc` |
| `umst-formal-double-slit` fiber | `035ea948ff812fddec5fead027e2c02ae96ff44520031df64816fca4f50a579b` (69 modules) |
| `umst-formal` fiber | `265db0ed86ef9d9efe089fb71307ebf508155272513a3f21ab7bc9b43350fa4d` (62 modules) |

**Note:** umst-formal fiber drift from prior lock (`534d9e18…`) was resolved via `make lean-catalog-export` + manifold lock bump per [`CATALOG_UPDATE_PROTOCOL.md`](CATALOG_UPDATE_PROTOCOL.md).

## Tail output (representative)

```
==> bidirectional catalog check (…/scripts/bidirectional_catalog_check.sh)
OK: committed catalog.json matches regen (ef0ed071fc82…, 119 modules)
bidirectional_catalog_check: OK
…
w8_publish_readiness: PASS=21 FAIL=0 SKIP=2
w8_publish_readiness: READY (prep automated; publish remains human-only)
w8_publish_readiness: OK
verify_umst_stack: OK
```

Full log: `/tmp/verify_transcript.txt` on the machine that produced this record.

## Reproduce

See [`VERIFY.md`](VERIFY.md) §1. Requires sibling checkouts `umst-formal-double-slit` and `umst-formal` with Lean trees present.
