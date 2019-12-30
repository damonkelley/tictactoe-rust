#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Token<'a>(&'a str);

impl<'a> Token<'a> {
    pub fn new(value: &'a str) -> Token<'a> {
        Token(value)
    }
}
