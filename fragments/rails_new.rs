// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Railroad diagram generation for grammars

use crate::peg::{Exp, ExpKind, Grammar, Rule};
use std::rc::Rc;

// pub const SYM_ETX: &str = "＃";
pub const SYM_ETX: &str = "$";
pub const SYM_EOL: &str = "⏎";

type Rails = Vec<Rc<str>>;

pub fn tracks(grammar: &Grammar) -> Rails {
    walk_grammar(grammar)
}

pub fn text(grammar: &Grammar) -> String {
    let tracks = walk_grammar(grammar);
    let tracks: Vec<String> = tracks
        .iter()
        .map(|s| s.trim_end().to_string())
        .collect();
    tracks.join("\n")
}

pub fn draw(grammar: &Grammar) {
    let tracks = walk_grammar(grammar);
    for line in tracks {
        println!("{}", line.trim_end());
    }
}

pub trait ToRailroad {
    fn railroads(&self) -> String;
}

impl ToRailroad for Grammar {
    fn railroads(&self) -> String {
        text(self)
    }
}

impl ToRailroad for Rule {
    fn railroads(&self) -> String {
        let track = walk_rule(self);
        let s: String = track
            .iter()
            .map(|t| t.trim_end().to_string())
            .collect();
        s
    }
}

impl ToRailroad for Exp {
    fn railroads(&self) -> String {
        let track = walk_exp(self);
        let s: String = track
            .iter()
            .map(|t| t.trim_end().to_string())
            .collect();
        s
    }
}

fn make_rail(s: &str) -> Rc<str> {
    s.into()
}

fn ulen(s: &str) -> usize {
    s.len()
}

#[track_caller]
fn assert_one_length(rails: &Rails) {
    if rails.is_empty() {
        return;
    }
    let len0 = ulen(&rails[0]);
    for rail in rails.iter().skip(1) {
        assert_eq!(
            ulen(rail),
            len0,
            "lengths differ: {} vs {}:\n[{}]\n[{}]",
            ulen(rail),
            len0,
            rails[0],
            rail,
        );
    }
}

#[track_caller]
fn assert_one_length_mut(rails: Rails) -> Rails {
    assert_one_length(&rails);
    rails
}

fn pad(s: &str, c: char, width: usize) -> String {
    let padding = width.saturating_sub(ulen(s));
    if padding == 0 {
        return s.to_string();
    }
    format!("{}{}", s, c.to_string().repeat(padding))
}

fn railpad(s: &str, maxl: usize) -> String {
    pad(s, '-', maxl)
}

fn blankpad(s: &str, maxl: usize) -> String {
    pad(s, ' ', maxl)
}

fn weld(left: &[Rc<str>], right: &[Rc<str>]) -> Rails {
    if left.is_empty() {
        return right.into();
    }
    if right.is_empty() {
        return left.into();
    }
    if left.iter().any(|s| s.contains(SYM_ETX)) {
        return left.into();
    }
    assert_one_length(&left.to_vec());
    assert_one_length(&right.to_vec());

    let len_left = ulen(&left[0]);
    let len_right = ulen(&right[0]);
    let out_height = left.len().max(right.len());
    let common_height = left.len().min(right.len());

    let mut out: Vec<Rc<str>> = left.to_vec();
    for i in 0..out_height {
        if i < common_height && !out[i].contains(SYM_ETX) {
            out[i] = format!("{:len_left$}{:len_right$}", out[i], right[i]).into();
        } else if i < out.len() {
            out[i] = format!("{}{:len_right$}", out[i], "").into();
        } else {
            out.push(format!("{:len_left$}{}", "", right[i]).into());
            assert_one_length(&out);
        }
    }

    assert_one_length_mut(out)
}

