// Tests for serde_json loading grammar/calc.json
const GRAMMAR_JSON: &str = include_str!("../grammar/tatsu.json");

#[test]
fn test_json_crate_parse_grammar() {
    let parsed: serde_json::Value = serde_json::from_str(GRAMMAR_JSON).unwrap();

    // Verify name field
    assert_eq!(parsed["name"], "TatSu");

    // Check rules exists
    assert!(parsed["rules"].is_array());

    assert!(parsed["directives"].is_object());
}

#[test]
fn test_json_crate_mutate_and_serialize() {
    let mut parsed: serde_json::Value = serde_json::from_str(GRAMMAR_JSON).unwrap();

    // Add missing fields
    parsed["analyzed"] = serde_json::json!(true);
    parsed["keywords"] = serde_json::json!([]);

    let output = serde_json::to_string(&parsed).unwrap();
    assert!(output.contains("\"analyzed\":true"));
    assert!(output.contains("\"keywords\":[]"));
}

#[test]
fn test_json_crate_serialize_then_parse() {
    let mut parsed: serde_json::Value = serde_json::from_str(GRAMMAR_JSON).unwrap();

    // Add missing fields
    parsed["analyzed"] = serde_json::json!(true);
    parsed["keywords"] = serde_json::json!([]);

    let modified = serde_json::to_string(&parsed).unwrap();

    // Parse the modified JSON
    let reparsed: serde_json::Value = serde_json::from_str(&modified).unwrap();
    assert_eq!(reparsed["name"], "TatSu");
    assert_eq!(reparsed["analyzed"], true);
}
