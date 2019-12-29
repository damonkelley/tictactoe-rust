pub mod game;

use game::Game;

#[derive(Debug)]
struct GameLoop<'a> {
    game: &'a dyn Game,
    output: &'a dyn Output,
    presenter: &'a dyn Presenter,
}

trait Output: std::fmt::Debug {
    fn display(&self, message: &str);
}

trait Presenter: std::fmt::Debug {
    fn present(&self, game: &dyn Game) -> &str;
}

#[allow(dead_code)]
impl<'a> GameLoop<'a> {
    fn play(&self) {
        self.output.display(self.presenter.present(self.game));

        while self.game.outcome().is_none() {
            self.game.make_move();
            self.output.display(self.presenter.present(self.game));
        }

        self.game.outcome().map(|_| self.output.display("Winner"));
    }

    fn new(game: &'a dyn Game, output: &'a dyn Output, presenter: &'a dyn Presenter) -> Self {
        GameLoop {
            game,
            output,
            presenter,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::{Outcome, Token};
    use std::cell::*;

    #[derive(Debug)]
    struct FakeGame {
        log: RefCell<Vec<String>>,
    }

    impl Game for FakeGame {
        fn outcome(&self) -> Option<Outcome> {
            match self.log().len() < 2 {
                true => None,
                false => Some(Outcome::Winner(Token::new(&"X"))),
            }
        }

        fn make_move(&self) -> &dyn Game {
            self.log.borrow_mut().push(String::from("make-move"));
            self
        }
    }

    impl FakeGame {
        pub fn log(&self) -> Vec<String> {
            self.log.borrow().clone()
        }
    }

    #[derive(Debug)]
    struct FakeOutput {
        log: RefCell<Vec<String>>,
    }

    impl Output for FakeOutput {
        fn display(&self, message: &str) {
            self.log.borrow_mut().push(String::from(message))
        }
    }

    impl FakeOutput {
        fn log(&self) -> Vec<String> {
            self.log.borrow().clone()
        }
    }

    #[derive(Debug)]
    struct TestPresenter {}

    impl Presenter for TestPresenter {
        fn present(&self, _game: &dyn Game) -> &str {
            "presented"
        }
    }

    #[test]
    fn it_will_make_moves_until_the_game_is_over() {
        let game = FakeGame {
            log: RefCell::new(Vec::new()),
        };

        let output = FakeOutput {
            log: RefCell::new(Vec::new()),
        };

        GameLoop::new(&game, &output, &TestPresenter {}).play();

        assert_eq!(game.log().join(" "), "make-move make-move")
    }

    #[test]
    fn it_will_output_the_winner() {
        let game = FakeGame {
            log: RefCell::new(Vec::new()),
        };

        let output = FakeOutput {
            log: RefCell::new(Vec::new()),
        };

        GameLoop::new(&game, &output, &TestPresenter {}).play();

        assert!(output.log().contains(&String::from("Winner")))
    }

    #[test]
    fn it_will_display_the_board() {
        let game = FakeGame {
            log: RefCell::new(Vec::new()),
        };

        let output = FakeOutput {
            log: RefCell::new(Vec::new()),
        };

        GameLoop::new(&game, &output, &TestPresenter {}).play();

        assert_eq!(
            output.log().join(" "),
            "presented presented presented Winner"
        )
    }
}
