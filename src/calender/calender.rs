use crate::tui::Tui;
use std::io::Error;

pub struct Calender {
    file: String,
    tui: Tui,
}

impl Calender {
    pub fn new(file: &str) -> Self {
        return Self {
            file: String::from(file),
            tui: Tui::new(),
        };
    }

    pub fn load(&mut self) -> Result<(), Error> {
        let res = self.tui.start();
        self.tui.stop();
        return res;
    }
}
