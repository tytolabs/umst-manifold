# Dependabot triage — `sha2` 0.10 → 0.11

**Status:** Deferred — keep `sha2 = "0.10"` in [`Cargo.toml`](../Cargo.toml) until catalog-drift is green with pinned upstream.

**Reason:** Dependabot PR `chore(deps): update sha2 0.10 → 0.11` failed **UMST catalog drift** (unrelated transitive churn risk during formal pin work). Re-open after Phase 2 pin + `VERIFY_TRANSCRIPT.md` land.

**Action when revisiting:** bump `sha2`, run full `verify_umst_stack.sh`, confirm `gate_adversarial` + catalog tests, merge only if green.
