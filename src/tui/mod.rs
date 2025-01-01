use std::io::{Error, Stdout};

use calender::Calender;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::event::{KeyCode, KeyEventKind},
    Terminal,
};

mod calender;
mod component;
mod daily;
mod event;
mod monthly;
mod selectable;
mod weekly;

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn new() -> Self {
        return Self {
            terminal: ratatui::init(),
        };
    }

    pub fn start(&mut self) -> Result<(), Error> {
        self.terminal.clear()?;
        loop {
            self.terminal.draw(|frame| {
                let calender = Calender::new();
                frame.render_widget(calender, frame.area());
            })?;
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
    pub fn stop(&self) {
        ratatui::restore();
    }
}
