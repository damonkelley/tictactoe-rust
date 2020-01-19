#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod game;
use game::Game;

trait RunContext {
    fn run(&self) -> bool;
}

trait UI {
    fn update(&mut self) -> &mut Self;
}

#[allow(dead_code)]
fn play(game: &mut impl Game, ui: &mut impl UI, context: &impl RunContext) {
    ui.update();
    while context.run() {
        game.turn();
        ui.update();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    struct TestContext {
        turns: RefCell<i32>,
    }

    impl RunContext for TestContext {
        fn run(&self) -> bool {
            if *self.turns.borrow() > 0 {
                *self.turns.borrow_mut() -= 1;
                true
            } else {
                false
            }
        }
    }

    struct TestUI {
        log: Vec<String>,
    }

    impl UI for TestUI {
        fn update(&mut self) -> &mut Self {
            self.log.push(String::from("update"));
            self
        }
    }

    struct TestGame {
        log: RefCell<Vec<String>>,
    }

    impl Game for TestGame {
        fn turn(&mut self) -> &mut Self {
            self.log.borrow_mut().push(String::from("turn"));
            self
        }
    }

    #[test]
    fn it_will_take_turns_in_the_run_context() {
        let mut game = TestGame {
            log: RefCell::new(vec![]),
        };

        let mut ui = TestUI { log: vec![] };

        play(
            &mut game,
            &mut ui,
            &TestContext {
                turns: RefCell::new(2),
            },
        );

        assert_eq!(game.log.borrow().clone(), ["turn", "turn"]);
    }

    #[test]
    fn it_will_display_game_to_the_user() {
        let mut game = TestGame {
            log: RefCell::new(vec![]),
        };

        let mut ui = TestUI { log: vec![] };

        play(
            &mut game,
            &mut ui,
            &TestContext {
                turns: RefCell::new(2),
            },
        );

        assert_eq!(ui.log.join(" "), "update update update");
    }
}
