// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::Cfg;
use crate::cfg::constants::*;
use crate::peg::exp::{Exp, ExpKind};
use crate::peg::{Grammar, Rule};
use crate::util::indent::IndentWriter;
use std::fmt::Display;

impl Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.pretty_print())
        } else {
            write!(f, "{:#?}", self)
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.pretty_print())
        } else {
            write!(f, "{:#?}", self)
        }
    }
}

impl Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.pretty_print())
        } else {
            write!(f, "{:#?}", self.kind)
        }
    }
}

impl Display for ExpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.pretty_print())
        } else {
            write!(f, "{:#?}", self)
        }
    }
}

pub trait PrettyPrint {
    fn pretty_print_with(&self, f: &mut IndentWriter) -> String;

    fn pretty_print(&self) -> String {
        let mut writer: IndentWriter = IndentWriter::new(4);
        self.pretty_print_with(&mut writer)
    }
}

impl PrettyPrint for Grammar {
    fn pretty_print_with(&self, writer: &mut IndentWriter) -> String {
        let directives_strs: Vec<_> = self
            .directives
            .iter()
            .filter_map(|d| match d {
                Cfg::Grammar(_) => Some(format!("@@{} :: {}", STR_GRAMMAR_NAME, self.name)),
                Cfg::Wsp(v) => Some(format!("@@{} :: /{}/", STR_WHITESPACE, v)),
                Cfg::Cmt(v) => Some(format!("@@{} :: /{}/", STR_COMMENTS, v)),
                Cfg::Eol(v) => Some(format!("@@{} :: /{}/", STR_EOL_COMMENTS, v)),
                Cfg::NameChars(v) => Some(format!("@@{} :: \"{}\"", STR_NAMECHARS, v)),

                Cfg::IgnoreCase => Some(format!("@@{} :: True", STR_IGNORECASE)),
                Cfg::NoNameGuard => Some(format!("@@{} :: False", STR_NAMEGUARD)),
                Cfg::NoLeftRecursion => Some(format!("@@{} :: False", STR_LEFTREC)),
                Cfg::NoParseInfo => Some(format!("@@{} :: False", STR_PARSEINFO)),
                Cfg::NoMemoization => Some(format!("@@{} :: False", STR_MEMOIZATION)),
                _ => None,
            })
            .collect();

        if !directives_strs.is_empty() {
            writer.writeln(&directives_strs.join("\n"));
            writer.writeln("");
        }

        let keyword_strs = self
            .keywords
            .chunks(8)
            .map(|chunk| format!("@@{} :: {}", STR_KEYWORD, chunk.join(" ")))
            .collect::<Vec<_>>();

        if !keyword_strs.is_empty() {
            writer.writeln(&keyword_strs.join("\n"));
            writer.writeln("");
        }

        for rule in self.rules() {
            let rule_str = rule.pretty_print();
            writer.writeln("");
            writer.writeln(&rule_str);
        }
        writer.take()
    }
}

impl PrettyPrint for Rule {
    fn pretty_print_with(&self, writer: &mut IndentWriter) -> String {
        let mut params_str = String::new();
        if !self.params.is_empty() {
            params_str = format!("[{}]", self.params.join(", "));
        }
        let mut pretty_exp = self.exp.pretty_print_with(writer);
        let start_str = if pretty_exp.lines().count() <= 1 {
            " "
        } else {
            ""
        };
        pretty_exp = "".to_string();
        writer.writeln(&format!(
            "{}{}:{}{}",
            self.name, params_str, start_str, pretty_exp,
        ));
        writer.take()
    }
}

impl PrettyPrint for Exp {
    fn pretty_print_with(&self, writer: &mut IndentWriter) -> String {
        self.kind.pretty_print_with(writer)
    }
}

