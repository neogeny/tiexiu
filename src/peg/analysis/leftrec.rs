// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::{BTreeSet, HashSet};

use crate::Grammar;
use crate::cfg::types::Str;
use crate::exp::{Exp, ExpKind};

fn first_calls(grammar: &Grammar, exp: &Exp) -> Vec<usize> {
    match &exp.kind {
        ExpKind::Call { name, .. } => grammar.get_rule_id(name).into_iter().collect(),
        ExpKind::RuleInclude { exp, .. } => exp
            .as_ref()
            .map(|e| first_calls(grammar, e))
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
        | ExpKind::PositiveClosure(inner) => first_calls(grammar, inner),

        ExpKind::Choice(items) => items
            .iter()
            .flat_map(|item| first_calls(grammar, item))
            .collect(),

        ExpKind::Sequence(items) => {
            let mut calls = Vec::new();
            for item in items {
                calls.extend(first_calls(grammar, item));
                if !item.is_nullable() {
                    break;
                }
            }
            calls
        }

        ExpKind::Join { exp, .. }
        | ExpKind::PositiveJoin { exp, .. }
        | ExpKind::Gather { exp, .. }
        | ExpKind::PositiveGather { exp, .. } => first_calls(grammar, exp),

        ExpKind::Nil
        | ExpKind::EmptyClosure
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

fn tarjan_scc(edges: &[Vec<usize>]) -> Vec<Vec<usize>> {
    struct TarjanState {
        index: Vec<i32>,
        lowlink: Vec<i32>,
        on_stack: Vec<bool>,
        stack: Vec<usize>,
        current_index: i32,
        sccs: Vec<Vec<usize>>,
    }

    impl TarjanState {
        fn new(n: usize) -> Self {
            Self {
                index: vec![-1; n],
                lowlink: vec![0; n],
                on_stack: vec![false; n],
                stack: Vec::new(),
                current_index: 0,
                sccs: Vec::new(),
            }
        }

        fn strongconnect(&mut self, v: usize, edges: &[Vec<usize>]) {
            self.index[v] = self.current_index;
            self.lowlink[v] = self.current_index;
            self.current_index += 1;
            self.stack.push(v);
            self.on_stack[v] = true;

            for &w in &edges[v] {
                if self.index[w] == -1 {
                    self.strongconnect(w, edges);
                    self.lowlink[v] = self.lowlink[v].min(self.lowlink[w]);
                } else if self.on_stack[w] {
                    self.lowlink[v] = self.lowlink[v].min(self.index[w]);
                }
            }

            if self.lowlink[v] == self.index[v] {
                let mut scc = Vec::new();
                loop {
                    let w = self.stack.pop().unwrap();
                    self.on_stack[w] = false;
                    scc.push(w);
                    if w == v {
                        break;
                    }
                }
                self.sccs.push(scc);
            }
        }

        fn run(mut self, edges: &[Vec<usize>]) -> Vec<Vec<usize>> {
            for v in 0..edges.len() {
                if self.index[v] == -1 {
                    self.strongconnect(v, edges);
                }
            }
            self.sccs
        }
    }

    TarjanState::new(edges.len()).run(edges)
}

fn find_cycles_in_scc(edges: &[Vec<usize>], scc: &[usize], start: usize) -> Vec<Vec<usize>> {
    let scc_set: HashSet<usize> = scc.iter().cloned().collect();
    let mut cycles = Vec::new();
    let mut path = Vec::new();

    fn dfs(
        node: usize,
        edges: &[Vec<usize>],
        scc_set: &HashSet<usize>,
        path: &mut Vec<usize>,
        cycles: &mut Vec<Vec<usize>>,
    ) {
        if let Some(pos) = path.iter().position(|&n| n == node) {
            cycles.push(path[pos..].to_vec());
            return;
        }
        path.push(node);
        for &child in &edges[node] {
            if scc_set.contains(&child) {
                dfs(child, edges, scc_set, path, cycles);
            }
        }
        path.pop();
    }

    dfs(start, edges, &scc_set, &mut path, &mut cycles);
    cycles
}

impl Grammar {
    pub(crate) fn mark_left_recursion(&mut self) {
        for rule in self.rules_mut() {
            rule.reset_left_recursion();
        }

        let edges: Vec<Vec<usize>> = self
            .rules()
            .map(|rule| first_calls(self, &rule.exp))
            .collect();

        let rule_names: Vec<Str> = self.rules().map(|r| r.name.clone()).collect();

        let sccs = tarjan_scc(&edges);

        for scc in &sccs {
            if scc.len() > 1 {
                for &id in scc {
                    if let Some(rule) = self.rules_mut().nth(id) {
                        rule.set_no_memo();
                    }
                }

                let mut leaders: BTreeSet<usize> = scc.iter().cloned().collect();

                for &start in scc {
                    let cycles = find_cycles_in_scc(&edges, scc, start);
                    for cycle in &cycles {
                        let cycle_set: HashSet<usize> = cycle.iter().cloned().collect();
                        leaders.retain(|id| cycle_set.contains(id));
                        if leaders.is_empty() {
                            break;
                        }
                    }
                    if leaders.is_empty() {
                        break;
                    }
                }

                if leaders.is_empty() {
                    leaders = scc.iter().cloned().collect();
                }

                let leader_id = *leaders
                    .iter()
                    .min_by(|a, b| rule_names[**a].cmp(&rule_names[**b]))
                    .unwrap();

                if let Some(rule) = self.rules_mut().nth(leader_id) {
                    rule.set_left_recursive();
                }
            } else if scc.len() == 1 {
                let id = scc[0];
                if edges[id].contains(&id) {
                    if let Some(rule) = self.rules_mut().nth(id) {
                        rule.set_left_recursive();
                        rule.set_no_memo();
                    }
                }
            }
        }
    }
}
