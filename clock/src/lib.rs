use std::fmt::{Display, Result, Formatter, Debug};

const MINUTES_IN_HOUR: i32    = 60;
const HOURS_IN_DAY: i32       = 24;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // add :02 inside curly braces to specify # of places
        // this forces leading 0's on numbers less than 10
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Reduce to minutes, find modulus with minutes in a day
        // Use rem_euclid to find modules, since % only finds remainder
        // and returns a different value when numbers are negative
        let total_min = (minutes + hours * MINUTES_IN_HOUR).rem_euclid(HOURS_IN_DAY * MINUTES_IN_HOUR);
        Self {
            hours: (total_min/MINUTES_IN_HOUR) % HOURS_IN_DAY,
            minutes: total_min % MINUTES_IN_HOUR,
        }
    }

    // To add minutes, we repeat the same logic used while creating an instance of Clock
    // and to reduce code complexity, we call the new() function
    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes+minutes)
    }
}
