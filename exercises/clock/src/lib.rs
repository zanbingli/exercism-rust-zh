use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clock = Clock {
            minutes: hours * 60 + minutes,
        };
        if clock.minutes >= 1440 {
            clock.minutes = clock.minutes % 1440;
        }
        while clock.minutes < 0 {
            clock.minutes += 1440;
        }
        clock
    }

    pub fn add_minutes(&self, _minutes: i32) -> Self {
        let mut clock = Clock {
            minutes: self.minutes + _minutes,
        };
        if clock.minutes >= 1440 {
            clock.minutes = clock.minutes % 1440;
        }
        while clock.minutes < 0 {
            clock.minutes += 1440;
        }
        clock
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
