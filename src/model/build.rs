// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::elements::Element;

#[inline]
pub fn cut() -> Element {
    Element::Cut
}

#[inline]
pub fn void() -> Element {
    Element::Void
}

#[inline]
pub fn fail() -> Element {
    Element::Fail
}

#[inline]
pub fn dot() -> Element {
    Element::Dot
}

#[inline]
pub fn eof() -> Element {
    Element::Eof
}

pub fn call(name: &str, exp: Element) -> Element {
    Element::Call(name.into(), exp.into())
}

pub fn token(name: &str) -> Element {
    Element::Token(name.into())
}

pub fn constant(value: &str) -> Element {
    Element::Constant(value.into())
}

pub fn alert(msg: &str, code: u8) -> Element {
    Element::Alert(msg.into(), code)
}

pub fn named(name: &str, model: Element) -> Element {
    Element::Named(name.into(), model.into())
}

pub fn named_list(name: &str, model: Element) -> Element {
    Element::NamedList(name.into(), model.into())
}

pub fn override_node(model: Element) -> Element {
    Element::Override(model.into())
}

pub fn override_list(model: Element) -> Element {
    Element::OverrideList(model.into())
}

pub fn group(model: Element) -> Element {
    Element::Group(model.into())
}

pub fn skip_group(model: Element) -> Element {
    Element::SkipGroup(model.into())
}

pub fn lookahead(model: Element) -> Element {
    Element::Lookahead(model.into())
}

pub fn negative_lookahead(model: Element) -> Element {
    Element::NegativeLookahead(model.into())
}

pub fn skip_to(model: Element) -> Element {
    Element::SkipTo(model.into())
}

pub fn sequence(models: Vec<Element>) -> Element {
    Element::Sequence(models.into_boxed_slice())
}

pub fn choice(models: Vec<Element>) -> Element {
    Element::Choice(models.into_boxed_slice())
}

pub fn optional(model: Element) -> Element {
    Element::Optional(model.into())
}

pub fn closure(model: Element) -> Element {
    Element::Closure(model.into())
}

pub fn positive_closure(model: Element) -> Element {
    Element::PositiveClosure(model.into())
}

pub fn join(exp: Element, sep: Element) -> Element {
    Element::Join {
        exp: exp.into(),
        sep: sep.into(),
    }
}

pub fn positive_join(exp: Element, sep: Element) -> Element {
    Element::PositiveJoin {
        exp: exp.into(),
        sep: sep.into(),
    }
}

pub fn gather(exp: Element, sep: Element) -> Element {
    Element::Gather {
        exp: exp.into(),
        sep: sep.into(),
    }
}

pub fn positive_gather(exp: Element, sep: Element) -> Element {
    Element::PositiveGather {
        exp: exp.into(),
        sep: sep.into(),
    }
}

impl Element {
    #[inline]
    pub fn cut() -> Self {
        Element::Cut
    }

    #[inline]
    pub fn void() -> Self {
        Element::Void
    }

    #[inline]
    pub fn fail() -> Self {
        Element::Fail
    }

    #[inline]
    pub fn dot() -> Self {
        Element::Dot
    }

    #[inline]
    pub fn eof() -> Self {
        Element::Eof
    }

    pub fn call(name: &str, exp: Element) -> Self {
        Element::Call(name.into(), exp.into())
    }

    pub fn token(name: &str) -> Self {
        Element::Token(name.into())
    }

    pub fn constant(value: &str) -> Self {
        Element::Constant(value.into())
    }

    pub fn alert(msg: &str, code: u8) -> Self {
        Element::Alert(msg.into(), code)
    }

    pub fn named(name: &str, model: Element) -> Self {
        Element::Named(name.into(), model.into())
    }

    pub fn named_list(name: &str, model: Element) -> Self {
        Element::NamedList(name.into(), model.into())
    }

    pub fn override_node(model: Element) -> Self {
        Element::Override(model.into())
    }

    pub fn override_list(model: Element) -> Self {
        Element::OverrideList(model.into())
    }

    pub fn group(model: Element) -> Self {
        Element::Group(model.into())
    }

    pub fn skip_group(model: Element) -> Self {
        Element::SkipGroup(model.into())
    }

    pub fn lookahead(model: Element) -> Self {
        Element::Lookahead(model.into())
    }

    pub fn negative_lookahead(model: Element) -> Self {
        Element::NegativeLookahead(model.into())
    }

    pub fn skip_to(model: Element) -> Self {
        Element::SkipTo(model.into())
    }

    pub fn sequence(models: Vec<Element>) -> Self {
        Element::Sequence(models.into_boxed_slice())
    }

    pub fn choice(models: Vec<Element>) -> Self {
        Element::Choice(models.into_boxed_slice())
    }

    pub fn optional(model: Element) -> Self {
        Element::Optional(model.into())
    }

    pub fn closure(model: Element) -> Self {
        Element::Closure(model.into())
    }

    pub fn positive_closure(model: Element) -> Self {
        Element::PositiveClosure(model.into())
    }

    pub fn join(exp: Element, sep: Element) -> Self {
        Element::Join {
            exp: exp.into(),
            sep: sep.into(),
        }
    }

    pub fn positive_join(exp: Element, sep: Element) -> Self {
        Element::PositiveJoin {
            exp: exp.into(),
            sep: sep.into(),
        }
    }

    pub fn gather(exp: Element, sep: Element) -> Self {
        Element::Gather {
            exp: exp.into(),
            sep: sep.into(),
        }
    }

    pub fn positive_gather(exp: Element, sep: Element) -> Self {
        Element::PositiveGather {
            exp: exp.into(),
            sep: sep.into(),
        }
    }
}
