#[macro_use]
extern crate linked_list_macro;
mod apple;
mod common;
mod game;
mod map;
mod snake;
use game::Reset;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

enum UpdateReason {
    Control(Key),
    Time,
}

fn main() {
    let (tx, rx): (Sender<UpdateReason>, Receiver<UpdateReason>) = mpsc::channel();

    // timer thread
    let timer_tx = tx.clone();
    let timer = std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        if timer_tx.send(UpdateReason::Time).is_err() {
            break;
        }
    });

    // thread for stdin events
    let stdin_tx = tx.clone();
    let stdin = std::thread::spawn(move || {
        let stdin = std::io::stdin();
        for c in stdin.keys() {
            if let Ok(r) = c {
                if stdin_tx.send(UpdateReason::Control(r)).is_err() {
                    break;
                }
            }
        }
    });

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    let size = match termion::terminal_size() {
        Ok(s) => s,
        Err(_) => (50, 50),
    };

    let mut game = game::Game::new(
        map::Map::new(size.0 as usize, size.1 as usize),
        snake::Snake::new(),
        apple::Apple::new(),
        &mut stdout,
    );

    game.tick().expect("Unideal initial conditions");

    loop {
        match rx.recv() {
            Ok(UpdateReason::Control(k)) => {
                let _ = match k {
                    Key::Char('c') => break,
                    Key::Char('q') => break,
                    Key::Left => game.turn_snake(common::Direction::Left),
                    Key::Right => game.turn_snake(common::Direction::Right),
                    Key::Up => game.turn_snake(common::Direction::Up),
                    Key::Down => game.turn_snake(common::Direction::Down),
                    _ => Ok(()),
                }
                .is_ok();
            }
            Ok(UpdateReason::Time) => match game.tick() {
                Ok(_) => {}
                Err(_) => game.reset(),
            },
            Err(_) => {}
        }
    }

    drop(game);
    drop(rx);

    println!("Thanks the game");

    stdin.join().unwrap();
    timer.join().unwrap();
}
