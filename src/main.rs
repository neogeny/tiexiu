// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use tiexiu::model::{ModelImpl, Call, Token, Sequence, Void, CanParse};
use tiexiu::input::StrCursor;
use tiexiu::engine::Ctx;

fn test_build() {
    let a = Token::new("a");
    let b = Token::new("b");
    let c = Token::new("c");
    let v = Void{};
    // let cl = Call::new("test");
    let seq = Sequence::from(vec![a, b]);

    let cur = StrCursor::new("a b c");
    let ctx = Ctx::new(cur);

    let cst = seq.parse(ctx);
    print!("{:?}", cst);

}

fn main() {
    println!("Hello, world!");
    test_build();
}