fn lay_out(tracks: &[Rails]) -> Rails {
    if tracks.is_empty() {
        return vec![];
    }
    if tracks.len() == 1 {
        return tracks[0].clone();
    }

    let maxl = tracks
        .iter()
        .filter_map(|t| t.first())
        .map(|s| ulen(s))
        .max()
        .unwrap_or(0);

    let mut out: Rails = Vec::new();

    for (ti, track) in tracks.iter().enumerate() {
        if track.is_empty() {
            continue;
        }

        let is_last = ti == tracks.len() - 1;
        let joint = &track[0];

        if !is_last {
            let first_line = format!("  +-{}-+ ", railpad(joint, maxl));
            out.push(first_line.into());

            for rail in &track[1..] {
                out.push(format!("  | {} | ", blankpad(rail, maxl))).into();
            }
        } else {
            let is_etx = joint.contains(SYM_ETX);
            if !is_etx {
                out.push(format!("  +-{}-+ ", railpad(joint, maxl))).into();
            } else {
                if let Some(last) = out.last_mut() {
                    let old = last.to_string();
                    let end = old.trim_end_matches("-+ ");
                    *last = format!("{}-+ ", end).into();
                }
                out.push(format!("  +-{}   ", blankpad(joint, maxl))).into();
            }

            for rail in &track[1..] {
                out.push(format!("  | {} | ", blankpad(rail, maxl))).into();
            }
        }
    }

    if !out.is_empty() {
        let first_track = &tracks[0];
        if !first_track.is_empty() {
            let joint = &first_track[0];
            let first_line = format!("--+-{}-+ ", railpad(joint, maxl));
            out[0] = first_line.into();
        }
    }

    assert_one_length_mut(out)
}

fn loop_(rails: &Rails) -> Rails {
    if rails.is_empty() {
        return vec![make_rail("---->----")];
    }

    let maxl = rails.iter().map(|s| ulen(s)).max().unwrap_or(0);
    let first = &rails[0];

    let mut out = vec![format!("--+>{}>--", railpad("", maxl)).into()];
    out.push(format!("  +>{}>+  ", railpad(first, maxl))).into();

    for rail in &rails[1..] {
        out.push(format!("  | {} |  ", blankpad(rail, maxl))).into();
    }
    out.push(format!("  +-{}<|  ", railpad("", maxl))).into();

    assert_one_length_mut(out)
}

fn stopnloop(rails: &Rails) -> Rails {
    if rails.is_empty() {
        return vec![make_rail("--->---")];
    }

    let maxl = rails.iter().map(|s| ulen(s)).max().unwrap_or(0);
    let first = &rails[0];

    let mut out = vec![format!("--+-{}>--", railpad(first, maxl)).into()];

    for rail in &rails[1..] {
        out.push(format!("  | {} |  ", blankpad(rail, maxl))).into();
    }
    out.push(format!("  +-{}<   ", railpad("", maxl))).into();

    assert_one_length_mut(out)
}

fn join_lists(tracks: impl IntoIterator<Item = Rails>) -> Rails {
    tracks.into_iter().flatten().collect()
}

