// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::{Exp, ExpKind};
use crate::util::pyre;

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
            Self::RuleInclude { exp, .. } => exp.as_ref().is_some_and(|e| e.is_nullable()),

            // Consumes nothing, always succeeds (or affects state only)
            Self::Eof => false,
            Self::Eol => true,

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
                pyre::compile(pattern)
                    .ok()
                    .and_then(|re| re.match_(""))
                    .is_some()
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
            Self::RuleInclude { exp, .. } => match exp {
                Some(e) => vec![e],
                _ => vec![],
            },

            // These don't lead to further rules
            Self::Cut
            | Self::Void
            | Self::Fail
            | Self::Dot
            | Self::Eof
            | Self::Eol
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
            Self::RuleInclude { .. } => vec![],

            // These don't lead to further rules
            Self::Cut
            | Self::Void
            | Self::Fail
            | Self::Dot
            | Self::Eof
            | Self::Eol
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nil_nullable() {
        let exp = Exp::nil();
        assert!(exp.is_nullable());
    }

    #[test]
    fn token_not_nullable() {
        let exp = Exp::token("a");
        assert!(!exp.is_nullable());
    }

    #[test]
    fn fail_not_nullable() {
        let exp = Exp::fail();
        assert!(!exp.is_nullable());
    }

    #[test]
    fn dot_not_nullable() {
        let exp = Exp::dot();
        assert!(!exp.is_nullable());
    }

    #[test]
    fn optional_nullable() {
        let exp = Exp::optional(Exp::token("a"));
        assert!(exp.is_nullable());
    }

    #[test]
    fn closure_nullable() {
        let exp = Exp::closure(Exp::token("a"));
        assert!(exp.is_nullable());
    }

    #[test]
    fn positive_closure_not_nullable() {
        let exp = Exp::positive_closure(Exp::token("a"));
        assert!(!exp.is_nullable());
    }

    #[test]
    fn pattern_nullable() {
        let exp = Exp::pattern(r"a*");
        assert!(exp.is_nullable());
    }

    #[test]
    fn pattern_not_nullable() {
        let exp = Exp::pattern(r"a+");
        assert!(!exp.is_nullable());
    }

    #[test]
    fn sequence_all_nullable() {
        let exp = Exp::sequence(vec![Exp::nil(), Exp::optional(Exp::token("a"))]);
        assert!(exp.is_nullable());
    }

    #[test]
    fn sequence_not_nullable() {
        let exp = Exp::sequence(vec![Exp::token("a"), Exp::token("b")]);
        assert!(!exp.is_nullable());
    }

    #[test]
    fn choice_nullable() {
        let exp = Exp::choice(vec![Exp::nil(), Exp::token("a")]);
        assert!(exp.is_nullable());
    }

    #[test]
    fn choice_not_nullable() {
        let exp = Exp::choice(vec![Exp::token("a"), Exp::token("b")]);
        assert!(!exp.is_nullable());
    }

    #[test]
    fn lookahead_nullable() {
        let exp = Exp::lookahead(Exp::token("a"));
        assert!(exp.is_nullable());
    }

    #[test]
    fn negative_lookahead_nullable() {
        let exp = Exp::negative_lookahead(Exp::token("a"));
        assert!(exp.is_nullable());
    }

    #[test]
    fn cut_nullable() {
        let exp = Exp::cut();
        assert!(exp.is_nullable());
    }

    #[test]
    fn void_nullable() {
        let exp = Exp::void();
        assert!(exp.is_nullable());
    }

    #[test]
    fn constant_nullable() {
        let exp = Exp::constant("test");
        assert!(exp.is_nullable());
    }

    #[test]
    fn eof_not_nullable() {
        let exp = Exp::eof();
        assert!(!exp.is_nullable());
    }

    #[test]
    fn join_nullable() {
        let exp = Exp::join(Exp::token(","), Exp::token("a"));
        assert!(exp.is_nullable());
    }

    #[test]
    fn positive_join_not_nullable() {
        let exp = Exp::positive_join(Exp::token(","), Exp::token("a"));
        assert!(!exp.is_nullable());
    }
}
