pub struct Str(pub Rc<str>);

impl Deref for Str {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

// Now this works perfectly:
fn print_len(s: &str) {
    println!("{}", s.len());
}

let my_str = Str(Rc::from("TieXiu"));
print_len(&my_str); // Coerces to &str
