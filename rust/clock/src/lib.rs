use std::fmt;

#[derive(Debug)]
pub struct Clock {
    time: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            time: (hours % 24) * 60 + (minutes % 1440),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{:02}:{:02}",
            self.time / 60,
            minutes = self.time % 60,
        ))
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Eq for Clock {}
