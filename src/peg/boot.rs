// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::peg::error::CompileError;
use crate::peg::Grammar;

/// The embedded TatSu PEG grammar serialized as JSON.
pub const TATSU_GRAMMAR_JSON: &str = include_str!("../../grammar/tatsu.json");

/// Loads the embedded TatSu grammar from its JSON representation.
pub(crate) fn boot_grammar() -> Result<Grammar, CompileError> {
    Grammar::from_json(TATSU_GRAMMAR_JSON).map_err(|e| CompileError::Bootstrap(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_bootstrap() -> Result<(), CompileError> {
        let grammar = boot_grammar()?;

        assert!(!grammar.name.is_empty(), "Grammar name should not be empty");

        Ok(())
    }

    #[test]
    fn has_required_rules() {
        let boot = boot_grammar().unwrap();

        let required = [
            "start",
            "grammar",
            "rule",
            "expre",
            "choice",
            "sequence",
            "option",
            "element",
            "term",
            "atom",
            "call",
            "named",
            "named_single",
            "named_list",
            "optional",
            "closure",
            "positive_closure",
            "lookahead",
            "negative_lookahead",
            "token",
            "pattern",
            "regex",
            "constant",
            "eof",
            "cut",
        ];

        for name in required {
            assert!(
                boot.get_rule_id(name).is_ok(),
                "Missing required rule: {}",
                name
            );
        }
    }
}
