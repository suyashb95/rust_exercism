use std::fmt;

const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;

#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        let mut new_hours = (hours + (minutes / MINUTES_PER_HOUR)) % HOURS_PER_DAY;
        let mut new_minutes = minutes % MINUTES_PER_HOUR;
        
        if new_minutes < 0 {
            new_minutes = MINUTES_PER_HOUR + new_minutes;
            new_hours -= 1;
        } 

        if new_hours < 0 {
            new_hours = HOURS_PER_DAY + new_hours;
        }        

        Clock {hours: new_hours, minutes: new_minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        let new_minutes = (self.hours * MINUTES_PER_HOUR) + self.minutes + minutes;
        Clock::new(0, new_minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
