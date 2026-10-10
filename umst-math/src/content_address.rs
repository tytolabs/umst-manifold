// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT

//! SSOT digest hex validation and Blake3 content addressing (shared kernel).

/// The Blake3 proof chain over content-addressed heads.
pub mod chain;

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
        let ok = b.is_ascii_hexdigit();
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

#[cfg(test)]
mod tests {
    use super::*;

    const MIXED_CASE_64: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789ABCDEF";

    #[test]
    fn ssot_digest_hex_accepts_64_mixed_case_hex() {
        assert!(is_ssot_digest_hex(MIXED_CASE_64));
    }

    #[test]
    fn ssot_digest_hex_rejects_length_63() {
        assert!(!is_ssot_digest_hex(&MIXED_CASE_64[..63]));
    }

    #[test]
    fn ssot_digest_hex_rejects_length_65() {
        let mut s = String::from(MIXED_CASE_64);
        s.push('0');
        assert!(!is_ssot_digest_hex(&s));
    }

    #[test]
    fn ssot_digest_hex_rejects_non_hex_byte() {
        let mut s = String::from(MIXED_CASE_64);
        s.replace_range(0..1, "g");
        assert!(!is_ssot_digest_hex(&s));
    }

    #[test]
    fn blake3_content_address_same_parts_is_deterministic() {
        let parts: &[&[u8]] = &[b"alpha", b"beta"];
        let a = blake3_content_address(parts);
        let b = blake3_content_address(parts);
        assert_eq!(a, b);
    }

    #[test]
    fn blake3_content_address_different_part_changes_digest() {
        let one = blake3_content_address(&[b"part-a"]);
        let two = blake3_content_address(&[b"part-b"]);
        assert_ne!(one, two);
    }

    #[test]
    fn blake3_content_address_different_part_count_changes_digest() {
        let single = blake3_content_address(&[b"same"]);
        let pair = blake3_content_address(&[b"same", b""]);
        assert_ne!(single, pair);
    }

    #[test]
    fn blake3_content_address_empty_part_is_deterministic_32_bytes() {
        let once = blake3_content_address(&[b""]);
        let again = blake3_content_address(&[b""]);
        assert_eq!(once.len(), 32);
        assert_eq!(once, again);
    }

    #[test]
    fn blake3_content_address_no_parts_is_deterministic_32_bytes() {
        let once = blake3_content_address(&[]);
        let again = blake3_content_address(&[]);
        assert_eq!(once.len(), 32);
        assert_eq!(once, again);
    }
}
