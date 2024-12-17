use std::io::Result;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    widgets::Row,
    DefaultTerminal,
};

use crate::calender::event::Event as Event_data;
use crate::calender::tui::event::Event;

use super::daily::Daily;

pub fn weekly(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            let s = chrono::offset::Utc::now();
            let e = s + chrono::Duration::hours(1);
            let _rows = [
                Row::new(vec!["test1", "test2"]),
                Row::new(vec!["test3", "test4"]),
            ];
            let day = Daily::new(vec![Event::new(Event_data::new("event1", "ev1 des", s, e))]);
            //let day = Table::new(rows, [Constraint::Percentage(5), Constraint::Percentage(5)])
            //    .white()
            //    .on_blue();
            frame.render_widget(day, frame.area());
        })?;
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
