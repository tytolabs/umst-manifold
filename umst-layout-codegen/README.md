SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
# `umst-layout-codegen` (retired — see foundations carve)

This directory is a **tombstone** after Path A1 step **a1-04** (FLEET-COMPOSER-H H71).

## SSOT

The canonical crate now lives at:

`umst/umst-foundations/crates/umst-layout-codegen`

(Legacy short form `umst-foundations/crates/umst-layout-codegen` is the same tree under the reorg prefix.)

## Consumers (retargeted @ H71)

| Consumer | Path |
|----------|------|
| `umst-algebra` | `../umst-layout-codegen` (foundations workspace) |
| `umst/umst-manifold/build.rs` | `../umst-foundations/crates/umst-layout-codegen` |
| `umst-math` | `../../umst-foundations/crates/umst-layout-codegen` |

## OP-5 status

**FAIL** — META-6 freeze-monotonicity re-verify not PASS. No stability relabel. See `docs/OP5_EXCEPTION_UMST_ALGEBRA.md`.
## 🔒 Confidentiality Notice

This repository contains proprietary information. Copyright (c) 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar. All rights reserved. Unauthorized copying, distribution, or use of these files, via any medium, is prohibited.
