# Catalog update protocol

When the Lean catalog changes in `umst-formal-double-slit`:

1. Regenerate export: `python3 tools/lean_export/export_catalog.py` (with `APPROVE_CROSS_REPO_MERGE=1` if merging fibers).
2. Commit `artifacts/catalog.json` + `artifacts/catalog.lock.json` in double-slit; push `master`.
3. In `umst-manifold`: update `artifacts/catalog.lock.json`, digest test in `src/runtime/catalog/tests.rs`, and `.umst-pins.toml` `[umst-formal-double-slit].sha` to the new double-slit commit.
4. Run `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh`; commit `docs/VERIFY_TRANSCRIPT.md` only on exit 0.
5. Push manifold `main`; confirm **UMST catalog drift** workflow green (pinned `ref:` + weekly cron).

## CI layouts

| Layout | Workflow | Upstream double-slit |
|--------|----------|----------------------|
| Standalone `umst-manifold` repo | `.github/workflows/umst-catalog-drift.yml` | Git checkout at `.umst-pins.toml` SHA |
| multi-repo workspace monorepo | root `.github/workflows/umst-catalog-drift.yml` | Sibling directory on disk (keep siblings in sync manually) |

## umst-math lockfile collision

Building egoff workspace with manifold `math-constants` can collide: egoff `umst-math` member vs manifold `vendor/umst-math`. Keep vendor in sync: `bash scripts/sync_vendor_umst_math.sh` in manifold before cross-workspace builds.
