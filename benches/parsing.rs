// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tiexiu::context::strctx::StrCtx;
use tiexiu::input::StrCursor;
use tiexiu::model::{Element, Grammar};

fn bench_token_parse(c: &mut Criterion) {
    let grammar = Grammar::default();
    let token = Element::Token("hello".into());
    let cursor: StrCursor = "hello world".into();
    let ctx = StrCtx::new(cursor, &grammar);

    c.bench_function("parse_single_token", |b| {
        b.iter_with_setup(
            || (ctx.clone(), token.clone()),
            // Pass the owned ctx. parse() will likely consume it or clone it.
            |(current_ctx, t)| black_box(t.parse(current_ctx)),
        );
    });
}

fn bench_sequence_parse(c: &mut Criterion) {
    let grammar = Grammar::default();
    let seq = Element::Sequence(
        [
            Element::Token("a".into()),
            Element::Token("b".into()),
            Element::Token("c".into()),
        ]
            .into(),
    );
    let cursor: StrCursor = "a b c".into();
    let ctx = StrCtx::new(cursor, &grammar);

    c.bench_function("parse_sequence_3_tokens", |b| {
        b.iter_with_setup(
            || (ctx.clone(), seq.clone()),
            |(current_ctx, s)| black_box(s.parse(current_ctx)),
        );
    });
}

fn bench_closure_parse(c: &mut Criterion) {
    let grammar = Grammar::default();
    let closure = Element::Closure(Box::new(Element::Token("a".into())));
    let cursor: StrCursor = "a a a a a a a a a a".into();
    let ctx = StrCtx::new(cursor, &grammar);

    c.bench_function("parse_closure_10_repetitions", |b| {
        b.iter_with_setup(
            || (ctx.clone(), closure.clone()),
            |(current_ctx, cl)| black_box(cl.parse(current_ctx)),
        );
    });
}

// ... Repeat the (current_ctx, element) pattern for other benches ...

criterion_group!(
    benches,
    bench_token_parse,
    bench_sequence_parse,
    bench_closure_parse,
    // Add others here
);
criterion_main!(benches);
