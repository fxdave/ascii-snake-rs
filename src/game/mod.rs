mod errors;
mod traits;
use super::common;
use std::io::Write;
use termion;

pub use errors::*;
pub use traits::*;

pub struct Game<'s, Map: MapTrait, Snake: SnakeTrait, Apple: AppleTrait<Snake>> {
    map: Map,
    snake: Snake,
    apple: Apple,
    stdout: &'s mut termion::raw::RawTerminal<std::io::Stdout>,
}

impl<'s, Map: MapTrait, Snake: SnakeTrait, Apple: AppleTrait<Snake>> Game<'s, Map, Snake, Apple> {
    pub fn new(
        map: Map,
        snake: Snake,
        mut apple: Apple,
        stdout: &'s mut termion::raw::RawTerminal<std::io::Stdout>,
    ) -> Game<Map, Snake, Apple> {
        let map_size = map.get_size();
        apple.spawn(&map_size, &snake);

        Game {
            map,
            snake,
            apple,
            stdout,
        }
    }

    pub fn tick(&mut self) -> Result<(), GameError> {
        let map_size = self.map.get_size();
        if self.can_eat() {
            self.snake.grow();
            self.apple.spawn(&map_size, &self.snake);
        }
        self.snake.step()?;
        self.draw();
        if !self.map.is_free_pos(self.snake.get_pos().clone()) {
            Err(GameError::KilledByWall)
        } else {
            Ok(())
        }
    }

    pub fn turn_snake(
        &mut self,
        direction: common::Direction,
    ) -> Result<(), SelfTurningDirectionError> {
        self.snake.set_direction(direction)?;
        self.draw();
        Ok(())
    }

    fn can_eat(&self) -> bool {
        self.apple.get_pos() == self.snake.get_pos()
    }

    fn draw(&mut self) {
        self.map.draw().flush(self.stdout);
        self.apple.draw().flush(self.stdout);
        self.snake.draw().flush(self.stdout);
        write!(self.stdout, "\n").expect("Couldn't write stdout");
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1)).expect("Couldn't write stdout");
        self.stdout.flush().expect("Couldn't flush stdout");
    }
}

impl<'s, Map: MapTrait, Snake: SnakeTrait, Apple: AppleTrait<Snake>> Reset
    for Game<'s, Map, Snake, Apple>
{
    fn reset(&mut self) {
        self.draw();
        self.snake.reset();

        let map_size = self.map.get_size();
        self.apple.spawn(&map_size, &self.snake);
    }
}
