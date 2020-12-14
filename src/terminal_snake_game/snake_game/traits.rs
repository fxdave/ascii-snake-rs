use super::game::direction::Direction;
use super::game::traits::Draw;

/// Methods that a game should be able to do
pub trait Game: Draw {
    /// This function is called when a unit time has spent
    fn tick(&mut self);

    /// Turns the character
    fn turn_character(&mut self, direction: Direction);

    // Casts
    fn as_draw(&self) -> &dyn Draw;
}
