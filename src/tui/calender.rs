use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::{component::Component, weekly::Weekly};

use crate::input::input::Input;

pub struct Calender {
    //views: Vec<Box<dyn Component>>,
    //months_view: (),
    weekly_view: Weekly,
}

impl Calender {
    pub fn new() -> Self {
        // TODO: add views
        return Self {
            weekly_view: Weekly::new([vec![], vec![], vec![], vec![], vec![], vec![], vec![]]),
        };
    }
}

impl Component for Calender {
    fn get_focused(&self) -> Option<Box<&dyn Component>> {
        let comp = self.weekly_view.get_focused();
        if comp.is_some() {
            return comp;
        }
        return None;
    }
    fn handle_input(&mut self, input: Option<Input>) -> Option<Input> {
        match input {
            Some(Input::Quit) => {}
            Some(_) => {}
            None => {}
        }
        // TODO: implement
        todo!();
    }
}

impl Widget for Calender {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let months_view_width = (area.width / 10) * 2;
        let width_of_weekly_view = (area.width / 10) * 8;
        let weekly_view_area = Rect::new(
            area.x + months_view_width,
            area.y,
            width_of_weekly_view,
            area.height,
        );
        self.weekly_view.render(weekly_view_area, buf);
    }
}
