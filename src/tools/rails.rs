// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Railroad diagram generation for grammars

use crate::cfg::constants::{SYM_EOL, SYM_ETX};
use crate::peg::{Exp, ExpKind, Grammar, Rule};
use std::rc::Rc;

type Rails = Vec<Rc<str>>;

pub fn tracks(grammar: &Grammar) -> Rails {
    walk_grammar(grammar)
}

pub fn text(grammar: &Grammar) -> String {
    let tracks = walk_grammar(grammar);
    let tracks: Vec<String> = tracks
        .iter()
        .map(|s| s.as_ref().trim_end().to_string())
        .collect();
    tracks.join("\n")
}

pub fn draw(grammar: &Grammar) {
    let tracks = walk_grammar(grammar);
    for line in tracks {
        println!("{}", line.as_ref().trim_end());
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
            .map(|t| t.as_ref().trim_end().to_string())
            .collect();
        s
    }
}

impl ToRailroad for Exp {
    fn railroads(&self) -> String {
        let track = walk_exp(self);
        let s: String = track
            .iter()
            .map(|t| t.as_ref().trim_end().to_string())
            .collect();
        s
    }
}

fn make_rail(s: &str) -> Rc<str> {
    s.into()
}

fn ulen(s: &str) -> usize {
    use unicode_width::UnicodeWidthStr;
    s.width()
}

fn assert_one_length(rails: Rails) -> Rails {
    rails
}

fn pad(s: &str, c: char, width: usize) -> String {
    let padding = width.saturating_sub(ulen(s));
    format!("{}{}", s, c.to_string().repeat(padding))
}

fn railpad(s: &str, maxl: usize) -> String {
    pad(s, '─', maxl)
}

fn blankpad(s: &str, maxl: usize) -> String {
    pad(s, ' ', maxl)
}

fn weld(a: &[Rc<str>], b: &[Rc<str>]) -> Rails {
    if a.is_empty() {
        return b.into();
    }
    if b.is_empty() {
        return a.into();
    }
    if a.iter().any(|s| s.as_ref().contains(SYM_ETX)) {
        return a.into();
    }

    let len_a = ulen(a[0].as_ref());
    let len_b = ulen(b[0].as_ref());
    let height = a.len().max(b.len());
    let common = a.len().min(b.len());

    let mut out: Vec<Rc<str>> = a.to_vec();
    for i in 0..height {
        if i < common {
            out[i] = format!("{}{}", out[i].as_ref(), b[i].as_ref()).into();
        } else if i < a.len() {
            out[i] = format!("{}{:len_b$}", out[i].as_ref(), "").into();
        } else {
            out.push(format!("{:len_a$}{}", "", b[i].as_ref()).into());
        }
    }

    assert_one_length(out)
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
        .map(|s| ulen(s.as_ref()))
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
            let first_line = if !joint.as_ref().contains(SYM_ETX) {
                format!("  ├─{}─┤ ", railpad(joint.as_ref(), maxl))
            } else {
                format!("  ├─{} │ ", blankpad(joint.as_ref(), maxl))
            };
            out.push(first_line.into());

            for rail in &track[1..] {
                out.push(format!("  │ {} │ ", blankpad(rail.as_ref(), maxl)).into());
            }
        } else {
            let is_etx = joint.as_ref().contains(SYM_ETX);
            if !is_etx {
                out.push(format!("  └─{}─┘ ", railpad(joint.as_ref(), maxl)).into());
            } else {
                if let Some(last) = out.last_mut() {
                    let old = last.as_ref().to_string();
                    let end = old.trim_end_matches("─┘ ");
                    *last = format!("{}─┘ ", end).into();
                }
                out.push(format!("  └─{}   ", blankpad(joint.as_ref(), maxl)).into());
            }

            for rail in &track[1..] {
                out.push(format!("    {}   ", blankpad(rail.as_ref(), maxl)).into());
            }
        }
    }

    if !out.is_empty() {
        let first_track = &tracks[0];
        if !first_track.is_empty() {
            let joint = &first_track[0];
            let first_line = if !joint.as_ref().contains(SYM_ETX) {
                format!("──┬─{}─┬─", railpad(joint.as_ref(), maxl))
            } else {
                format!("──┬─{} ┬─", blankpad(joint.as_ref(), maxl))
            };
            out[0] = first_line.into();
        }
    }

    out
}

