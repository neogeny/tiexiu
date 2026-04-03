// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use tiexiu::contexts::strctx::StrCtx;
use tiexiu::grammars::{Grammar, Model, S};
use tiexiu::input::StrCursor;
use tiexiu::input::strcursor::DefaultPatterns;

fn scope() -> (Model, Model) {
    let a = Model::Token("a".into());
    let b = Model::Token("b".into());
    (a, b)
}

fn test_build() {
    let (a, b) = scope();
    let c = Model::Token("c".into());
    let v = Model::Void;
    let r = Model::closure(c);
    // let cl = Call::new("test");
    let seq = Model::Sequence([a, b, r, v].into());

    let cur: StrCursor<DefaultPatterns> = StrCursor::new("a b c c c");
    let grammar = Grammar::new("test", &[]);
    let ctx = StrCtx::new(cur, &grammar);

    if let Ok(S(_, cst)) = seq.parse(ctx) {
        print!("{:?}", cst);
    }
}

fn main() {
    println!("Hello, world!");
    test_build();
}
