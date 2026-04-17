// Copyright (g) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Grammar;
use super::exp::{Exp, ExpKind};

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    First,
    Visiting,
    Visited,
}

struct Analyzer<'a> {
    grammar: &'a mut Grammar,
    state: Vec<State>,
    stack: Vec<usize>,
    edges: Vec<Vec<usize>>,
}

impl<'a> Analyzer<'a> {
    fn new(grammar: &'a mut Grammar) -> Self {
        let edges = grammar
            .rules()
            .map(|rule| Self::first_calls(grammar, &rule.exp))
            .collect::<Vec<_>>();
        let len = edges.len();

        Self {
            grammar,
            state: vec![State::First; len],
            stack: Vec::new(),
            edges,
        }
    }

    fn first_calls(grammar: &Grammar, exp: &Exp) -> Vec<usize> {
        match &exp.kind {
            ExpKind::Call { name, .. } => grammar.get_rule_id(name).into_iter().collect(),
            ExpKind::RuleInclude { exp, .. } => exp
                .as_ref()
                .map(|e| Self::first_calls(grammar, e))
                .unwrap_or_default(),

            ExpKind::Named(_, inner)
            | ExpKind::NamedList(_, inner)
            | ExpKind::Override(inner)
            | ExpKind::OverrideList(inner)
            | ExpKind::Group(inner)
            | ExpKind::SkipGroup(inner)
            | ExpKind::Lookahead(inner)
            | ExpKind::NegativeLookahead(inner)
            | ExpKind::SkipTo(inner)
            | ExpKind::Alt(inner)
            | ExpKind::Optional(inner)
            | ExpKind::Closure(inner)
            | ExpKind::PositiveClosure(inner) => Self::first_calls(grammar, inner),

            ExpKind::Choice(items) => items
                .iter()
                .flat_map(|item| Self::first_calls(grammar, item))
                .collect(),

            ExpKind::Sequence(items) => {
                let mut calls = Vec::new();
                for item in items {
                    calls.extend(Self::first_calls(grammar, item));
                    if !item.is_nullable() {
                        break;
                    }
                }
                calls
            }

            ExpKind::Join { exp, .. }
            | ExpKind::PositiveJoin { exp, .. }
            | ExpKind::Gather { exp, .. }
            | ExpKind::PositiveGather { exp, .. } => Self::first_calls(grammar, exp),

            ExpKind::Nil
            | ExpKind::Cut
            | ExpKind::Void
            | ExpKind::Fail
            | ExpKind::Dot
            | ExpKind::Eof
            | ExpKind::Eol
            | ExpKind::Token(_)
            | ExpKind::Pattern(_)
            | ExpKind::Constant(_)
            | ExpKind::Alert(_, _) => Vec::new(),
        }
    }

    fn dfs(&mut self, rule_id: usize) {
        match self.state[rule_id] {
            State::Visited | State::Visiting => return,
            State::First => {}
        }

        self.state[rule_id] = State::Visiting;
        self.stack.push(rule_id);

        for child_id in self.edges[rule_id].clone() {
            match self.state[child_id] {
                State::First => self.dfs(child_id),
                State::Visiting => self.mark_cycle(child_id),
                State::Visited => {}
            }
        }

        self.stack.pop();
        self.state[rule_id] = State::Visited;
    }

    fn mark_cycle(&mut self, child_id: usize) {
        if let Some(start) = self.stack.iter().position(|id| *id == child_id) {
            for rule_id in &self.stack[start..] {
                if let Some(rule) = self.grammar.rules_mut().nth(*rule_id) {
                    rule.set_left_recursive();
                    rule.set_no_memo();
                }
            }
        }
    }
}

impl Grammar {
    pub fn mark_left_recursion(&mut self) {
        for rule in self.rules_mut() {
            rule.reset_left_recursion();
        }

        let mut analyzer = Analyzer::new(self);
        for rule_id in 0..analyzer.edges.len() {
            analyzer.dfs(rule_id);
        }
    }
}
