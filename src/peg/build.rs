// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::{Exp, ParserExp};

#[inline]
pub fn cut() -> Exp {
    Exp {
        exp: ParserExp::Cut,
    }
}

#[inline]
pub fn void() -> Exp {
    Exp {
        exp: ParserExp::Void,
    }
}

#[inline]
pub fn fail() -> Exp {
    Exp {
        exp: ParserExp::Fail,
    }
}

#[inline]
pub fn dot() -> Exp {
    Exp {
        exp: ParserExp::Dot,
    }
}

#[inline]
pub fn eof() -> Exp {
    Exp {
        exp: ParserExp::Eof,
    }
}

pub fn call(name: &str, exp: Exp) -> Exp {
    Exp {
        exp: ParserExp::Call(name.into(), exp.into()),
    }
}

pub fn token(name: &str) -> Exp {
    Exp {
        exp: ParserExp::Token(name.into()),
    }
}

pub fn constant(value: &str) -> Exp {
    Exp {
        exp: ParserExp::Constant(value.into()),
    }
}

pub fn alert(msg: &str, code: u8) -> Exp {
    Exp {
        exp: ParserExp::Alert(msg.into(), code),
    }
}

pub fn named(name: &str, model: Exp) -> Exp {
    Exp {
        exp: ParserExp::Named(name.into(), model.into()),
    }
}

pub fn named_list(name: &str, model: Exp) -> Exp {
    Exp {
        exp: ParserExp::NamedList(name.into(), model.into()),
    }
}

pub fn override_node(model: Exp) -> Exp {
    Exp {
        exp: ParserExp::Override(model.into()),
    }
}

pub fn override_list(model: Exp) -> Exp {
    Exp {
        exp: ParserExp::OverrideList(model.into()),
    }
}

pub fn group(model: Exp) -> Exp {
    Exp {
        exp: ParserExp::Group(model.into()),
    }
}

pub fn skip_group(model: Exp) -> Exp {
    Exp {
        exp: ParserExp::SkipGroup(model.into()),
    }
}

pub fn lookahead(model: Exp) -> Exp {
    Exp {
        exp: ParserExp::Lookahead(model.into()),
    }
}

pub fn negative_lookahead(model: Exp) -> Exp {
    Exp {
        exp: ParserExp::NegativeLookahead(model.into()),
    }
}

pub fn skip_to(model: Exp) -> Exp {
    Exp {
        exp: ParserExp::SkipTo(model.into()),
    }
}

pub fn sequence(models: Vec<Exp>) -> Exp {
    Exp {
        exp: ParserExp::Sequence(models.into_boxed_slice()),
    }
}

pub fn choice(models: Vec<Exp>) -> Exp {
    Exp {
        exp: ParserExp::Choice(models.into_boxed_slice()),
    }
}

pub fn optional(model: Exp) -> Exp {
    Exp {
        exp: ParserExp::Optional(model.into()),
    }
}

pub fn closure(model: Exp) -> Exp {
    Exp {
        exp: ParserExp::Closure(model.into()),
    }
}

pub fn positive_closure(model: Exp) -> Exp {
    Exp {
        exp: ParserExp::PositiveClosure(model.into()),
    }
}

pub fn join(exp: Exp, sep: Exp) -> Exp {
    Exp {
        exp: ParserExp::Join {
            exp: exp.into(),
            sep: sep.into(),
        },
    }
}

pub fn positive_join(exp: Exp, sep: Exp) -> Exp {
    Exp {
        exp: ParserExp::PositiveJoin {
            exp: exp.into(),
            sep: sep.into(),
        },
    }
}

pub fn gather(exp: Exp, sep: Exp) -> Exp {
    Exp {
        exp: ParserExp::Gather {
            exp: exp.into(),
            sep: sep.into(),
        },
    }
}

pub fn positive_gather(exp: Exp, sep: Exp) -> Exp {
    Exp {
        exp: ParserExp::PositiveGather {
            exp: exp.into(),
            sep: sep.into(),
        },
    }
}

impl Exp {
    #[inline]
    pub fn alt(exp: Self) -> Self {
        Self {
            exp: ParserExp::Alt(exp.into()),
        }
    }

