// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Sparse 2D tensors (COO / “CSR slice” e‑bisim path).
//! See [`csr::SparseTensor`] for the public surface.
//!
//! Vendored from: `tensors/sparse.rs` in
//! tytolabs/umst-prototype-2a@9c0434d
//! SPDX-License-Identifier: MIT
//! Adaptation: `mod` + `csr` split (RED §0.8 e‑bisim).

mod csr;

pub use csr::{SparseScalar, SparseTensor};
