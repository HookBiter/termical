use chrono::{Duration, Utc};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::{daily::Daily, event::Event, selectable::Selectable};
use crate::calender::event::Event as EventData;

pub struct Weekly {
    days: [Daily; 7],
}

impl Weekly {
    pub fn new() -> Self {
        //testing
        let mut e = Event::new(EventData::new(
            "test",
            "test",
            Utc::now(),
            Utc::now() + Duration::hours(1),
        ));
        e.mark();
        //testing end
        return Self {
            days: [
                Daily::new(vec![e]),
                Daily::new(vec![]),
                Daily::new(vec![]),
                Daily::new(vec![]),
                Daily::new(vec![]),
                Daily::new(vec![]),
                Daily::new(vec![]),
            ],
        };
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
