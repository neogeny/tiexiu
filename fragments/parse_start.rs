pub fn parse_start<C: Cursor>(
    start_rule_name: &str,
    input: C,
    grammar: Arc<Grammar<C>>
) -> ParseResult<C> {
    // 1. Prepare the lookup map (the librarian's catalog)
    let rule_map = grammar.to_rule_map();

    // 2. Create the Context
    let ctx = Ctx::new(input, rule_map);

    // 3. Find the entry point
    let start_rule = grammar.find(start_rule_name).expect("Entry rule not found");

    // 4. Begin the recursive descent
    start_rule.parse(ctx)
}
