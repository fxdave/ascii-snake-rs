use super::traits::*;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use termion::event::Key;
use termion::input::TermRead;

pub struct TerminalEventStream;

impl EventStream for TerminalEventStream {
    fn start() -> Receiver<Event> {
        let (tx, rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();

        Self::start_timer_thread(tx.clone());
        Self::start_stdin_thread(tx);

        rx
    }
}

impl TerminalEventStream {
    fn start_timer_thread(tx: Sender<Event>) {
        // timer thread
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
            if tx.send(Event::Time).is_err() {
                println!("Timer is down");
                break;
            }
        });
    }

    fn start_stdin_thread(tx: Sender<Event>) {
        // thread for stdin events
        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            for c in stdin.keys() {
                if let Ok(r) = c {
                    if tx.send(Event::Key(r)).is_err() {
                        println!("Stdin is down");
                        break;
                    }
                    if let Key::Char('q') = r {
                        break;
                    }
                }
            }
        });
    }
}
