#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug, PartialEq)]
pub struct Directed<T>(pub Direction, pub T);
