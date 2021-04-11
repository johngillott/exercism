use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    wall: i32,
}

const MINUTES_IN_DAY: i32 = 60 * 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { wall: 0 }.add_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let t = (self.wall + minutes) % MINUTES_IN_DAY;

        Clock {
            wall: if t > 0 { t } else { MINUTES_IN_DAY + t },
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{:02}:{:02}",
            (self.wall / 60) % 24,    // HH
            minutes = self.wall % 60, // MM
        ))
    }
}
