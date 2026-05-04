// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use tiexiu::context::StrCtx;
use tiexiu::input::StrCursor;
use tiexiu::peg::{Exp, Grammar};

fn bench_token_parse(c: &mut Criterion) {
    let token = Exp::token("hello");
    let cursor: StrCursor = "hello world".into();
    let ctx = StrCtx::new(cursor, &[]);

    c.bench_function("parse_single_token", |b| {
        b.iter_with_setup(
            || (ctx.clone(), token.clone()),
            |(current_ctx, t)| black_box(t.parse(current_ctx)),
        );
    });
}

fn bench_sequence_parse(c: &mut Criterion) {
    let seq = Exp::sequence([Exp::token("a"), Exp::token("b"), Exp::token("c")].into());
    let cursor: StrCursor = "a b c".into();
    let ctx = StrCtx::new(cursor, &[]);

    c.bench_function("parse_sequence_3_tokens", |b| {
        b.iter_with_setup(
            || (ctx.clone(), seq.clone()),
            |(current_ctx, s)| black_box(s.parse(current_ctx)),
        );
    });
}

fn bench_choice_parse(c: &mut Criterion) {
    let choice = Exp::choice([Exp::token("x"), Exp::token("y"), Exp::token("z")].into());

    c.bench_function("parse_choice_first_match", |b| {
        let cursor: StrCursor = "x rest".into();
        let ctx = StrCtx::new(cursor, &[]);
        b.iter_with_setup(
            || (ctx.clone(), choice.clone()),
            |(current_ctx, ch)| black_box(ch.parse(current_ctx)),
        );
    });

    c.bench_function("parse_choice_last_match", |b| {
        let cursor: StrCursor = "z rest".into();
        let ctx = StrCtx::new(cursor, &[]);
        b.iter_with_setup(
            || (ctx.clone(), choice.clone()),
            |(current_ctx, ch)| black_box(ch.parse(current_ctx)),
        );
    });
}

fn bench_closure_parse(c: &mut Criterion) {
    let closure = Exp::closure(Exp::token("a"));
    let cursor: StrCursor = "a a a a a a a a a a".into();
    let ctx = StrCtx::new(cursor, &[]);

    c.bench_function("parse_closure_10_repetitions", |b| {
        b.iter_with_setup(
            || (ctx.clone(), closure.clone()),
            |(current_ctx, cl)| black_box(cl.parse(current_ctx)),
        );
    });
}

fn bench_nested_expression(c: &mut Criterion) {
    let expr = Exp::sequence(
        [
            Exp::token("start"),
            Exp::closure(Exp::choice(
                [Exp::token("foo"), Exp::token("bar"), Exp::token("baz")].into(),
            )),
            Exp::token("end"),
        ]
        .into(),
    );
    let cursor: StrCursor = "start foo bar baz foo bar end".into();
    let ctx = StrCtx::new(cursor, &[]);

    c.bench_function("parse_nested_expression", |b| {
        b.iter_with_setup(
            || (ctx.clone(), expr.clone()),
            |(current_ctx, e)| black_box(e.parse(current_ctx)),
        );
    });
}

fn bench_context_clone(c: &mut Criterion) {
    let cursor: StrCursor = "some text to parse".into();
    let ctx = StrCtx::new(cursor, &[]);

    c.bench_function("context_clone_cow", |b| {
        b.iter(|| black_box(ctx.clone()));
    });
}

fn bench_grammar_from_json(c: &mut Criterion) {
    let json = std::fs::read_to_string("grammar/calc.json").expect("calc.json missing");

    c.bench_function("grammar_load_calc_json", |b| {
        b.iter(|| black_box(Grammar::from_json(&json).unwrap()));
    });
}

fn bench_optional_parse(c: &mut Criterion) {
    let opt = Exp::optional(Exp::token("maybe"));

    c.bench_function("parse_optional_present", |b| {
        let cursor: StrCursor = "maybe rest".into();
        let ctx = StrCtx::new(cursor, &[]);
        b.iter_with_setup(
            || (ctx.clone(), opt.clone()),
            |(current_ctx, o)| black_box(o.parse(current_ctx)),
        );
    });

    c.bench_function("parse_optional_absent", |b| {
        let cursor: StrCursor = "other rest".into();
        let ctx = StrCtx::new(cursor, &[]);
        b.iter_with_setup(
            || (ctx.clone(), opt.clone()),
            |(current_ctx, o)| black_box(o.parse(current_ctx)),
        );
    });
}

fn bench_lookahead_parse(c: &mut Criterion) {
    let la = Exp::lookahead(Exp::token("peek"));
    let cursor: StrCursor = "peek rest".into();
    let ctx = StrCtx::new(cursor, &[]);

    c.bench_function("parse_lookahead", |b| {
        b.iter_with_setup(
            || (ctx.clone(), la.clone()),
            |(current_ctx, l)| black_box(l.parse(current_ctx)),
        );
    });
}

fn bench_named_parse(c: &mut Criterion) {
    let named = Exp::named("label", Exp::token("value"));
    let cursor: StrCursor = "value rest".into();
    let ctx = StrCtx::new(cursor, &[]);

    c.bench_function("parse_named_element", |b| {
        b.iter_with_setup(
            || (ctx.clone(), named.clone()),
            |(current_ctx, n)| black_box(n.parse(current_ctx)),
        );
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
