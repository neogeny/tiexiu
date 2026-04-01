// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::Model;

#[inline]
pub fn cut() -> Model {
    Model::Cut
}

#[inline]
pub fn void() -> Model {
    Model::Void
}

#[inline]
pub fn fail() -> Model {
    Model::Fail
}

#[inline]
pub fn dot() -> Model {
    Model::Dot
}

#[inline]
pub fn eof() -> Model {
    Model::Eof
}

pub fn call(name: &str) -> Model {
    Model::Call(name.into())
}

pub fn token(name: &str) -> Model {
    Model::Token(name.into())
}

pub fn constant(value: &str) -> Model {
    Model::Constant(value.into())
}

pub fn alert(msg: &str, code: u8) -> Model {
    Model::Alert(msg.into(), code)
}

pub fn named(name: &str, model: Model) -> Model {
    Model::Named(name.into(), model.into())
}

pub fn named_list(name: &str, model: Model) -> Model {
    Model::NamedList(name.into(), model.into())
}

pub fn override_node(model: Model) -> Model {
    Model::Override(model.into())
}

pub fn override_list(model: Model) -> Model {
    Model::OverrideList(model.into())
}

pub fn group(model: Model) -> Model {
    Model::Group(model.into())
}

pub fn skip_group(model: Model) -> Model {
    Model::SkipGroup(model.into())
}

pub fn lookahead(model: Model) -> Model {
    Model::Lookahead(model.into())
}

pub fn negative_lookahead(model: Model) -> Model {
    Model::NegativeLookahead(model.into())
}

pub fn skip_to(model: Model) -> Model {
    Model::SkipTo(model.into())
}

pub fn sequence(models: Vec<Model>) -> Model {
    Model::Sequence(models.into_boxed_slice())
}

pub fn choice(models: Vec<Model>) -> Model {
    Model::Choice(models.into_boxed_slice())
}

pub fn optional(model: Model) -> Model {
    Model::Optional(model.into())
}

pub fn closure(model: Model) -> Model {
    Model::Closure(model.into())
}

pub fn positive_closure(model: Model) -> Model {
    Model::PositiveClosure(model.into())
}

pub fn join(exp: Model, sep: Model) -> Model {
    Model::Join {
        exp: exp.into(),
        sep: sep.into(),
    }
}

pub fn positive_join(exp: Model, sep: Model) -> Model {
    Model::PositiveJoin {
        exp: exp.into(),
        sep: sep.into(),
    }
}

pub fn gather(exp: Model, sep: Model) -> Model {
    Model::Gather {
        exp: exp.into(),
        sep: sep.into(),
    }
}

pub fn positive_gather(exp: Model, sep: Model) -> Model {
    Model::PositiveGather {
        exp: exp.into(),
        sep: sep.into(),
    }
}

impl Model {
    #[inline]
    pub fn cut() -> Self {
        Model::Cut
    }

    #[inline]
    pub fn void() -> Self {
        Model::Void
    }

    #[inline]
    pub fn fail() -> Self {
        Model::Fail
    }

    #[inline]
    pub fn dot() -> Self {
        Model::Dot
    }

    #[inline]
    pub fn eof() -> Self {
        Model::Eof
    }

    pub fn call(name: &str) -> Self {
        Model::Call(name.into())
    }

    pub fn token(name: &str) -> Self {
        Model::Token(name.into())
    }

    pub fn constant(value: &str) -> Self {
        Model::Constant(value.into())
    }

    pub fn alert(msg: &str, code: u8) -> Self {
        Model::Alert(msg.into(), code)
    }

    pub fn named(name: &str, model: Model) -> Self {
        Model::Named(name.into(), model.into())
    }

    pub fn named_list(name: &str, model: Model) -> Self {
        Model::NamedList(name.into(), model.into())
    }

    pub fn override_node(model: Model) -> Self {
        Model::Override(model.into())
    }

    pub fn override_list(model: Model) -> Self {
        Model::OverrideList(model.into())
    }

    pub fn group(model: Model) -> Self {
        Model::Group(model.into())
    }

    pub fn skip_group(model: Model) -> Self {
        Model::SkipGroup(model.into())
    }

    pub fn lookahead(model: Model) -> Self {
        Model::Lookahead(model.into())
    }

    pub fn negative_lookahead(model: Model) -> Self {
        Model::NegativeLookahead(model.into())
    }

    pub fn skip_to(model: Model) -> Self {
        Model::SkipTo(model.into())
    }

    pub fn sequence(models: Vec<Model>) -> Self {
        Model::Sequence(models.into_boxed_slice())
    }

    pub fn choice(models: Vec<Model>) -> Self {
        Model::Choice(models.into_boxed_slice())
    }

    pub fn optional(model: Model) -> Self {
        Model::Optional(model.into())
    }

    pub fn closure(model: Model) -> Self {
        Model::Closure(model.into())
    }

    pub fn positive_closure(model: Model) -> Self {
        Model::PositiveClosure(model.into())
    }

    pub fn join(exp: Model, sep: Model) -> Self {
        Model::Join {
            exp: exp.into(),
            sep: sep.into(),
        }
    }

    pub fn positive_join(exp: Model, sep: Model) -> Self {
        Model::PositiveJoin {
            exp: exp.into(),
            sep: sep.into(),
        }
    }

    pub fn gather(exp: Model, sep: Model) -> Self {
        Model::Gather {
            exp: exp.into(),
            sep: sep.into(),
        }
    }

    pub fn positive_gather(exp: Model, sep: Model) -> Self {
        Model::PositiveGather {
            exp: exp.into(),
            sep: sep.into(),
        }
    }
}
