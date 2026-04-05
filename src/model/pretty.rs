// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::elements::Element;
use crate::util::indent::IndentWriter;
use std::fmt;

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer: IndentWriter = IndentWriter::new(4);
        let result = self.pretty_print(&mut writer);
        write!(f, "{}", result)
    }
}

impl Element {
    fn pretty_print(&self, f: &mut IndentWriter) -> String {
        match &self {
            Element::Nil => "".to_string(),
            Element::RuleInclude { name, exp: _ } => {
                format!(">{}", name)
            }
            Element::Cut => "~".into(),
            Element::Void => "()".into(),
            Element::Fail => "!()".into(),
            Element::Dot => ".".into(),
            Element::Eof => "$".into(),

            Element::Call(name, _exp) => name.to_string(),

            Element::Token(token) => format!("\"{}\"", token),
            Element::Pattern(pattern) => {
                if pattern.contains("/") {
                    format!("?\"{}\"", pattern)
                } else {
                    format!("/{}/", pattern)
                }
            }
            Element::Constant(literal) => {
                if literal.lines().count() > 1 {
                    format!("`{}`", literal)
                } else {
                    format!("```{}```", literal)
                }
            }
            Element::Alert(literal, level) => {
                let level_str = "^".repeat(*level as usize);
                format!("{}`{}`", level_str, literal)
            }

            Element::Named(name, exp) => format!("{}={}", name, exp.pretty_print(f)),
            Element::NamedList(name, exp) => format!("{}+={}", name, exp.pretty_print(f)),
            Element::Override(exp) => format!("={}", exp.pretty_print(f)),
            Element::OverrideList(exp) => format!("+={}", exp.pretty_print(f)),

            Element::Group(exp) => {
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
            Element::SkipGroup(exp) => {
                format!("(?:{})", exp.pretty_print(f))
            }

            Element::Lookahead(exp) => {
                format!("&{}", exp.pretty_print(f))
            }
            Element::NegativeLookahead(exp) => {
                format!("!{}", exp.pretty_print(f))
            }
            Element::SkipTo(exp) => {
                format!("->{}", exp.pretty_print(f))
            }

            Element::Sequence(sequence) => {
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
            Element::Alt(exp) => exp.to_string(),
            Element::Choice(options) => {
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
            Element::Optional(exp) => {
                format!("[{}]", exp.pretty_print(f))
            }
            Element::Closure(exp) => {
                format!("{{{}}}*", exp.pretty_print(f))
            }
            Element::PositiveClosure(exp) => {
                format!("{{{}}}+", exp.pretty_print(f))
            }

            Element::Join { exp, sep } => {
                format!("{}%{{{}}}*", sep.pretty_print(f), exp.pretty_print(f))
            }
            Element::PositiveJoin { exp, sep } => {
                format!("{}%{{{}}}+", sep.pretty_print(f), exp.pretty_print(f))
            }
            Element::Gather { exp, sep } => {
                format!("{}.{{{}}}*", sep.pretty_print(f), exp.pretty_print(f))
            }
            Element::PositiveGather { exp, sep } => {
                format!("{}.{{{}}}+", sep.pretty_print(f), exp.pretty_print(f))
            }
        }
    }
}
