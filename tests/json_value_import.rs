use serde_json::Value;
use tiexiu::peg::grammar::Grammar;

const TATSU_JSON: &str = include_str!("../grammar/tatsu.json");
const CALC_JSON: &str = include_str!("../grammar/calc.json");
const RULE_INCLUDE_NO_NAME_JSON: &str = include_str!("./fixtures/rule_include_no_name.json");
const RULE_INCLUDE_WITH_EXP_JSON: &str = include_str!("./fixtures/rule_include_with_exp.json");

#[test]
fn test_grammar_from_json() {
    let grammar = Grammar::serde_from_json(TATSU_JSON).expect("Failed to convert");
    assert_eq!(grammar.name, "TatSu");
    let rule_count = grammar.rules().count();
    assert!(rule_count > 0, "Expected rules, got {}", rule_count);
}

#[test]
fn test_grammar_from_serde_value() {
    let value: Value = serde_json::from_str(CALC_JSON).expect("Failed to parse JSON");
    let grammar = Grammar::from_serde_json_value(&value).expect("Failed to convert");
    assert_eq!(grammar.name, "CALC");
}

#[test]
fn test_grammar_from_json_error_reporting() {
    let value: Value =
        serde_json::from_str(RULE_INCLUDE_NO_NAME_JSON).expect("Failed to parse JSON");
    let result = Grammar::from_serde_json_value(&value);

    match result {
        Ok(g) => {
            println!(
                "Grammar created: {} with {} rules",
                g.name,
                g.rules().count()
            );
        }
        Err(e) => {
            println!("Error reported: {}", e);
            let err_str = e.to_string();
            assert!(
                err_str.contains("rules[9]") && err_str.contains("name"),
                "Error should include rule index and field. Got: {}",
                err_str
            );
        }
    }
}

#[test]
fn test_grammar_from_json_with_rule_include_exp() {
    let result = Grammar::serde_from_json(RULE_INCLUDE_WITH_EXP_JSON);
    match result {
        Ok(g) => {
            assert_eq!(g.name, "TatSu");
            assert!(g.rules().count() > 0);
        }
        Err(e) => panic!("Expected success, got error: {}", e),
    }
}
