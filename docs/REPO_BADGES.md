# Repository badges

**Canonical SSOT:** [`tyto-workspace/docs/repo-badges.v1.json`](../../docs/repo-badges.v1.json) (machine-readable catalog).

This file records human rules only.

## Policy

| Allowed per README | Forbidden |
|--------------------|-----------|
| 1× GitHub Actions CI badge (primary workflow from JSON) | Meta “Badge SSOT” badge |
| 1× License | Static shields pretending to be build status (`witness:`, `p2p:`, `layer:`) |
| Optional 1× doc link chip (e.g. Agent MCP) | More than 3 shields total |
| Stack hub only: catalog drift badge on manifold + workspace | Duplicating catalog drift on every repo |

## Public copy

- Use **role lines** from JSON (`role` field), not Track/Tier engineering jargon.
- Website cards: CI badge + role line + link to verify doc.

## Website sync

[`studiotyto-website/src/repo-badges.ts`](../../studiotyto-website/src/repo-badges.ts) mirrors JSON entries with `website: true`. Update JSON first, then sync TypeScript.
