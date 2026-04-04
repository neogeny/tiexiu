// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::elements::Element;

impl Element {
    pub fn is_nullable(&self) -> bool {
        match self {
            // Consumes nothing, always succeeds (or affects state only)
            Element::Cut
            | Element::Void
            | Element::Eof
            | Element::Lookahead(_)
            | Element::NegativeLookahead(_)
            | Element::Optional(_)
            | Element::Constant(_)
            | Element::Alert(..)
            | Element::Closure(_) => true,

            // Always consumes (or fails), never succeeds with zero width
            Element::Fail | Element::Dot | Element::Token(_) => false,

            Element::Pattern(pattern) => {
                // true if it CAN match the empty string (is nullable)
                regex::Regex::new(pattern)
                    .map(|re| re.is_match(""))
                    .unwrap_or(false)
            }

            // Transparent wrappers
            Element::Group(m)
            | Element::SkipGroup(m)
            | Element::Override(m)
            | Element::Named(_, m)
            | Element::OverrideList(m)
            | Element::NamedList(_, m) => m.is_nullable(),

            // Logic-based variants
            Element::Choice(models) => models.iter().any(|m| m.is_nullable()),
            Element::Sequence(models) => models.iter().all(|m| m.is_nullable()),
            Element::PositiveClosure(m) => m.is_nullable(),

            // Join/Gather variants
            Element::Join { .. } | Element::Gather { .. } => true, // These can match zero times
            Element::PositiveJoin { exp, .. } | Element::PositiveGather { exp, .. } => {
                exp.is_nullable()
            }

            // Special cases
            Element::SkipTo(_) => false, // SkipTo must find a match to succeed

            Element::Call(_name) => {
                // In a stateless walker, you cannot determine this without
                // looking up the definition of _name in the grammar.
                false
            }
        }
    }

    pub fn callable_from(&self) -> Vec<&Element> {
        match self {
            // These don't lead to further rules
            Element::Cut
            | Element::Void
            | Element::Fail
            | Element::Dot
            | Element::Eof
            | Element::Token(_)
            | Element::Pattern(_)
            | Element::Constant(_)
            | Element::Alert(..)
            | Element::Call(_) => vec![],

            // Transparent wrappers: return the inner expression
            Element::Group(m)
            | Element::SkipGroup(m)
            | Element::Override(m)
            | Element::Named(_, m)
            | Element::OverrideList(m)
            | Element::NamedList(_, m)
            | Element::Lookahead(m)
            | Element::NegativeLookahead(m)
            | Element::Optional(m)
            | Element::Closure(m)
            | Element::PositiveClosure(m)
            | Element::SkipTo(m) => vec![m.as_ref()],

            // Choice: Any option is a potential "next" step
            Element::Choice(models) => models.iter().collect(),

            // Sequence: Collect all leading nullable elements plus the first non-nullable one
            Element::Sequence(models) => {
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
            Element::Join { exp, .. }
            | Element::PositiveJoin { exp, .. }
            | Element::Gather { exp, .. }
            | Element::PositiveGather { exp, .. } => vec![exp.as_ref()],
        }
    }
}
