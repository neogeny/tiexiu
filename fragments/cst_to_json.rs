use serde_json::{Map, Value};

/// Converts a Cst (and its nested Ast) into a JSON-compatible Value.
/// This acts as the boundary layer, requiring no Serialize traits on internal types.
pub fn cst_to_json(cst: &Cst) -> Value {
    match cst {
        // Basic terminal values
        Cst::Token(s) | Cst::Literal(s) => Value::String(s.to_string()),

        // Structural passthrough
        Cst::Item(child) => cst_to_json(child),

        // Iterables
        Cst::List(items) | Cst::Closure(items) => {
            let array = items.iter().map(|c| cst_to_json(c)).collect();
            Value::Array(array)
        }

        // Pre-transformed Structured Mapping
        Cst::Ast(ast) => {
            let mut obj = Map::new();
            for (key, value) in &ast.fields {
                obj.insert(key.clone(), cst_to_json(value));
            }
            Value::Object(obj)
        }

        // These exist in the enum but, as noted, are usually
        // folded into an Ast before reaching the boundary.
        Cst::Named(name, child) => {
            let mut obj = Map::new();
            obj.insert(name.to_string(), cst_to_json(child));
            Value::Object(obj)
        }

        Cst::NamedList(name, child) => {
            let mut obj = Map::new();
            obj.insert(name.to_string(), Value::Array(vec![cst_to_json(child)]));
            Value::Object(obj)
        }

        // Empty results
        Cst::Nil => Value::Null,
    }
}
