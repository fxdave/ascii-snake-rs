mod snake_game;
mod traits;
mod update_reason;
mod events;
use snake_game::game::{
    direction::{Directed, Direction},
    draw_instruction::Symbol,
};
use std::io::Write;
use termion::event::Key;
use termion::raw::IntoRawMode;
use traits::*;
use update_reason::{Control, UpdateReason};

pub struct TerminalSnakeGame {
    snake_game: Box<dyn SnakeGame>,
}

impl TerminalSnakeGame {
    pub fn new() -> TerminalSnakeGame {
        let size: (usize, usize) = match termion::terminal_size() {
            Ok((w, h)) => (w as usize, h as usize),
            Err(_) => (50, 50),
        };

        TerminalSnakeGame {
            snake_game: Box::new(snake_game::SnakeGame::new(size)),
        }
    }

    pub fn main(&mut self) {
        let event_stream = events::TerminalEventStream::start();

        let mut stdout = std::io::stdout().into_raw_mode().unwrap();
        write!(stdout, "{}", termion::cursor::Hide).unwrap();
        loop {
            match event_stream.recv().expect("Channel has stopped.") {
                Event::Time => self.snake_game.update(UpdateReason::Time),
                Event::Key(Key::Up) => self
                    .snake_game
                    .update(UpdateReason::Control(Control::Turn(Direction::Up))),
                Event::Key(Key::Down) => self
                    .snake_game
                    .update(UpdateReason::Control(Control::Turn(Direction::Down))),
                Event::Key(Key::Left) => self
                    .snake_game
                    .update(UpdateReason::Control(Control::Turn(Direction::Left))),
                Event::Key(Key::Right) => self
                    .snake_game
                    .update(UpdateReason::Control(Control::Turn(Direction::Right))),
                Event::Key(_) => break,
            };
            let paint = self.snake_game.draw();
            for instruction in paint {
                let chars: String = instruction
                    .shape
                    .iter()
                    .map(|directed_symbol| match directed_symbol {
                        Directed(Direction::Up, Symbol::SnakeHead) => "▲",
                        Directed(Direction::Left, Symbol::SnakeHead) => "◄",
                        Directed(Direction::Right, Symbol::SnakeHead) => "►",
                        Directed(Direction::Down, Symbol::SnakeHead) => "▼",
                        Directed(_, Symbol::Wall) => "#",
                        Directed(_, Symbol::SnakeBody) => "#",
                        Directed(_, Symbol::Empty) => " ",
                        Directed(_, Symbol::Apple) => "♥",
                    })
                    .collect();
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(
                        instruction.pos.x as u16 + 1,
                        instruction.pos.y as u16 + 1
                    ),
                    chars
                )
                .expect("Couldn't write stdout");
            }

            write!(stdout, "{}", termion::cursor::Goto(1, 1)).expect("Couldn't write stdout");
            stdout.flush().expect("Couldn't flush stdout");
        }

        write!(stdout, "{}{}", termion::cursor::Show, termion::clear::All).unwrap();
        stdout.flush().expect("Couldn't flush stdout");
    }
}
