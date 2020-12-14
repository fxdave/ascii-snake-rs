use super::snake_game::game::direction::Direction;
pub enum Control {
    Turn(Direction),
}

pub enum UpdateReason {
    Control(Control),
    Time,
}
