use super::years::is_leap_year;

pub enum Months {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

/// Returns the number of days of a month in a year
///
/// * `month` - of the year to get number of days of
/// * `year` - to account for leap years
pub fn number_of_days_in_the_month(month: &Months, year: &i32) -> u8 {
    match month {
        Months::January
        | Months::March
        | Months::May
        | Months::July
        | Months::August
        | Months::October
        | Months::December => return 31,
        Months::April | Months::June | Months::September | Months::November => return 30,
        Months::February => {
            if is_leap_year(year) {
                return 29;
            } else {
                return 28;
            }
        }
    }
}

/// Returns a month enum value from a unsigned integer month number
pub fn number_to_month(month: &u8) -> Option<Months> {
    match month {
        1 => Some(Months::January),
        2 => Some(Months::February),
        3 => Some(Months::March),
        4 => Some(Months::April),
        5 => Some(Months::May),
        6 => Some(Months::June),
        7 => Some(Months::July),
        8 => Some(Months::August),
        9 => Some(Months::September),
        10 => Some(Months::October),
        11 => Some(Months::November),
        12 => Some(Months::December),
        _ => None,
    }
}
