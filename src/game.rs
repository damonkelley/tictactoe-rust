use crate::board::{Board, Space};
use crate::token::Token;
use std::cell::RefCell;

use std::fmt;

pub trait Game: std::fmt::Debug {
    fn outcome(&self) -> Option<Outcome>;
    fn make_move(&self) -> &dyn Game;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Outcome<'a> {
    Winner(Token<'a>),
    Draw,
}

trait GetInput {
    fn get(&self) -> Option<Space>;
}

struct Turn<'a> {
    tokens: RefCell<Box<dyn Iterator<Item = Token<'a>> + 'a>>,
}

impl<'a> Turn<'a> {
    fn new(tokens: &'a Vec<Token<'a>>) -> Self {
        Turn {
            tokens: RefCell::new(Box::new(tokens.iter().copied().cycle())),
        }
    }

    fn next(&self) -> Option<Token<'a>> {
        self.tokens.borrow_mut().next()
    }
}

struct TicTacToe<'a> {
    board: &'a Board<'a>,
    get_input: &'a dyn GetInput,
    turn: Turn<'a>,
}

impl<'a> fmt::Debug for TicTacToe<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Game {{ board: {:?} }}", self.board)
    }
}

impl<'a> Game for TicTacToe<'a> {
    fn outcome(&self) -> Option<Outcome> {
        match self.find_winner() {
            Some(token) => Some(Outcome::Winner(token)),
            None if self.board.full() => Some(Outcome::Draw),
            None => None,
        }
    }

    fn make_move(&self) -> &dyn Game {
        let maybe_input = self.get_input.get();
        let maybe_token = self.turn.next();

        maybe_input.and_then(|space| maybe_token.map(|token| self.board.put(space, token)));

        return self;
    }
}

type Combination = Vec<Space>;
type Combinations = Vec<Combination>;

impl<'a> TicTacToe<'a> {
    pub fn new(
        board: &'a Board<'a>,
        get_input: &'a dyn GetInput,
        tokens: &'a Vec<Token<'a>>,
    ) -> TicTacToe<'a> {
        TicTacToe {
            board,
            get_input,
            turn: Turn::new(tokens),
        }
    }

    fn combinations(&self) -> Combinations {
        vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![3, 6, 9],
            vec![1, 5, 9],
            vec![3, 5, 7],
        ]
    }

    fn find_winner(&self) -> Option<Token> {
        let winning_combination = self
            .combinations()
            .iter()
            .map(|spaces| self.find_uniq_tokens(spaces))
            .find(|item| item.len() == 1);

        winning_combination.and_then(|combo| combo.first().map(|&combo| combo))
    }

    fn find_uniq_tokens(&self, spaces: &Combination) -> Vec<Token> {
        let mut tokens = spaces
            .iter()
            .map(|&space| self.board.get(space))
            .flatten()
            .collect::<Vec<Token>>();

        tokens.dedup();

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StubbedInput<'a> {
        moves: RefCell<Box<dyn Iterator<Item = Space> + 'a>>,
    }

    impl<'a> GetInput for StubbedInput<'a> {
        fn get(&self) -> Option<Space> {
            self.moves.borrow_mut().next()
        }
    }

    fn stubbed_input<'a>(moves: &'a Vec<Space>) -> StubbedInput<'a> {
        StubbedInput {
            moves: RefCell::new(Box::new(moves.iter().copied())),
        }
    }

    #[test]
    fn outcome_will_be_none_when_the_game_is_in_progress() {
        assert_eq!(
            TicTacToe::new(
                &Board::new(),
                &stubbed_input(&vec![1]),
                &vec![Token::new("X"), Token::new("O")]
            )
            .outcome(),
            None
        )
    }

    #[test]
    fn outcome_will_be_winner_when_the_game_has_a_winner() {
        let combinations = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
            [1, 4, 7],
            [2, 5, 8],
            [3, 6, 9],
            [1, 5, 9],
            [3, 5, 7],
        ];

        for combination in combinations.iter() {
            let moves = vec![1];
            let get_input = stubbed_input(&moves);
            let tokens = vec![Token::new("X"), Token::new("O")];
            let board = Board::new();

            combination.iter().for_each(|&space| {
                board.put(space, Token::new("X"));
            });

            assert_eq!(
                TicTacToe::new(&board, &get_input, &tokens).outcome(),
                Some(Outcome::Winner(Token::new("X")))
            );
        }
    }

    #[test]
    fn outcome_will_be_a_tie_when_there_is_no_winner_and_no_more_spaces() {
        let moves = vec![1];
        let get_input = stubbed_input(&moves);
        let tokens = vec![Token::new("X"), Token::new("O")];
        let board = Board::new();

        board
            .put(1, Token::new("X"))
            .put(2, Token::new("O"))
            .put(3, Token::new("X"))
            .put(4, Token::new("X"))
            .put(5, Token::new("O"))
            .put(6, Token::new("X"))
            .put(7, Token::new("O"))
            .put(8, Token::new("X"))
            .put(9, Token::new("O"));

        assert_eq!(
            TicTacToe::new(&board, &get_input, &tokens).outcome(),
            Some(Outcome::Draw)
        );
    }

    #[test]
    fn make_move_will_place_a_token_on_the_board() {
        let moves = vec![1, 2, 3, 4, 5];
        let get_input = stubbed_input(&moves);
        let tokens = vec![Token::new("X"), Token::new("O")];

        let board = Board::new();

        let game = TicTacToe::new(&board, &get_input, &tokens);

        (1..=5).map(|_| game.make_move()).for_each(drop);

        assert_eq!(
            board.get(1),
            Some(Token::new("X")),
            "It will make the first move"
        );
        assert_eq!(
            board.get(2),
            Some(Token::new("O")),
            "It will make the second move"
        );
        assert_eq!(
            board.get(3),
            Some(Token::new("X")),
            "It will make the third move"
        );
        assert_eq!(
            board.get(4),
            Some(Token::new("O")),
            "It will make the fourth move"
        );
        assert_eq!(
            board.get(5),
            Some(Token::new("X")),
            "It will make the fifth move"
        );
    }
}
