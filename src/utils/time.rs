use std::cmp::Ordering;

pub struct Time {
    hour: u8,
    minutes: u8,
    seconds: u8,
}

impl Time {
    pub fn new(hour: u8, minutes: u8, seconds: u8) -> Option<Self> {
        if hour >= 24 || minutes >= 60 || seconds >= 60 {
            return None;
        }
        return Some(Self {
            hour,
            minutes,
            seconds,
        });
    }

    pub fn min() -> Self {
        return Self {
            hour: 0,
            minutes: 0,
            seconds: 0,
        };
    }
    pub fn max() -> Self {
        return Self {
            hour: 23,
            minutes: 59,
            seconds: 59,
        };
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hour == other.hour {
            if self.minutes == other.minutes {
                if self.seconds == other.seconds {
                    return Ordering::Equal;
                }
                if self.seconds < other.seconds {
                    return Ordering::Less;
                }
                return Ordering::Greater;
            }
            if self.minutes < other.minutes {
                return Ordering::Less;
            }
            return Ordering::Greater;
        }
        if self.hour < other.hour {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }
}
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}
impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        if self.cmp(other) == Ordering::Equal {
            return true;
        }
        return false;
    }
}
impl Eq for Time {}
