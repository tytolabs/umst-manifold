// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT

//! SSOT digest hex validation and Blake3 content addressing (shared kernel).

/// Whether `s` is a well-formed 64-char lowercase/uppercase hex digest.
#[must_use]
pub const fn is_ssot_digest_hex(s: &str) -> bool {
    if s.len() != 64 {
        return false;
    }
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < 64 {
        let b = bytes[i];
        let ok = matches!(b, b'A'..=b'F' | b'a'..=b'f' | b'0'..=b'9');
        if !ok {
            return false;
        }
        i += 1;
    }
    true
}

/// Blake3 content address over a canonical payload (parts separated by a zero byte).
#[must_use]
pub fn blake3_content_address(parts: &[&[u8]]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    for p in parts {
        h.update(p);
        h.update(&[0]);
    }
    *h.finalize().as_bytes()
}
