// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::elements::{Element, ParserElem};

#[inline]
pub fn cut() -> Element {
    Element {
        parser: ParserElem::Cut,
    }
}

#[inline]
pub fn void() -> Element {
    Element {
        parser: ParserElem::Void,
    }
}

#[inline]
pub fn fail() -> Element {
    Element {
        parser: ParserElem::Fail,
    }
}

#[inline]
pub fn dot() -> Element {
    Element {
        parser: ParserElem::Dot,
    }
}

#[inline]
pub fn eof() -> Element {
    Element {
        parser: ParserElem::Eof,
    }
}

pub fn call(name: &str, exp: Element) -> Element {
    Element {
        parser: ParserElem::Call(name.into(), exp.into()),
    }
}

pub fn token(name: &str) -> Element {
    Element {
        parser: ParserElem::Token(name.into()),
    }
}

pub fn constant(value: &str) -> Element {
    Element {
        parser: ParserElem::Constant(value.into()),
    }
}

pub fn alert(msg: &str, code: u8) -> Element {
    Element {
        parser: ParserElem::Alert(msg.into(), code),
    }
}

pub fn named(name: &str, model: Element) -> Element {
    Element {
        parser: ParserElem::Named(name.into(), model.into()),
    }
}

pub fn named_list(name: &str, model: Element) -> Element {
    Element {
        parser: ParserElem::NamedList(name.into(), model.into()),
    }
}

pub fn override_node(model: Element) -> Element {
    Element {
        parser: ParserElem::Override(model.into()),
    }
}

pub fn override_list(model: Element) -> Element {
    Element {
        parser: ParserElem::OverrideList(model.into()),
    }
}

pub fn group(model: Element) -> Element {
    Element {
        parser: ParserElem::Group(model.into()),
    }
}

pub fn skip_group(model: Element) -> Element {
    Element {
        parser: ParserElem::SkipGroup(model.into()),
    }
}

pub fn lookahead(model: Element) -> Element {
    Element {
        parser: ParserElem::Lookahead(model.into()),
    }
}

pub fn negative_lookahead(model: Element) -> Element {
    Element {
        parser: ParserElem::NegativeLookahead(model.into()),
    }
}

pub fn skip_to(model: Element) -> Element {
    Element {
        parser: ParserElem::SkipTo(model.into()),
    }
}

pub fn sequence(models: Vec<Element>) -> Element {
    Element {
        parser: ParserElem::Sequence(models.into_boxed_slice()),
    }
}

pub fn choice(models: Vec<Element>) -> Element {
    Element {
        parser: ParserElem::Choice(models.into_boxed_slice()),
    }
}

pub fn optional(model: Element) -> Element {
    Element {
        parser: ParserElem::Optional(model.into()),
    }
}

pub fn closure(model: Element) -> Element {
    Element {
        parser: ParserElem::Closure(model.into()),
    }
}

pub fn positive_closure(model: Element) -> Element {
    Element {
        parser: ParserElem::PositiveClosure(model.into()),
    }
}

pub fn join(exp: Element, sep: Element) -> Element {
    Element {
        parser: ParserElem::Join {
            exp: exp.into(),
            sep: sep.into(),
        },
    }
}

pub fn positive_join(exp: Element, sep: Element) -> Element {
    Element {
        parser: ParserElem::PositiveJoin {
            exp: exp.into(),
            sep: sep.into(),
        },
    }
}

pub fn gather(exp: Element, sep: Element) -> Element {
    Element {
        parser: ParserElem::Gather {
            exp: exp.into(),
            sep: sep.into(),
        },
    }
}

pub fn positive_gather(exp: Element, sep: Element) -> Element {
    Element {
        parser: ParserElem::PositiveGather {
            exp: exp.into(),
            sep: sep.into(),
        },
    }
}

impl Element {
    #[inline]
    pub fn alt(exp: Self) -> Self {
        Self {
            parser: ParserElem::Alt(exp.into()),
        }
    }

    #[inline]
    pub fn pattern(pattern: &str) -> Self {
        Self {
            parser: ParserElem::Pattern(pattern.into()),
        }
    }
    #[inline]
    pub fn nil() -> Self {
        Self {
            parser: ParserElem::Nil,
        }
    }

