use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;

pub type Space = i32;

#[derive(Debug, Default)]
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
            .flat_map(|&space| self.get(space))
            .collect::<Vec<Token>>();

        tokens.len() == 9
    }

    pub fn get(&self, space: Space) -> Option<Token> {
        self.state.borrow().get(&space).copied()
    }

    #[must_use]
    pub fn new() -> Self {
        Board {
            state: RefCell::new(HashMap::new()),
        }
    }
}
