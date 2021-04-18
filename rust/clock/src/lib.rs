use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    wall: i32,
    _a: (), // non-public field to prevent struct literal initialization of Clock.
}

const MINUTES_IN_DAY: i32 = 60 * 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { wall: 0, _a: () }.add_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let t = (self.wall + minutes) % MINUTES_IN_DAY;

        Clock {
            wall: if t > 0 { t } else { MINUTES_IN_DAY + t },
            _a: (),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{:02}:{:02}",
            self.wall.div_euclid(60).rem_euclid(24), // HH
            self.wall.rem_euclid(60)                 // MM
        ))
    }
}
