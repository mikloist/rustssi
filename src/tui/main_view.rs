use std::{
    io::{self, Stdout},
};

use termion::{
    raw::{IntoRawMode, RawTerminal},
};

use tui::{backend::TermionBackend, Frame, Terminal};

type RawTerminalT = RawTerminal<Stdout>;
type BackendT = TermionBackend<RawTerminalT>;
type TerminalT = Terminal<BackendT>;

pub struct MainView {
    terminal: TerminalT,
    isRunning: bool,
}

impl MainView {
    pub fn new() -> io::Result<MainView> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(MainView {
            terminal: terminal,
            isRunning: false,
        })
    }

    fn main_loop(&mut self) {
        while self.isRunning {
            self.terminal.draw(MainView::draw_main_view);
        }
    }

    fn stop_running(&mut self) {
        self.isRunning = false;
    }

    fn draw_main_view(_f: &mut Frame<BackendT>) {}
}

/*
let stdout = io::stdout().into_raw_mode()?;
let backend = TermionBackend::new(stdout);
let mut terminal = Terminal::new(backend)?;
terminal.clear()?;
let mut ev = events();
loop {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default().title("Block 2").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })?;
    match ev.try_recv() {
        Ok(v) => {
            if v == Key::Char('q') {
                break;
            }
        }
        Err(mpsc::TryRecvError::Empty) => continue,
        Err(mpsc::TryRecvError::Disconnected) => break,
    }
}
terminal.clear()?;
return Ok(()); */
