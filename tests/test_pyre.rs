use tiexiu::util::pyre::Pattern;

#[test]
fn test_search_and_positions() {
    let p = Pattern::new(r"\d+").unwrap();
    let m = p.search("abc 123 def").expect("should find digits");
    assert_eq!(m.start(None), 4);
    assert_eq!(m.end(None), 7);
    assert_eq!(m.group(0).unwrap(), "123");

    // match_ requires the match to start at position 0
    assert!(p.match_("123abc").is_some());
    assert!(p.match_("x123").is_none());

    // fullmatch requires the match to cover the entire input
    assert!(p.fullmatch("123").is_some());
    assert!(p.fullmatch("123abc").is_none());
}

#[test]
fn test_split_and_subn() {
    let p = Pattern::new(r",").unwrap();
    let parts = p.split("a,b,c", None);
    assert_eq!(
        parts,
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );

    let sp = Pattern::new(r"\s+").unwrap();
    let (res, count) = sp.subn(" ", "a   b  c", None);
    assert_eq!(res, "a b c");
    assert_eq!(count, 2);
}

#[test]
fn test_findall_behavior() {
    let p = Pattern::new(r"(\d+)-(\w+)").unwrap();
    // current implementation returns the first capture group when groups>1
    let found = p.findall("123-abc 456-def");
    assert_eq!(
        found,
        vec![
            vec!["123".to_string(), "abc".to_string()],
            vec!["456".to_string(), "def".to_string()],
        ]
    );
}

#[test]
fn test_match_zero_width_lookahead_at_start() {
    let p = Pattern::new(r"(?=\s*(?:\r?\n|\r)\S)").unwrap();

    let m = p.match_("\nnext").expect("lookahead should match at start");
    assert_eq!(m.start(None), 0);
    assert_eq!(m.end(None), 0);
}

#[test]
fn test_match_endrule_unindented_branch() {
    let p = Pattern::new(r"\s*[;]|(?=\s*(?:\r?\n|\r)\S)|(?:\s*(?:\r?\n|\r)){2,}[;]?").unwrap();

    let m = p
        .match_("\nnext_rule")
        .expect("ENDRULE should match before an unindented next rule");
    assert_eq!(m.start(None), 0);
    assert_eq!(m.end(None), 0);
}

#[test]
fn test_match_endrule_blankline_branch() {
    let p = Pattern::new(r"\s*[;]|(?=\s*(?:\r?\n|\r)\S)|(?:\s*(?:\r?\n|\r)){2,}[;]?").unwrap();

    let m = p
        .match_("\n\n")
        .expect("ENDRULE should match repeated line endings");
    assert_eq!(m.group(0).unwrap(), "\n\n");
}

#[test]
fn test_match_endrule_crlf_branch() {
    let p = Pattern::new(r"\s*[;]|(?=\s*(?:\r?\n|\r)\S)|(?:\s*(?:\r?\n|\r)){2,}[;]?").unwrap();

    let m = p
        .match_("\r\nnext_rule")
        .expect("ENDRULE should match CRLF before an unindented next rule");
    assert_eq!(m.start(None), 0);
    assert_eq!(m.end(None), 0);
}
