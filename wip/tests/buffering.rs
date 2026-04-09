// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's buffering_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::parse;

// ============================================================================
// Buffer Tests
// ============================================================================

#[test]
fn test_pos_consistency() {
    let text = "0123456789";
    let buf = crate::buffer::Buffer::new(text);
    let mut line = 0;
    let mut col = 0;
    for (p, c) in text.chars().enumerate() {
        let info = buf.lineinfo(p);
        assert_eq!(line, info.line);
        assert_eq!(col, info.col);
        let d = buf.peek(p);
        assert_eq!(Some(c), d);
    }
}

#[test]
fn test_next_consistency() {
    let text = "hello world";
    let buf = crate::buffer::Buffer::new(text);
    while !buf.atend() {
        let info = buf.lineinfo();
        assert_eq!(buf.line, info.line);
        assert_eq!(buf.col, info.col);
        buf.next();
    }
}

#[test]
fn test_goto_consistency() {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    let text = "01234567890123456789";
    let buf = crate::buffer::Buffer::new(text);
    let mut state = RandomState::new();
    for _ in 0..100 {
        let mut hasher = state.build_hasher();
        hasher.write(text.as_bytes());
        let pos = (hasher.finish() as usize) % text.len();
        buf.goto(pos);
        let info = buf.lineinfo();
        assert_eq!(buf.line, info.line);
        assert_eq!(buf.col, info.col);
    }
}

#[test]
fn test_line_consistency() {
    let text = "line1\nline2\nline3";
    let buf = crate::buffer::Buffer::new(text);
    let lines: Vec<&str> = text.lines().collect();
    for (n, line) in lines.iter().enumerate() {
        assert_eq!(buf.get_line(n), Some(*line));
    }
}

#[test]
fn test_line_info_consistency() {
    let text = "0123456789\nabcdefghij\nXYZ";
    let buf = crate::buffer::Buffer::new(text);
    let lines: Vec<&str> = text.lines().collect();
    let mut line = 0;
    let mut col = 0;
    let mut start = 0;
    for (n, c) in text.chars().enumerate() {
        let info = buf.lineinfo(n);
        assert_eq!(line, info.line);
        assert_eq!(col, info.col);
        assert_eq!(start, info.start);
        assert_eq!(lines[line], info.text);
        col += 1;
        if c == '\n' {
            line += 1;
            col = 0;
            start = n + 1;
        }
    }
}

#[test]
fn test_linecount() {
    assert_eq!(crate::buffer::Buffer::new("").linecount(), 1);
    assert_eq!(crate::buffer::Buffer::new("Hello World!").linecount(), 1);
    assert_eq!(crate::buffer::Buffer::new("\n").linecount(), 2);
}

#[test]
fn test_namechars() {
    let grammar = r#"
        @@namechars :: '-'
        start =
            | "key" ~ ";"
            | "key-word" ~ ";"
            | "key-word-extra" ~ ";"
            ;
    "#;

    let ast = parse(grammar, "key-word-extra;");
    assert_eq!(ast, vec!["key-word-extra", ";"]);
}
