// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use tiexiu::input::StrCursor;
use tiexiu::model::{Element, Grammar, Parser, S};
use tiexiu::state::strctx::StrCtx;
use tiexiu::ui::cli;

fn scope() -> (Element, Element) {
    let a = Element::token("a");
    let b = Element::token("b");
    (a, b)
}

fn test_build() {
    let (a, b) = scope();
    let c = Element::token("c");
    let v = Element::void();
    let r = Element::closure(c);
    let n = Element::named("test", b);
    let seq = Element::sequence([a, n, r, v].into());

    let cur: StrCursor = StrCursor::new("a b c c c");
    let grammar = Grammar::new("test", &[]);
    let ctx = StrCtx::new(cur, &grammar);

    if let Ok(S(_, tree)) = seq.parse(ctx) {
        println!("{}", tree);
        println!("{}", tree.trimmed());
    }
}

fn main() {
    println!("Hello, world!");
    let bootg = Grammar::boot().unwrap();
    println!("Loaded!");
    println!("{}", bootg);
    test_build();
    cli::cli();
}
