// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::grammar::Grammar;
use crate::grammars::{Model, Rule};
use std::collections::HashMap;

pub fn mark_left_recursion(mut grammar: Grammar) -> Grammar {
    {
        let mut analysis = LeftRecursionAnalysis::new(&mut grammar.rulemap);
        analysis.run();
    }
    grammar
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    First,
    Cutoff,
    Visited,
}

struct LeftRecursionAnalysis<'a, 'g> {
    // Note: The keys in the rulemap are now &'g str to match your Grammar definition
    rules: &'a mut HashMap<&'g str, Rule<'g>>,
    node_state: HashMap<*const Model, Status>,
    node_depth: HashMap<*const Model, usize>,
    depth_stack: Vec<isize>,
    depth: usize,
}

impl<'a, 'g> LeftRecursionAnalysis<'a, 'g> {
    fn new(rules: &'a mut HashMap<&'g str, Rule<'g>>) -> Self {
        Self {
            rules,
            node_state: HashMap::new(),
            node_depth: HashMap::new(),
            depth_stack: vec![-1],
            depth: 0,
        }
    }

    fn run(&mut self) {
        // Collect keys (Rule names) to avoid borrowing conflicts
        let names: Vec<&'g str> = self.rules.keys().cloned().collect();
        for name in names {
            let rhs = self.rules.get(name).unwrap().rhs;
            self.dfs(rhs);
        }
    }

    fn dfs(&mut self, node: &'g Model) {
        let ptr = node as *const Model;

        if *self.node_state.get(&ptr).unwrap_or(&Status::First) != Status::First {
            return;
        }

        self.node_state.insert(ptr, Status::Cutoff);
        self.node_depth.insert(ptr, self.depth);
        self.depth += 1;

        for child in node.callable_from() {
            if let Model::Call(target_name) = child {
                // target_name is a &String inside the Model,
                // but handle_call accepts a &str.
                self.handle_call(target_name);
            } else {
                self.dfs(child);
            }
        }

        self.node_depth.remove(&ptr);
        self.depth -= 1;
        self.node_state.insert(ptr, Status::Visited);
    }

    fn handle_call(&mut self, target_name: &str) {
        let target_rhs = {
            let Some(rule) = self.rules.get(target_name) else {
                return;
            };
            rule.rhs as *const Model
        };

        self.depth_stack.push(self.depth as isize);

        let rhs = self.rules.get(target_name).unwrap().rhs;
        self.dfs(rhs);

        let is_cutoff = self.node_state.get(&target_rhs) == Some(&Status::Cutoff);
        let target_depth = *self.node_depth.get(&target_rhs).unwrap_or(&0) as isize;
        let parent_depth = *self.depth_stack.last().unwrap();

        if is_cutoff && target_depth > parent_depth
            && let Some(rule) = self.rules.get_mut(target_name) {
                rule.is_memo = false;
                rule.is_lrec = true;
            }

        self.depth_stack.pop();
    }
}
