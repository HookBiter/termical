use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::{component::Component, daily::Daily, event::Event};
use crate::input::input::Input;

pub struct Weekly {
    days: [Daily; 7],
}

impl Weekly {
    pub fn new(days: [Vec<Event>; 7]) -> Self {
        let mut days = days.to_vec();
        days.reverse();
        return Self {
            days: [
                Daily::new(days.pop().unwrap()),
                Daily::new(days.pop().unwrap()),
                Daily::new(days.pop().unwrap()),
                Daily::new(days.pop().unwrap()),
                Daily::new(days.pop().unwrap()),
                Daily::new(days.pop().unwrap()),
                Daily::new(days.pop().unwrap()),
            ],
        };
    }
}

impl Component for Weekly {
    fn get_focused(&self) -> Option<Box<&dyn Component>> {
        for day in self.days.iter() {
            let comp = day.get_focused();
            if comp.is_some() {
                return comp;
            }
        }
        return None;
    }
    fn handle_input(&mut self, input: Option<Input>) -> Option<Input> {
        todo!();
    }
}

impl Widget for Weekly {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let day_width = area.width / 7;
        let day_rects: [Rect; 7] = [
            Rect {
                x: area.x,
                y: area.y,
                width: day_width,
                height: area.height,
            },
            Rect {
                x: area.x + day_width,
                y: area.y,
                width: day_width,
                height: area.height,
            },
            Rect {
                x: area.x + 2 * day_width,
                y: area.y,
                width: day_width,
                height: area.height,
            },
            Rect {
                x: area.x + 3 * day_width,
                y: area.y,
                width: day_width,
                height: area.height,
            },
            Rect {
                x: area.x + 4 * day_width,
                y: area.y,
                width: day_width,
                height: area.height,
            },
            Rect {
                x: area.x + 5 * day_width,
                y: area.y,
                width: day_width,
                height: area.height,
            },
            Rect {
                x: area.x + 6 * day_width,
                y: area.y,
                width: day_width,
                height: area.height,
            },
        ];
        for (i, day) in self.days.into_iter().enumerate() {
            let rect = day_rects.get(i).unwrap();
            day.render(*rect, buf);
        }
    }
}
