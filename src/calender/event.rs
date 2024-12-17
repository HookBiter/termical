use chrono::{DateTime, Utc};
use std::cmp::Ordering;

pub struct Event {
    pub name: String,
    pub description: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Event {
    pub fn new(name: &str, description: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        return Self {
            name: String::from(name),
            description: String::from(description),
            start,
            end,
        };
    }

    pub fn are_events_intersecting(e1: Event, e2: Event) -> bool {
        if e1.start >= e2.end || e1.end <= e2.start {
            return false;
        }
        return true;
    }
}

impl Clone for Event {
    fn clone(&self) -> Self {
        return Self {
            name: self.name.clone(),
            description: self.description.clone(),
            start: self.start.clone(),
            end: self.end.clone(),
        };
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.end < other.start {
            return Ordering::Less;
        }
        if self.start > other.end {
            return Ordering::Greater;
        }
        if self.start < other.start {
            return Ordering::Less;
        }
        if self.start > other.start {
            return Ordering::Greater;
        }
        if self.start == other.start {
            if self.end < other.end {
                return Ordering::Less;
            }
            if self.end > other.end {
                return Ordering::Greater;
            }
        }
        return Ordering::Equal;
    }
}
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        if self.cmp(other) == Ordering::Equal {
            return true;
        }
        return false;
    }
}
impl Eq for Event {}
