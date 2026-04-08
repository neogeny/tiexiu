// copyright (c) 2026 juancarlo añez (apalala@gmail.com)
// spdx-license-identifier: mit or apache-2.0

use super::exp::{Exp, ExpKind};

impl Exp {
    #[inline]
    pub fn new(exp: ExpKind) -> Self {
        Self {
            kind: exp,
            lookahead: [].into(),
        }
    }

    #[inline]
    pub fn alt(exp: Self) -> Self {
        Self::new(ExpKind::Alt(exp.into()))
    }

    #[inline]
    pub fn pattern(pattern: &str) -> Self {
        Self::new(ExpKind::Pattern(pattern.into()))
    }

    #[inline]
    pub fn nil() -> Self {
        Self::new(ExpKind::Nil)
    }

    pub fn rule_include(name: &str) -> Self {
        Self::new(ExpKind::RuleInclude {
            name: name.into(),
            exp: None,
        })
    }

    #[inline]
    pub fn cut() -> Self {
        Self::new(ExpKind::Cut)
    }

    #[inline]
    pub fn void() -> Self {
        Self::new(ExpKind::Void)
    }

    #[inline]
    pub fn fail() -> Self {
        Self::new(ExpKind::Fail)
    }

    #[inline]
    pub fn dot() -> Self {
        Self::new(ExpKind::Dot)
    }

    #[inline]
    pub fn eof() -> Self {
        Self::new(ExpKind::Eof)
    }

    #[inline]
    pub fn call(name: &str) -> Self {
        Self::new(ExpKind::Call {
            name: name.into(),
            rule: None,
        })
    }

    #[inline]
    pub fn token(name: &str) -> Self {
        Self::new(ExpKind::Token(name.into()))
    }

    #[inline]
    pub fn constant(value: &str) -> Self {
        Self::new(ExpKind::Constant(value.into()))
    }

    #[inline]
    pub fn alert(msg: &str, code: u8) -> Self {
        Self::new(ExpKind::Alert(msg.into(), code))
    }

    #[inline]
    pub fn named(name: &str, model: Self) -> Self {
        Self::new(ExpKind::Named(name.into(), model.into()))
    }

    #[inline]
    pub fn named_list(name: &str, model: Self) -> Self {
        Self::new(ExpKind::NamedList(name.into(), model.into()))
    }

    #[inline]
    pub fn override_node(model: Self) -> Self {
        Self::new(ExpKind::Override(model.into()))
    }

    #[inline]
    pub fn override_list(model: Self) -> Self {
        Self::new(ExpKind::OverrideList(model.into()))
    }

    #[inline]
    pub fn group(model: Self) -> Self {
        Self::new(ExpKind::Group(model.into()))
    }

    #[inline]
    pub fn skip_group(model: Self) -> Self {
        Self::new(ExpKind::SkipGroup(model.into()))
    }

    #[inline]
    pub fn lookahead(model: Self) -> Self {
        Self::new(ExpKind::Lookahead(model.into()))
    }

    #[inline]
    pub fn negative_lookahead(model: Self) -> Self {
        Self::new(ExpKind::NegativeLookahead(model.into()))
    }

    #[inline]
    pub fn skip_to(model: Self) -> Self {
        Self::new(ExpKind::SkipTo(model.into()))
    }

    #[inline]
    pub fn sequence(models: Vec<Self>) -> Self {
        Self::new(ExpKind::Sequence(models.into_boxed_slice()))
    }

    #[inline]
    pub fn choice(models: Vec<Self>) -> Self {
        Self::new(ExpKind::Choice(models.into_boxed_slice()))
    }

    #[inline]
    pub fn optional(model: Self) -> Self {
        Self::new(ExpKind::Optional(model.into()))
    }

    #[inline]
    pub fn closure(model: Self) -> Self {
        Self::new(ExpKind::Closure(model.into()))
    }

    #[inline]
    pub fn positive_closure(model: Self) -> Self {
        Self::new(ExpKind::PositiveClosure(model.into()))
    }

    #[inline]
    pub fn join(exp: Self, sep: Self) -> Self {
        Self::new(ExpKind::Join {
            exp: exp.into(),
            sep: sep.into(),
        })
    }

    #[inline]
    pub fn positive_join(exp: Self, sep: Self) -> Self {
        Self::new(ExpKind::PositiveJoin {
            exp: exp.into(),
            sep: sep.into(),
        })
    }

    #[inline]
    pub fn gather(exp: Self, sep: Self) -> Self {
        Self::new(ExpKind::Gather {
            exp: exp.into(),
            sep: sep.into(),
        })
    }

    #[inline]
    pub fn positive_gather(exp: Self, sep: Self) -> Self {
        Self::new(ExpKind::PositiveGather {
            exp: exp.into(),
            sep: sep.into(),
        })
    }
}
