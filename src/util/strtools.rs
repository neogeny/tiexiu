// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! String utilities for generating valid identifiers.

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::util::pyre::compile;

/// Strict keywords that cannot be used as identifiers (e.g., let, fn).
pub const STRICT_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

/// Reserved keywords currently unused but kept for future language features.
pub const RESERVED_KEYWORDS: &[&str] = &[
    "abstract", "become", "box", "do", "final", "gen", "macro", "override", "priv", "try",
    "typeof", "unsized", "virtual", "yield",
];

/// Weak keywords that are only reserved in specific contexts (e.g., union).
pub const WEAK_KEYWORDS: &[&str] = &["macro_rules", "union", "static", "dyn", "raw", "safe"];

fn isreserved(name: &str) -> bool {
    STRICT_KEYWORDS.contains(&name)
        || RESERVED_KEYWORDS.contains(&name)
        || WEAK_KEYWORDS.contains(&name)
}

/// Sanitizes a string into a valid identifier, using `plug` for replacements.
pub fn safe_name(name: &str, plug: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("Input string cannot be empty.".into());
    }
    if plug.is_empty() || !plug.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(format!(
            "Invalid plug: '{}'. Must be non-empty and alphanumeric.",
            plug
        ));
    }
    if !is_valid_identifier(plug) {
        return Err(format!(
            "Invalid plug: '{}'. Must be valid in identifiers.",
            plug
        ));
    }

    let mut result = name.to_string();

    // Replace non-word characters with plug
    let non_word = compile(r"\W").map_err(|e| e.to_string())?;
    result = non_word.sub(plug, &result, None);

    // If still not valid, filter to alphanumeric only
    if !is_valid_identifier(&result) {
        result = name
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' {
                    c
                } else {
                    plug.chars().next().unwrap()
                }
            })
            .collect();
    }

    // Handle leading digit
    if !result.is_empty() && result.chars().next().unwrap().is_ascii_digit() {
        let prefix = if plug.chars().next().unwrap().is_ascii_digit() {
            "_"
        } else {
            ""
        };
        result = format!("{}{}", prefix, result);
    }

    // Make valid identifier
    if !is_valid_identifier(&result) {
        result = format!("{}{}", plug, result);
    }

    // Append plug if reserved
    while isreserved(&result) {
        result = format!("{}{}", result, plug);
    }

    if !is_valid_identifier(&result) {
        return Err(format!("Failed to sanitize '{}' into '{}'", name, result));
    }

    Ok(result)
}

fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Returns the display width of a string respecting Unicode widths.
pub fn unicode_display_len(s: &str) -> usize {
    s.width()
}

/// Returns the display width of a character respecting Unicode widths.
pub fn unicode_width(c: char) -> usize {
    c.width().unwrap_or(0)
}

/// Returns the number of lines in a string (minimum 1).
///
/// Uses Rust's `str::lines()` which splits on both `\n` and `\r\n`.
pub fn linecount(s: &str) -> usize {
    s.lines().count().max(1)
}

/// Returns true if the string contains more than one line.
pub fn ismultiline(s: &str) -> bool {
    linecount(s) > 1
}

/// SLOC (Source Lines of Code) result with "Editor View" semantics.
#[derive(Debug, Clone, Copy, Default)]
pub struct LineCount {
    /// Total number of lines.
    pub totl: usize,
    /// Blank (empty or whitespace-only) lines.
    pub blnk: usize,
    /// Comment lines.
    pub cmnt: usize,
    /// Code lines.
    pub code: usize,
}

/// Counts source lines of code (SLOC) with "Editor View" semantics.
///
/// Memory-efficient streaming via line iteration.
/// The default comment marker is `#`.
pub fn countlines(s: &str) -> LineCount {
    countlines_with_comment(s, "#")
}

/// Like [`countlines`], but with a configurable comment marker set.
///
/// `cmtstr` contains characters that start a comment (e.g. `"#"` or `"#;"`).
/// Equivalent to Python's `countlines(s, cmtstr)`.
pub fn countlines_with_comment(s: &str, cmtstr: &str) -> LineCount {
    let mut totl = 0usize;
    let mut blnk = 0usize;
    let mut cmnt = 0usize;
    let mut code = 0usize;

    for line in s.lines() {
        totl += 1;

        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            blnk += 1;
        } else if trimmed.starts_with(|c: char| cmtstr.contains(c)) {
            cmnt += 1;
        } else {
            code += 1;
        }
    }

    // Editor View adjustment:
    // If the text ends with a break, there is a final ghost line.
    if totl == 0 && s.ends_with(['\n', '\r']) {
        totl += 1;
        blnk += 1;
    }

    debug_assert_eq!(totl, blnk + cmnt + code);
    LineCount {
        totl,
        blnk,
        cmnt,
        code,
    }
}