fn loop_(rails: &Rails) -> Rails {
    if rails.is_empty() {
        return vec![make_rail("───>───")];
    }

    let maxl = rails.iter().map(|s| ulen(s.as_ref())).max().unwrap_or(0);
    let first = &rails[0];

    let mut out = vec![format!("──┬→{}─┬──", railpad("", maxl)).into()];
    out.push(format!("  ├→{}─┤  ", railpad(first.as_ref(), maxl)).into());

    for rail in &rails[1..] {
        out.push(format!("  │ {} │  ", blankpad(rail.as_ref(), maxl)).into());
    }
    out.push(format!("  └─{}<┘  ", railpad("", maxl)).into());

    assert_one_length(out)
}

fn stopnloop(rails: &Rails) -> Rails {
    if rails.is_empty() {
        return vec![make_rail("───>───")];
    }

    let maxl = rails.iter().map(|s| ulen(s.as_ref())).max().unwrap_or(0);
    let first = &rails[0];

    let mut out = vec![format!("──┬─{}─┬──", railpad(first.as_ref(), maxl)).into()];

    for rail in &rails[1..] {
        out.push(format!("  │ {} │  ", blankpad(rail.as_ref(), maxl)).into());
    }
    out.push(format!("  └─{}<┘  ", railpad("", maxl)).into());

    assert_one_length(out)
}

fn join_lists(tracks: impl IntoIterator<Item = Rails>) -> Rails {
    tracks.into_iter().flatten().collect()
}

#[allow(clippy::redundant_closure)]
fn walk_exp(exp: &Exp) -> Rails {
    match &exp.kind {
        ExpKind::EmptyClosure => vec![make_rail("[]")],
        ExpKind::Void => vec![make_rail(" ∅ ")],
        ExpKind::Fail => vec![make_rail(" ⚠ ")],
        ExpKind::Cut => vec![make_rail(" ✂ ")],
        ExpKind::Dot => vec![make_rail(" ∀ ")],
        ExpKind::Eof => vec![make_rail(&format!("⇥{} ", SYM_ETX))],
        ExpKind::Eol => vec![make_rail(&format!("⇥{} ", SYM_EOL))],

        ExpKind::Token(t) => vec![make_rail(&format!("{:?}", t))],
        ExpKind::Pattern(p) => {
            let pat = p.trim_start_matches("r'").trim_end_matches('\'');
            vec![make_rail(&format!("/{}/─", pat))]
        }
        ExpKind::Constant(c) => vec![make_rail(&format!("`{}`", c))],
        ExpKind::Alert(c, level) => vec![make_rail(&format!(
            "{}^`{}`",
            "^".repeat(*level as usize),
            c
        ))],

        ExpKind::Call { name, .. } => vec![make_rail(name.as_ref())],
        ExpKind::RuleInclude { name, .. } => vec![make_rail(&format!(" >({}) ", name))],

        ExpKind::Optional(inner) => {
            let out = weld(&[make_rail("→")], &walk_exp(inner));
            lay_out(&[out, vec![make_rail("→")]])
        }

        ExpKind::Closure(inner) => loop_(&walk_exp(inner)),
        ExpKind::PositiveClosure(inner) => stopnloop(&walk_exp(inner)),

        ExpKind::Join { exp, sep } => {
            let sep = weld(&walk_exp(sep), &[make_rail(" ✂ ─")]);
            let out = weld(&sep, &walk_exp(exp));
            loop_(&out)
        }
        ExpKind::PositiveJoin { exp, sep } => {
            let sep = weld(&walk_exp(sep), &[make_rail(" ✂ ─")]);
            let out = weld(&sep, &walk_exp(exp));
            stopnloop(&out)
        }
        ExpKind::Gather { exp, sep } => {
            let sep = weld(&walk_exp(sep), &[make_rail(" │ ")]);
            let out = weld(&sep, &walk_exp(exp));
            loop_(&out)
        }
        ExpKind::PositiveGather { exp, sep } => {
            let sep = weld(&walk_exp(sep), &[make_rail(" │ ")]);
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
            &[make_rail(&format!(" {}=(", name))],
            &weld(&walk_exp(inner), &[make_rail(") ")]),
        ),

        ExpKind::NamedList(name, inner) => weld(
            &[make_rail(&format!(" {}+=(", name))],
            &weld(&walk_exp(inner), &[make_rail(") ")]),
        ),

        ExpKind::Group(inner) => walk_exp(inner),
        ExpKind::SkipGroup(inner) => walk_exp(inner),

        ExpKind::Lookahead(inner) => weld(
            &[make_rail("─ &[")],
            &weld(&walk_exp(inner), &[make_rail("]")]),
        ),

        ExpKind::NegativeLookahead(inner) => weld(
            &[make_rail("─ ![")],
            &weld(&walk_exp(inner), &[make_rail("]")]),
        ),

        ExpKind::Override(inner) => weld(
            &[make_rail(" =(")],
            &weld(&walk_exp(inner), &[make_rail(") ")]),
        ),

        ExpKind::OverrideList(inner) => {
            let content = walk_exp(inner);
            let _content_len = ulen(content[0].as_ref());
            weld(&[make_rail(" +=(")], &weld(&content, &[make_rail(") ")]))
        }

        ExpKind::Nil => vec![make_rail("")],
    }
}

