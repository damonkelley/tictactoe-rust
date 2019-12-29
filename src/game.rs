use crate::board::{Board, Space};
use crate::token::Token;

pub trait Game: std::fmt::Debug {
    fn outcome(&self) -> Option<Outcome>;
    fn make_move(&self) -> &dyn Game;
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Winner(Token),
    Draw,
}

#[derive(Debug)]
struct TicTacToe {
    board: Board,
}

impl Game for TicTacToe {
    fn outcome(&self) -> Option<Outcome> {
        match self.find_winner() {
            Some(token) => Some(Outcome::Winner(token)),
            None if self.board.full() => Some(Outcome::Draw),
            None => None,
        }
    }

    fn make_move(&self) -> &dyn Game {
        return self;
    }
}

type Combination = Vec<Space>;
type Combinations = Vec<Combination>;

impl TicTacToe {
    pub fn new(board: Board) -> TicTacToe {
        TicTacToe { board }
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

        winning_combination.and_then(|combo| combo.first().map(Token::from))
    }

    fn find_uniq_tokens(&self, spaces: &Combination) -> Vec<Token> {
        let mut tokens = spaces
            .iter()
            .map(|space| self.board.get(*space))
            .flatten()
            .collect::<Vec<Token>>();

        tokens.dedup();

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome_will_be_none_when_the_game_is_in_progress() {
        assert_eq!(TicTacToe::new(Board::new()).outcome(), None)
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
            let board = Board::new();

            combination.iter().for_each(|space| {
                board.put(*space, Token::new(&"X"));
            });

            assert_eq!(
                TicTacToe::new(board).outcome(),
                Some(Outcome::Winner(Token::new(&"X")))
            )
        }
    }

    #[test]
    fn outcome_will_be_a_tie_when_there_is_no_winner_and_no_more_spaces() {
        let board = Board::new();

        board
            .put(1, Token::new(&"X"))
            .put(2, Token::new(&"O"))
            .put(3, Token::new(&"X"))
            .put(4, Token::new(&"X"))
            .put(5, Token::new(&"O"))
            .put(6, Token::new(&"X"))
            .put(7, Token::new(&"O"))
            .put(8, Token::new(&"X"))
            .put(9, Token::new(&"O"));

        assert_eq!(TicTacToe::new(board).outcome(), Some(Outcome::Draw))
    }
}
