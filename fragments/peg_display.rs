// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::peg::exp::{Exp, ExpKind};
use crate::peg::{Grammar, Rule};
use crate::util::indent::IndentWriter;
use std::fmt::Display;

impl Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.pretty_print())?
        }
        // This builder exists on Formatter and works for both traits.
        f.debug_struct("Grammar")
            .field("name", &self.name)
            .field("analyzed", &self.analyzed)
            .field("directives", &self.directives)
            .field("rules", &self.rules.len())
            .finish()
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.pretty_print())?
        }
        f.debug_struct("Rule")
            .field("name", &self.name)
            .field("params", &self.params)
            .field("flags", &self.flags)
            .field("exp", &self.exp)
            .finish()
    }

}

impl Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}

impl fmt::Display for ExpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Unit variants
            Self::Nil => write!(f, "Nil"),
            Self::Cut => write!(f, "Cut"),
            Self::Void => write!(f, "Void"),
            Self::Fail => write!(f, "Fail"),
            Self::Dot => write!(f, "Dot"),
            Self::Eof => write!(f, "Eof"),

            // Struct-like variants
            Self::Call { name, rule } => f.debug_struct("Call")
                .field("name", name)
                .field("rule", rule)
                .finish(),

            Self::Join { exp, sep } => f.debug_struct("Join")
                .field("exp", exp)
                .field("sep", sep)
                .finish(),

            Self::RuleInclude { name, exp } => f.debug_struct("RuleInclude")
                .field("name", name)
                .field("exp", exp)
                .finish(),

            // Tuple variants
            Self::Token(s) => f.debug_tuple("Token").field(s).finish(),
            Self::Pattern(s) => f.debug_tuple("Pattern").field(s).finish(),
            Self::Named(n, e) => f.debug_tuple("Named").field(n).field(e).finish(),
            Self::Sequence(arr) => f.debug_tuple("Sequence").field(arr).finish(),
            Self::Choice(arr) => f.debug_tuple("Choice").field(arr).finish(),

            Self::Alert(s, level) => f.debug_tuple("Alert").field(s).field(level).finish(),
            _ => write!(f, "{:?}", self), // Fallback to Debug if you're in a hurry
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
        writer.writeln(&format!("@@grammar:: {}", self.name));

        // TODO: Write the rest of the directives
        // TODO: Write keywords

        for rule in self.rules() {
            writer.writeln("");
            let rule_str = rule.pretty_print_with(writer);
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
        let pretty_exp = self.exp.pretty_print_with(writer);
        let start_str = if pretty_exp.lines().count() <= 1 {
            " "
        } else {
            ""
        };
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
            ExpKind::Nil => "".to_string(),
            ExpKind::RuleInclude { name, exp: _ } => {
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

