// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use tiexiu::context::str::StrCtx;
use tiexiu::model::{Grammar, E, S};
use tiexiu::input::StrCursor;
use tiexiu::input::str::DefaultPatterns;

fn scope() -> (E, E) {
    let a = E::Token("a".into());
    let b = E::Token("b".into());
    (a, b)
}

fn test_build() {
    let (a, b) = scope();
    let c = E::Token("c".into());
    let v = E::Void;
    let r = E::closure(c);
    // let cl = Call::new("test");
    let n = E::named("test", b);
    let seq = E::Sequence([a, n, r, v].into());

    let cur: StrCursor<DefaultPatterns> = StrCursor::new("a b c c c");
    let grammar = Grammar::new("test", &[]);
    let ctx = StrCtx::new(cur, &grammar);

    if let Ok(S(_, cst)) = seq.parse(ctx) {
        println!("{}", cst);
        println!("{}", cst.node());
    }
}

fn main() {
    println!("Hello, world!");
    test_build();
}
