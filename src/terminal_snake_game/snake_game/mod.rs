mod apple;
pub mod game;
mod map;
mod snake;
mod traits;
use super::traits::SnakeGame as SnakeGameTrait;
use super::update_reason::{Control, UpdateReason};
use game::draw_instruction::DrawInstruction;
use game::traits::Draw;
use traits::*;

pub struct SnakeGame {
    game: Box<dyn Game>,
}

impl SnakeGame {
    pub fn new(size: (usize, usize)) -> SnakeGame {
        let mut game = Box::new(game::Game::new(
            Box::new(map::Map::new(size.0, size.1)),
            Box::new(snake::Snake::new()),
            Box::new(apple::Apple::new()),
        ));

        game.tick();

        SnakeGame { game }
    }
}

impl Draw for SnakeGame {
    fn draw(&self) -> Vec<DrawInstruction> {
        self.game.as_draw().draw()
    }
}

impl SnakeGameTrait for SnakeGame {
    fn update(&mut self, reason: UpdateReason) {
        match reason {
            UpdateReason::Control(Control::Turn(direction)) => self.game.turn_character(direction),
            UpdateReason::Time => self.game.tick(),
        }
    }

    fn as_draw(&self) -> &dyn Draw {
        self
    }
}