    #[inline]
    pub fn pattern(pattern: &str) -> Self {
        Self {
            exp: ParserExp::Pattern(pattern.into()),
        }
    }
    #[inline]
    pub fn nil() -> Self {
        Self {
            exp: ParserExp::Nil,
        }
    }

    pub fn rule_include(name: &str, exp: Self) -> Self {
        Self {
            exp: ParserExp::RuleInclude {
                name: name.into(),
                exp: exp.into(),
            },
        }
    }

    #[inline]
    pub fn cut() -> Self {
        Self {
            exp: ParserExp::Cut,
        }
    }

    #[inline]
    pub fn void() -> Self {
        Self {
            exp: ParserExp::Void,
        }
    }

    #[inline]
    pub fn fail() -> Self {
        Self {
            exp: ParserExp::Fail,
        }
    }

    #[inline]
    pub fn dot() -> Self {
        Self {
            exp: ParserExp::Dot,
        }
    }

    #[inline]
    pub fn eof() -> Self {
        Self {
            exp: ParserExp::Eof,
        }
    }

    pub fn call(name: &str, exp: Self) -> Self {
        Self {
            exp: ParserExp::Call(name.into(), exp.into()),
        }
    }

    pub fn token(name: &str) -> Self {
        Self {
            exp: ParserExp::Token(name.into()),
        }
    }

    pub fn constant(value: &str) -> Self {
        Self {
            exp: ParserExp::Constant(value.into()),
        }
    }

    pub fn alert(msg: &str, code: u8) -> Self {
        Self {
            exp: ParserExp::Alert(msg.into(), code),
        }
    }

    pub fn named(name: &str, model: Self) -> Self {
        Self {
            exp: ParserExp::Named(name.into(), model.into()),
        }
    }

    pub fn named_list(name: &str, model: Self) -> Self {
        Self {
            exp: ParserExp::NamedList(name.into(), model.into()),
        }
    }

    pub fn override_node(model: Self) -> Self {
        Self {
            exp: ParserExp::Override(model.into()),
        }
    }

    pub fn override_list(model: Self) -> Self {
        Self {
            exp: ParserExp::OverrideList(model.into()),
        }
    }

    pub fn group(model: Self) -> Self {
        Self {
            exp: ParserExp::Group(model.into()),
        }
    }

    pub fn skip_group(model: Self) -> Self {
        Self {
            exp: ParserExp::SkipGroup(model.into()),
        }
    }

    pub fn lookahead(model: Self) -> Self {
        Self {
            exp: ParserExp::Lookahead(model.into()),
        }
    }

    pub fn negative_lookahead(model: Self) -> Self {
        Self {
            exp: ParserExp::NegativeLookahead(model.into()),
        }
    }

    pub fn skip_to(model: Self) -> Self {
        Self {
            exp: ParserExp::SkipTo(model.into()),
        }
    }

    pub fn sequence(models: Vec<Self>) -> Self {
        Self {
            exp: ParserExp::Sequence(models.into_boxed_slice()),
        }
    }

    pub fn choice(models: Vec<Self>) -> Self {
        Self {
            exp: ParserExp::Choice(models.into_boxed_slice()),
        }
    }

    pub fn optional(model: Self) -> Self {
        Self {
            exp: ParserExp::Optional(model.into()),
        }
    }

    pub fn closure(model: Self) -> Self {
        Self {
            exp: ParserExp::Closure(model.into()),
        }
    }

    pub fn positive_closure(model: Self) -> Self {
        Self {
            exp: ParserExp::PositiveClosure(model.into()),
        }
    }

    pub fn join(exp: Self, sep: Self) -> Self {
        Self {
            exp: ParserExp::Join {
                exp: exp.into(),
                sep: sep.into(),
            },
        }
    }

    pub fn positive_join(exp: Self, sep: Self) -> Self {
        Self {
            exp: ParserExp::PositiveJoin {
                exp: exp.into(),
                sep: sep.into(),
            },
        }
    }

    pub fn gather(exp: Self, sep: Self) -> Self {
        Self {
            exp: ParserExp::Gather {
                exp: exp.into(),
                sep: sep.into(),
            },
        }
    }

    pub fn positive_gather(exp: Self, sep: Self) -> Self {
        Self {
            exp: ParserExp::PositiveGather {
                exp: exp.into(),
                sep: sep.into(),
            },
        }
    }
}
