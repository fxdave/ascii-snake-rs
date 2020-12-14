pub mod direction;
pub mod draw_instruction;
pub mod errors;
pub mod traits;
pub mod vec2;

use super::traits::Game as GameTrait;
use direction::Direction;
use traits::*;

pub struct Game {
    map: Box<dyn Map>,
    character: Box<dyn Character>,
    food: Box<dyn Food>,
}

impl Game {
    pub fn new(map: Box<dyn Map>, character: Box<dyn Character>, mut food: Box<dyn Food>) -> Game {
        food.spawn(&map.get_size(), character.as_is_free_pos());

        Game {
            map,
            character,
            food,
        }
    }

    fn step_character(&mut self) -> Result<(), GameError> {
        let map_size = self.map.get_size();
        if self.character.can_eat(self.food.get_pos()) {
            self.character.grow();
            self.food.spawn(&map_size, self.character.as_is_free_pos());
        }
        self.character.step()?;
        self.draw();

        if !self.map.is_free_pos(self.character.get_head_pos()) {
            Err(GameError::KilledByWall)
        } else {
            Ok(())
        }
    }
}

impl Draw for Game {
    fn draw(&self) -> Vec<draw_instruction::DrawInstruction> {
        let mut map = self.map.as_draw().draw();
        let food = self.food.as_draw().draw();
        let character = self.character.as_draw().draw();

        for instruction in food.into_iter().chain(character.into_iter()) {
            map.get_mut(instruction.pos.y)
                .expect("Map does not cover the food's positon")
                .shape
                .splice(
                    (instruction.pos.x)..(instruction.shape.len() + instruction.pos.x),
                    instruction.shape,
                );
        }

        map
    }
}

impl GameTrait for Game {
    fn tick(&mut self) {
        match self.step_character() {
            Ok(()) => (),
            Err(GameError::SelfEatingStepError(_)) => self.reset(),
            Err(GameError::KilledByWall) => self.reset(),
            Err(_) => self.reset(),
        };
    }

    fn turn_character(&mut self, direction: Direction) {
        match self.character.turn(direction) {
            Ok(()) => (),
            // on self turning direction, we do nothing
            Err(SelfTurningDirectionError) => (),
        };
        self.draw();
    }

    fn as_draw(&self) -> &dyn Draw {
        self
    }
}

impl Reset for Game {
    fn reset(&mut self) {
        self.draw();
        self.character.reset();

        let map_size = self.map.get_size();
        self.food.spawn(&map_size, self.character.as_is_free_pos());
    }
}
