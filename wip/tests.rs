// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Work-in-progress tests translated from TatSu
//!
//! These modules contain Rust tests translated from TatSu's Python tests.
//! They are skeletal and will not compile until TieXiu implements the
//! full EBNF parsing bootstrap.
//!
//! To run clippy over these tests (no compilation required):
//!     cargo clippy --all-features -- -W unused --allow dead_code

pub mod alerts;
pub mod asjson;
pub mod ast;
pub mod bench_parse;
pub mod buffering;
pub mod cli;
pub mod codegen;
pub mod constants;
pub mod defines;
pub mod diagram;
pub mod directive;
pub mod ebnf;
pub mod error;
pub mod firstfollow;
pub mod ideps;
pub mod join;
pub mod keyword;
pub mod left_recursion;
pub mod lookahead;
pub mod misc;
pub mod model;
pub mod ngmodel;
pub mod parameter;
pub mod parser_equivalence;
pub mod parsing;
pub mod pattern;
pub mod pickle;
pub mod pretty;
pub mod railroads;
pub mod regexp;
pub mod return_value;
pub mod safe_name;
pub mod safeeval_stress;
pub mod safeeval;
pub mod semantics;
pub mod string;
pub mod strtools;
pub mod syntax;
pub mod typetools;
pub mod walker;
pub mod z_bootstrap;
