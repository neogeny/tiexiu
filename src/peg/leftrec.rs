use super::{Exp, Grammar};
use crate::peg::exp::ParserExp;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    First,
    Cutoff,
    Visited,
}

struct Analyzer<'a> {
    grammar: &'a mut Grammar,
    node_state: HashMap<*const Exp, State>,
    node_depth: HashMap<*const Exp, usize>,
    depth_stack: Vec<isize>,
    depth: usize,
}

impl<'a> Analyzer<'a> {
    fn new(grammar: &'a mut Grammar) -> Self {
        Self {
            grammar,
            node_state: HashMap::new(),
            node_depth: HashMap::new(),
            depth_stack: vec![-1],
            depth: 0,
        }
    }

    fn dfs(&mut self, node: &Exp) {
        let ptr = node as *const Exp;

        // if node_state[node] != State.FIRST: return
        if *self.node_state.get(&ptr).unwrap_or(&State::First) != State::First {
            return;
        }

        // node_state[node] = State.CUTOFF
        self.node_state.insert(ptr, State::Cutoff);

        // leftrec = isinstance(node, Rule) -- In your peg, Rule is the RHS or accessed via Call
        // Since we are traversing Elements, we check if this element is a "Call"
        // to treat it like the start of a Rule logic.
        let mut is_rule: bool = false;
        if let ParserExp::Call(name, _) = &node.exp {
            let thisname: String = name.deref().into();
            if self
                .grammar
                .rulemap
                .get(&thisname)
                .is_some_and(|r| r.is_left_recursive())
            {
                self.depth_stack.push(self.depth as isize);
                is_rule = true;
            }
        }

        self.node_depth.insert(ptr, self.depth);
        self.depth += 1;

        // try:
        for child in node.callable_from() {
            self.dfs(child);

            // afterEdge
            let child_ptr = child as *const Exp;
            let child_state = *self.node_state.get(&child_ptr).unwrap_or(&State::First);
            let child_depth = *self.node_depth.get(&child_ptr).unwrap_or(&0) as isize;

            if child_state == State::Cutoff && child_depth > *self.depth_stack.last().unwrap() {
                // This is a cycle. We need to mark the rule and turn off memoization.
                if let ParserExp::Call(target_name, _) = &child.exp {
                    self.grammar.mark_as_lrec(target_name);

                    // Python: child_rules = (n for n in node_depth if isinstance(n, Rule))
                    // We invalidate memoization for all rules currently "on the line"
                    for n_ptr in self.node_depth.keys() {
                        self.grammar.set_no_memo_for(*n_ptr);
                    }
                }
            }
        }

        // finally: (afterNode)
        if is_rule {
            self.depth_stack.pop();
        }
        self.node_depth.remove(&ptr);
        self.depth -= 1;
        self.node_state.insert(ptr, State::Visited);
    }
}

// Helper methods on Grammar to handle the mutation without locking Rule objects
impl Grammar {
    pub fn mark_left_recursion(&mut self) {
        // Reset status
        for rule in &mut self.rules {
            rule.reset_left_recursion();
        }

        let mut analyzer = Analyzer::new(self);

        // We must collect the references first to avoid borrowing self.rules while analyzer holds &mut self
        let roots: Vec<*const Exp> = analyzer
            .grammar
            .rules
            .iter()
            .map(|r| &r.exp as *const Exp)
            .collect();

        for ptr in roots {
            unsafe { analyzer.dfs(&*ptr) };
        }
    }

    fn mark_as_lrec(&mut self, name: &str) {
        if let Some(rule) = self.rulemap.get_mut(name) {
            rule.set_left_recursive();
        }
    }

    fn set_no_memo_for(&mut self, ptr: *const Exp) {
        // Find which rule owns this RHS pointer and kill its memo
        for rule in &mut self.rules {
            if std::ptr::eq(&rule.exp, ptr) {
                rule.set_no_memo();
            }
        }
    }
}
