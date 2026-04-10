use tiexiu::input::StrCursor;
use tiexiu::peg::{Exp, ExpKind, Grammar};
use tiexiu::state::corectx::CoreCtx;

pub fn check_exp_for_unlinked(exp: &Exp, path: &str, grammar: &Grammar) {
    match &exp.kind {
        ExpKind::Call { name, rule } => {
            if rule.is_none() {
                println!("  {}: Call to '{}' is NOT linked", path, name);
                match grammar.get_rule(name) {
                    Ok(r) => println!(
                        "    BUT '{}' exists in grammar as rule '{}'",
                        name, r.meta.name
                    ),
                    Err(_) => println!("    AND '{}' does NOT exist in grammar", name),
                }
            }
        }
        ExpKind::Sequence(items) => {
            for (i, item) in items.iter().enumerate() {
                check_exp_for_unlinked(item, &format!("{}[{}]", path, i), grammar);
            }
        }
        ExpKind::Choice(options) => {
            for (i, opt) in options.iter().enumerate() {
                check_exp_for_unlinked(opt, &format!("{}[{}]", path, i), grammar);
            }
        }
        ExpKind::Alt(inner) => {
            check_exp_for_unlinked(inner, &format!("{}:alt", path), grammar);
        }
        ExpKind::Named(_, inner) | ExpKind::NamedList(_, inner) => {
            check_exp_for_unlinked(inner, path, grammar);
        }
        ExpKind::Optional(inner)
        | ExpKind::Group(inner)
        | ExpKind::Lookahead(inner)
        | ExpKind::NegativeLookahead(inner)
        | ExpKind::SkipTo(inner)
        | ExpKind::Closure(inner)
        | ExpKind::PositiveClosure(inner)
        | ExpKind::SkipGroup(inner)
        | ExpKind::Override(inner)
        | ExpKind::OverrideList(inner) => {
            check_exp_for_unlinked(inner, path, grammar);
        }
        ExpKind::Join { exp: e1, sep }
        | ExpKind::PositiveJoin { exp: e1, sep }
        | ExpKind::Gather { exp: e1, sep }
        | ExpKind::PositiveGather { exp: e1, sep } => {
            check_exp_for_unlinked(e1, &format!("{}.exp", path), grammar);
            check_exp_for_unlinked(sep, &format!("{}.sep", path), grammar);
        }
        ExpKind::RuleInclude { name: ri_name, exp } => {
            if exp.is_none() {
                println!("  {}: RuleInclude '{}' is NOT resolved", path, ri_name);
                match grammar.get_rule(ri_name) {
                    Ok(r) => println!("    Rule '{}' exists in grammar", r.meta.name),
                    Err(_) => println!("    AND '{}' does NOT exist in grammar", ri_name),
                }
            }
        }
        _ => {}
    }
}

#[test]
#[ignore]
fn test_linker_debug() {
    let boot = tiexiu::json::boot::boot_grammar().expect("Failed to load boot grammar");

    println!("=== Checking boot grammar structure ===\n");

    println!("Rules in boot grammar:");
    for rule in boot.rules() {
        println!("  - {}", rule.meta.name);
    }

    println!("\n=== Checking key rules for linking issues ===\n");

    if let Ok(start_rule) = boot.get_rule("start") {
        println!("Checking 'start' rule:");
        check_exp_for_unlinked(&start_rule.exp, "start", &boot);
    }

    if let Ok(grammar_rule) = boot.get_rule("grammar") {
        println!("\nChecking 'grammar' rule:");
        check_exp_for_unlinked(&grammar_rule.exp, "grammar", &boot);
    }

    if let Ok(rule_rule) = boot.get_rule("rule") {
        println!("\nChecking 'rule' rule:");
        check_exp_for_unlinked(&rule_rule.exp, "rule", &boot);
    }

    println!("\n=== Trying to parse EBNF ===");
    let ebnf_text =
        std::fs::read_to_string("grammar/tatsu.tatsu").expect("Failed to read tatsu.tatsu");
    let cursor = StrCursor::new(&ebnf_text);
    let ctx = CoreCtx::new(cursor);

    match boot.parse(ctx) {
        Ok(_) => {
            println!("SUCCESS!");
        }
        Err(failure) => {
            println!("FAILED: {:?}", failure);
            println!("Error at mark: {}", failure.mark);
            println!("Error message: {:?}", failure.source);
            panic!("Parsing failed");
        }
    }
}
