use super::{direction::Directed, vec2::Vec2};

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Empty,
    Wall,
    Apple,
    SnakeBody,
    SnakeHead,
}

#[derive(Debug, PartialEq)]
pub struct DrawInstruction {
    pub pos: Vec2, 
    pub shape: Vec<Directed<Symbol>>,
}