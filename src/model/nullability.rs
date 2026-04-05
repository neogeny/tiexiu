// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::elements::{Element, ParserElem};

impl Element {
    pub fn is_nullable(&self) -> bool {
        self.parser.is_nullable()
    }

    pub fn callable_from(&self) -> Vec<&Element> {
        self.parser.callable_from()
    }
}

impl ParserElem {
    pub fn is_nullable(&self) -> bool {
        match self {
            ParserElem::Nil => true,
            ParserElem::RuleInclude { name: _, exp } => exp.is_nullable(),

            // Consumes nothing, always succeeds (or affects state only)
            ParserElem::Eof => false,

            ParserElem::Cut
            | ParserElem::Void
            | ParserElem::Lookahead(_)
            | ParserElem::NegativeLookahead(_)
            | ParserElem::Optional(_)
            | ParserElem::Constant(_)
            | ParserElem::Alert(..)
            | ParserElem::Closure(_) => true,

            // Always consumes (or fails), never succeeds with zero width
            ParserElem::Fail | ParserElem::Dot | ParserElem::Token(_) => false,

            ParserElem::Pattern(pattern) => {
                // true if it CAN match the empty string (is nullable)
                regex::Regex::new(pattern)
                    .map(|re| re.is_match(""))
                    .unwrap_or(false)
            }

            // Transparent wrappers
            ParserElem::Group(m)
            | ParserElem::SkipGroup(m)
            | ParserElem::Override(m)
            | ParserElem::Named(_, m)
            | ParserElem::OverrideList(m)
            | ParserElem::NamedList(_, m) => m.is_nullable(),

            // Logic-based variants
            ParserElem::Alt(m) => m.is_nullable(),
            ParserElem::Choice(models) => models.iter().any(|m| m.is_nullable()),
            ParserElem::Sequence(models) => models.iter().all(|m| m.is_nullable()),
            ParserElem::PositiveClosure(m) => m.is_nullable(),

            // Join/Gather variants
            ParserElem::Join { .. } | ParserElem::Gather { .. } => true, // These can match zero times
            ParserElem::PositiveJoin { exp, .. } | ParserElem::PositiveGather { exp, .. } => {
                exp.is_nullable()
            }

            // Special cases
            ParserElem::SkipTo(_) => false, // SkipTo must find a match to succeed

            ParserElem::Call(_name, _exp) => {
                // In a stateless walker, you cannot determine this without
                // looking up the definition of _name in the grammar.
                false
            }
        }
    }

    pub fn callable_from(&self) -> Vec<&Element> {
        match self {
            ParserElem::Nil => vec![],
            ParserElem::RuleInclude { name: _, exp } => vec![exp.as_ref()],

            // These don't lead to further rules
            ParserElem::Cut
            | ParserElem::Void
            | ParserElem::Fail
            | ParserElem::Dot
            | ParserElem::Eof
            | ParserElem::Token(_)
            | ParserElem::Pattern(_)
            | ParserElem::Constant(_)
            | ParserElem::Alert(..) => vec![],

            // NOTE: left recursion detection handles this by resolving by name
            ParserElem::Call(_, _) => vec![],

            // Transparent wrappers: return the inner expression
            ParserElem::Group(m)
            | ParserElem::SkipGroup(m)
            | ParserElem::Override(m)
            | ParserElem::Named(_, m)
            | ParserElem::OverrideList(m)
            | ParserElem::NamedList(_, m)
            | ParserElem::Lookahead(m)
            | ParserElem::NegativeLookahead(m)
            | ParserElem::Optional(m)
            | ParserElem::Closure(m)
            | ParserElem::PositiveClosure(m)
            | ParserElem::Alt(m)
            | ParserElem::SkipTo(m) => vec![m.as_ref()],

            // Choice: Any option is a potential "next" step
            ParserElem::Choice(models) => models.iter().collect(),

            // Sequence: Collect all leading nullable elements plus the first non-nullable one
            ParserElem::Sequence(models) => {
                let mut result = Vec::new();
                for m in models {
                    result.push(m);
                    if !m.is_nullable() {
                        break;
                    }
                }
                result
            }

            // Join/Gather variants: the expression is always reachable
            ParserElem::Join { exp, .. }
            | ParserElem::PositiveJoin { exp, .. }
            | ParserElem::Gather { exp, .. }
            | ParserElem::PositiveGather { exp, .. } => vec![exp.as_ref()],
        }
    }
}
