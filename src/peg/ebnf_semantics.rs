// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::context::Semantics;
use crate::peg::error::ParseResult;
use crate::trees::Tree::Bottom;
use crate::trees::{Tree, Tree};
use crate::types::{Ref, Str};

/// Default semantics for all Grammar parsing.
///
/// Converts `Meta` nodes (produced by the boot grammar's `@name`/`@int`/etc.)
/// into strongly-typed `NameMeta`/`IntMeta`/etc. variants, and unwraps legacy
/// `"bool"` typename nodes that the old boot grammar produced directly.
#[derive(Debug, Default)]
pub struct EBNFGrammarSemantics;

pub fn new_ebnf_grammar_sematics() -> Ref<dyn Semantics> {
    Ref::new(EBNFGrammarSemantics::new())
}

impl EBNFGrammarSemantics {
    fn new() -> Self {
        EBNFGrammarSemantics
    }
}

impl Semantics for EBNFGrammarSemantics {
    fn apply(&self, node: Tree, _rule_name: &str, _params: &[Str]) -> ParseResult {
        match node.as_ref() {
            // Meta → typed variants (boot grammar @name/@int/@uint/@float/@bool)
            Tree::Node { typename, tree } if typename == "Meta" => {
                let text = tree.value();
                let new_typename: Str = match text.as_ref() {
                    "name" => "NameMeta",
                    "int" => "IntMeta",
                    "uint" => "UIntMeta",
                    "float" => "FloatMeta",
                    "bool" => "BoolMeta",
                    _ => return Ok(Bottom.into()),
                }
                .into();
                Ok(Tree::Node {
                    typename: new_typename,
                    tree: tree.clone(),
                }
                .into())
            }
            // Legacy bare "bool" typename — unwrap transparently
            Tree::Node { typename: t, tree } if t == "bool" => Ok(tree.clone()),
            _ => Ok(Bottom.into()),
        }
    }
}
