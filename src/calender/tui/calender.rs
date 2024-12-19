use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::{component::Component, weekly::Weekly};

pub struct Calender {
    views: Vec<Box<dyn Component>>,
}

impl Calender {
    pub fn new() -> Self {
        // TODO: add views
        return Self {
            views: vec![Box::new(Weekly::new())],
        };
    }
}

impl Component for Calender {
    fn get_focused(&self) -> Option<Box<&dyn Component>> {
        for view in self.views.iter() {
            let comp = view.get_focused();
            if comp.is_some() {
                return comp;
            }
        }
        return None;
    }
    fn handle_input(&mut self, input: Option<crossterm::event::Event>) {
        // TODO: implement
        todo!();
    }
}

impl Widget for Calender {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        todo!();
    }
}
