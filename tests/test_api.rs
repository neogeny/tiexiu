// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

#[test]
#[ignore]
fn test_parse() {
    let tree = tiexiu::api::parse("start = /a/", "a", &[]).expect("Failed to parse");
    eprintln!("TREE {:?}", tree);
    assert!(matches!(tree, tiexiu::trees::Tree::Text(..)));
}

#[test]
#[ignore]
fn test_parse_to_json() {
    let json_str = tiexiu::api::parse_to_json("start = /a/", "a", &[]).expect("Failed to parse");
    eprintln!("TREE {:?}", json_str);
    assert!(json_str.contains("\"a\""));
}
