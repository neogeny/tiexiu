#[test]
#[cfg(feature = "bootstrap")]
fn bootstrap_round_trip() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use tiexiu::{compile, input::StrCursor, parse, peg::Parser, state::corectx::CoreCtx};

    // Read the grammar text
    let grammar_text = r#"
            @@grammar :: Simple

            start: expression

            expression: number '+' number

            number: /\d+/
            "#
    .trim();

    // Load the boot grammar and sanity-check it has the `rule` production
    let boot = tiexiu::json::boot::boot_grammar().expect("Failed to load boot grammar");
    let rule_name = "start";
    assert!(
        boot.get_rule_id(rule_name).is_ok(),
        "boot grammar missing '{}'",
        rule_name
    );

    // Parse the grammar using the bootstrap grammar
    let tree1 = parse(grammar_text).expect("Failed to parse grammar with bootstrap");

    // Compile the parsed tree into a Grammar
    let grammar = compile(grammar_text).expect("Failed to compile grammar");

    // Now parse the grammar text again using the compiled grammar
    let cursor = StrCursor::new(grammar_text);
    let ctx = CoreCtx::new(cursor);
    let tree2 = match grammar.parse(ctx) {
        Ok(tiexiu::peg::S(_, tree)) => tree,
        Err(e) => panic!("Failed to parse with compiled grammar: {:?}", e),
    };

    // Assert the trees are equivalent by comparing hashes of their JSON representations.
    // Hash comparison avoids printing very large diffs in test failures.
    let json1 = tree1
        .to_json_string()
        .expect("Failed to serialize tree1 to JSON");
    let json2 = tree2
        .to_json_string()
        .expect("Failed to serialize tree2 to JSON");

    let mut h1 = DefaultHasher::new();
    json1.hash(&mut h1);
    let h1 = h1.finish();

    let mut h2 = DefaultHasher::new();
    json2.hash(&mut h2);
    let h2 = h2.finish();

    if h1 != h2 {
        panic!(
            "Round-trip parsing produced different trees (hash1={:016x} len1={} hash2={:016x} len2={})",
            h1,
            json1.len(),
            h2,
            json2.len()
        );
    }
}
