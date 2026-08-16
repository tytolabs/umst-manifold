SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
<!--
-->

# Security Policy

## Supported versions

Security fixes are provided for the latest minor release on the default
branch. Older releases may be patched at the maintainers' discretion.

| Version | Supported |
|---------|-----------|
| 0.1.x   | yes       |

## Reporting a vulnerability

**Please do not open a public GitHub issue for a security vulnerability.**

Send a private report to **santhoshshyamsundar@tyto.studio** with:

- A description of the issue and its impact.
- A minimal reproducer (Rust snippet, input tensor, or test case).
- The affected version(s) and platform(s).
- Optional: a suggested fix.

This project is maintained by a two-person team, so we do not commit to
fixed turnaround SLAs. You can expect:

- Acknowledgement as soon as the report is read; in the typical case
  within a few days.
- A triage assessment and rough timeline once we have reproduced the
  issue.
- Public disclosure only after a fix is available, or by mutual
  agreement.

If you have not heard back within two weeks, please send a polite
follow-up to the same address — the original message may have been
missed.

## Scope

Both correctness defects (incorrect physics that could be exploited to
bypass the thermodynamic admissibility check) and standard software
vulnerabilities are in scope.

## Out of scope

- Issues in third-party dependencies — please report those upstream and
  notify us so we can pin or patch.
- Theoretical attacks on cryptographic primitives we do not use.
