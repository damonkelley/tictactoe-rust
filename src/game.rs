pub trait Game {
    fn turn(&mut self) -> &mut Self;
}

trait Board {
    fn put(&mut self, space: Space, token: String) -> Result<String, String>;
}

type Space = i32;

trait Player {
    fn choose(&self) -> Space;
    fn token(&self) -> String;
}

type Players<'a> = Box<dyn Iterator<Item = &'a dyn Player> + 'a>;

struct TicTacToe<'a> {
    board: &'a mut dyn Board,
    players: Players<'a>,
}

impl<'a> Game for TicTacToe<'a> {
    fn turn(&mut self) -> &mut Self {
        self.players
            .next()
            .map(|player| self.board.put(player.choose(), player.token()));
        self
    }
}

#[allow(dead_code)]
impl<'a> TicTacToe<'a> {
    fn new(board: &'a mut impl Board, players: &'a [&'a dyn Player]) -> Self {
        TicTacToe {
            board,
            players: Box::new(players.iter().copied().cycle()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    struct TestPlayer<'a> {
        moves: RefCell<Box<dyn Iterator<Item = i32> + 'a>>,
    }

    impl<'a> TestPlayer<'a> {
        fn new(moves: &'a [i32]) -> Self {
            TestPlayer {
                moves: RefCell::new(Box::new(moves.iter().copied())),
            }
        }
    }

    impl<'a> Player for TestPlayer<'a> {
        fn choose(&self) -> Space {
            self.moves.borrow_mut().next().unwrap()
        }
        fn token(&self) -> String {
            String::from("X")
        }
    }

    struct TestBoard {
        moves: Vec<i32>,
    }

    impl Board for TestBoard {
        fn put(&mut self, space: Space, _token: String) -> Result<String, String> {
            self.moves.push(space);
            Ok(String::from("something"))
        }
    }

    impl TestBoard {
        fn new() -> Self {
            Self { moves: vec![] }
        }
    }

    #[test]
    fn turn_will_make_the_next_players_move() {
        let mut board = TestBoard::new();
        let player_one = TestPlayer::new(&[2, 5]);
        let player_two = TestPlayer::new(&[4, 3]);
        let players = vec![&player_one as &dyn Player, &player_two];

        TicTacToe::new(&mut board, &players)
            .turn()
            .turn()
            .turn()
            .turn();

        assert_eq!(board.moves, vec![2, 4, 5, 3])
    }
}
