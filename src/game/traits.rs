pub use super::errors::*;
use mockall::automock;
use std::io::Write;
use termion;

use crate::common::{Direction, Pos, Size};
pub trait Spawn<T: IsFreePos> {
    fn spawn(&mut self, boundary: &Size, position_checker: &T);
}

pub trait Step {
    fn step(&mut self) -> Result<(), SelfEatingStepError>;
}

pub trait DirectionSet {
    fn set_direction(&mut self, direction: Direction) -> Result<(), SelfTurningDirectionError>;
}

pub trait Grow {
    fn grow(&mut self);
}

pub trait PositionGet {
    fn get_pos(&self) -> &Pos;
}

#[derive(Debug, PartialEq)]
pub struct DrawInstruction(pub Pos, pub String);

pub trait Flush {
    fn flush(&self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>);
}

pub trait Draw {
    fn draw(&self) -> Vec<DrawInstruction>;
}

pub trait Reset {
    fn reset(&mut self);
}

impl Flush for DrawInstruction {
    fn flush(&self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(self.0.x as u16 + 1, self.0.y as u16 + 1),
            self.1
        )
        .expect("Couldn't write stdout");
    }
}

impl Flush for Vec<DrawInstruction> {
    fn flush(&self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
        for i in self {
            i.flush(stdout);
        }
    }
}

#[automock]
pub trait IsFreePos {
    fn is_free_pos(&self, pos: Pos) -> bool;
}

pub trait SizeGet {
    fn get_size(&self) -> Size;
}

pub trait MapTrait: IsFreePos + Draw + SizeGet {}
pub trait SnakeTrait: Grow + Step + DirectionSet + PositionGet + Draw + IsFreePos + Reset {}
pub trait AppleTrait<T: IsFreePos>: Spawn<T> + PositionGet + Draw {}
