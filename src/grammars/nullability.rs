// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::Model;

impl Model {
    pub fn is_nullable(&self) -> bool {
        match self {
            // Consumes nothing, always succeeds (or affects state only)
            Model::Cut
            | Model::Void
            | Model::Eof
            | Model::Lookahead(_)
            | Model::NegativeLookahead(_)
            | Model::Optional(_)
            | Model::Constant(_)
            | Model::Alert(..)
            | Model::Closure(_) => true,

            // Always consumes (or fails), never succeeds with zero width
            Model::Fail | Model::Dot | Model::Token(_) => false,

            // Transparent wrappers
            Model::Group(m)
            | Model::SkipGroup(m)
            | Model::Override(m)
            | Model::Named(_, m)
            | Model::OverrideList(m)
            | Model::NamedList(_, m) => m.is_nullable(),

            // Logic-based variants
            Model::Choice(models) => models.iter().any(|m| m.is_nullable()),
            Model::Sequence(models) => models.iter().all(|m| m.is_nullable()),
            Model::PositiveClosure(m) => m.is_nullable(),

            // Join/Gather variants
            Model::Join { .. } | Model::Gather { .. } => true, // These can match zero times
            Model::PositiveJoin { exp, .. } | Model::PositiveGather { exp, .. } => {
                exp.is_nullable()
            }

            // Special cases
            Model::SkipTo(_) => false, // SkipTo must find a match to succeed

            Model::Call(_name) => {
                // In a stateless walker, you cannot determine this without
                // looking up the definition of _name in the grammar.
                false
            }
        }
    }

    pub fn callable_from(&self) -> Vec<&Model> {
        match self {
            // These don't lead to further rules
            Model::Cut
            | Model::Void
            | Model::Fail
            | Model::Dot
            | Model::Eof
            | Model::Token(_)
            | Model::Constant(_)
            | Model::Alert(..)
            | Model::Call(_) => vec![],

            // Transparent wrappers: return the inner expression
            Model::Group(m)
            | Model::SkipGroup(m)
            | Model::Override(m)
            | Model::Named(_, m)
            | Model::OverrideList(m)
            | Model::NamedList(_, m)
            | Model::Lookahead(m)
            | Model::NegativeLookahead(m)
            | Model::Optional(m)
            | Model::Closure(m)
            | Model::PositiveClosure(m)
            | Model::SkipTo(m) => vec![m.as_ref()],

            // Choice: Any option is a potential "next" step
            Model::Choice(models) => models.iter().collect(),

            // Sequence: Collect all leading nullable elements plus the first non-nullable one
            Model::Sequence(models) => {
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
            Model::Join { exp, .. }
            | Model::PositiveJoin { exp, .. }
            | Model::Gather { exp, .. }
            | Model::PositiveGather { exp, .. } => vec![exp.as_ref()],
        }
    }
}
