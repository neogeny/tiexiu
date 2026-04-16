// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Railroad diagram generation for grammars
//!
//! This module provides the ability to visualize PEG grammars as railroad
//! diagrams, similar to railroad syntax diagrams used in language specifications.
//!
//! Based on the Python implementation in tatsu/tatsu/railroads/

use crate::peg::{Exp, ExpKind, Grammar, Rule};

type Rails = Vec<String>;

const ETX: char = '．'; // End of text marker

pub fn tracks(grammar: &Grammar) -> Rails {
    walk_grammar(grammar)
}

pub fn railroads(grammar: &Grammar) -> String {
    tracks(grammar).join("\n")
}

pub fn print_railroads(grammar: &Grammar) {
    for line in tracks(grammar) {
        println!("{}", line.trim_end());
    }
}

fn unicode_width(s: &str) -> usize {
    // Simplified version - full implementation would use unicode-width crate
    s.chars().count()
}

fn pad(s: &str, c: char, max_len: usize) -> String {
    let width = unicode_width(s);
    format!(
        "{}{}",
        s,
        c.to_string().repeat(max_len.saturating_sub(width))
    )
}

fn rail_pad(s: &str, max_len: usize) -> String {
    pad(s, '─', max_len)
}

fn blank_pad(s: &str, max_len: usize) -> String {
    pad(s, ' ', max_len)
}

fn assert_one_length(rails: Rails) -> Rails {
    if rails.is_empty() {
        return vec![];
    }
    let len = unicode_width(&rails[0]);
    for rail in &rails {
        assert_eq!(unicode_width(rail), len, "rails have different lengths");
    }
    rails
}

fn lay_out(tracks: Vec<Rails>) -> Rails {
    if tracks.is_empty() {
        return vec![];
    }
    if tracks.len() == 1 {
        return tracks[0].clone();
    }

    let max_len = tracks
        .iter()
        .filter_map(|t| t.first())
        .map(|s| unicode_width(s))
        .max()
        .unwrap_or(0);

    let mut out: Rails = vec![];

    for (i, rails) in tracks.iter().enumerate() {
        if rails.is_empty() {
            continue;
        }

        let joint = &rails[0];
        let is_last = i == tracks.len() - 1;
        let has_etx = joint.contains(ETX);

        let prefix = if is_last {
            if has_etx {
                if let Some(l) = out.last_mut() {
                    l.truncate(l.len().saturating_sub(3));
                }
                format!("  └─{}   ", blank_pad(joint, max_len))
            } else {
                format!("  └─{}─┘ ", rail_pad(joint, max_len))
            }
        } else if has_etx {
            format!("  ├─{} │ ", blank_pad(joint, max_len))
        } else {
            format!("  ├─{}─┤ ", rail_pad(joint, max_len))
        };
        out.push(prefix);

        for rail in rails.iter().skip(1) {
            out.push(format!("  │ {} │ ", blank_pad(rail, max_len)));
        }
    }

    // First rail
    let first_joint = &tracks[0][0];
    let first_has_etx = first_joint.contains(ETX);
    out[0] = if first_has_etx {
        format!("──┬─{} ┬─", blank_pad(first_joint, max_len))
    } else {
        format!("──┬─{}─┬─", rail_pad(first_joint, max_len))
    };

    assert_one_length(out)
}

fn loop_tail(rails: &[String], max_len: usize) -> Rails {
    let mut out = vec![];
    for line in rails {
        out.push(format!("  │ {} │  ", blank_pad(line, max_len)));
    }
    out.push(format!("  └─{}<┘  ", rail_pad("", max_len)));
    assert_one_length(out)
}

fn stopn_loop(rails: &[String]) -> Rails {
    if rails.is_empty() {
        return vec!["───>───".to_string()];
    }

    let max_len = rails.iter().map(|s| unicode_width(s)).max().unwrap_or(0);

    let mut out = vec![];
    out.push(format!("──┬─{}─┬──", rail_pad(&rails[0], max_len)));
    out.extend(loop_tail(&rails[1..], max_len));
    assert_one_length(out)
}

fn loop_(rails: &[String]) -> Rails {
    if rails.is_empty() {
        return vec!["───>───".to_string()];
    }

    let max_len = rails.iter().map(|s| unicode_width(s)).max().unwrap_or(0);

    let mut out = vec![];
    out.push(format!("──┬→{}─┬──", rail_pad("", max_len)));
    out.push(format!("  ├→{}─┤  ", rail_pad(&rails[0], max_len)));
    out.extend(loop_tail(&rails[1..], max_len));
    assert_one_length(out)
}

