use std::usize;

use crate::calender::tui::event::Event;
use crate::utils::time::Time;
use chrono::Timelike;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    symbols::{border::ROUNDED, line::HORIZONTAL},
    widgets::{Block, Widget},
};

pub struct Daily {
    empty_color: Color,
    filled_color: Color,
    events: Vec<Event>,
}

impl Daily {
    pub fn new(mut events: Vec<Event>) -> Self {
        if !events.is_empty() {
            events.sort();
            /*
            let start = chrono::offset::Utc::now();
            let start = start
                - Duration::seconds(
                    (start.hour() * 60 * 60 + start.minute() * 60 + start.second()) as i64,
                );
            let start = Time::new(
                start.hour() as u8,
                start.minute() as u8,
                start.second() as u8,
            )
            .unwrap();
            */
            let start = Time::min();
            let first_event = events.get(0).unwrap().event.as_ref().unwrap();
            let first_event_time = Time::new(
                first_event.start.hour() as u8,
                first_event.start.minute() as u8,
                first_event.start.second() as u8,
            )
            .unwrap();
            if start < first_event_time {
                events.insert(0, Event::empty());
            }
            let mut reached: usize = 0;
            let tmp_events = events.to_vec();
            let events_start_len = events.len();
            let mut inserted: usize = 0;
            while reached < events_start_len {
                for (i, event) in tmp_events.iter().enumerate() {
                    if i < reached {
                        continue;
                    }
                    if event.event.is_none() {
                        continue;
                    }
                    if tmp_events.len() - 1 == i {
                        let last_event_end = event.event.as_ref().unwrap().end;
                        if Time::new(
                            last_event_end.hour() as u8,
                            last_event_end.minute() as u8,
                            last_event_end.second() as u8,
                        )
                        .unwrap()
                            < Time::max()
                        {
                            events.push(Event::empty());
                        }
                        reached = i + 1;
                        continue;
                    }
                    if event.event.as_ref().unwrap().end
                        < tmp_events.get(i + 1).unwrap().event.as_ref().unwrap().start
                    {
                        events.insert(i + 1 + inserted, Event::empty());
                        inserted += 1;
                        reached = i + 1;
                    }
                    if i > reached {
                        reached = i;
                    }
                }
            }
        }
        return Self {
            events,
            empty_color: Color::Gray,
            filled_color: Color::DarkGray,
        };
    }

    fn draw_background(&self, area: Rect, buf: &mut Buffer) {
        for x in area.x..area.x + area.width {
            for y in area.y..area.y + area.height {
                buf[(x, y)].set_bg(self.empty_color);
            }
        }
    }
    fn draw_hour_lines(&self, area: Rect, buf: &mut Buffer) {
        let hour_height = area.height / 24;
        for y in 0..24 {
            for x in area.x..area.x + area.width {
                buf[(x, y * hour_height)]
                    .set_symbol(HORIZONTAL)
                    .set_style(Color::DarkGray);
            }
        }
    }

    fn draw_events_blocks(&self, area: Rect, buf: &mut Buffer) {
        let block_height = area.height / self.events.len() as u16;
        for (i, event) in self.events.iter().enumerate() {
            let block_area = Rect {
                x: area.x,
                y: area.y + i as u16 * block_height,
                width: area.width,
                height: block_height,
            };
            event.clone().render(block_area, buf);
        }
    }
}

impl Widget for Daily {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().white().on_blue();
        block.border_set(ROUNDED).render(area, buf);
        let _hour_height = area.height / 24;
        self.draw_background(area, buf);
        //self.draw_hour_lines(area, buf);
        self.draw_events_blocks(area, buf);
    }
}