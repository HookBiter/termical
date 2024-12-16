/// Returns whether a year is a leap year
///
/// a leap year is year with 366 days instead of 365 days
/// https://en.wikipedia.org/wiki/Leap_year
/// calculation is based on:
/// https://learn.microsoft.com/en-us/office/troubleshoot/excel/determine-a-leap-year
///
/// * `year` - the year number
pub fn is_leap_year(year: &i32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year & 400 != 0 {
                return false;
            }
        }
    }
    return true;
}