fn weld_two(left: Rails, right: Rails) -> Rails {
    if right.is_empty() || left.iter().any(|l| l.contains(ETX)) {
        return left;
    }
    if left.is_empty() {
        return right;
    }

    let left_len = unicode_width(&left[0]);
    let right_len = unicode_width(&right[0]);
    let out_height = left.len().max(right.len());
    let common_height = left.len().min(right.len());

    let mut out = left.clone();
    for i in 0..out_height {
        if i < common_height && !out[i].contains(ETX) {
            out[i].push_str(&right[i]);
        } else if i < left.len() {
            out[i].push_str(&" ".repeat(right_len));
        } else {
            out.push(format!("{}{}", " ".repeat(left_len), right[i]));
        }
    }

    assert_one_length(out)
}

fn weld(tracks: &[Rails]) -> Rails {
    if tracks.is_empty() {
        return vec![];
    }
    let mut out = tracks[0].clone();
    for rails in &tracks[1..] {
        out = weld_two(out, rails.clone());
    }
    out
}

fn walk_grammar(grammar: &Grammar) -> Rails {
    let mut result = vec![];
    for rule in grammar.rules() {
        result.extend(walk_rule(rule));
    }
    result
}

fn walk_rule(rule: &Rule) -> Rails {
    let mut parts: Vec<String> = vec![];

    // Decorators
    if !rule.params.is_empty() {
        let params: Vec<String> = rule.params.iter().map(|p| p.to_string()).collect();
        parts.push(format!("[{}]", params.join(", ")));
    }

    parts.push(rule.name.to_string());
    parts.push(" ●─".to_string());

    let start = parts.join("");
    let rule_rails = walk_exp(&rule.exp);
    let combined = weld(&[vec![start], rule_rails]);
    let mut out = combined;
    out.push("─■".to_string());
    out.push(" ".repeat(unicode_width(&out[0])));

    assert_one_length(out)
}

fn walk_exp(exp: &Exp) -> Rails {
    match &exp.kind {
        ExpKind::Void => vec![" ∅ ".to_string()],
        ExpKind::Fail => vec![" ⚠ ".to_string()],
        ExpKind::Cut => vec![" ✂ ─".to_string()],
        ExpKind::Dot => vec![" ∀ ".to_string()],
        ExpKind::Eof => vec![format!("⇥{} ", ETX)],
        ExpKind::Token(t) => vec![format!("{:?}", t)],
        ExpKind::Pattern(p) => vec![format!("/{}/─", p)],
        ExpKind::Constant(c) => vec![format!("`{}`", c)],
        ExpKind::Call { name, .. } => vec![name.to_string()],
        ExpKind::RuleInclude { name, .. } => vec![format!(" >({}) ", name)],

        ExpKind::Optional(inner) => {
            let inner_rails = walk_exp(inner);
            let prefixed = weld(&[vec!["→".to_string()], inner_rails.clone()]);
            lay_out(vec![prefixed, vec!["→".to_string()]])
        }

        ExpKind::Closure(inner) => loop_(&walk_exp(inner)),
        ExpKind::PositiveClosure(inner) => stopn_loop(&walk_exp(inner)),

        ExpKind::Sequence(items) => {
            let rails: Vec<Rails> = items.iter().map(walk_exp).collect();
            weld(&rails)
        }

        ExpKind::Choice(options) => {
            let tracks: Vec<Rails> = options.iter().map(walk_exp).collect();
            lay_out(tracks)
        }

        ExpKind::Named(name, inner) => {
            let prefixed = weld(&[vec![format!(" `{}`(", name)], walk_exp(inner)]);
            weld(&[prefixed, vec![")".to_string()]])
        }

        ExpKind::Group(inner) => walk_exp(inner),

        ExpKind::Lookahead(inner) => weld(&[
            vec!["─ &[".to_string()],
            walk_exp(inner),
            vec!["]".to_string()],
        ]),

        ExpKind::NegativeLookahead(inner) => weld(&[
            vec!["─ ![".to_string()],
            walk_exp(inner),
            vec!["]".to_string()],
        ]),

        _ => vec![format!("<{:?}>", exp.kind)],
    }
}
