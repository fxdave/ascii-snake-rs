use super::snake_game::game::traits::Draw;
use super::update_reason::UpdateReason;

/// Methods that a game should be able to do
pub trait SnakeGame: Draw {
    /// This function is called when a relevant event is triggered
    fn update(&mut self, reason: UpdateReason);

    // Casts
    fn as_draw(&self) -> &dyn Draw;
}
