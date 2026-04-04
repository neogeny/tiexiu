use tiexiu::{Grammar, Ctx}; // Assuming tiexiu is your crate name

#[test]
fn test_calc_expression() {
    // 1. Load the grammar (could use your include_str! or read from disk)
    let grammar_json = std::fs::read_to_string("grammar/calc.json")
        .expect("Failed to read calc.json");

    let grammar = Grammar::from_json(&grammar_json)
        .expect("Failed to bootstrap calc grammar");

    // 2. Set up a state with input
    let mut ctx = Ctx::new("2 + 2 * 3");

    // 3. Play!
    let result = grammar.parse(&mut ctx, "expression");

    assert!(result.is_ok());
    // ... further assertions on the CST ...
}
