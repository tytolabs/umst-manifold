//! Tokenisation and lightweight text normalisation (ASCII/CJK fast path).
//!
//! Moved from Egoff `physics.rs` — see `egoff/egoffimprov.md` Phase 2 §5.4.

use std::collections::HashMap;

/// Split text into tokens, removing punctuation.
///
/// Handles CJK (Chinese, Japanese, Korean) characters as individual tokens.
///
/// Proof: classical token distribution / mesoscopic gate input (see `egoff/egoffimprov.md` §5.4).
/// DOI: 10.5281/zenodo.19159660
pub fn tokenise(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_word = String::new();

    for c in text.chars() {
        let is_cjk = (c as u32) >= 0x4E00 && (c as u32) <= 0x9FFF;

        if c.is_whitespace() || is_cjk {
            if !current_word.is_empty() {
                tokens.push(current_word.to_lowercase());
                current_word = String::new();
            }
            if is_cjk {
                tokens.push(c.to_string());
            }
        } else if c.is_alphanumeric() || c == '\'' {
            current_word.push(c);
        } else if !current_word.is_empty() {
            tokens.push(current_word.to_lowercase());
            current_word = String::new();
        }
    }

    if !current_word.is_empty() {
        tokens.push(current_word.to_lowercase());
    }

    tokens.into_iter().filter(|w| !w.is_empty()).collect()
}

const THINK_OPEN: &str = "<redacted_thinking>";
const THINK_CLOSE: &str = "</redacted_thinking>";

/// Strip `<redacted_thinking>...</redacted_thinking>` traces before entropy measures.
///
/// Proof: orthogonal preprocessing so token/entropy estimators ignore fenced model-internals (Oracle input hygiene; same mesoscopic gate story as [`tokenise`]).
/// DOI: 10.5281/zenodo.19159660
pub fn strip_think_tags(text: &str) -> String {
    let mut result = String::new();
    let mut remaining = text;
    while let Some(start) = remaining.find(THINK_OPEN) {
        result.push_str(&remaining[..start]);
        remaining = &remaining[start + THINK_OPEN.len()..];
        if let Some(end) = remaining.find(THINK_CLOSE) {
            remaining = &remaining[end + THINK_CLOSE.len()..];
        } else {
            remaining = "";
            break;
        }
    }
    result.push_str(remaining);
    result
}

/// Empirical token distribution from a token sequence (classical mesoscopic gate input).
///
/// Proof: empirical mass vector for Shannon **H** on token multiset (see [`tokenise`] / `egoff/egoffimprov.md` §5.4).
/// DOI: 10.5281/zenodo.19159660
pub fn token_distribution(tokens: &[String]) -> HashMap<String, f64> {
    if tokens.is_empty() {
        return HashMap::new();
    }
    let n = tokens.len() as f64;
    let mut counts: HashMap<String, usize> = HashMap::new();
    for t in tokens {
        *counts.entry(t.clone()).or_insert(0) += 1;
    }
    counts.into_iter().map(|(k, v)| (k, v as f64 / n)).collect()
}
