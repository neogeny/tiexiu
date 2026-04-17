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
    assert_eq!(blank_line(input), Some(7));
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
    assert_eq!(empty_line(""), None);
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
