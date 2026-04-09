#[test]
#[cfg(feature = "bootstrap")]
fn test_debug_boot_grammar() {
    let boot = tiexiu::json::boot::boot_grammar().expect("Failed to load boot grammar");
    println!("Boot grammar rules:");
    for rule in boot.rules() {
        println!("  - {}", rule.info.name);
    }

    println!("\nChecking if 'rule' is in index...");
    match boot.get_rule_id("rule") {
        Ok(id) => println!("  Found 'rule' at id {}", id),
        Err(e) => println!("  ERROR: {}", e),
    }

    println!("\nTrying to get 'rule' rule...");
    match boot.get_rule("rule") {
        Ok(r) => println!("  Found: {:?}", r),
        Err(e) => println!("  ERROR: {}", e),
    }

    println!("\nChecking start rule...");
    if let Ok(start_rule) = boot.get_rule("start") {
        println!("  start.exp = {:?}", start_rule.exp);
    }
}
