use std::mem;

// Inside your transformation visitor
if let Element::Include(rule_name) = &mut *node {
    if let Some(target_rule) = rulemap.get(rule_name) {
        // We take the current node, replace it with a temporary 'Void'
        // and get the actual target expression to put back in its place.
        let target_expression = target_rule.exp.clone();
        let _ = mem::replace(node, target_expression);
    }
}
