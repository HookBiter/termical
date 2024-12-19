use crate::calender::event::Event as Event_Data;
use std::cmp::Ordering;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{self, Color, Stylize},
    symbols::border::PLAIN,
    widgets::{Block, Widget},
};

use super::{component::Component, selectable::Selectable};

pub struct Event {
    is_marks: bool,
    pub event: Option<Event_Data>,
}

impl Event {
    pub fn new(event: Event_Data) -> Self {
        return Event {
            event: Some(event),
            is_marks: false,
        };
    }

    pub fn empty() -> Self {
        return Event {
            event: None,
            is_marks: false,
        };
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
            is_marks: self.is_marks.clone(),
            event: self.event.clone(),
        };
    }
}

impl Component for Event {
    fn get_focused(&self) -> Option<Box<&dyn Component>> {
        if self.is_marks {
            return Some(Box::new(self));
        }
        return None;
    }
    fn handle_input(&mut self, input: Option<crossterm::event::Event>) {
        todo!();
    }
}

impl Widget for Event {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let marks_border_color = Color::Magenta;
        let border_style = style::Style::new().fg(marks_border_color);
        let mut block = Block::bordered().border_set(PLAIN);
        if self.is_marks {
            block = block.border_style(border_style)
        }
        match self.event {
            Some(event) => {
                block.on_light_cyan().title(event.name).render(area, buf);
            }
            None => {
                block.on_dark_gray().title("Empty").render(area, buf);
            }
        }
    }
}

impl Selectable for Event {
    fn mark(&mut self) {
        self.is_marks = true;
    }
    fn unmark(&mut self) {
        self.is_marks = false;
    }
    fn select(&mut self) {}
    fn deselect(&mut self) {}
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
