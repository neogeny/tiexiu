// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::elements::E;

#[inline]
pub fn cut() -> E {
    E::Cut
}

#[inline]
pub fn void() -> E {
    E::Void
}

#[inline]
pub fn fail() -> E {
    E::Fail
}

#[inline]
pub fn dot() -> E {
    E::Dot
}

#[inline]
pub fn eof() -> E {
    E::Eof
}

pub fn call(name: &str) -> E {
    E::Call(name.into())
}

pub fn token(name: &str) -> E {
    E::Token(name.into())
}

pub fn constant(value: &str) -> E {
    E::Constant(value.into())
}

pub fn alert(msg: &str, code: u8) -> E {
    E::Alert(msg.into(), code)
}

pub fn named(name: &str, model: E) -> E {
    E::Named(name.into(), model.into())
}

pub fn named_list(name: &str, model: E) -> E {
    E::NamedList(name.into(), model.into())
}

pub fn override_node(model: E) -> E {
    E::Override(model.into())
}

pub fn override_list(model: E) -> E {
    E::OverrideList(model.into())
}

pub fn group(model: E) -> E {
    E::Group(model.into())
}

pub fn skip_group(model: E) -> E {
    E::SkipGroup(model.into())
}

pub fn lookahead(model: E) -> E {
    E::Lookahead(model.into())
}

pub fn negative_lookahead(model: E) -> E {
    E::NegativeLookahead(model.into())
}

pub fn skip_to(model: E) -> E {
    E::SkipTo(model.into())
}

pub fn sequence(models: Vec<E>) -> E {
    E::Sequence(models.into_boxed_slice())
}

pub fn choice(models: Vec<E>) -> E {
    E::Choice(models.into_boxed_slice())
}

pub fn optional(model: E) -> E {
    E::Optional(model.into())
}

pub fn closure(model: E) -> E {
    E::Closure(model.into())
}

pub fn positive_closure(model: E) -> E {
    E::PositiveClosure(model.into())
}

pub fn join(exp: E, sep: E) -> E {
    E::Join {
        exp: exp.into(),
        sep: sep.into(),
    }
}

pub fn positive_join(exp: E, sep: E) -> E {
    E::PositiveJoin {
        exp: exp.into(),
        sep: sep.into(),
    }
}

pub fn gather(exp: E, sep: E) -> E {
    E::Gather {
        exp: exp.into(),
        sep: sep.into(),
    }
}

pub fn positive_gather(exp: E, sep: E) -> E {
    E::PositiveGather {
        exp: exp.into(),
        sep: sep.into(),
    }
}

impl E {
    #[inline]
    pub fn cut() -> Self {
        E::Cut
    }

    #[inline]
    pub fn void() -> Self {
        E::Void
    }

    #[inline]
    pub fn fail() -> Self {
        E::Fail
    }

    #[inline]
    pub fn dot() -> Self {
        E::Dot
    }

    #[inline]
    pub fn eof() -> Self {
        E::Eof
    }

    pub fn call(name: &str) -> Self {
        E::Call(name.into())
    }

    pub fn token(name: &str) -> Self {
        E::Token(name.into())
    }

    pub fn constant(value: &str) -> Self {
        E::Constant(value.into())
    }

    pub fn alert(msg: &str, code: u8) -> Self {
        E::Alert(msg.into(), code)
    }

    pub fn named(name: &str, model: E) -> Self {
        E::Named(name.into(), model.into())
    }

    pub fn named_list(name: &str, model: E) -> Self {
        E::NamedList(name.into(), model.into())
    }

    pub fn override_node(model: E) -> Self {
        E::Override(model.into())
    }

    pub fn override_list(model: E) -> Self {
        E::OverrideList(model.into())
    }

    pub fn group(model: E) -> Self {
        E::Group(model.into())
    }

    pub fn skip_group(model: E) -> Self {
        E::SkipGroup(model.into())
    }

    pub fn lookahead(model: E) -> Self {
        E::Lookahead(model.into())
    }

    pub fn negative_lookahead(model: E) -> Self {
        E::NegativeLookahead(model.into())
    }

    pub fn skip_to(model: E) -> Self {
        E::SkipTo(model.into())
    }

    pub fn sequence(models: Vec<E>) -> Self {
        E::Sequence(models.into_boxed_slice())
    }

    pub fn choice(models: Vec<E>) -> Self {
        E::Choice(models.into_boxed_slice())
    }

    pub fn optional(model: E) -> Self {
        E::Optional(model.into())
    }

    pub fn closure(model: E) -> Self {
        E::Closure(model.into())
    }

    pub fn positive_closure(model: E) -> Self {
        E::PositiveClosure(model.into())
    }

    pub fn join(exp: E, sep: E) -> Self {
        E::Join {
            exp: exp.into(),
            sep: sep.into(),
        }
    }

    pub fn positive_join(exp: E, sep: E) -> Self {
        E::PositiveJoin {
            exp: exp.into(),
            sep: sep.into(),
        }
    }

    pub fn gather(exp: E, sep: E) -> Self {
        E::Gather {
            exp: exp.into(),
            sep: sep.into(),
        }
    }

    pub fn positive_gather(exp: E, sep: E) -> Self {
        E::PositiveGather {
            exp: exp.into(),
            sep: sep.into(),
        }
    }
}
