// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

// --- Private Internal Modules ---
// These are files inside src/peg/ (e.g., src/peg/build.rs)
mod build;

/// PEG parsing error types.
pub mod error;
/// PEG expression types (Exp, ExpKind).
pub mod exp;
/// The Grammar type and its parsing methods.
pub mod grammar;
/// The Parser trait for parsing at a context.
pub mod parser;
/// Rule type and its parsing logic.
pub mod rule;

pub(crate) mod analysis;
pub(crate) mod boot;
pub(crate) mod ebnf_semantics;
pub(crate) mod fold;
pub(crate) mod parsing;
pub(crate) mod pretty;

/// Re-export of error types.
pub use error::{CompileError, ParseFailure};
/// Re-export of expression types.
pub use exp::{Exp, ExpKind};
/// Re-export of Grammar.
pub use grammar::Grammar;
/// Re-export of the Parser trait.
pub use parser::Parser;
/// Re-export of Rule.
pub use rule::Rule;
