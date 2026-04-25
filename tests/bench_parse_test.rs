// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's benchmarks/bench_parse_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use tiexiu::Result;
use tiexiu::compile;
use tiexiu::engine::StrCtx;

const CALC_GRAMMAR: &str = r#"
    @@grammar::CALC

    start: expression $

    expression:
        | expression '+' term
        | expression '-' term
        | term

    term:
        | term '*' factor
        | term '/' factor
        | factor

    factor:
        | '(' expression ')'
        | number

    number: /\d+/
"#;

#[test]
fn test_bench_compile_calc_grammar() -> Result<()> {
    compile(CALC_GRAMMAR, &[])?;
    Ok(())
}

#[test]
#[ignore = "TODO"]
fn test_bench_parse_arithmetic_expression() -> tiexiu::Result<()> {
    // Benchmark: parse an arithmetic expression
    let model = compile(CALC_GRAMMAR, &[])?;
    let ctx = StrCtx::from("3 + 5 * ( 10 - 20 )");
    let _ = model.parse(ctx);
    Ok(())
}

#[test]
#[ignore = "TODO"]
fn test_bench_parse_complex_expression() -> tiexiu::Result<()> {
    // Benchmark: parse a more complex nested expression
    let model = compile(CALC_GRAMMAR, &[])?;
    let ctx = StrCtx::from("((1 + 2) * (3 + 4)) + ((5 - 6) * (7 + 8)) - 9 * (10 + 11)");
    let _result = model.parse(ctx);
    Ok(())
}
