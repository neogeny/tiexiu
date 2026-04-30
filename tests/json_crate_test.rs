// Tests for json crate loading grammar/java.json

#[test]
fn test_json_crate_parse_java_grammar() {
    let java_json = include_str!("../grammar/java.json");
    let parsed = json::parse(java_json).unwrap();

    // Verify name field
    assert_eq!(parsed["name"].as_str(), Some("Java"));

    // Check rules exists
    assert!(parsed["rules"].is_array());

    // Check directives is an array (not object like in tatsu.json)
    assert!(parsed["directives"].is_array());
}

#[test]
fn test_json_crate_mutate_and_serialize() {
    let java_json = include_str!("../grammar/java.json");
    let mut parsed = json::parse(java_json).unwrap();

    // Add missing fields
    parsed["analyzed"] = json::JsonValue::Boolean(true);
    parsed["keywords"] = json::JsonValue::new_array();

    // Verify the modified JSON contains added fields
    let output = parsed.dump();
    assert!(output.contains("\"analyzed\":true"));
    assert!(output.contains("\"keywords\":[]"));
}

#[test]
fn test_json_crate_serialize_then_parse() {
    let java_json = include_str!("../grammar/java.json");
    let mut parsed = json::parse(java_json).unwrap();

    // Add missing fields
    parsed["analyzed"] = json::JsonValue::Boolean(true);
    parsed["keywords"] = json::JsonValue::new_array();

    let modified = parsed.dump();

    // Parse the modified JSON
    let reparsed = json::parse(&modified).unwrap();
    assert_eq!(reparsed["name"].as_str(), Some("Java"));
    assert_eq!(reparsed["analyzed"].as_bool(), Some(true));
}
