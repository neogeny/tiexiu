// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use tiexiu::state::strctx::StrCtx;
use tiexiu::input::StrCursor;
use tiexiu::model::{Element, Grammar, S};

fn scope() -> (Element, Element) {
    let a = Element::Token("a".into());
    let b = Element::Token("b".into());
    (a, b)
}

fn test_build() {
    let (a, b) = scope();
    let c = Element::Token("c".into());
    let v = Element::Void;
    let r = Element::closure(c);
    // let cl = Call::new("test");
    let n = Element::named("test", b);
    let seq = Element::Sequence([a, n, r, v].into());

    let cur: StrCursor = StrCursor::new("a b c c c");
    let grammar = Grammar::new("test", &[]);
    let ctx = StrCtx::new(cur, &grammar);

    if let Ok(S(_, tree)) = seq.parse(ctx) {
        println!("{}", tree);
        println!("{}", tree.trimmed());
    }
}

fn cli() {
    use clap::Parser;
    use tiexiu::ui::cli::{Cli, Commands};
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
