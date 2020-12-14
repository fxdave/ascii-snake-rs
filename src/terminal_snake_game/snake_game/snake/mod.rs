use super::game::{direction::Directed, direction::Direction, vec2::Vec2};
use super::game::{
    draw_instruction::{DrawInstruction, Symbol},
    errors::{SelfEatingStepError, SelfTurningDirectionError},
    traits::{Character, Draw, IsFreePos, Reset},
};
use std::collections::LinkedList;

pub struct Snake {
    body: LinkedList<Directed<Vec2>>,
    growing: bool,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: Self::get_new_body(),
            growing: false,
        }
    }

    fn get_new_body() -> LinkedList<Directed<Vec2>> {
        linked_list! {
            Directed(Direction::Right, Vec2 { x: 3, y: 3 }),
            Directed(Direction::Right, Vec2 { x: 4, y: 3 }),
            Directed(Direction::Down, Vec2 { x: 5, y: 3 }),
            Directed(Direction::Down, Vec2 { x: 5, y: 4 }),
        }
    }

    fn is_self_eating_step(&self, step: &Directed<Vec2>) -> bool {
        self.body
            .iter()
            .any(|i| i.1.x == step.1.x && i.1.y == step.1.y)
    }

    fn get_head(&self) -> &Directed<Vec2> {
        self.body.back().expect("The snake appears to be empty")
    }

    fn get_head_mut(&mut self) -> &mut Directed<Vec2> {
        self.body.back_mut().expect("The snake appears to be empty")
    }

    fn get_next_step(&self, last: &Directed<Vec2>) -> Directed<Vec2> {
        match last {
            Directed(Direction::Up, pos) => Directed(
                Direction::Up,
                Vec2 {
                    x: pos.x,
                    y: pos.y - 1,
                },
            ),
            Directed(Direction::Left, pos) => Directed(
                Direction::Left,
                Vec2 {
                    x: pos.x - 1,
                    y: pos.y,
                },
            ),
            Directed(Direction::Right, pos) => Directed(
                Direction::Right,
                Vec2 {
                    x: pos.x + 1,
                    y: pos.y,
                },
            ),
            Directed(Direction::Down, pos) => Directed(
                Direction::Down,
                Vec2 {
                    x: pos.x,
                    y: pos.y + 1,
                },
            ),
        }
    }
}

impl Character for Snake {
    fn step(&mut self) -> Result<(), SelfEatingStepError> {
        match self.growing {
            // In case of growing we don't delete the tail.
            true => {
                self.growing = false;
            }

            // In normal case we delete the tail.
            // This would look like a movement.
            false => {
                let _ = self.body.pop_front();
            }
        }

        let last_step = self.get_head();
        let new_step = self.get_next_step(last_step);

        if self.is_self_eating_step(&new_step) {
            Err(SelfEatingStepError)
        } else {
            self.body.push_back(new_step);
            Ok(())
        }
    }

    fn grow(&mut self) {
        self.growing = true
    }

    fn turn(&mut self, direction: Direction) -> Result<(), SelfTurningDirectionError> {
        let next_step = {
            let head = self.get_head();
            let last_step = Directed(direction.clone(), head.1.clone());
            self.get_next_step(&last_step)
        };
        match self.is_self_eating_step(&next_step) {
            true => Err(SelfTurningDirectionError),
            false => {
                self.get_head_mut().0 = direction;
                Ok(())
            }
        }
    }

    fn get_head_pos(&self) -> &Vec2 {
        &self.get_head().1
    }

    fn as_draw(&self) -> &dyn Draw {
        self
    }
    fn as_is_free_pos(&self) -> &dyn IsFreePos {
        self
    }
    fn as_reset(&self) -> &dyn Reset {
        self
    }
}

impl Draw for Snake {
    fn draw(&self) -> Vec<DrawInstruction> {
        let mut paint: Vec<DrawInstruction> = self
            .body
            .iter()
            .map(|part| DrawInstruction {
                pos: part.1.clone(),
                shape: vec![Directed(part.0.clone(), Symbol::SnakeBody)],
            })
            .collect();

        paint.last_mut().expect("Empty snake").shape.first_mut().unwrap().1 = Symbol::SnakeHead;

        paint
    }
}

impl IsFreePos for Snake {
    fn is_free_pos(&self, pos: &Vec2) -> bool {
        self.body.iter().find(|x| &x.1 == pos).is_none()
    }
}

impl Reset for Snake {
    fn reset(&mut self) {
        self.body = Self::get_new_body();
        self.growing = false;
    }
}

#[cfg(test)]
mod snake_test;
