use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    state: RefCell<HashMap<i32, Token>>,
}

impl Board {
    pub fn put(&self, space: i32, token: Token) -> &Self {
        self.state.borrow_mut().insert(space, token);
        self
    }

    pub fn full(&self) -> bool {
        let tokens = [1, 2, 3, 4, 5, 6, 7, 8, 9]
            .iter()
            .map(|space| self.get(*space))
            .flatten()
            .collect::<Vec<Token>>();

        return tokens.len() == 9;
    }

    pub fn get(&self, space: i32) -> Option<Token> {
        self.state.borrow().get(&space).map(Token::from)
    }

    pub fn new() -> Self {
        Board {
            state: RefCell::new(HashMap::new()),
        }
    }
}