#[allow(clippy::redundant_closure)]
fn walk_exp(exp: &Exp) -> Rails {
    match &exp.kind {
        ExpKind::Void => vec![make_rail(" + ")],
        ExpKind::Fail => vec![make_rail(" ! ")]  ,
        ExpKind::Cut => vec![make_rail(" / ")],
        ExpKind::Dot => vec![make_rail(" . ")],
        ExpKind::Eof => vec![make_rail(&format!("> {}", SYM_ETX))],
        ExpKind::Eol => vec![make_rail(&format!("> {}", SYM_EOL))],

        ExpKind::Token(t) => vec![make_rail(&format!("{:?}", t))],
        ExpKind::Pattern(p) => {
            let pat = p.trim_start_matches("r'").trim_end_matches('\'');
            vec![make_rail(&format!("/{}/", pat))]
        }
        ExpKind::Constant(c) => vec![make_rail(&format!("`{}`", c))],
        ExpKind::Alert(c, level) => vec![make_rail(&format!(
            "{}^`{}`",
            "^".repeat(*level as usize),
            c
        ))],

        ExpKind::Call { name, .. } => vec![make_rail(name)],
        ExpKind::RuleInclude { name, .. } => vec![make_rail(&format!(" >({}) ", name))],

        ExpKind::Optional(inner) => {
            let out = weld(&[make_rail(">")], &walk_exp(inner));
            lay_out(&[out, vec![make_rail(">")]])
        }

        ExpKind::Closure(inner) => loop_(&walk_exp(inner)),
        ExpKind::PositiveClosure(inner) => stopnloop(&walk_exp(inner)),

        ExpKind::Join { exp, sep } => {
            let sep = weld(&walk_exp(sep), &[make_rail(" / ")]);
            let out = weld(&sep, &walk_exp(exp));
            loop_(&out)
        }
        ExpKind::PositiveJoin { exp, sep } => {
            let sep = weld(&walk_exp(sep), &[make_rail(" / ")]);
            let out = weld(&sep, &walk_exp(exp));
            stopnloop(&out)
        }
        ExpKind::Gather { exp, sep } => {
            let sep = weld(&walk_exp(sep), &[make_rail(" | ")]);
            let out = weld(&sep, &walk_exp(exp));
            loop_(&out)
        }
        ExpKind::PositiveGather { exp, sep } => {
            let sep = weld(&walk_exp(sep), &[make_rail(" | ")]);
            let out = weld(&sep, &walk_exp(exp));
            stopnloop(&out)
        }

        ExpKind::SkipTo(inner) => weld(
            &[make_rail(" ->(")],
            &weld(&walk_exp(inner), &[make_rail(")")]),
        ),

        ExpKind::Sequence(items) => {
            if items.is_empty() {
                vec![make_rail("")]
            } else {
                let mut result = walk_exp(&items[0]);
                for item in &items[1..] {
                    result = weld(&result, &walk_exp(item));
                }
                result
            }
        }

        ExpKind::Choice(options) => {
            let tracks: Vec<Rails> = options.iter().map(walk_exp).collect();
            lay_out(&tracks)
        }
        ExpKind::Alt(inner) => walk_exp(inner),

        ExpKind::Named(name, inner) => weld(
            &[make_rail(&format!(" {}=( ", name))],
            &weld(&walk_exp(inner), &[make_rail(" )")]),
        ),

        ExpKind::NamedList(name, inner) => weld(
            &[make_rail(&format!(" {}+=( ", name))],
            &weld(&walk_exp(inner), &[make_rail(" )")]),
        ),

        ExpKind::Group(inner) => walk_exp(inner),
        ExpKind::SkipGroup(inner) => walk_exp(inner),

        ExpKind::Lookahead(inner) => weld(
            &[make_rail("= [")],
            &weld(&walk_exp(inner), &[make_rail("]")]),
        ),

        ExpKind::NegativeLookahead(inner) => weld(
            &[make_rail("= [")],
            &weld(&walk_exp(inner), &[make_rail("]")]),
        ),

        ExpKind::Override(inner) => weld(
            &[make_rail(" =(")],
            &weld(&walk_exp(inner), &[make_rail(") ")]),
        ),

        ExpKind::OverrideList(inner) => {
            let content = walk_exp(inner);
            weld(&[make_rail(" =(")], &weld(&content, &[make_rail(") ")]))
        }

        ExpKind::Nil => vec![make_rail("")],
    }
}

fn walk_rule(rule: &Rule) -> Rails {
    let leftrec = if rule.is_left_recursive() {
        "#".to_string()
    } else if !rule.is_memoizable() {
        "*".to_string()
    } else {
        String::new()
    };

    let mut out = vec![make_rail(&format!("{}{} *-", rule.name, leftrec))];
    out = weld(&out, &walk_exp(&rule.exp));
    out = weld(&out, &[make_rail("-#")]);

    let len0 = ulen(&out[0]);
    let padding = " ".repeat(len0);
    let out: Rails = out
        .into_iter()
        .map(|s| format!("{}{}", s, padding).into())
        .collect();
    out.push(" ".repeat(ulen(&out[0])).into());

    assert_one_length_mut(out)
}

fn walk_grammar(grammar: &Grammar) -> Rails {
    let tracks: Vec<Rails> = grammar.rules().map(walk_rule).collect();
    join_lists(tracks)
}