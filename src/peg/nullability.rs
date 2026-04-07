// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::{Exp, ExpKind};

impl Exp {
    pub fn is_nullable(&self) -> bool {
        self.kind.is_nullable()
    }

    pub fn callable_from(&self) -> Vec<&Exp> {
        self.kind.callable_from()
    }
}

impl ExpKind {
    pub fn is_nullable(&self) -> bool {
        match self {
            ExpKind::Nil => true,
            ExpKind::RuleInclude { name: _, exp } => exp.is_nullable(),

            // Consumes nothing, always succeeds (or affects state only)
            ExpKind::Eof => false,

            ExpKind::Cut
            | ExpKind::Void
            | ExpKind::Lookahead(_)
            | ExpKind::NegativeLookahead(_)
            | ExpKind::Optional(_)
            | ExpKind::Constant(_)
            | ExpKind::Alert(..)
            | ExpKind::Closure(_) => true,

            // Always consumes (or fails), never succeeds with zero width
            ExpKind::Fail | ExpKind::Dot | ExpKind::Token(_) => false,

            ExpKind::Pattern(pattern) => {
                // true if it CAN match the empty string (is nullable)
                regex::Regex::new(pattern)
                    .map(|re| re.is_match(""))
                    .unwrap_or(false)
            }

            // Transparent wrappers
            ExpKind::Group(m)
            | ExpKind::SkipGroup(m)
            | ExpKind::Override(m)
            | ExpKind::Named(_, m)
            | ExpKind::OverrideList(m)
            | ExpKind::NamedList(_, m) => m.is_nullable(),

            // Logic-based variants
            ExpKind::Alt(m) => m.is_nullable(),
            ExpKind::Choice(models) => models.iter().any(|m| m.is_nullable()),
            ExpKind::Sequence(models) => models.iter().all(|m| m.is_nullable()),
            ExpKind::PositiveClosure(m) => m.is_nullable(),

            // Join/Gather variants
            ExpKind::Join { .. } | ExpKind::Gather { .. } => true, // These can match zero times
            ExpKind::PositiveJoin { exp, .. } | ExpKind::PositiveGather { exp, .. } => {
                exp.is_nullable()
            }

            // Special cases
            ExpKind::SkipTo(_) => false, // SkipTo must find a match to succeed

            ExpKind::Call(_name, _exp) => {
                // In a stateless walker, you cannot determine this without
                // looking up the definition of _name in the grammar.
                false
            }
        }
    }

    //noinspection DuplicatedCode
    pub fn callable_from(&self) -> Vec<&Exp> {
        match self {
            ExpKind::Nil => vec![],
            ExpKind::RuleInclude { name: _, exp } => vec![exp.as_ref()],

            // These don't lead to further rules
            ExpKind::Cut
            | ExpKind::Void
            | ExpKind::Fail
            | ExpKind::Dot
            | ExpKind::Eof
            | ExpKind::Token(_)
            | ExpKind::Pattern(_)
            | ExpKind::Constant(_)
            | ExpKind::Alert(..) => vec![],

            // NOTE: left recursion detection handles this by resolving by name
            ExpKind::Call(_, _) => vec![],

            // Transparent wrappers: return the inner expression
            ExpKind::Group(m)
            | ExpKind::SkipGroup(m)
            | ExpKind::Override(m)
            | ExpKind::Named(_, m)
            | ExpKind::OverrideList(m)
            | ExpKind::NamedList(_, m)
            | ExpKind::Lookahead(m)
            | ExpKind::NegativeLookahead(m)
            | ExpKind::Optional(m)
            | ExpKind::Closure(m)
            | ExpKind::PositiveClosure(m)
            | ExpKind::Alt(m)
            | ExpKind::SkipTo(m) => vec![m.as_ref()],

            // Choice: Any option is a potential "next" step
            ExpKind::Choice(models) => models.iter().collect(),

            // Sequence: Collect all leading nullable elements plus the first non-nullable one
            ExpKind::Sequence(models) => {
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
            ExpKind::Join { exp, .. }
            | ExpKind::PositiveJoin { exp, .. }
            | ExpKind::Gather { exp, .. }
            | ExpKind::PositiveGather { exp, .. } => vec![exp.as_ref()],
        }
    }

    //noinspection DuplicatedCode
    pub fn callable_from_mut(&mut self) -> Vec<&mut Exp> {
        match self {
            ExpKind::Nil => vec![],
            ExpKind::RuleInclude { name: _, exp } => vec![exp.as_mut()],

            // These don't lead to further rules
            ExpKind::Cut
            | ExpKind::Void
            | ExpKind::Fail
            | ExpKind::Dot
            | ExpKind::Eof
            | ExpKind::Token(_)
            | ExpKind::Pattern(_)
            | ExpKind::Constant(_)
            | ExpKind::Alert(..) => vec![],

            // NOTE: left recursion detection handles this by resolving by name
            ExpKind::Call(_, _) => vec![],

            // Transparent wrappers: return the inner expression
            ExpKind::Group(m)
            | ExpKind::SkipGroup(m)
            | ExpKind::Override(m)
            | ExpKind::Named(_, m)
            | ExpKind::OverrideList(m)
            | ExpKind::NamedList(_, m)
            | ExpKind::Lookahead(m)
            | ExpKind::NegativeLookahead(m)
            | ExpKind::Optional(m)
            | ExpKind::Closure(m)
            | ExpKind::PositiveClosure(m)
            | ExpKind::Alt(m)
            | ExpKind::SkipTo(m) => vec![m.as_mut()],

            // Choice: Any option is a potential "next" step
            ExpKind::Choice(models) => models.iter_mut().collect(),

            // Sequence: Collect all leading nullable elements plus the first non-nullable one
            ExpKind::Sequence(models) => {
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
            ExpKind::Join { exp, .. }
            | ExpKind::PositiveJoin { exp, .. }
            | ExpKind::Gather { exp, .. }
            | ExpKind::PositiveGather { exp, .. } => vec![exp.as_mut()],
        }
    }
}
