// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

// #![allow(dead_code)]
pub mod build;
pub mod grammar;
pub mod leftrec;
pub mod model;
pub mod nullability;
pub mod parser;
pub mod repeat;
pub mod rule;

pub use grammar::Grammar;
pub use model::Model;
pub use parser::{ParseResult, Parser, S};
pub use rule::Rule;
