// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::model::grammar::Grammar;

pub const TATSU_GRAMMAR_JSON: &str = include_str!("../../grammar/tatsu.json");

impl Grammar {
    /// Loads the internal TatSu grammar embedded in the binary.
    pub fn boot() -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_json(TATSU_GRAMMAR_JSON)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_bootstrap() -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", TATSU_GRAMMAR_JSON);
        let grammar = Grammar::boot()?;

        assert!(!grammar.name.is_empty(), "Grammar name should not be empty");
        assert!(
            !grammar.rulemap.is_empty(),
            "Bootstrap grammar should have rulemap"
        );
        let has_start =
            grammar.rulemap.contains_key("start") || grammar.rulemap.contains_key("grammar");

        assert!(has_start, "Bootstrap grammar missing a starting rule");

        println!("Successfully bootstrapped grammar: {}", grammar.name);
        println!("Total rulemap loaded: {}", grammar.rulemap.len());

        Ok(())
    }
}
