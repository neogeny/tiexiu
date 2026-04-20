// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::json::error::JsonError;
use crate::peg::Grammar;

pub const TATSU_GRAMMAR_JSON: &str = include_str!("../../grammar/tatsu.json");

/// Loads the embedded TatSu grammar from its JSON representation.
pub(crate) fn boot_grammar() -> Result<Grammar, JsonError> {
    Grammar::serde_from_json(TATSU_GRAMMAR_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_bootstrap() -> Result<(), JsonError> {
        // println!("{}", TATSU_GRAMMAR_JSON);
        let grammar = boot_grammar()?;

        assert!(!grammar.name.is_empty(), "Grammar name should not be empty");

        // FIXME: these tests refr to the unavailable Rule.rulemap
        // {
        //     assert!(
        //         !grammar.rulemap.is_empty(),
        //         "Bootstrap grammar should have rulemap"
        //     );
        //     let has_start =
        //         grammar.rulemap.contains_key("start") || grammar.rulemap.contains_key("grammar");
        //     println!("Total rulemap loaded: {}", grammar.rulemap.len());
        //     assert!(has_start, "Bootstrap grammar missing a starting rule");
        // }
        // println!("Successfully bootstrapped grammar: {}", grammar.name);

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
