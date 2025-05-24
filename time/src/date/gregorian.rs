use crate::StandardCalendar;
use crate::calendar::Calendar;

/// A date in the [Gregorian Calendar](https://en.wikipedia.org/wiki/Gregorian_calendar).
pub struct GregorianDate {
    year: i128,
    day: u16,
}

impl Calendar for GregorianDate {
    type Day = u16;
    type Month = u8;

    fn to_standard(&self) -> StandardCalendar {
        // TODO: fix
        StandardCalendar::new(self.year * 365 + self.day as i128)
    }

    fn from_standard(standard: &StandardCalendar) -> Self {
        // TODO: fix
        Self {
            year: standard.days / 365,
            day: (standard.days % 365) as u16,
        }
    }

    fn day(&self) -> Self::Day {
        // TODO: fix
        self.day
    }

    fn year(&self) -> crate::Year {
        self.year
    }

    fn month(&self) -> Self::Month {
        // TODO: fix
        (self.day / 12) as u8
    }

    fn reference_date() -> Self {
        Self { year: 1, day: 0 }
    }
    fn add_days(&mut self, days: i128) {
        // TODO: fix
        self.year += days / 365;
        self.day += (days % 365) as u16;
    }

    fn as_days(&self) -> i128 {
        // TODO: fix
        self.year * 365 + self.day as i128
    }

    /// Returns whether the date is a leap year.
    ///
    /// Leap years represent added days to the year, in order to mantain sync with Earth's rotation.
    ///
    /// In the [Gregorian Calendar](https://en.wikipedia.org/wiki/Gregorian_calendar), a leap year happens in all years that
    /// are divisible by 4, except by those that are divisible by 100, except in turn those that are divisible by 400.
    fn is_leap_year(&self) -> bool {
        self.year % 4 == 0 && !(self.year % 100 == 0 && !(self.year % 400 == 0))
    }
}
