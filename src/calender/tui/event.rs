use crate::calender::event::Event as Event_Data;
use std::cmp::Ordering;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border::PLAIN,
    widgets::{Block, Widget},
};

pub struct Event {
    pub event: Option<Event_Data>,
}

impl Event {
    pub fn new(event: Event_Data) -> Self {
        return Event { event: Some(event) };
    }

    pub fn empty() -> Self {
        return Event { event: None };
    }

    pub fn is_empty(&self) -> bool {
        if self.event.is_none() {
            return true;
        }
        return false;
    }
}

impl Clone for Event {
    fn clone(&self) -> Self {
        return Self {
            event: self.event.clone(),
        };
    }
}

impl Widget for Event {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.event {
            Some(event) => {
                Block::bordered()
                    .on_cyan()
                    .title(event.name)
                    .border_set(PLAIN)
                    .render(area, buf);
            }
            None => {
                Block::bordered()
                    .on_dark_gray()
                    .title("Empty")
                    .border_set(PLAIN)
                    .render(area, buf);
            }
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.event.cmp(&other.event);
    }
}
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.event.partial_cmp(&other.event);
    }
}
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        return self.event.eq(&other.event);
    }
}
impl Eq for Event {}