/// Converts a CamelCase identifier to snake_case.
pub fn to_snake_case(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Ok(name.into());
    }

    // Convert CamelCase to snake_case using direct character analysis
    let mut result = String::new();
    let chars: Vec<char> = name.chars().collect();

    for i in 0..chars.len() {
        let c = chars[i];
        if c.is_uppercase() {
            if i > 0 {
                // Check if previous char was lowercase or next is lowercase
                let prev_lower = chars[i - 1].is_lowercase();
                let next_lower = if i + 1 < chars.len() {
                    chars[i + 1].is_lowercase()
                } else {
                    false
                };
                if prev_lower || next_lower {
                    result.push('_');
                }
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    // Make valid Python identifier
    safe_name(&result, "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_name_valid() {
        let cases = [
            ("valid_name", "_", "valid_name"),
            ("123invalid", "_", "_123invalid"),
            ("name-with-dash", "_", "name_with_dash"),
            ("let", "_", "let_"),
            ("fn", "_", "fn_"),
            ("mut", "_", "mut_"),
        ];

        for (name, plug, expected) in cases {
            let result = safe_name(name, plug).unwrap();
            assert_eq!(
                result, expected,
                "safe_name({}, {}) = {} expected {}",
                name, plug, result, expected
            );
        }
    }

    #[test]
    fn test_safe_name_unicode() {
        let result = safe_name("name", "_").unwrap();
        assert_eq!(result, "name");
    }

    #[test]
    fn test_to_snake_case() {
        let cases = [
            ("someName", "some_name"),
            ("SomeName", "some_name"),
            ("XMLHttpRequest", "xml_http_request"),
        ];

        for (name, expected) in cases {
            let result = to_snake_case(name).unwrap();
            assert_eq!(
                result, expected,
                "to_snake_case({}) = {} expected {}",
                name, result, expected
            );
        }
    }

    #[test]
    fn test_unicode_display_len() {
        assert_eq!(unicode_display_len("abc"), 3);
        assert_eq!(unicode_display_len("蛇"), 2);
        assert_eq!(unicode_display_len("🐍 Py"), 5);
    }

    #[test]
    fn test_visual_linecount() {
        assert_eq!(linecount(""), 1);
        assert_eq!(linecount("hello"), 1);
        assert_eq!(linecount("hello\n"), 1);
        assert_eq!(linecount("\n\n"), 2);
        assert_eq!(linecount("win\r\nline"), 2);
        assert_eq!(linecount("mac\rline"), 1);
    }

    #[test]
    fn test_linecount_delta() {
        assert_eq!(linecount("") - 1, 0);
        assert_eq!(linecount("hello\n") - 1, 0);
        assert_eq!(linecount("win\r\n") - 1, 0);
    }

    #[test]
    fn test_ismultiline() {
        assert!(!ismultiline(""));
        assert!(!ismultiline("hello"));
        assert!(ismultiline("hello\nworld"));
        assert!(ismultiline("line1\nline2"));
    }

    #[test]
    fn test_sloc_consistency() {
        let result = countlines("");
        assert_eq!(result.totl, 0);
        assert_eq!(result.blnk, 0);
        assert_eq!(result.cmnt, 0);
        assert_eq!(result.code, 0);

        let result = countlines("x=1\n");
        assert_eq!(result.totl, 1);
        assert_eq!(result.blnk, 0);
        assert_eq!(result.cmnt, 0);
        assert_eq!(result.code, 1);

        let result = countlines("\n\n");
        assert_eq!(result.totl, 2);
        assert_eq!(result.blnk, 2);
        assert_eq!(result.cmnt, 0);
        assert_eq!(result.code, 0);

        // comment lines
        let result = countlines("# foo\nx=1\n");
        assert_eq!(result.totl, 2);
        assert_eq!(result.blnk, 0);
        assert_eq!(result.cmnt, 1);
        assert_eq!(result.code, 1);

        // blank lines (whitespace-only)
        let result = countlines("  \nx=1\n");
        assert_eq!(result.totl, 2);
        assert_eq!(result.blnk, 1);
        assert_eq!(result.cmnt, 0);
        assert_eq!(result.code, 1);

        // \r\n handling
        let result = countlines("x=1\r\ny=2");
        assert_eq!(result.totl, 2);
        assert_eq!(result.blnk, 0);
        assert_eq!(result.cmnt, 0);
        assert_eq!(result.code, 2);

        // custom comment marker
        let result = countlines_with_comment("; foo\nx=1\n", ";");
        assert_eq!(result.totl, 2);
        assert_eq!(result.blnk, 0);
        assert_eq!(result.cmnt, 1);
        assert_eq!(result.code, 1);
    }
}
