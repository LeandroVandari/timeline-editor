use std::num::{NonZero, NonZeroI128};

use crate::StandardCalendar;
use crate::calendar::Calendar;

/// A date in the [Gregorian Calendar](https://en.wikipedia.org/wiki/Gregorian_calendar).
#[derive(Debug)]
pub struct Date {
    year: Year,
    month: Month,
    day: u8,
}

impl Date {
    const REG_DAYS_IN_MONTH: [<Self as Calendar>::Day; 12] =
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const LEAP_DAYS_IN_MONTH: [<Self as Calendar>::Day; 12] =
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    /// Creates a date in the Gregorian Calendar from the day, month and year.
    ///
    /// # Examples
    /// ```
    /// use time::{Calendar, date::gregorian::{Date, Month}};
    ///
    /// let my_birthday = Date::from_parts(2008, Month::April, 22);
    /// assert!(my_birthday.is_ok());
    /// let my_birthday = my_birthday.unwrap();
    /// assert_eq!(my_birthday.year(), 2008);
    /// assert_eq!(my_birthday.month(), Month::April);
    /// assert_eq!(my_birthday.day(), 22);
    ///
    /// // Leap year :)
    /// assert!(Date::from_parts(2020, Month::February, 29).is_ok());
    /// // Error: Not a leap year
    /// assert!(Date::from_parts(1900, Month::February, 29).is_err());
    /// // Leap year, because it's divisible by 400 ;)
    /// assert!(Date::from_parts(2000, Month::February, 29).is_ok());
    /// ```
    pub fn from_parts(
        year: Year,
        month: <Self as Calendar>::Month,
        day: <Self as Calendar>::Day,
    ) -> Result<Self, errors::DateCreationError> {
        // convert to the appropriate list indices
        let days_in_month = if Self::is_leap_year(year) {
            Self::LEAP_DAYS_IN_MONTH
        } else {
            Self::REG_DAYS_IN_MONTH
        };

        // Subtract one because the list is 0-indexed.
        if !(1..=days_in_month[month as usize - 1]).contains(&day) {
            return Err(errors::DateCreationError::InvalidDay(day));
        }

        Ok(Self { year, day, month })
    }
}

impl From<&Date> for StandardCalendar {
    fn from(date: &Date) -> Self {
        // TODO: fix
        StandardCalendar {
            days: date.year * 365 + date.day as i128,
        }
    }
}

impl From<StandardCalendar> for Date {
    fn from(standard: StandardCalendar) -> Self {
        // TODO: fix
        Self {
            year: standard.days / 365,
            month: Month::try_from(((standard.days % 365) / 12) as u8 + 1).unwrap(),
            day: (standard.days % 365) as u8,
        }
    }
}

impl Calendar for Date {
    type Day = u8;
    type Month = Month;
    type Year = Year;

    fn day(&self) -> Self::Day {
        self.day
    }

    fn year(&self) -> Self::Year {
        self.year
    }

    fn month(&self) -> Self::Month {
        self.month
    }

    fn reference_date() -> Self {
        Self {
            year: 1,
            month: Month::January,
            day: 0,
        }
    }
    fn add_days(&mut self, days: i128) {
        // TODO: fix
        self.year += days / 365;
        self.day += (days % 365) as u8;
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
    ///
    /// # Examples
    /// ```
    /// # use time::{Calendar, date::gregorian};
    /// assert!(gregorian::Date::is_leap_year(2020));
    /// assert!(gregorian::Date::is_leap_year(2000));
    ///
    /// assert!(!gregorian::Date::is_leap_year(1900));
    /// assert!(!gregorian::Date::is_leap_year(2017));
    /// assert!(!gregorian::Date::is_leap_year(2018));
    /// ```
    fn is_leap_year(year: Self::Year) -> bool {
        year % 4 == 0 && ((year % 400 == 0) || year % 100 != 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year(std::num::NonZeroI128);

impl Year {
    pub fn new(year: std::num::NonZeroI128) -> Self {
        Self(year)
    }
}

impl<T> std::ops::Add<NonZero<T: std::num::ZeroablePrimitive>> for Year {
    type Output = Year;
    fn add(self, rhs: NonZero<T: std::num::ZeroablePrimitive>) -> Self::Output {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl TryFrom<u8> for Month {
    type Error = errors::DateCreationError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::January,
            2 => Self::February,
            3 => Self::March,
            4 => Self::April,
            5 => Self::May,
            6 => Self::June,
            7 => Self::July,
            8 => Self::August,
            9 => Self::September,
            10 => Self::October,
            11 => Self::November,
            12 => Self::December,
            other => return Err(errors::DateCreationError::InvalidMonth(other)),
        })
    }
}

mod errors {
    use crate::calendar::Calendar;

    use super::Date;

    #[derive(Debug, Clone, Copy)]
    pub enum DateCreationError {
        InvalidMonth(u8),
        InvalidDay(<Date as Calendar>::Day),
    }
}
