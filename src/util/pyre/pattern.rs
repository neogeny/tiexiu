// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Regex Pretty Print - converts regex patterns to readable Python raw strings
//!
//! This module uses the pyre Trait Pattern to validate the regex,
//! not fancy_regex.

use super::Pattern;

pub fn truncate_pattern(pattern: &str, max: usize) -> Box<str> {
    if pattern.len() <= max {
        return pattern.into();
    }

    let end = pattern
        .char_indices()
        .map(|(i, _)| i)
        .nth(max)
        .unwrap_or(pattern.len());

    format!("{}...", &pattern[..end]).into_boxed_str()
}

pub fn regexpp(regex: impl AsRef<str>) -> Result<String, String> {
    let pattern_text = regex.as_ref();

    let ctrl_map: &[(char, &str)] = &[('\n', r"\n"), ('\r', r"\r"), ('\t', r"\t")];

    let mut result = String::new();
    for c in pattern_text.chars() {
        let mut found = false;
        for (from, to) in ctrl_map {
            if c == *from {
                result.push_str(to);
                found = true;
                break;
            }
        }
        if !found {
            result.push(c);
        }
    }

    if result.ends_with('\\') {
        let backslash_count = result.len() - result.trim_end_matches('\\').len();
        if !backslash_count.is_multiple_of(2) {
            result.push('\\');
        }
    }

    let single_quotes = result.matches('\'').count();
    let double_quotes = result.matches('"').count();

    let output = if result.ends_with('\'') || single_quotes > double_quotes {
        let escaped = result.replace('\\', "\\\\").replace('"', "\\\"");
        format!("r\"{}\"", escaped)
    } else {
        let escaped = result.replace('\\', "\\\\").replace('\'', "\\\'");
        format!("r'{}'", escaped)
    };

    let inner = &output[2..output.len() - 1];
    Pattern::new(inner).map_err(|e| format!("Generated invalid regex: {}", e))?;

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regexpp_simple() {
        let result = regexpp(r"\d+").unwrap();
        assert!(result.starts_with("r\"") || result.starts_with("r'"));
    }

    #[test]
    fn test_regexpp_with_backslash() {
        let _ = regexpp(r"\\d").unwrap();
    }

    #[test]
    fn test_regexpp_with_tab() {
        let result = regexpp("a\tb").unwrap();
        assert!(result.contains(r"\t"));
    }
}
