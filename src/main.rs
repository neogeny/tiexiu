// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0
use tiexiu::engine::StrCtx;
use tiexiu::error::Result;
use tiexiu::input::StrCursor;
use tiexiu::peg::{Exp, Grammar, Yeap};
use tiexiu::ui::cli;

#[allow(dead_code)]
fn scope() -> (Exp, Exp) {
    let a = Exp::token("a");
    let b = Exp::token("b");
    (a, b)
}

#[allow(dead_code)]
fn test_build() {
    let (a, b) = scope();
    let c = Exp::token("c");
    let v = Exp::void();
    let r = Exp::closure(c);
    let n = Exp::named("test", b);
    let seq = Exp::sequence([a, n, r, v].into());

    let cur: StrCursor = StrCursor::new("a b c c c");
    let _grammar = Grammar::new("test", &[]);
    let ctx = StrCtx::new(cur, &[]);

    if let Ok(Yeap(_, tree)) = seq.parse(ctx) {
        println!("{}", tree);
        println!("{}", tree.into_node_tree());
    }
}

fn main() -> Result<()> {
    match cli::cli() {
        Err(err) => {
            eprintln!("{:#?}", err);
            Err(err)
        }
        ok => ok,
    }
}
