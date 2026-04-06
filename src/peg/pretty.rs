// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::{Exp, ParserExp};
use crate::util::indent::IndentWriter;
use std::fmt;

impl Exp {
    pub fn pretty_print(&self, f: &mut IndentWriter) -> String {
        self.exp.pretty_print(f)
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.exp.fmt(f)
    }
}

impl fmt::Display for ParserExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer: IndentWriter = IndentWriter::new(4);
        let result = self.pretty_print(&mut writer);
        write!(f, "{}", result)
    }
}

impl ParserExp {
    fn pretty_print(&self, f: &mut IndentWriter) -> String {
        match &self {
            ParserExp::Nil => "".to_string(),
            ParserExp::RuleInclude { name, exp: _ } => {
                format!(">{}", name)
            }
            ParserExp::Cut => "~".into(),
            ParserExp::Void => "()".into(),
            ParserExp::Fail => "!()".into(),
            ParserExp::Dot => ".".into(),
            ParserExp::Eof => "$".into(),

            ParserExp::Call(name, _exp) => name.to_string(),

            ParserExp::Token(token) => format!("\"{}\"", token),
            ParserExp::Pattern(pattern) => {
                if pattern.contains("/") {
                    format!("?\"{}\"", pattern)
                } else {
                    format!("/{}/", pattern)
                }
            }
            ParserExp::Constant(literal) => {
                if literal.lines().count() > 1 {
                    format!("`{}`", literal)
                } else {
                    format!("```{}```", literal)
                }
            }
            ParserExp::Alert(literal, level) => {
                let level_str = "^".repeat(*level as usize);
                format!("{}`{}`", level_str, literal)
            }

            ParserExp::Named(name, exp) => format!("{}={}", name, exp.pretty_print(f)),
            ParserExp::NamedList(name, exp) => format!("{}+={}", name, exp.pretty_print(f)),
            ParserExp::Override(exp) => format!("={}", exp.pretty_print(f)),
            ParserExp::OverrideList(exp) => format!("+={}", exp.pretty_print(f)),

            ParserExp::Group(exp) => {
                let exp_str = exp.pretty_print(f);
                if exp_str.lines().count() <= 1 {
                    format!("({})", exp_str)
                } else {
                    f.take();
                    f.writeln("(");
                    f.with_indent(|f| {
                        f.writeln(&exp_str);
                    });
                    f.writeln(")");
                    f.take()
                }
            }
            ParserExp::SkipGroup(exp) => {
                format!("(?:{})", exp.pretty_print(f))
            }

            ParserExp::Lookahead(exp) => {
                format!("&{}", exp.pretty_print(f))
            }
            ParserExp::NegativeLookahead(exp) => {
                format!("!{}", exp.pretty_print(f))
            }
            ParserExp::SkipTo(exp) => {
                format!("->{}", exp.pretty_print(f))
            }

            ParserExp::Sequence(sequence) => {
                let pretty = sequence
                    .iter()
                    .map(|s| s.pretty_print(f))
                    .collect::<Vec<_>>();
                let folded = f.fold(0, &pretty, "", "", "").take();
                if folded.lines().count() > 1 {
                    return format!("\n{}", folded);
                }
                pretty.join(" ")
            }
            ParserExp::Alt(exp) => exp.to_string(),
            ParserExp::Choice(options) => {
                let mut pretty = options
                    .iter()
                    .map(|o| format!("| {}", o.pretty_print(f)))
                    .collect::<Vec<_>>();

                // fold for multi-line
                let folded = f.fold(0, &pretty, "", "", "").take();
                if folded.lines().count() > 1 {
                    return format!("\n{}", folded);
                }

                // fold again for single line
                pretty = options
                    .iter()
                    .map(|o| o.pretty_print(f))
                    .collect::<Vec<_>>();
                pretty.join(" | ")
            }
            ParserExp::Optional(exp) => {
                format!("[{}]", exp.pretty_print(f))
            }
            ParserExp::Closure(exp) => {
                format!("{{{}}}*", exp.pretty_print(f))
            }
            ParserExp::PositiveClosure(exp) => {
                format!("{{{}}}+", exp.pretty_print(f))
            }

            ParserExp::Join { exp, sep } => {
                format!("{}%{{{}}}*", sep.pretty_print(f), exp.pretty_print(f))
            }
            ParserExp::PositiveJoin { exp, sep } => {
                format!("{}%{{{}}}+", sep.pretty_print(f), exp.pretty_print(f))
            }
            ParserExp::Gather { exp, sep } => {
                format!("{}.{{{}}}*", sep.pretty_print(f), exp.pretty_print(f))
            }
            ParserExp::PositiveGather { exp, sep } => {
                format!("{}.{{{}}}+", sep.pretty_print(f), exp.pretty_print(f))
            }
        }
    }
}
