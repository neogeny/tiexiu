use crate::model::{CanParse, ParseResult};
use crate::engine::Ctx;


#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub token: &'static str,
}

impl Token {
    pub fn new(token: &'static str) -> Self {
        Self {
            token,
        }
    }
}

impl CanParse for Token
{
    fn parse<'a>(&self, ctx: Ctx<'a>) -> ParseResult<'a> {
        ctx.token(&self.token)
    }
}


