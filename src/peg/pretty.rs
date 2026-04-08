// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::{Exp, ExpKind};
use crate::util::indent::IndentWriter;
use std::fmt;

impl Exp {
    pub fn pretty_print(&self, f: &mut IndentWriter) -> String {
        self.kind.pretty_print(f)
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl fmt::Display for ExpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer: IndentWriter = IndentWriter::new(4);
        let result = self.pretty_print(&mut writer);
        write!(f, "{}", result)
    }
}

impl ExpKind {
    fn pretty_print(&self, f: &mut IndentWriter) -> String {
        match &self {
            ExpKind::Nil => "".to_string(),
            ExpKind::RuleInclude { name, rule: _ } => {
                format!(">{}", name)
            }
            ExpKind::Cut => "~".into(),
            ExpKind::Void => "()".into(),
            ExpKind::Fail => "!()".into(),
            ExpKind::Dot => ".".into(),
            ExpKind::Eof => "$".into(),

            ExpKind::Call { name, .. } => name.to_string(),

            ExpKind::Token(token) => format!("\"{}\"", token),
            ExpKind::Pattern(pattern) => {
                if pattern.contains("/") {
                    format!("?\"{}\"", pattern)
                } else {
                    format!("/{}/", pattern)
                }
            }
            ExpKind::Constant(literal) => {
                if literal.lines().count() <= 1 {
                    format!("`{}`", literal)
                } else {
                    format!("```{}```", literal)
                }
            }
            ExpKind::Alert(literal, level) => {
                let level_str = "^".repeat(*level as usize);
                format!("{}`{}`", level_str, literal)
            }

            ExpKind::Named(name, exp) => format!("{}={}", name, exp.pretty_print(f)),
            ExpKind::NamedList(name, exp) => format!("{}+={}", name, exp.pretty_print(f)),
            ExpKind::Override(exp) => format!("={}", exp.pretty_print(f)),
            ExpKind::OverrideList(exp) => format!("+={}", exp.pretty_print(f)),

            ExpKind::Group(exp) => {
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
            ExpKind::SkipGroup(exp) => {
                format!("(?:{})", exp.pretty_print(f))
            }

            ExpKind::Lookahead(exp) => {
                format!("&{}", exp.pretty_print(f))
            }
            ExpKind::NegativeLookahead(exp) => {
                format!("!{}", exp.pretty_print(f))
            }
            ExpKind::SkipTo(exp) => {
                format!("->{}", exp.pretty_print(f))
            }

            ExpKind::Sequence(sequence) => {
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
            ExpKind::Alt(exp) => exp.to_string(),
            ExpKind::Choice(options) => {
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
            ExpKind::Optional(exp) => {
                format!("[{}]", exp.pretty_print(f))
            }
            ExpKind::Closure(exp) => {
                format!("{{{}}}*", exp.pretty_print(f))
            }
            ExpKind::PositiveClosure(exp) => {
                format!("{{{}}}+", exp.pretty_print(f))
            }

            ExpKind::Join { exp, sep } => {
                format!("{}%{{{}}}*", sep.pretty_print(f), exp.pretty_print(f))
            }
            ExpKind::PositiveJoin { exp, sep } => {
                format!("{}%{{{}}}+", sep.pretty_print(f), exp.pretty_print(f))
            }
            ExpKind::Gather { exp, sep } => {
                format!("{}.{{{}}}*", sep.pretty_print(f), exp.pretty_print(f))
            }
            ExpKind::PositiveGather { exp, sep } => {
                format!("{}.{{{}}}+", sep.pretty_print(f), exp.pretty_print(f))
            }
        }
    }
}
