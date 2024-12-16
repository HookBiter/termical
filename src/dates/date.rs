use super::months::{number_of_days_in_the_month, number_to_month, Months};

pub struct Date {
    day: u8,
    month: Months,
    year: i32,
}

impl Date {
    pub fn new(day: u8, month: u8, year: i32) -> Option<Self> {
        let month = number_to_month(&month);
        if month.is_none() {
            return None;
        }
        let month = month.unwrap();
        if day > number_of_days_in_the_month(&month, &year) {
            return None;
        }
        return Some(Self { day, month, year });
    }
}