    pub fn rule_include(name: &str, exp: Self) -> Self {
        Self {
            parser: ParserElem::RuleInclude {
                name: name.into(),
                exp: exp.into(),
            },
        }
    }

    #[inline]
    pub fn cut() -> Self {
        Self {
            parser: ParserElem::Cut,
        }
    }

    #[inline]
    pub fn void() -> Self {
        Self {
            parser: ParserElem::Void,
        }
    }

    #[inline]
    pub fn fail() -> Self {
        Self {
            parser: ParserElem::Fail,
        }
    }

    #[inline]
    pub fn dot() -> Self {
        Self {
            parser: ParserElem::Dot,
        }
    }

    #[inline]
    pub fn eof() -> Self {
        Self {
            parser: ParserElem::Eof,
        }
    }

    pub fn call(name: &str, exp: Self) -> Self {
        Self {
            parser: ParserElem::Call(name.into(), exp.into()),
        }
    }

    pub fn token(name: &str) -> Self {
        Self {
            parser: ParserElem::Token(name.into()),
        }
    }

    pub fn constant(value: &str) -> Self {
        Self {
            parser: ParserElem::Constant(value.into()),
        }
    }

    pub fn alert(msg: &str, code: u8) -> Self {
        Self {
            parser: ParserElem::Alert(msg.into(), code),
        }
    }

    pub fn named(name: &str, model: Self) -> Self {
        Self {
            parser: ParserElem::Named(name.into(), model.into()),
        }
    }

    pub fn named_list(name: &str, model: Self) -> Self {
        Self {
            parser: ParserElem::NamedList(name.into(), model.into()),
        }
    }

    pub fn override_node(model: Self) -> Self {
        Self {
            parser: ParserElem::Override(model.into()),
        }
    }

    pub fn override_list(model: Self) -> Self {
        Self {
            parser: ParserElem::OverrideList(model.into()),
        }
    }

    pub fn group(model: Self) -> Self {
        Self {
            parser: ParserElem::Group(model.into()),
        }
    }

    pub fn skip_group(model: Self) -> Self {
        Self {
            parser: ParserElem::SkipGroup(model.into()),
        }
    }

    pub fn lookahead(model: Self) -> Self {
        Self {
            parser: ParserElem::Lookahead(model.into()),
        }
    }

    pub fn negative_lookahead(model: Self) -> Self {
        Self {
            parser: ParserElem::NegativeLookahead(model.into()),
        }
    }

    pub fn skip_to(model: Self) -> Self {
        Self {
            parser: ParserElem::SkipTo(model.into()),
        }
    }

    pub fn sequence(models: Vec<Self>) -> Self {
        Self {
            parser: ParserElem::Sequence(models.into_boxed_slice()),
        }
    }

    pub fn choice(models: Vec<Self>) -> Self {
        Self {
            parser: ParserElem::Choice(models.into_boxed_slice()),
        }
    }

    pub fn optional(model: Self) -> Self {
        Self {
            parser: ParserElem::Optional(model.into()),
        }
    }

    pub fn closure(model: Self) -> Self {
        Self {
            parser: ParserElem::Closure(model.into()),
        }
    }

    pub fn positive_closure(model: Self) -> Self {
        Self {
            parser: ParserElem::PositiveClosure(model.into()),
        }
    }

    pub fn join(exp: Self, sep: Self) -> Self {
        Self {
            parser: ParserElem::Join {
                exp: exp.into(),
                sep: sep.into(),
            },
        }
    }

    pub fn positive_join(exp: Self, sep: Self) -> Self {
        Self {
            parser: ParserElem::PositiveJoin {
                exp: exp.into(),
                sep: sep.into(),
            },
        }
    }

    pub fn gather(exp: Self, sep: Self) -> Self {
        Self {
            parser: ParserElem::Gather {
                exp: exp.into(),
                sep: sep.into(),
            },
        }
    }

    pub fn positive_gather(exp: Self, sep: Self) -> Self {
        Self {
            parser: ParserElem::PositiveGather {
                exp: exp.into(),
                sep: sep.into(),
            },
        }
    }
}
