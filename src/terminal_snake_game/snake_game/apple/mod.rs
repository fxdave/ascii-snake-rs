use super::game::{
    direction::{Directed, Direction},
    vec2::Vec2,
};
use super::game::{
    draw_instruction::{DrawInstruction, Symbol},
    traits::{Draw, Food, IsFreePos},
};
use rand::{thread_rng, Rng};

pub struct Apple {
    pub pos: Vec2,
}

impl Apple {
    pub fn new() -> Apple {
        Apple {
            pos: Vec2 { x: 0, y: 0 },
        }
    }
}

impl Food for Apple {
    fn spawn(&mut self, boundary: &Vec2, position_checker: &dyn IsFreePos) {
        let mut x: usize;
        let mut y: usize;

        while {
            x = thread_rng().gen_range(1, boundary.x - 2);
            y = thread_rng().gen_range(1, boundary.y - 2);
            !position_checker.is_free_pos(&Vec2 { x, y })
        } {}

        self.pos = Vec2 { x, y }
    }

    fn get_pos(&self) -> &Vec2 {
        &self.pos
    }

    fn as_draw(&self) -> &dyn Draw {
        self
    }
}

impl Draw for Apple {
    fn draw(&self) -> Vec<DrawInstruction> {
        let pos = self.pos.clone();
        let shape = vec![Directed(Direction::Up, Symbol::Apple)];
        vec![DrawInstruction { pos, shape }]
    }
}

#[cfg(test)]
mod apple_test;
