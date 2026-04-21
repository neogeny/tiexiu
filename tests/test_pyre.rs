use tiexiu::Error;
use tiexiu::Result;
use tiexiu::util::pyre::Pattern;

#[test]
fn test_search_and_positions() -> Result<()> {
    let p = Pattern::new(r"\d+")?;
    let m = p
        .search("abc 123 def")
        .ok_or_else(|| Error::from("should find digits"))?;
    assert_eq!(m.start(None), 4);
    assert_eq!(m.end(None), 7);
    assert_eq!(m.group(0).ok_or_else(|| Error::from("no group 0"))?, "123");

    // match_ requires the match to start at position 0
    assert!(p.match_("123abc").is_some());
    assert!(p.match_("x123").is_none());

    // fullmatch requires the match to cover the entire input
    assert!(p.fullmatch("123").is_some());
    assert!(p.fullmatch("123abc").is_none());
    Ok(())
}

#[test]
fn test_split_and_subn() -> Result<()> {
    let p = Pattern::new(r",")?;
    let parts = p.split("a,b,c", None);
    assert_eq!(
        parts,
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );

    let sp = Pattern::new(r"\s+")?;
    let (res, count) = sp.subn(" ", "a   b  c", None);
    assert_eq!(res, "a b c");
    assert_eq!(count, 2);
    Ok(())
}

#[test]
fn test_findall_behavior() -> Result<()> {
    let p = Pattern::new(r"(\d+)-(\w+)")?;
    // current implementation returns the first capture group when groups>1
    let found = p.findall("123-abc 456-def");
    assert_eq!(
        found,
        vec![
            vec!["123".to_string(), "abc".to_string()],
            vec!["456".to_string(), "def".to_string()],
        ]
    );
    Ok(())
}

#[test]
fn test_match_zero_width_lookahead_at_start() -> Result<()> {
    let p = Pattern::new(r"(?=\s*(?:\r?\n|\r)\S)")?;

    let m = p
        .match_("\nnext")
        .ok_or_else(|| Error::from("lookahead should match at start"))?;
    assert_eq!(m.start(None), 0);
    assert_eq!(m.end(None), 0);
    Ok(())
}

#[test]
fn test_match_endrule_unindented_branch() -> Result<()> {
    let p = Pattern::new(r"\s*[;]|(?=\s*(?:\r?\n|\r)\S)|(?:\s*(?:\r?\n|\r)){2,}[;]?")?;

    let m = p
        .match_("\nnext_rule")
        .ok_or_else(|| Error::from("ENDRULE should match before an unindented next rule"))?;
    assert_eq!(m.start(None), 0);
    assert_eq!(m.end(None), 0);
    Ok(())
}

#[test]
fn test_match_endrule_blankline_branch() -> Result<()> {
    let p = Pattern::new(r"\s*[;]|(?=\s*(?:\r?\n|\r)\S)|(?:\s*(?:\r?\n|\r)){2,}[;?]")?;

    let m = p
        .match_("\n\n")
        .ok_or_else(|| Error::from("ENDRULE should match repeated line endings"))?;
    assert_eq!(m.group(0).ok_or_else(|| Error::from("no group 0"))?, "\n\n");
    Ok(())
}

#[test]
fn test_match_endrule_crlf_branch() -> Result<()> {
    let p = Pattern::new(r"\s*[;]|(?=\s*(?:\r?\n|\r)\S)|(?:\s*(?:\r?\n|\r)){2,}[;?]")?;

    let m = p
        .match_("\r\nnext_rule")
        .ok_or_else(|| Error::from("ENDRULE should match CRLF before an unindented next rule"))?;
    assert_eq!(m.start(None), 0);
    assert_eq!(m.end(None), 0);
    Ok(())
}

#[test]
fn test_multiline_dollar_with_anchor() -> Result<()> {
    let p = Pattern::new(r"(?m)[ \t]*$")?;
    let m = p
        .match_("  \n")
        .ok_or_else(|| Error::from("should match trailing whitespace"))?;
    assert_eq!(m.group(0).ok_or_else(|| Error::from("no group 0"))?, "  ");
    Ok(())
}

#[test]
fn test_tatsu_eol_patterns() -> Result<()> {
    let p1 = Pattern::new(r"(?m)[ \t]*$")?;
    let p2 = Pattern::new(r"(?m)(?:\r?\n|\r)?")?;

    let text = "  \nNext line";

    let m1 = p1
        .match_(text)
        .ok_or_else(|| Error::from("p1 should match"))?;
    assert_eq!(m1.group(0).unwrap(), "  ");

    let text2 = &text[m1.end(None) as usize..];
    let m2 = p2
        .match_(text2)
        .ok_or_else(|| Error::from("p2 should match"))?;
    assert_eq!(m2.group(0).unwrap(), "\n");

    Ok(())
}
