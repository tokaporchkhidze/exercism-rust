use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    const MINUTES_PER_HOUR: i32 = 60;
    const MINUTES_PER_DAY: i32 = 24 * 60;
    const MIDNIGHT: i32 = 24;

    fn format_total_minutes(normalized_total_minutes: i32) -> (i32, i32) {
        let mut hours = normalized_total_minutes / Self::MINUTES_PER_HOUR;
        if hours == Self::MIDNIGHT {
            hours = 0;
        }
        let minutes = normalized_total_minutes % Self::MINUTES_PER_HOUR;
        (hours, minutes)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let normalized_total_minutes = (hours * Self::MINUTES_PER_HOUR + minutes).rem_euclid(Self::MINUTES_PER_DAY);
        let (hours,minutes) = Self::format_total_minutes(normalized_total_minutes);
        Self {
            hours,
            minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let normalized_total_minutes = (self.hours * Self::MINUTES_PER_HOUR + self.minutes + minutes).rem_euclid(Self::MINUTES_PER_DAY);
        let (hours, minutes) = Self::format_total_minutes(normalized_total_minutes);
        Self {
            hours,
            minutes
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
