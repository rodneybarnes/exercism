use std::fmt;
use time::ext::NumericalDuration;
use time::Time;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hour: u8,
    minute: u8
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let t = Time::MIDNIGHT + i64::from(hours).hours() + i64::from(minutes).minutes();
        Clock { hour: t.hour(), minute: t.minute() }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_time = Time::from_hms(self.hour, self.minute, 0).unwrap() + i64::from(minutes).minutes();
        Clock { hour: new_time.hour(), minute: new_time.minute() }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}
