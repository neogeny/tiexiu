// copyright (c) 2026 juancarlo añez (apalala@gmail.com)
// spdx-license-identifier: mit or apache-2.0

use super::exp::{Exp, ExpKind};

/// Constructor methods for PEG expression types.
impl Exp {
    /// Creates a new `Exp` with the given kind, no lookahead, and no defines.
    #[inline]
    pub fn new(exp: ExpKind) -> Self {
        Self {
            kind: exp,
            la: None,
            df: None,
        }
    }

    /// Wraps an expression in an `Alt` (alternative within a choice).
    #[inline]
    pub fn alt(exp: Self) -> Self {
        Self::new(ExpKind::Alt(exp.into()))
    }

    /// Matches a regex pattern at the current position.
    #[inline]
    pub fn pattern(pattern: &str) -> Self {
        crate::util::pyre::compile(pattern).expect("Invalid regex pattern");
        Self::new(ExpKind::Pattern(pattern.into()))
    }

    /// A no-op expression that always succeeds without consuming input.
    #[inline]
    pub fn nil() -> Self {
        Self::new(ExpKind::Nil)
    }

    /// Includes rules from another grammar.
    pub fn rule_include(name: &str) -> Self {
        Self::new(ExpKind::RuleInclude {
            name: name.into(),
            exp: None,
        })
    }

    /// Includes rules from another grammar with an override expression.
    pub fn rule_include_with(name: &str, exp: Self) -> Self {
        Self::new(ExpKind::RuleInclude {
            name: name.into(),
            exp: Some(exp.into()),
        })
    }

    /// The cut operator `~` — commits to the current choice.
    #[inline]
    pub fn cut() -> Self {
        Self::new(ExpKind::Cut)
    }

    /// Consumes whitespace and produces nothing.
    #[inline]
    pub fn void() -> Self {
        Self::new(ExpKind::Void)
    }

    /// An expression that always fails.
    #[inline]
    pub fn fail() -> Self {
        Self::new(ExpKind::Fail)
    }

    /// Matches any single character (`.` in PEG).
    #[inline]
    pub fn dot() -> Self {
        Self::new(ExpKind::Dot)
    }

    /// Matches end of input (`$` in PEG).
    #[inline]
    pub fn eof() -> Self {
        Self::new(ExpKind::Eof)
    }

    /// Matches end of line.
    #[inline]
    pub fn eol() -> Self {
        Self::new(ExpKind::Eol)
    }

    /// A call to another rule by name.
    #[inline]
    pub fn call(name: &str) -> Self {
        Self::new(ExpKind::Call {
            name: name.into(),
            rule: None,
        })
    }

    /// Matches an exact token string.
    #[inline]
    pub fn token(name: &str) -> Self {
        Self::new(ExpKind::Token(name.into()))
    }

    /// Produces a constant value without consuming input.
    #[inline]
    pub fn constant(value: &str) -> Self {
        Self::new(ExpKind::Constant(value.into()))
    }

    /// Raises a parsing alert with a message and severity code.
    #[inline]
    pub fn alert(msg: &str, code: u8) -> Self {
        Self::new(ExpKind::Alert(msg.into(), code))
    }

    /// Names the result of an expression for tree construction.
    #[inline]
    pub fn named(name: &str, model: Self) -> Self {
        Self::new(ExpKind::Named(name.into(), model.into()))
    }

    /// Names the result as a list for tree construction.
    #[inline]
    pub fn named_list(name: &str, model: Self) -> Self {
        Self::new(ExpKind::NamedList(name.into(), model.into()))
    }

    /// Overrides the result with a new tree node.
    #[inline]
    pub fn override_node(model: Self) -> Self {
        Self::new(ExpKind::Override(model.into()))
    }

    /// Overrides the result as a list tree node.
    #[inline]
    pub fn override_list(model: Self) -> Self {
        Self::new(ExpKind::OverrideList(model.into()))
    }

    /// Groups an expression (introduces a nested scope).
    #[inline]
    pub fn group(model: Self) -> Self {
        Self::new(ExpKind::Group(model.into()))
    }

    /// Group whose result is discarded (no wrapping).
    #[inline]
    pub fn skip_group(model: Self) -> Self {
        Self::new(ExpKind::SkipGroup(model.into()))
    }

    /// Positive lookahead — succeeds if the inner expression matches without consuming input.
    #[inline]
    pub fn lookahead(model: Self) -> Self {
        Self::new(ExpKind::Lookahead(model.into()))
    }

    /// Negative lookahead — succeeds if the inner expression fails.
    #[inline]
    pub fn negative_lookahead(model: Self) -> Self {
        Self::new(ExpKind::NegativeLookahead(model.into()))
    }

    /// Skips input until the inner expression matches.
    #[inline]
    pub fn skip_to(model: Self) -> Self {
        Self::new(ExpKind::SkipTo(model.into()))
    }

    /// An ordered sequence of expressions (concatenation).
    #[inline]
    pub fn sequence(models: Vec<Self>) -> Self {
        Self::new(ExpKind::Sequence(models.into_boxed_slice()))
    }

    /// An ordered choice — tries alternatives in sequence, picks the first match.
    #[inline]
    pub fn choice(models: Vec<Self>) -> Self {
        // Do this in favor of existing tests
        let alts = models
            .iter()
            .cloned()
            .map(|model| {
                if let ExpKind::Alt(_) = model.kind {
                    model
                } else {
                    Exp::alt(model)
                }
            })
            .collect::<Vec<_>>();
        Self::new(ExpKind::Choice(alts.into_boxed_slice()))
    }

    /// Optional expression — succeeds even if the inner expression fails.
    #[inline]
    pub fn optional(model: Self) -> Self {
        Self::new(ExpKind::Optional(model.into()))
    }

    /// An empty closure that produces an empty sequence.
    #[inline]
    pub fn empty_closure() -> Self {
        Self::new(ExpKind::EmptyClosure)
    }

    /// Kleene star — matches zero or more repetitions.
    #[inline]
    pub fn closure(model: Self) -> Self {
        Self::new(ExpKind::Closure(model.into()))
    }

    /// Positive closure — matches one or more repetitions.
    #[inline]
    pub fn positive_closure(model: Self) -> Self {
        Self::new(ExpKind::PositiveClosure(model.into()))
    }

    /// Join — matches a sequence separated by `sep`, keeping both.
    #[inline]
    pub fn join(exp: Self, sep: Self) -> Self {
        Self::new(ExpKind::Join {
            exp: exp.into(),
            sep: sep.into(),
        })
    }

    /// Positive join — matches one or more repetitions separated by `sep`.
    #[inline]
    pub fn positive_join(exp: Self, sep: Self) -> Self {
        Self::new(ExpKind::PositiveJoin {
            exp: exp.into(),
            sep: sep.into(),
        })
    }

    /// Gather — matches a left-fold sequence separated by `sep`.
    #[inline]
    pub fn gather(exp: Self, sep: Self) -> Self {
        Self::new(ExpKind::Gather {
            exp: exp.into(),
            sep: sep.into(),
        })
    }

    /// Positive gather — one or more left-fold repetitions separated by `sep`.
    #[inline]
    pub fn positive_gather(exp: Self, sep: Self) -> Self {
        Self::new(ExpKind::PositiveGather {
            exp: exp.into(),
            sep: sep.into(),
        })
    }
}
