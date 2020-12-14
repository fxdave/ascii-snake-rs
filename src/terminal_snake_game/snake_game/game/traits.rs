use super::draw_instruction::DrawInstruction;
pub use super::errors::*;
use super::{direction::Direction, vec2::Vec2};
use mockall::automock;

/// Objects that can draw themselves
pub trait Draw {
    /// Returns same draw instructions that can be run
    fn draw(&self) -> Vec<DrawInstruction>;
}

/// Objects that can reset themselves
pub trait Reset {
    /// Reset the state of the object
    fn reset(&mut self);
}

/// Objects that can check wheter the given position is free or not
#[automock]
pub trait IsFreePos {
    /// Returns true if the object does not take place on the given position
    /// Returns false otherwise
    fn is_free_pos(&self, pos: &Vec2) -> bool;
}

/// Methods that an Reward should be able to do
pub trait Food: Draw {
    /// Generates a random position inside the given boundary until getting a free position
    fn spawn(&mut self, boundary: &Vec2, position_checker: &dyn IsFreePos);

    /// Returns the position of the apple
    fn get_pos(&self) -> &Vec2;

    // Casts:
    fn as_draw(&self) -> &dyn Draw;
}

/// Methods that a character should be able to do
pub trait Character: Draw + IsFreePos + Reset {
    /// Grows the character somehow
    fn grow(&mut self);

    /// Turns the character
    fn turn(&mut self, direction: Direction) -> Result<(), SelfTurningDirectionError>;

    /// Moves character to the next position
    fn step(&mut self) -> Result<(), SelfEatingStepError>;

    /// Returns the position of the character's head
    fn get_head_pos(&self) -> &Vec2;

    // Returns whether the character's head is on the position or not
    fn can_eat(&self, pos: &Vec2) -> bool {
        self.get_head_pos() == pos
    }

    // Casts:
    fn as_draw(&self) -> &dyn Draw;
    fn as_is_free_pos(&self) -> &dyn IsFreePos;
    fn as_reset(&self) -> &dyn Reset;
}

/// Methods that a Map should be able to do
pub trait Map: Draw + IsFreePos {
    /// Returns the size of the map
    fn get_size(&self) -> Vec2;

    // Casts:
    fn as_draw(&self) -> &dyn Draw;
    fn as_is_free_pos(&self) -> &dyn IsFreePos;
}