use crate::StandardCalendar;
use crate::calendar::Calendar;

pub struct GregorianDate {
    year: i128,
    day: u16
}

impl Calendar for GregorianDate {
    type Day = u16;
    type Month = u8;

    fn to_standard(&self) -> StandardCalendar {
        StandardCalendar::new(self.year * 365 + self.day as i128)
    }

    fn from_standard(standard: &StandardCalendar) -> Self {
        Self { year: standard.days / 365, day: (standard.days % 365) as u16 }
    }

    fn day(&self) -> Self::Day {
        self.day
    }

    fn year(&self) -> crate::Year {
        self.year
    }

    fn month(&self) -> Self::Month {
        (self.day / 12) as u8
    }

    fn start_date() -> Self {
        Self { year: 0, day: 0 }
    }
    fn add_days(&mut self, days: i128) {
        self.year += days / 365;
        self.day += (days % 365) as u16;
    }

    fn as_days(&mut self) -> i128 {
        self.year * 365 + self.day as i128
    }
}