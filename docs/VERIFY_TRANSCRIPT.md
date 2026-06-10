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
| `upstream_catalog_digest_hex` | `37bf5a18d9f55d6bc671bfac431f2e67df85cc6936780e9ac762765651521ad7` |
| `umst-formal-double-slit` fiber | `ecb4b177bee1148d8cef8bcd129d95e94609e1c6f303d416a3e566441a6bd113` (69 modules) |
| `umst-formal` fiber | `53c43970db00d9b4ae5b11ff1078ccc6b77f03e8ce6573b2e4e3811b6076b1c4` (62 modules) |

**Note:** umst-formal fiber drift from prior lock (`534d9e18…`) was resolved via `make lean-catalog-export` + manifold lock bump per [`CATALOG_UPDATE_PROTOCOL.md`](CATALOG_UPDATE_PROTOCOL.md).

## Tail output (representative)

```
==> bidirectional catalog check (…/scripts/bidirectional_catalog_check.sh)
OK: committed catalog.json matches regen (37bf5a18d9f5…, 119 modules)
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
