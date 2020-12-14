#[macro_use]
extern crate linked_list_macro;
mod terminal_snake_game;

fn main() {
    terminal_snake_game::TerminalSnakeGame::new().main();
}
