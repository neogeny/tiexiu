// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::{Exp, ExpKind};

impl Exp {
    pub fn callable_from(&self) -> Vec<&Exp> {
        self.kind.callable_from()
    }

    pub fn callable_from_mut(&mut self) -> Vec<&mut Exp> {
        self.kind.callable_from_mut()
    }

    pub fn is_nullable(&self) -> bool {
        self.kind.is_nullable()
    }
}

impl ExpKind {
    pub fn is_nullable(&self) -> bool {
        match &self {
            Self::Nil => true,
            Self::RuleInclude { name: _, exp } => exp.is_nullable(),

            // Consumes nothing, always succeeds (or affects state only)
            Self::Eof => false,

            Self::Cut
            | Self::Void
            | Self::Lookahead(_)
            | Self::NegativeLookahead(_)
            | Self::Optional(_)
            | Self::Constant(_)
            | Self::Alert(..)
            | Self::Closure(_) => true,

            // Always consumes (or fails), never succeeds with zero width
            Self::Fail | Self::Dot | Self::Token(_) => false,

            Self::Pattern(pattern) => {
                // true if it CAN match the empty string (is nullable)
                regex::Regex::new(pattern)
                    .map(|re| re.is_match(""))
                    .unwrap_or(false)
            }

            // Transparent wrappers
            Self::Group(m)
            | Self::SkipGroup(m)
            | Self::Override(m)
            | Self::Named(_, m)
            | Self::OverrideList(m)
            | Self::NamedList(_, m) => m.is_nullable(),

            // Logic-based variants
            Self::Alt(m) => m.is_nullable(),
            Self::Choice(models) => models.iter().any(|m| m.is_nullable()),
            Self::Sequence(models) => models.iter().all(|m| m.is_nullable()),
            Self::PositiveClosure(m) => m.is_nullable(),

            // Join/Gather variants
            Self::Join { .. } | Self::Gather { .. } => true, // These can match zero times
            Self::PositiveJoin { exp, .. } | Self::PositiveGather { exp, .. } => exp.is_nullable(),

            // Special cases
            Self::SkipTo(_) => false, // SkipTo must find a match to succeed

            Self::Call { .. } => {
                // In a stateless walker, you cannot determine this without
                // looking up the definition of _name in the grammar.
                false
            }
        }
    }
    //noinspection DuplicatedCode
    pub fn callable_from(&self) -> Vec<&Exp> {
        match &self {
            Self::Nil => vec![],
            Self::RuleInclude { name: _, exp } => vec![exp.as_ref()],

            // These don't lead to further rules
            Self::Cut
            | Self::Void
            | Self::Fail
            | Self::Dot
            | Self::Eof
            | Self::Token(_)
            | Self::Pattern(_)
            | Self::Constant(_)
            | Self::Alert(..) => vec![],

            // NOTE: left recursion detection handles this by resolving by name
            Self::Call { .. } => vec![],

            // Transparent wrappers: return the inner expression
            Self::Group(m)
            | Self::SkipGroup(m)
            | Self::Override(m)
            | Self::Named(_, m)
            | Self::OverrideList(m)
            | Self::NamedList(_, m)
            | Self::Lookahead(m)
            | Self::NegativeLookahead(m)
            | Self::Optional(m)
            | Self::Closure(m)
            | Self::PositiveClosure(m)
            | Self::Alt(m)
            | Self::SkipTo(m) => vec![m.as_ref()],

            // Choice: Any option is a potential "next" step
            Self::Choice(models) => models.iter().collect(),

            // Sequence: Collect all leading nullable elements plus the first non-nullable one
            Self::Sequence(models) => {
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
            Self::Join { exp, .. }
            | Self::PositiveJoin { exp, .. }
            | Self::Gather { exp, .. }
            | Self::PositiveGather { exp, .. } => vec![exp.as_ref()],
        }
    }

    //noinspection DuplicatedCode
    pub fn callable_from_mut(&mut self) -> Vec<&mut Exp> {
        match self {
            Self::Nil => vec![],
            Self::RuleInclude { name: _, exp } => vec![exp.as_mut()],

            // These don't lead to further rules
            Self::Cut
            | Self::Void
            | Self::Fail
            | Self::Dot
            | Self::Eof
            | Self::Token(_)
            | Self::Pattern(_)
            | Self::Constant(_)
            | Self::Alert(..) => vec![],

            // NOTE: left recursion detection handles this by resolving by name
            Self::Call { .. } => vec![],

            // Transparent wrappers: return the inner expression
            Self::Group(m)
            | Self::SkipGroup(m)
            | Self::Override(m)
            | Self::Named(_, m)
            | Self::OverrideList(m)
            | Self::NamedList(_, m)
            | Self::Lookahead(m)
            | Self::NegativeLookahead(m)
            | Self::Optional(m)
            | Self::Closure(m)
            | Self::PositiveClosure(m)
            | Self::Alt(m)
            | Self::SkipTo(m) => vec![m.as_mut()],

            // Choice: Any option is a potential "next" step
            Self::Choice(models) => models.iter_mut().collect(),

            // Sequence: Collect all leading nullable elements plus the first non-nullable one
            Self::Sequence(models) => {
                let mut result = Vec::new();
                for m in models {
                    let nullable = m.is_nullable();
                    result.push(m);
                    if !nullable {
                        break;
                    }
                }
                result
            }

            // Join/Gather variants: the expression is always reachable
            Self::Join { exp, .. }
            | Self::PositiveJoin { exp, .. }
            | Self::Gather { exp, .. }
            | Self::PositiveGather { exp, .. } => vec![exp.as_mut()],
        }
    }
}
