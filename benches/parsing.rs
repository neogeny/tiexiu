// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use tiexiu::context::StrCtx;
use tiexiu::input::StrCursor;
use tiexiu::peg::{Exp, Grammar};

fn bench_token_parse(c: &mut Criterion) {
    let token = Exp::token("hello");
    let cursor: StrCursor = "hello world".into();

    c.bench_function("parse_single_token", |b| {
        b.iter_with_setup(
            || (StrCtx::new(cursor.clone(), &[]), token.clone()),
            |(mut ctx, t)| black_box(t.parse_at(&mut ctx)),
        );
    });
}

fn bench_sequence_parse(c: &mut Criterion) {
    let seq = Exp::sequence([Exp::token("a"), Exp::token("b"), Exp::token("c")].into());

    c.bench_function("parse_sequence_3_tokens", |b| {
        b.iter_with_setup(
            || (StrCtx::new(StrCursor::new("a b c"), &[]), seq.clone()),
            |(mut ctx, s)| black_box(s.parse_at(&mut ctx)),
        );
    });
}

fn bench_choice_parse(c: &mut Criterion) {
    let choice = Exp::choice([Exp::token("x"), Exp::token("y"), Exp::token("z")].into());

    c.bench_function("parse_choice_first_match", |b| {
        b.iter_with_setup(
            || (StrCtx::new(StrCursor::new("x rest"), &[]), choice.clone()),
            |(mut ctx, ch)| black_box(ch.parse_at(&mut ctx)),
        );
    });

    c.bench_function("parse_choice_last_match", |b| {
        b.iter_with_setup(
            || (StrCtx::new(StrCursor::new("z rest"), &[]), choice.clone()),
            |(mut ctx, ch)| black_box(ch.parse_at(&mut ctx)),
        );
    });
}

fn bench_closure_parse(c: &mut Criterion) {
    let closure = Exp::closure(Exp::token("a"));

    c.bench_function("parse_closure_10_repetitions", |b| {
        b.iter_with_setup(
            || {
                (
                    StrCtx::new(StrCursor::new("a a a a a a a a a a"), &[]),
                    closure.clone(),
                )
            },
            |(mut ctx, cl)| black_box(cl.parse_at(&mut ctx)),
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

    c.bench_function("parse_nested_expression", |b| {
        b.iter_with_setup(
            || {
                (
                    StrCtx::new(StrCursor::new("start foo bar baz foo bar end"), &[]),
                    expr.clone(),
                )
            },
            |(mut ctx, e)| black_box(e.parse_at(&mut ctx)),
        );
    });
}

fn bench_grammar_from_json(c: &mut Criterion) {
    let json = include_str!("../grammar/calc.json");

    c.bench_function("grammar_load_calc_json", |b| {
        b.iter(|| black_box(Grammar::from_json(json).unwrap()));
    });
}

fn bench_optional_parse(c: &mut Criterion) {
    let opt = Exp::optional(Exp::token("maybe"));

    c.bench_function("parse_optional_present", |b| {
        b.iter_with_setup(
            || (StrCtx::new(StrCursor::new("maybe rest"), &[]), opt.clone()),
            |(mut ctx, o)| black_box(o.parse_at(&mut ctx)),
        );
    });

    c.bench_function("parse_optional_absent", |b| {
        b.iter_with_setup(
            || (StrCtx::new(StrCursor::new("other rest"), &[]), opt.clone()),
            |(mut ctx, o)| black_box(o.parse_at(&mut ctx)),
        );
    });
}

fn bench_lookahead_parse(c: &mut Criterion) {
    let la = Exp::lookahead(Exp::token("peek"));

    c.bench_function("parse_lookahead", |b| {
        b.iter_with_setup(
            || (StrCtx::new(StrCursor::new("peek rest"), &[]), la.clone()),
            |(mut ctx, l)| black_box(l.parse_at(&mut ctx)),
        );
    });
}

fn bench_named_parse(c: &mut Criterion) {
    let named = Exp::named("label", Exp::token("value"));

    c.bench_function("parse_named_element", |b| {
        b.iter_with_setup(
            || {
                (
                    StrCtx::new(StrCursor::new("value rest"), &[]),
                    named.clone(),
                )
            },
            |(mut ctx, n)| black_box(n.parse_at(&mut ctx)),
        );
    });
}

// --- 10a. End-to-end benchmark ---

fn bench_end_to_end(c: &mut Criterion) {
    let grammar_src = include_str!("../grammar/calc.ebnf");
    let input = "1 + 2 * 3";

    c.bench_function("e2e_compile_parse_json", |b| {
        b.iter(|| {
            let grammar = tiexiu::api::compile(grammar_src, &[]).unwrap();
            let tree = tiexiu::parse_input(&grammar, input, &[]).unwrap();
            black_box(tree.to_json_string());
        });
    });
}

// --- 10b. Input-size-scaling benchmarks ---

fn bench_scaling_sequence(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling_sequence");
    for n in [10, 100, 1_000] {
        let tokens: Vec<&str> = (0..n).map(|_| "x").collect();
        let input = tokens.join(" ");
        let seq = Exp::sequence((0..n).map(|_| Exp::token("x")).collect::<Vec<_>>());
        group.bench_with_input(BenchmarkId::from_parameter(n), &seq, |b, seq| {
            b.iter_with_setup(
                || (StrCtx::new(StrCursor::new(&input), &[]), seq.clone()),
                |(mut ctx, s)| black_box(s.parse_at(&mut ctx)),
            );
        });
    }
    group.finish();
}

fn bench_scaling_choice(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling_choice");
    for n in [10, 100, 1_000] {
        let alts: Vec<Exp> = (0..n).map(|i| Exp::token(&format!("alt{i}"))).collect();
        let last = format!("alt{}", n - 1);
        let input = format!("{last} rest");
        let choice = Exp::choice(alts);
        group.bench_with_input(BenchmarkId::from_parameter(n), &choice, |b, ch| {
            b.iter_with_setup(
                || (StrCtx::new(StrCursor::new(&input), &[]), ch.clone()),
                |(mut ctx, c)| black_box(c.parse_at(&mut ctx)),
            );
        });
    }
    group.finish();
}

fn bench_scaling_closure(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling_closure");
    for n in [10, 100, 1_000] {
        let input = (0..n).map(|_| "a").collect::<Vec<_>>().join(" ");
        let closure = Exp::closure(Exp::token("a"));
        group.bench_with_input(BenchmarkId::from_parameter(n), &closure, |b, cl| {
            b.iter_with_setup(
                || (StrCtx::new(StrCursor::new(&input), &[]), cl.clone()),
                |(mut ctx, c)| black_box(c.parse_at(&mut ctx)),
            );
        });
    }
    group.finish();
}

// --- 10d. TatSu grammar benchmark ---

fn bench_tatsu_grammar(c: &mut Criterion) {
    let grammar_src = include_str!("../grammar/tatsu.ebnf");

    let mut group = c.benchmark_group("tatsu_grammar");
    group.sample_size(10);
    group.bench_function("compile_tatsu_grammar", |b| {
        b.iter(|| black_box(tiexiu::api::compile(grammar_src, &[]).unwrap()));
    });
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().without_plots();
    targets = bench_token_parse,
            bench_sequence_parse,
            bench_choice_parse,
            bench_closure_parse,
            bench_nested_expression,
            bench_grammar_from_json,
            bench_optional_parse,
            bench_lookahead_parse,
            bench_named_parse,
            bench_end_to_end,
            bench_scaling_sequence,
            bench_scaling_choice,
            bench_scaling_closure,
            bench_tatsu_grammar,
);
criterion_main!(benches);
