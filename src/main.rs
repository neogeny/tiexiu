// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use tiexiu::context::strctx::StrCtx;
use tiexiu::input::StrCursor;
use tiexiu::model::{E, Grammar, S};

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

    let cur: StrCursor = StrCursor::new("a b c c c");
    let grammar = Grammar::new("test", &[]);
    let ctx = StrCtx::new(cur, &grammar);

    if let Ok(S(_, cst)) = seq.parse(ctx) {
        println!("{}", cst);
        println!("{}", cst.node());
    }
}

fn cli() {
    use clap::Parser;
    use tiexiu::tool::cli::{Cli, Commands};
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { grammar, input, .. } => {
            println!("Ready to parse {} with grammar {}", input, grammar);
        }
    }
}

fn main() {
    cli();
    println!("Hello, world!");
    test_build();
}
