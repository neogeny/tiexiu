// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use tiexiu::context::str::StrCtx;
use tiexiu::input::StrCursor;
use tiexiu::input::str::DefaultPatterns;
use tiexiu::model::{E, Grammar};

fn bench_token_parse(c: &mut Criterion) {
    let token = E::Token("hello".into());
    let grammar = Grammar::default();

    c.bench_function("parse_single_token", |b| {
        b.iter(|| {
            let cursor: StrCursor<DefaultPatterns> = StrCursor::new("hello world");
            let ctx = StrCtx::new(cursor, &grammar);
            black_box(token.parse(ctx))
        });
    });
}

fn bench_sequence_parse(c: &mut Criterion) {
    let seq = E::Sequence(
        [
            E::Token("a".into()),
            E::Token("b".into()),
            E::Token("c".into()),
        ]
        .into(),
    );
    let grammar = Grammar::default();

    c.bench_function("parse_sequence_3_tokens", |b| {
        b.iter(|| {
            let cursor: StrCursor<DefaultPatterns> = StrCursor::new("a b c");
            let ctx = StrCtx::new(cursor, &grammar);
            black_box(seq.parse(ctx))
        });
    });
}

fn bench_choice_parse(c: &mut Criterion) {
    let choice = E::Choice(
        [
            E::Token("x".into()),
            E::Token("y".into()),
            E::Token("z".into()),
        ]
        .into(),
    );
    let grammar = Grammar::default();

    c.bench_function("parse_choice_first_match", |b| {
        b.iter(|| {
            let cursor: StrCursor<DefaultPatterns> = StrCursor::new("x rest");
            let ctx = StrCtx::new(cursor, &grammar);
            black_box(choice.parse(ctx))
        });
    });

    c.bench_function("parse_choice_last_match", |b| {
        b.iter(|| {
            let cursor: StrCursor<DefaultPatterns> = StrCursor::new("z rest");
            let ctx = StrCtx::new(cursor, &grammar);
            black_box(choice.parse(ctx))
        });
    });
}

fn bench_closure_parse(c: &mut Criterion) {
    let closure = E::Closure(Box::new(E::Token("a".into())));
    let grammar = Grammar::default();

    c.bench_function("parse_closure_10_repetitions", |b| {
        b.iter(|| {
            let cursor: StrCursor<DefaultPatterns> = StrCursor::new("a a a a a a a a a a");
            let ctx = StrCtx::new(cursor, &grammar);
            black_box(closure.parse(ctx))
        });
    });
}

fn bench_nested_expression(c: &mut Criterion) {
    let expr = E::Sequence(
        [
            E::Token("start".into()),
            E::Closure(Box::new(E::Choice(
                [
                    E::Token("foo".into()),
                    E::Token("bar".into()),
                    E::Token("baz".into()),
                ]
                .into(),
            ))),
            E::Token("end".into()),
        ]
        .into(),
    );
    let grammar = Grammar::default();

    c.bench_function("parse_nested_expression", |b| {
        b.iter(|| {
            let cursor: StrCursor<DefaultPatterns> =
                StrCursor::new("start foo bar baz foo bar end");
            let ctx = StrCtx::new(cursor, &grammar);
            black_box(expr.parse(ctx))
        });
    });
}

fn bench_context_clone(c: &mut Criterion) {
    let grammar = Grammar::default();

    c.bench_function("context_clone_cow", |b| {
        let cursor: StrCursor<DefaultPatterns> = StrCursor::new("some text to parse");
        let ctx = StrCtx::new(cursor, &grammar);
        b.iter(|| black_box(ctx.clone()));
    });
}

fn bench_grammar_from_json(c: &mut Criterion) {
    let json = std::fs::read_to_string("grammar/calc.json").expect("calc.json should exist");

    c.bench_function("grammar_load_calc_json", |b| {
        b.iter(|| black_box(Grammar::from_json(&json).unwrap()));
    });
}

fn bench_optional_parse(c: &mut Criterion) {
    let opt = E::Optional(Box::new(E::Token("maybe".into())));
    let grammar = Grammar::default();

    c.bench_function("parse_optional_present", |b| {
        b.iter(|| {
            let cursor: StrCursor<DefaultPatterns> = StrCursor::new("maybe rest");
            let ctx = StrCtx::new(cursor, &grammar);
            black_box(opt.parse(ctx))
        });
    });

    c.bench_function("parse_optional_absent", |b| {
        b.iter(|| {
            let cursor: StrCursor<DefaultPatterns> = StrCursor::new("other rest");
            let ctx = StrCtx::new(cursor, &grammar);
            black_box(opt.parse(ctx))
        });
    });
}

fn bench_lookahead_parse(c: &mut Criterion) {
    let la = E::Lookahead(Box::new(E::Token("peek".into())));
    let grammar = Grammar::default();

    c.bench_function("parse_lookahead", |b| {
        b.iter(|| {
            let cursor: StrCursor<DefaultPatterns> = StrCursor::new("peek rest");
            let ctx = StrCtx::new(cursor, &grammar);
            black_box(la.parse(ctx))
        });
    });
}

fn bench_named_parse(c: &mut Criterion) {
    let named = E::Named("label".into(), Box::new(E::Token("value".into())));
    let grammar = Grammar::default();

    c.bench_function("parse_named_element", |b| {
        b.iter(|| {
            let cursor: StrCursor<DefaultPatterns> = StrCursor::new("value rest");
            let ctx = StrCtx::new(cursor, &grammar);
            black_box(named.parse(ctx))
        });
    });
}

criterion_group!(
    benches,
    bench_token_parse,
    bench_sequence_parse,
    bench_choice_parse,
    bench_closure_parse,
    bench_nested_expression,
    bench_context_clone,
    bench_grammar_from_json,
    bench_optional_parse,
    bench_lookahead_parse,
    bench_named_parse,
);
criterion_main!(benches);