fn walk_rule(rule: &Rule) -> Rails {
    let leftrec = if rule.is_left_recursive() {
        "⟳".to_string()
    } else if !rule.is_memoizable() {
        "⊬".to_string()
    } else {
        String::new()
    };

    let mut out = vec![make_rail(&format!("{}{} ●─", rule.name, leftrec))];
    out = weld(&out, &walk_exp(&rule.exp));
    out = weld(&out, &[make_rail("─■")]);

    let len0 = ulen(out[0].as_ref());
    let padding = " ".repeat(len0);
    let mut out: Rails = out
        .into_iter()
        .map(|s| format!("{}{}", s.as_ref(), padding).into())
        .collect();
    out.push(" ".repeat(ulen(&out[0])).into());

    assert_one_length(out)
}

fn walk_grammar(grammar: &Grammar) -> Rails {
    let tracks: Vec<Rails> = grammar.rules().map(walk_rule).collect();
    join_lists(tracks)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg::constants::PATH_CALC_GRAMMAR_JSON;

    #[test]
    fn test_make_rail() {
        let rail = make_rail("foo");
        assert_eq!(rail.as_ref(), "foo");
    }

    #[test]
    fn test_weld_empty_left() {
        let a: Rails = vec![];
        let b = vec![make_rail("x")];
        let result = weld(&a, &b);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].as_ref(), "x");
    }

    #[test]
    fn test_weld_empty_right() {
        let a = vec![make_rail("x")];
        let b: Rails = vec![];
        let result = weld(&a, &b);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].as_ref(), "x");
    }

    #[test]
    fn test_weld_simple() {
        let a = vec![make_rail("a")];
        let b = vec![make_rail("b")];
        let result = weld(&a, &b);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].as_ref(), "ab");
    }

    #[test]
    fn test_weld_different_heights() {
        let a = vec![make_rail("a"), make_rail("b")];
        let b = vec![make_rail("c")];
        let result = weld(&a, &b);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_lay_out_empty() {
        let tracks: Vec<Rails> = vec![];
        let result = lay_out(&tracks);
        assert!(result.is_empty());
    }

    #[test]
    fn test_lay_out_single() {
        let tracks = vec![vec![make_rail("foo")]];
        let result = lay_out(&tracks);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_lay_out_two_tracks() {
        let track_a = vec![make_rail("a")];
        let track_b = vec![make_rail("b")];
        let result = lay_out(&[track_a, track_b]);
        let output: String = result
            .iter()
            .map(|s| s.as_ref())
            .collect::<Vec<_>>()
            .join("\n");
        eprintln!("lay_out two:\n{}", output);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_loop_empty() {
        let result = loop_(&vec![]);
        assert_eq!(result.len(), 1);
        assert!(result[0].contains("───>───"));
    }

    #[test]
    fn test_loop_with_content() {
        let inner = vec![make_rail("foo")];
        let result = loop_(&inner);
        let output: String = result
            .iter()
            .map(|s| s.as_ref())
            .collect::<Vec<_>>()
            .join("\n");
        eprintln!("loop:\n{}", output);
        assert!(result.len() >= 2);
    }

    #[test]
    fn test_simple_grammar() {
        use crate::api::compile;
        let grammar = compile(
            r"
        start = 'a';
        ",
            &[],
        )
        .expect("compile failed");
        let result = grammar.railroads();
        eprintln!("simple grammar railroads:\n{}", result);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_calc_json() {
        use crate::api::load;
        let grammar_src = std::fs::read_to_string(PATH_CALC_GRAMMAR_JSON).expect("read file");
        let grammar = load(&grammar_src, &[]).expect("load failed");
        let result = grammar.railroads();
        eprintln!("calc.json railroads:\n{}", result);
    }
}
