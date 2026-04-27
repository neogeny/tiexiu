// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

// --- Private Internal Modules ---
// These are files inside src/peg/ (e.g., src/peg/build.rs)
mod build;
mod choices;
mod compiler;
mod defines;
mod leftrec;
mod linker;
mod lookahead;
mod nullability;
mod repeat;

// --- Public Submodules ---
pub mod error;
pub mod exp;
pub mod fold;
pub mod grammar;
pub mod nope;
pub mod parser;
pub mod pretty;
pub mod rule;

// --- Public Re-exports (The "Facade") ---
// This allows users to call `tiexiu::peg::Grammar`
// instead of `tiexiu::peg::grammar::Grammar`
pub use compiler::GrammarCompiler;
pub use error::{CompileError, CompileResult, ParseError};
pub use exp::{Exp, ExpKind};
pub use grammar::Grammar;
pub use nope::Nope;
pub use parser::{ParseResult, Parser, Yeap};
pub use rule::Rule;
