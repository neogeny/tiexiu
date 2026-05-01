// Tests for json crate loading grammar/java.json
const GRAMMAR_JSON: &str = include_str!("../grammar/tatsu.json");

#[test]
fn test_json_crate_parse_grammar() {
    let parsed = json::parse(GRAMMAR_JSON).unwrap();

    // Verify name field
    assert_eq!(parsed["name"].as_str(), Some("TatSu"));

    // Check rules exists
    assert!(parsed["rules"].is_array());

    assert!(parsed["directives"].is_object());
}

#[test]
fn test_json_crate_mutate_and_serialize() {
    let mut parsed = json::parse(GRAMMAR_JSON).unwrap();

    // Add missing fields
    parsed["analyzed"] = json::JsonValue::Boolean(true);
    parsed["keywords"] = json::JsonValue::new_array();

    let output = parsed.dump();
    assert!(output.contains("\"analyzed\":true"));
    assert!(output.contains("\"keywords\":[]"));
}

#[test]
fn test_json_crate_serialize_then_parse() {
    let mut parsed = json::parse(GRAMMAR_JSON).unwrap();

    // Add missing fields
    parsed["analyzed"] = json::JsonValue::Boolean(true);
    parsed["keywords"] = json::JsonValue::new_array();

    let modified = parsed.dump();

    // Parse the modified JSON
    let reparsed = json::parse(&modified).unwrap();
    assert_eq!(reparsed["name"].as_str(), Some("TatSu"));
    assert_eq!(reparsed["analyzed"].as_bool(), Some(true));
}
