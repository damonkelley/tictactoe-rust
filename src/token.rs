#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Token(String);

impl Token {
    pub fn new(value: &str) -> Token {
        Token(String::from(value))
    }

    pub fn from(token: &Token) -> Token {
        Token::new(&token.0)
    }
}
