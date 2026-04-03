// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::elements::E;

impl E {
    pub fn is_nullable(&self) -> bool {
        match self {
            // Consumes nothing, always succeeds (or affects state only)
            E::Cut
            | E::Void
            | E::Eof
            | E::Lookahead(_)
            | E::NegativeLookahead(_)
            | E::Optional(_)
            | E::Constant(_)
            | E::Alert(..)
            | E::Closure(_) => true,

            // Always consumes (or fails), never succeeds with zero width
            E::Fail | E::Dot | E::Token(_) => false,

            // Transparent wrappers
            E::Group(m)
            | E::SkipGroup(m)
            | E::Override(m)
            | E::Named(_, m)
            | E::OverrideList(m)
            | E::NamedList(_, m) => m.is_nullable(),

            // Logic-based variants
            E::Choice(models) => models.iter().any(|m| m.is_nullable()),
            E::Sequence(models) => models.iter().all(|m| m.is_nullable()),
            E::PositiveClosure(m) => m.is_nullable(),

            // Join/Gather variants
            E::Join { .. } | E::Gather { .. } => true, // These can match zero times
            E::PositiveJoin { exp, .. } | E::PositiveGather { exp, .. } => exp.is_nullable(),

            // Special cases
            E::SkipTo(_) => false, // SkipTo must find a match to succeed

            E::Call(_name) => {
                // In a stateless walker, you cannot determine this without
                // looking up the definition of _name in the grammar.
                false
            }
        }
    }

    pub fn callable_from(&self) -> Vec<&E> {
        match self {
            // These don't lead to further rules
            E::Cut
            | E::Void
            | E::Fail
            | E::Dot
            | E::Eof
            | E::Token(_)
            | E::Constant(_)
            | E::Alert(..)
            | E::Call(_) => vec![],

            // Transparent wrappers: return the inner expression
            E::Group(m)
            | E::SkipGroup(m)
            | E::Override(m)
            | E::Named(_, m)
            | E::OverrideList(m)
            | E::NamedList(_, m)
            | E::Lookahead(m)
            | E::NegativeLookahead(m)
            | E::Optional(m)
            | E::Closure(m)
            | E::PositiveClosure(m)
            | E::SkipTo(m) => vec![m.as_ref()],

            // Choice: Any option is a potential "next" step
            E::Choice(models) => models.iter().collect(),

            // Sequence: Collect all leading nullable elements plus the first non-nullable one
            E::Sequence(models) => {
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
            E::Join { exp, .. }
            | E::PositiveJoin { exp, .. }
            | E::Gather { exp, .. }
            | E::PositiveGather { exp, .. } => vec![exp.as_ref()],
        }
    }
}
