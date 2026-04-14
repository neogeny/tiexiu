// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

mod build;
mod compiler;
pub mod error;
pub mod exp;
pub mod fold;
pub mod grammar;
mod leftrec;
pub mod linker;
mod lookahead;
mod nullability;
pub mod parser;
mod pretty;
mod repeat;
pub mod rule;

pub use compiler::{CompileError, CompileResult, GrammarCompiler};
pub use error::ParseError;
pub use exp::{Exp, ExpKind};
pub use grammar::Grammar;
pub use parser::{Nope, ParseResult, Parser, Succ};
pub use rule::Rule;