impl PrettyPrint for ExpKind {
    fn pretty_print_with(&self, writer: &mut IndentWriter) -> String {
        match &self {
            ExpKind::EmptyClosure => "{}".to_string(),
            ExpKind::Nil => "".to_string(),
            ExpKind::RuleInclude { name, exp: _ } => {
                format!(">{}", name)
            }
            ExpKind::Cut => "~".into(),
            ExpKind::Void => "()".into(),
            ExpKind::Fail => "!()".into(),
            ExpKind::Dot => ".".into(),
            ExpKind::Eof => "$".into(),
            ExpKind::Eol => "$->".into(),

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

            ExpKind::Named(name, exp) => format!("{}={}", name, exp.pretty_print_with(writer)),
            ExpKind::NamedList(name, exp) => format!("{}+={}", name, exp.pretty_print_with(writer)),
            ExpKind::Override(exp) => format!("={}", exp.pretty_print_with(writer)),
            ExpKind::OverrideList(exp) => format!("+={}", exp.pretty_print_with(writer)),

            ExpKind::Group(exp) => {
                let exp_str = exp.pretty_print_with(writer);
                if exp_str.lines().count() <= 1 {
                    format!("({})", exp_str)
                } else {
                    writer.take();
                    writer.writeln("(");
                    writer.with_indent(|w| {
                        w.writeln(&exp_str);
                    });
                    writer.writeln(")");
                    writer.take()
                }
            }
            ExpKind::SkipGroup(exp) => {
                format!("(?:{})", exp.pretty_print_with(writer))
            }

            ExpKind::Lookahead(exp) => {
                format!("&{}", exp.pretty_print_with(writer))
            }
            ExpKind::NegativeLookahead(exp) => {
                format!("!{}", exp.pretty_print_with(writer))
            }
            ExpKind::SkipTo(exp) => {
                format!("->{}", exp.pretty_print_with(writer))
            }

            ExpKind::Sequence(sequence) => {
                let pretty = sequence
                    .iter()
                    .map(|s| s.pretty_print_with(writer))
                    .collect::<Vec<_>>();
                let folded = writer.fold(0, &pretty, "", "", "").take();
                if folded.lines().count() > 1 {
                    return format!("\n{}", folded);
                }
                pretty.join(" ")
            }
            ExpKind::Alt(exp) => exp.pretty_print_with(writer),
            ExpKind::Choice(options) => {
                let mut pretty = options
                    .iter()
                    .map(|o| format!("| {}", o.pretty_print_with(writer)))
                    .collect::<Vec<_>>();

                // fold for multi-line
                let folded = writer.fold(0, &pretty, "", "", "").take();
                if folded.lines().count() > 1 {
                    return format!("\n{}", folded);
                }

                // fold again for single line
                pretty = options
                    .iter()
                    .map(|o| o.pretty_print_with(writer))
                    .collect::<Vec<_>>();
                pretty.join(" | ")
            }
            ExpKind::Optional(exp) => {
                format!("[{}]", exp.pretty_print_with(writer))
            }
            ExpKind::Closure(exp) => {
                format!("{{{}}}*", exp.pretty_print_with(writer))
            }
            ExpKind::PositiveClosure(exp) => {
                format!("{{{}}}+", exp.pretty_print_with(writer))
            }

            ExpKind::Join { exp, sep } => {
                format!(
                    "{}%{{{}}}*",
                    sep.pretty_print_with(writer),
                    exp.pretty_print_with(writer)
                )
            }
            ExpKind::PositiveJoin { exp, sep } => {
                format!(
                    "{}%{{{}}}+",
                    sep.pretty_print_with(writer),
                    exp.pretty_print_with(writer)
                )
            }
            ExpKind::Gather { exp, sep } => {
                format!(
                    "{}.{{{}}}*",
                    sep.pretty_print_with(writer),
                    exp.pretty_print_with(writer)
                )
            }
            ExpKind::PositiveGather { exp, sep } => {
                format!(
                    "{}.{{{}}}+",
                    sep.pretty_print_with(writer),
                    exp.pretty_print_with(writer)
                )
            }
        }
    }
}
