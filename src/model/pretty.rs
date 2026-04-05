// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::elements::{Element, ParserElem};
use crate::util::indent::IndentWriter;
use std::fmt;

impl Element {
    pub fn pretty_print(&self, f: &mut IndentWriter) -> String {
        self.parser.pretty_print(f)
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.parser.fmt(f)
    }
}

impl fmt::Display for ParserElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer: IndentWriter = IndentWriter::new(4);
        let result = self.pretty_print(&mut writer);
        write!(f, "{}", result)
    }
}

impl ParserElem {
    fn pretty_print(&self, f: &mut IndentWriter) -> String {
        match &self {
            ParserElem::Nil => "".to_string(),
            ParserElem::RuleInclude { name, exp: _ } => {
                format!(">{}", name)
            }
            ParserElem::Cut => "~".into(),
            ParserElem::Void => "()".into(),
            ParserElem::Fail => "!()".into(),
            ParserElem::Dot => ".".into(),
            ParserElem::Eof => "$".into(),

            ParserElem::Call(name, _exp) => name.to_string(),

            ParserElem::Token(token) => format!("\"{}\"", token),
            ParserElem::Pattern(pattern) => {
                if pattern.contains("/") {
                    format!("?\"{}\"", pattern)
                } else {
                    format!("/{}/", pattern)
                }
            }
            ParserElem::Constant(literal) => {
                if literal.lines().count() > 1 {
                    format!("`{}`", literal)
                } else {
                    format!("```{}```", literal)
                }
            }
            ParserElem::Alert(literal, level) => {
                let level_str = "^".repeat(*level as usize);
                format!("{}`{}`", level_str, literal)
            }

            ParserElem::Named(name, exp) => format!("{}={}", name, exp.pretty_print(f)),
            ParserElem::NamedList(name, exp) => format!("{}+={}", name, exp.pretty_print(f)),
            ParserElem::Override(exp) => format!("={}", exp.pretty_print(f)),
            ParserElem::OverrideList(exp) => format!("+={}", exp.pretty_print(f)),

            ParserElem::Group(exp) => {
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
            ParserElem::SkipGroup(exp) => {
                format!("(?:{})", exp.pretty_print(f))
            }

            ParserElem::Lookahead(exp) => {
                format!("&{}", exp.pretty_print(f))
            }
            ParserElem::NegativeLookahead(exp) => {
                format!("!{}", exp.pretty_print(f))
            }
            ParserElem::SkipTo(exp) => {
                format!("->{}", exp.pretty_print(f))
            }

            ParserElem::Sequence(sequence) => {
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
            ParserElem::Alt(exp) => exp.to_string(),
            ParserElem::Choice(options) => {
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
            ParserElem::Optional(exp) => {
                format!("[{}]", exp.pretty_print(f))
            }
            ParserElem::Closure(exp) => {
                format!("{{{}}}*", exp.pretty_print(f))
            }
            ParserElem::PositiveClosure(exp) => {
                format!("{{{}}}+", exp.pretty_print(f))
            }

            ParserElem::Join { exp, sep } => {
                format!("{}%{{{}}}*", sep.pretty_print(f), exp.pretty_print(f))
            }
            ParserElem::PositiveJoin { exp, sep } => {
                format!("{}%{{{}}}+", sep.pretty_print(f), exp.pretty_print(f))
            }
            ParserElem::Gather { exp, sep } => {
                format!("{}.{{{}}}*", sep.pretty_print(f), exp.pretty_print(f))
            }
            ParserElem::PositiveGather { exp, sep } => {
                format!("{}.{{{}}}+", sep.pretty_print(f), exp.pretty_print(f))
            }
        }
    }
}
