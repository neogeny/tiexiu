/// Detects a single empty/whitespace line or the end of the input.
/// Returns Some(0) if text is empty, or the byte offset to the next line.
pub fn empty_line(text: &str) -> Option<usize> {
    if text.is_empty() {
        return Some(0);
    }

    let base_ptr = text.as_ptr() as usize;
    let mut lines = text.lines().peekable();

    // If lines.next() is None on a non-empty string,
    // it's a structural anomaly, but usually text.is_empty() catches it.
    let current = lines.next()?;

    if current.trim().is_empty() {
        match lines.peek() {
            Some(next) => Some(next.as_ptr() as usize - base_ptr),
            None => Some(text.len()),
        }
    } else {
        None
    }
}

/// Detects a "blank line" by identifying two consecutive empty-line boundaries.
/// Returns Some(0) if text is empty.
pub fn blank_line(text: &str) -> Option<usize> {
    let first_offset = empty_line(text)?;
    let second_offset = empty_line(&text[first_offset..])?;
    Some(first_offset + second_offset)
}

/// Returns the byte length of leading whitespace on the current line.
/// Does not care about newlines; it just looks at the start of the slice.
pub fn indent_len(text: &str) -> usize {
    match text.lines().next() {
        None => 0, // End of text has 0 indent
        Some(line) => line
            .chars()
            .take_while(|c| c.is_whitespace())
            .map(|c| c.len_utf8())
            .sum(),
    }
}

/// Detects a Dedent to the margin:
/// An empty line boundary followed by a line with zero leading spaces.
pub fn dedent(text: &str) -> Option<usize> {
    let offset = empty_line(text)?;
    if indent_len(&text[offset..]) == 0 {
        Some(offset)
    } else {
        None
    }
}

/// The indent of the next line
pub fn indent(text: &str) -> Option<usize> {
    let offset = empty_line(text)?;
    let next_part = &text[offset..];

    // If the next line starts with 0 whitespace, it's a dedent.
    // This includes the end of the file (next_part.is_empty()).
    let amount = indent_len(next_part);
    if amount > 0 {
        Some(offset + amount)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_newline_is_blank_at_eot() {
        // A single newline followed by EOT is a blank line
        // because the second empty_line() call sees the empty string.
        assert_eq!(blank_line("\n"), Some(1));
    }

    #[test]
    fn test_empty_input_is_blank() {
        // Ultimate edge case: nothingness is a blank line.
        assert_eq!(blank_line(""), Some(0));
    }

    #[test]
    fn test_standard_blank_line() {
        // Two actual newlines
        assert_eq!(blank_line("\n\n"), Some(2));
    }

    #[test]
    fn test_whitespace_blank_line() {
        // Whitespace lines are still blank lines
        assert_eq!(blank_line("  \n  \n"), Some(6));
    }

    #[test]
    fn test_non_blank_line() {
        // Content prevents the match
        assert_eq!(blank_line("rule\n\n"), None);
        // First line blank, second has content -> Not a blank line separator
        assert_eq!(blank_line("\nrule"), None);
    }

    #[test]
    fn test_empty_line_basic() {
        // Should capture the line and the terminator
        assert_eq!(empty_line("  \nrule"), Some(3));
        assert_eq!(empty_line("  \r\nrule"), Some(4));
    }

    #[test]
    fn test_empty_line_content_fails() {
        assert_eq!(empty_line("content\n"), None);
    }

    #[test]
    fn test_blank_line_requires_two() {
        let input = "  \n  \nrule";
        // blank_line() should return the offset to "rule"
        assert_eq!(blank_line(input), Some(6));
    }

    #[test]
    fn test_blank_line_fails_on_single() {
        let input = "  \nrule";
        assert_eq!(blank_line(input), None);
    }

    #[test]
    fn test_trailing_empty_lines() {
        let input = "  \n  ";
        assert_eq!(empty_line(input), Some(3));
        assert_eq!(blank_line(input), Some(5));
    }

    #[test]
    fn test_none_verifications() {
        assert_eq!(empty_line("text"), None);
        assert_eq!(blank_line(" \ntext"), None);
        assert_eq!(empty_line(""), Some(0));
    }

    #[test]
    fn test_blank_line_and_end() {
        // Two empty lines at the end of a string
        assert_eq!(blank_line(" \n "), Some(3));
    }

    #[test]
    fn test_multiple_blank_lines() {
        // Two empty lines at the end of a string
        assert_eq!(blank_line("  \n\n\n"), Some(4));
    }
}

#[cfg(test)]
mod dedent_tests {
    use super::*;

    #[test]
    fn test_dedent_standard() {
        // Line 1: empty (2 spaces + \n)
        // Line 2: "rule" (0 spaces)
        // Should succeed and return the offset to "rule"
        let input = "  \nrule";
        assert_eq!(dedent(input), Some(3));
    }

    #[test]
    fn test_dedent_negative() {
        // Line 1: empty (1 space + \n)
        // Line 2: "  indented" (2 spaces)
        // Should return None because it is NOT a dedent to the margin
        let input = " \n  indented";
        assert_eq!(dedent(input), None);
    }

    #[test]
    fn test_dedent_at_end_of_file() {
        // Line 1: empty (1 space + \n)
        // Line 2: End of text (0 spaces)
        // Should succeed because EOT is the ultimate margin
        let input = " \n";
        assert_eq!(dedent(input), Some(2));
    }

    #[test]
    fn test_dedent_on_empty_string() {
        // Ultimate margin case
        assert_eq!(dedent(""), Some(0));
    }

    #[test]
    fn test_indent_len_with_tabs() {
        // Ensure indent_len counts bytes correctly for tabs
        assert_eq!(indent_len("\t\tcontent"), 2);
        // Ensure it doesn't count the newline as a space
        assert_eq!(indent_len("\t\n"), 1);
    }

    #[test]
    fn test_dedent_with_windows_line_endings() {
        // Line 1: empty (space + \r \n) -> length 3
        // Line 2: "content" -> offset 3
        let input = " \r\ncontent";
        assert_eq!(dedent(input), Some(3));
    }
}
