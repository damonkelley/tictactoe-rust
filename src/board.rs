use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;

pub type Space = i32;

#[derive(Debug)]
pub struct Board<'a> {
    state: RefCell<HashMap<Space, Token<'a>>>,
}

impl<'a> Board<'a> {
    pub fn put(&self, space: Space, token: Token<'a>) -> &Self {
        self.state.borrow_mut().insert(space, token);
        self
    }

    pub fn full(&self) -> bool {
        let tokens = [1, 2, 3, 4, 5, 6, 7, 8, 9]
            .iter()
            .map(|&space| self.get(space))
            .flatten()
            .collect::<Vec<Token>>();

        return tokens.len() == 9;
    }

    pub fn get(&self, space: Space) -> Option<Token> {
        self.state.borrow().get(&space).map(|&token| token)
    }

    pub fn new() -> Self {
        Board {
            state: RefCell::new(HashMap::new()),
        }
    }
}
