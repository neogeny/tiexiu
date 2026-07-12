// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use tiexiu::Result;

#[test]
fn test_parse() -> Result<()> {
    let tree = tiexiu::api::parse_grammar("start = /a/", &[])?;
    let parser = tiexiu::api::compile("start = /a/", &[])?;

    eprintln!("TREE\n{:?}", tree);
    eprintln!("PARSER\n{:?}", parser);
    use tiexiu::peg::ExpKind;
    for rule in parser.rules() {
        match &rule.exp.kind {
            ExpKind::Pattern(p) => {
                assert_eq!(p.as_ref(), "a");
            }
            other => panic!("Unexpected: {:?}", other),
        }
    }
    Ok(())
}

#[test]
fn test_parse_to_json() -> Result<()> {
    let tree = tiexiu::api::parse("start = /a/", "a", &[])?;
    let json_str = tree.to_json_string();
    eprintln!("TREE {:?}", json_str);
    assert!(json_str.contains("\"a\""));
    Ok(())
}
