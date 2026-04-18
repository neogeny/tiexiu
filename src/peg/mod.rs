// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

mod build;
mod compiler;
mod defines;
mod leftrec;
mod linker;
mod lookahead;
mod nullability;
mod repeat;

pub mod error;
pub mod exp;
pub mod fold;
pub mod grammar;
pub mod parser;
pub mod pretty;
pub mod rule;

pub use compiler::GrammarCompiler;
pub use error::CompileError;
pub use error::CompileResult;
pub use error::ParseError;
pub use exp::{Exp, ExpKind};
pub use grammar::Grammar;
pub use parser::{Nope, ParseResult, Parser, Succ};
pub use rule::Rule;
