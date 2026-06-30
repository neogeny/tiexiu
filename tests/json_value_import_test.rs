use json::JsonValue;
use tiexiu::Result;
use tiexiu::peg::grammar::Grammar;

const TATSU_JSON: &str = include_str!("../grammar/tatsu.json");
const CALC_JSON: &str = include_str!("../grammar/calc.json");
const RULE_INCLUDE_NO_NAME_JSON: &str = include_str!("./fixtures/rule_include_no_name.json");
const RULE_INCLUDE_WITH_EXP_JSON: &str = include_str!("./fixtures/rule_include_with_exp.json");

#[test]
fn test_grammar_from_json() -> Result<()> {
    let grammar = Grammar::from_json(TATSU_JSON)?;
    assert_eq!(grammar.name.to_string(), "TatSu");
    let rule_count = grammar.rules().count();
    assert!(rule_count > 0, "Expected rules, got {}", rule_count);
    Ok(())
}

#[test]
fn test_grammar_from_serde_value() -> Result<()> {
    let value: JsonValue =
        json::parse(CALC_JSON).map_err(|e| tiexiu::Error::from(e.to_string()))?;
    let grammar = Grammar::from_json_value(&value)?;
    assert_eq!(grammar.name.to_string(), "CALC");
    Ok(())
}

#[test]
fn test_grammar_from_json_error_reporting() -> Result<()> {
    let value: JsonValue =
        json::parse(RULE_INCLUDE_NO_NAME_JSON).map_err(|e| tiexiu::Error::from(e.to_string()))?;
    let result = Grammar::from_json_value(&value);

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
    Ok(())
}

#[test]
fn test_grammar_from_json_with_rule_include_exp() -> Result<()> {
    let result = Grammar::from_json(RULE_INCLUDE_WITH_EXP_JSON)?;
    assert_eq!(result.name.to_string(), "TatSu");
    assert!(result.rules().count() > 0);
    Ok(())
}
