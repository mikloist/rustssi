use std::{io, sync::mpsc, thread};

use termion::{event::Key, input::TermRead};

pub enum UICommandType {
    Quit,
    Text,
}

struct UICommand {
    command_type: UICommandType,
    text: String,
}

impl UICommand {
    fn parse(text: String) -> UICommand {
        if text == "/quit" {
            return UICommand {
                command_type: UICommandType::Quit,
                text: text,
            };
        }
        return UICommand {
            command_type: UICommandType::Text,
            text: text,
        };
    }
}

struct InputHandler {
    event_receiver: mpsc::Receiver<Key>,
}

impl InputHandler {
    fn new() -> InputHandler {
        let rx = InputHandler::event_receiver();
        InputHandler { event_receiver: rx }
    }

    fn add_event_listener(&mut self) {}

    fn handle_input(&mut self, rx: &mpsc::Receiver<Key>) {
        loop {
            match rx.try_recv() {
                Ok(v) => {
                    if v == Key::Char('q') {
                        break;
                    }
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    break;
                }
            }
        }
    }

    fn event_receiver() -> mpsc::Receiver<Key> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let stdin = io::stdin();
            for evt in stdin.keys() {
                if let Ok(key) = evt {
                    if let Err(err) = tx.send(key) {
                        eprintln!("{}", err);
                        return;
                    }
                }
            }
        });
        rx
    }
}
