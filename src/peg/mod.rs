// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

// --- Private Internal Modules ---
// These are files inside src/peg/ (e.g., src/peg/build.rs)
mod build;

// --- Public Submodules ---
pub mod analysis;
pub mod error;
pub mod exp;
pub mod fold;
pub mod grammar;
pub mod parser;
pub mod parsing;
pub mod pretty;
pub mod rule;

// --- Public Re-exports (The "Facade") ---
// This allows users to call `tiexiu::peg::Grammar`
// instead of `tiexiu::peg::grammar::Grammar`
pub use error::{CompileError, CompileResult, ParseError};
pub use exp::{Exp, ExpKind};
pub use grammar::Grammar;
pub use parser::Parser;
pub use rule::Rule;
