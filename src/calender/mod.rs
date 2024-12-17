mod event;
mod tui;

use std::io::Result;
pub fn app() -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = tui::weekly::weekly(terminal);
    ratatui::restore();
    return app_result;
}
