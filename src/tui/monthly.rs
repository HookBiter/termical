use chrono::{Datelike, Month, TimeZone, Weekday};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Widget},
};

use super::component::Component;
use crate::input::input::Input;

pub struct Monthly {
    first_day: Weekday,
    month: Month,
    is_marked: bool,
    marked_day: u8,
    number_of_days: u8,
    number_of_weeks: u8,
}

impl Monthly {
    pub fn new(month: chrono::Month, mut year: i32) -> Self {
        let date = chrono::Utc
            .with_ymd_and_hms(year, month.number_from_month(), 1, 1, 1, 1)
            .unwrap();
        if month == Month::December {
            year = year + 1;
        }
        let duration_of_month = chrono::Utc
            .with_ymd_and_hms(year, month.succ().number_from_month(), 1, 1, 1, 1)
            .unwrap()
            .signed_duration_since(date);
        let days_in_month = duration_of_month.num_days();
        let mut weeks_in_month = 1;
        let mut weekday = date.weekday();
        for _ in 1..days_in_month {
            weekday = weekday.succ();
            if weekday == Weekday::Sun {
                weeks_in_month += 1;
            }
        }
        return Self {
            first_day: date.weekday(),
            month,
            is_marked: false,
            marked_day: 0,
            number_of_days: days_in_month as u8,
            number_of_weeks: weeks_in_month as u8,
        };
    }
}
impl Component for Monthly {
    fn get_focused(&self) -> Option<Box<&dyn Component>> {
        if self.is_marked {
            return Some(Box::new(self));
        }
        return None;
    }
    fn handle_input(&mut self, input: Option<Input>) -> Option<Input> {
        todo!();
    }
}
impl Widget for Monthly {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let style = Style::default().on_dark_gray();
        let layout = Layout::vertical([
            Constraint::Length(Some(Style::default()).is_some().into()),
            Constraint::Length(Some(Style::default()).is_some().into()),
            Constraint::Fill(1),
        ]);
        let [month_header, weekday_header, days_area] = layout.areas(area);
        Line::styled(format!(" {}", self.month.name()), Style::default())
            .alignment(Alignment::Left)
            .render(month_header, buf);
        Span::styled(" Su Mo Tu We Th Fr Sa", Style::default()).render(weekday_header, buf);
        let day_width = days_area.width / 7;
        let day_height = (days_area.height / self.number_of_weeks as u16) / 3;
        let days_area = Rect::new(
            days_area.x,
            days_area.y,
            DAYS_RECT_WIDTH,
            self.number_of_weeks as u16,
        );
        let background_block = Block::new().on_dark_gray().render(days_area, buf);
        for week_num in 0..self.number_of_weeks {
            for weekday in 0..7 {
                let day = 7 * week_num as i64 + weekday as i64
                    - self.first_day.num_days_from_sunday() as i64
                    + 1;
                if day > 0 && day <= self.number_of_days as i64 {
                    /*let day_area = Rect::new(
                        days_area.x + weekday * day_width,
                        days_area.y + week_num as u16 * day_height,
                        day_width,
                        day_height,
                    );
                    */
                    let day_area = Rect::new(
                        days_area.x + weekday * 3 + 1,
                        days_area.y + week_num as u16,
                        3,
                        1,
                    );
                    let mut day_name = day.to_string();
                    if day_name.len() == 1 {
                        day_name = vec![String::from(" "), day_name].join("");
                    }
                    Span::styled(day_name, Style::default()).render(day_area, buf);
                }
            }
        }
    }
}

const DAYS_RECT_WIDTH: u16 = 7 * 3 + 1;
