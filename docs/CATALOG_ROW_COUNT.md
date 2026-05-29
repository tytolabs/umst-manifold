# Catalog vs traceability counts

| Surface | Count | Source |
|---------|------:|--------|
| Lean export modules | **119** | `umst-formal-double-slit/artifacts/catalog.json` (unified: 69 double-slit + 50 `umst-formal`) |
| Traceability rows | **48** | `docs/claims-vs-proofs.md` § Lean ↔ catalog_id ↔ Rust |
| Catalog modules not mapped in that table | **73** | Appendix A in `claims-vs-proofs.md` |

Reconcile after catalog export: `APPROVE_CROSS_REPO_MERGE=1` + `export_catalog.py --lean-root Lean --also-lean-root ../umst-formal/Lean`, then refresh manifold `artifacts/catalog.lock.json`.
