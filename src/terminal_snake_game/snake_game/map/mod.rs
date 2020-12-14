use super::game::{direction::{Directed, Direction}, vec2::Vec2};
use super::game::{
    draw_instruction::{DrawInstruction, Symbol},
    traits::{Draw, IsFreePos, Map as MapTrait},
};
use std::mem;

#[derive(Clone)]
enum MapElement {
    Empty,
    Block,
}

pub struct Map {
    content: Vec<Vec<MapElement>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut content = vec![vec![MapElement::Empty; width]; height];

        for line in &mut content {
            let _ = mem::replace(line.first_mut().unwrap(), MapElement::Block);
            let _ = mem::replace(line.last_mut().unwrap(), MapElement::Block);
        }
        let _ = mem::replace(content.first_mut().unwrap(), vec![MapElement::Block; width]);
        let _ = mem::replace(content.last_mut().unwrap(), vec![MapElement::Block; width]);

        Map { content }
    }
}

impl IsFreePos for Map {
    fn is_free_pos(&self, pos: &Vec2) -> bool {
        match self.content.get((&pos).y).and_then(|row| row.get((&pos).x)) {
            Some(MapElement::Block) => false,
            Some(MapElement::Empty) => true,
            None => panic!("Cell not found on {:?}", &pos),
        }
    }
}

impl Draw for Map {
    fn draw(&self) -> Vec<DrawInstruction> {
        let instructions = self
            .content
            .iter()
            .enumerate()
            .map(|(y, row)| {
                DrawInstruction {
                    pos: Vec2 { x: 0, y },
                    shape: row.iter()
                        .map(|el| match el {
                            MapElement::Empty => Directed(Direction::Up, Symbol::Empty),
                            MapElement::Block => Directed(Direction::Up, Symbol::Wall),
                        })
                        .collect(),
                }
            })
            .collect();

        instructions
    }
}

impl MapTrait for Map {
    fn get_size(&self) -> Vec2 {
        Vec2 {
            x: self.content.get(0).expect("Empty map").len(),
            y: self.content.len(),
        }
    }

    fn as_draw(&self) -> &dyn Draw {
        return self;
    }

    fn as_is_free_pos(&self) -> &dyn IsFreePos {
        return self;
    }
}

#[cfg(test)]
mod map_test;
