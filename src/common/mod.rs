#[derive(PartialEq, Debug, Clone)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

pub type Size = Pos;

#[derive(PartialEq, Clone)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}
pub struct Directed<T>(pub Direction, pub T);
