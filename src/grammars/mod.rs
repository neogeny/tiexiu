// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

#![allow(dead_code)]
pub mod grammar;
mod leftrec;
pub mod model;
mod nullability;
pub mod parser;
pub mod repeat;
pub mod rule;

pub use model::Model;
pub use parser::{ParseResult, Parser};
pub use rule::Rule;
