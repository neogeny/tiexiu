// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::E;
use super::Grammar;
use std::collections::HashMap;

pub fn mark_left_recursion(grammar: &mut Grammar) {
    let mut analysis = LeftRecursionAnalysis::new(grammar);
    analysis.run()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    First,
    Cutoff,
    Visited,
}

struct LeftRecursionAnalysis<'a> {
    grammar: &'a mut Grammar,
    node_state: HashMap<*const E, Status>,
    node_depth: HashMap<*const E, usize>,
    depth_stack: Vec<isize>,
    rule_name_stack: Vec<String>,
    depth: usize,
}

impl<'a> LeftRecursionAnalysis<'a> {
    fn new(grammar: &'a mut Grammar) -> Self {
        Self {
            grammar,
            node_state: HashMap::new(),
            node_depth: HashMap::new(),
            depth_stack: vec![-1],
            rule_name_stack: vec![],
            depth: 0,
        }
    }

    fn run(&mut self) {
        let models = &self
            .grammar
            .rulemap
            .values()
            .map(|r| r.rhs.clone())
            .collect::<Vec<_>>();
        for model in models {
            self.dfs(model);
        }
    }

    fn dfs(&mut self, node: &E) {
        let ptr = node as *const E;

        if *self.node_state.get(&ptr).unwrap_or(&Status::First) != Status::First {
            return;
        }

        self.node_state.insert(ptr, Status::Cutoff);
        self.node_depth.insert(ptr, self.depth);
        self.depth += 1;

        for child in node.callable_from() {
            if let E::Call(target_name) = child {
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
            let Some(rule) = self.grammar.rulemap.get(target_name) else {
                return;
            };
            &rule.rhs as *const E
        };

        let rule = self.grammar.rulemap.get(target_name).unwrap();
        let is_leftrec = rule.is_left_recursive();
        if is_leftrec {
            self.depth_stack.push(self.depth as isize);
            self.rule_name_stack.push(rule.name.clone());
        }
        self.dfs(&rule.rhs.clone());

        let is_cutoff = self.node_state.get(&target_rhs) == Some(&Status::Cutoff);
        let target_depth = *self.node_depth.get(&target_rhs).unwrap_or(&0) as isize;
        let parent_depth = *self.depth_stack.last().unwrap();

        if is_cutoff
            && target_depth > parent_depth
            && let Some(rule) = self.grammar.rulemap.get_mut(target_name)
        {
            rule.set_left_recursive();
            for name in self.rule_name_stack.iter() {
                if let Some(rule) = self.grammar.rulemap.get_mut(name) {
                    rule.set_no_memo();
                }
            }
        }

        if is_leftrec {
            self.depth_stack.pop();
            self.rule_name_stack.pop();
        }
    }
}
