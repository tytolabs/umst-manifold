// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: LicenseRef-TYTO-Undecided

//! Blake3 proof chain: each head is `blake3(previous head || label)` (lifted from egoff `proof_chain`, E-3).
//!
//! A chain is a left fold of [`extend_head`] over its labels, so the head after `labels₁ ++ labels₂` is the
//! fold of `labels₂` from the head after `labels₁`. Heads travel as 64-character hex; a continuity check
//! accepts only the exact current head. The proposal commitment is the Blake3 digest of the committed
//! bytes, or the zero block when nothing is committed.

/// A chain head (32 bytes).
pub type ChainHead = [u8; 32];

/// Length of a head in hex.
pub const CHAIN_HEAD_HEX_LEN: usize = 64;

/// The genesis head (all zeros) in hex.
pub const ZERO_HEAD_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// The next head: `blake3(prev || label)`.
#[must_use]
pub fn extend_head(prev: &ChainHead, label: &[u8]) -> ChainHead {
    let mut hasher = blake3::Hasher::new();
    hasher.update(prev);
    hasher.update(label);
    *hasher.finalize().as_bytes()
}

/// The head after extending `start` by each label in order.
#[must_use]
pub fn fold_chain<'a>(start: &ChainHead, labels: impl IntoIterator<Item = &'a [u8]>) -> ChainHead {
    labels
        .into_iter()
        .fold(*start, |head, label| extend_head(&head, label))
}

/// Lowercase hex of a head.
#[must_use]
pub fn head_to_hex(head: &ChainHead) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(CHAIN_HEAD_HEX_LEN);
    for byte in head {
        s.push(char::from(DIGITS[usize::from(byte >> 4)]));
        s.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    s
}

/// Parse 64 hex characters (either case) into a head; any other length or character is refused.
#[must_use]
pub fn try_hex_to_head(s: &str) -> Option<ChainHead> {
    fn nibble(c: u8) -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(c - b'a' + 10),
            b'A'..=b'F' => Some(c - b'A' + 10),
            _ => None,
        }
    }
    let bytes = s.as_bytes();
    if bytes.len() != CHAIN_HEAD_HEX_LEN {
        return None;
    }
    let mut out = [0u8; 32];
    for (slot, pair) in out.iter_mut().zip(bytes.chunks_exact(2)) {
        *slot = (nibble(pair[0])? << 4) | nibble(pair[1])?;
    }
    Some(out)
}

/// Continuity: the caller supplied exactly the current head.
#[must_use]
pub fn check_continuity(expected: &ChainHead, provided_hex: Option<&str>) -> bool {
    provided_hex
        .and_then(try_hex_to_head)
        .is_some_and(|h| h == *expected)
}

/// Blake3 digest of the committed bytes; the zero block when nothing is committed.
#[must_use]
pub fn commitment_digest(committed: Option<&[u8]>) -> ChainHead {
    committed.map_or([0u8; 32], |b| *blake3::hash(b).as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    fn head_of(bytes: Vec<u8>) -> ChainHead {
        let mut h = [0u8; 32];
        for (slot, b) in h.iter_mut().zip(bytes) {
            *slot = b;
        }
        h
    }

    quickcheck! {
        fn hex_round_trips(bytes: Vec<u8>) -> bool {
            let h = head_of(bytes);
            let hex = head_to_hex(&h);
            hex.len() == CHAIN_HEAD_HEX_LEN && try_hex_to_head(&hex) == Some(h)
                && try_hex_to_head(&hex.to_uppercase()) == Some(h)
        }

        fn extend_is_blake3_of_the_concatenation(bytes: Vec<u8>, label: Vec<u8>) -> bool {
            let h = head_of(bytes);
            let mut cat = h.to_vec();
            cat.extend_from_slice(&label);
            extend_head(&h, &label) == *blake3::hash(&cat).as_bytes()
        }

        fn fold_splits_at_any_point(labels: Vec<Vec<u8>>, cut: usize) -> bool {
            let cut = if labels.is_empty() { 0 } else { cut % (labels.len() + 1) };
            let all = fold_chain(&[0; 32], labels.iter().map(Vec::as_slice));
            let mid = fold_chain(&[0; 32], labels[..cut].iter().map(Vec::as_slice));
            all == fold_chain(&mid, labels[cut..].iter().map(Vec::as_slice))
        }

        fn continuity_accepts_only_the_current_head(bytes: Vec<u8>, other: Vec<u8>) -> bool {
            let h = head_of(bytes);
            let o = head_of(other);
            check_continuity(&h, Some(&head_to_hex(&h)))
                && (o == h || !check_continuity(&h, Some(&head_to_hex(&o))))
                && !check_continuity(&h, None)
        }

        fn forks_diverge(bytes: Vec<u8>, a: Vec<u8>, b: Vec<u8>) -> bool {
            let h = head_of(bytes);
            a == b || extend_head(&h, &a) != extend_head(&h, &b)
        }
    }

    #[test]
    fn malformed_hex_is_refused() {
        assert_eq!(try_hex_to_head("00"), None);
        assert_eq!(try_hex_to_head(&"g".repeat(64)), None);
        assert_eq!(try_hex_to_head(ZERO_HEAD_HEX), Some([0; 32]));
        assert_eq!(commitment_digest(None), [0; 32]);
        assert_eq!(
            commitment_digest(Some(b"alpha")),
            *blake3::hash(b"alpha").as_bytes()
        );
    }
}
