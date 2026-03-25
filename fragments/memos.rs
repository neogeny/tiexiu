pub struct Ctx<'a> {
    pub rest: &'a str,
    // The "Secret" sauce for Left Recursion:
    pub memo: HashMap<(usize, RuleId), MemoEntry>,
}
