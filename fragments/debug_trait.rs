use std::fmt;

pub struct User {
    pub name: String,
    pub api_key: String,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("api_key", &"***MASKED***") // Hide the actual key
            .finish()
    }
}
