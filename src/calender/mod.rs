pub mod calender;
pub mod event;

use ratatui::{
    crossterm::event::{KeyCode, KeyEventKind},
    DefaultTerminal,
};
use std::io::Result;
pub fn app() -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    return app_result;
}

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            //let widget = test_data();
            //frame.render_widget(widget, frame.area());
        })?;
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

/*
fn test_data() -> Weekly {
    /*
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
    */
    //testing
    let mut e = Event::new(EventData::new(
        "test",
        "test",
        Utc::now(),
        Utc::now() + Duration::hours(1),
    ));
    e.mark();
    let week = [
        Daily::new(vec![e]),
        Daily::new(vec![]),
        Daily::new(vec![]),
        Daily::new(vec![]),
        Daily::new(vec![]),
        Daily::new(vec![]),
        Daily::new(vec![]),
    ];
    //testing end
    return Weekly::new(week);
}
*/
