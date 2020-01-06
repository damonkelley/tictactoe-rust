#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

trait RunContext {
    fn run(&self) -> bool;
}

trait Game {
    fn turn(&mut self) -> &mut Self;
}

#[allow(dead_code)]
fn play(game: &mut impl Game, context: &impl RunContext) {
    while context.run() {
        game.turn();
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

        play(
            &mut game,
            &TestContext {
                turns: RefCell::new(2),
            },
        );

        assert_eq!(game.log.borrow().clone(), ["turn", "turn"]);
    }
}
