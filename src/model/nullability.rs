// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::{Exp, ParserExp};

impl Exp {
    pub fn is_nullable(&self) -> bool {
        self.exp.is_nullable()
    }

    pub fn callable_from(&self) -> Vec<&Exp> {
        self.exp.callable_from()
    }
}

impl ParserExp {
    pub fn is_nullable(&self) -> bool {
        match self {
            ParserExp::Nil => true,
            ParserExp::RuleInclude { name: _, exp } => exp.is_nullable(),

            // Consumes nothing, always succeeds (or affects state only)
            ParserExp::Eof => false,

            ParserExp::Cut
            | ParserExp::Void
            | ParserExp::Lookahead(_)
            | ParserExp::NegativeLookahead(_)
            | ParserExp::Optional(_)
            | ParserExp::Constant(_)
            | ParserExp::Alert(..)
            | ParserExp::Closure(_) => true,

            // Always consumes (or fails), never succeeds with zero width
            ParserExp::Fail | ParserExp::Dot | ParserExp::Token(_) => false,

            ParserExp::Pattern(pattern) => {
                // true if it CAN match the empty string (is nullable)
                regex::Regex::new(pattern)
                    .map(|re| re.is_match(""))
                    .unwrap_or(false)
            }

            // Transparent wrappers
            ParserExp::Group(m)
            | ParserExp::SkipGroup(m)
            | ParserExp::Override(m)
            | ParserExp::Named(_, m)
            | ParserExp::OverrideList(m)
            | ParserExp::NamedList(_, m) => m.is_nullable(),

            // Logic-based variants
            ParserExp::Alt(m) => m.is_nullable(),
            ParserExp::Choice(models) => models.iter().any(|m| m.is_nullable()),
            ParserExp::Sequence(models) => models.iter().all(|m| m.is_nullable()),
            ParserExp::PositiveClosure(m) => m.is_nullable(),

            // Join/Gather variants
            ParserExp::Join { .. } | ParserExp::Gather { .. } => true, // These can match zero times
            ParserExp::PositiveJoin { exp, .. } | ParserExp::PositiveGather { exp, .. } => {
                exp.is_nullable()
            }

            // Special cases
            ParserExp::SkipTo(_) => false, // SkipTo must find a match to succeed

            ParserExp::Call(_name, _exp) => {
                // In a stateless walker, you cannot determine this without
                // looking up the definition of _name in the grammar.
                false
            }
        }
    }

    pub fn callable_from(&self) -> Vec<&Exp> {
        match self {
            ParserExp::Nil => vec![],
            ParserExp::RuleInclude { name: _, exp } => vec![exp.as_ref()],

            // These don't lead to further rules
            ParserExp::Cut
            | ParserExp::Void
            | ParserExp::Fail
            | ParserExp::Dot
            | ParserExp::Eof
            | ParserExp::Token(_)
            | ParserExp::Pattern(_)
            | ParserExp::Constant(_)
            | ParserExp::Alert(..) => vec![],

            // NOTE: left recursion detection handles this by resolving by name
            ParserExp::Call(_, _) => vec![],

            // Transparent wrappers: return the inner expression
            ParserExp::Group(m)
            | ParserExp::SkipGroup(m)
            | ParserExp::Override(m)
            | ParserExp::Named(_, m)
            | ParserExp::OverrideList(m)
            | ParserExp::NamedList(_, m)
            | ParserExp::Lookahead(m)
            | ParserExp::NegativeLookahead(m)
            | ParserExp::Optional(m)
            | ParserExp::Closure(m)
            | ParserExp::PositiveClosure(m)
            | ParserExp::Alt(m)
            | ParserExp::SkipTo(m) => vec![m.as_ref()],

            // Choice: Any option is a potential "next" step
            ParserExp::Choice(models) => models.iter().collect(),

            // Sequence: Collect all leading nullable elements plus the first non-nullable one
            ParserExp::Sequence(models) => {
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
            ParserExp::Join { exp, .. }
            | ParserExp::PositiveJoin { exp, .. }
            | ParserExp::Gather { exp, .. }
            | ParserExp::PositiveGather { exp, .. } => vec![exp.as_ref()],
        }
    }
}
