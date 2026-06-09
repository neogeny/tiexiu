// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::peg::error::ParseResult;
use crate::trees::Tree::Bottom;
use crate::trees::TreeRef;
use crate::types::Str;
use std::fmt::Debug;
use std::sync::Arc;

/// A no-op semantics implementation that always falls through to default handling.
#[derive(Debug, Clone, Copy, Default)]
pub struct NullSemantics;

/// Trait for semantics actions applied after a rule parses its expression.
///
/// Each rule's result is passed through `apply()` after folding and before
/// the default param-based `Node` wrapping.
///
/// # Return value conventions
/// - `Ok(Tree::Bottom)` — not handled; let default param-based wrapping proceed.
/// - `Ok(tree)` — semantics has transformed the result; use as-is.
/// - `Err(nope)` — semantics failure; abort the parse.
pub trait Semantics: Debug + Send + Sync {
    fn apply(&self, node: TreeRef, rule_name: &str, params: &[Str]) -> ParseResult;
}

impl Semantics for NullSemantics {
    fn apply(&self, _node: TreeRef, _rule_name: &str, _params: &[Str]) -> ParseResult {
        Ok(Bottom.into())
    }
}

/// Thread-safe reference to a semantics implementation.
pub type SemanticsRef = Arc<dyn Semantics>;
