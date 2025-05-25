use crate::StandardCalendar;
use crate::calendar::Calendar;

/// A date in the [Gregorian Calendar](https://en.wikipedia.org/wiki/Gregorian_calendar).
#[derive(Debug)]
pub struct GregorianDate {
    year: i128,
    month: u8,
    day: u8
}

impl GregorianDate {
    
    const REG_DAYS_IN_MONTH: [<Self as Calendar>::Day; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const LEAP_DAYS_IN_MONTH: [<Self as Calendar>::Day; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    /// Creates a date in the Gregorian Calendar from the day, month and year.
    /// 
    /// # Examples
    /// ```
    /// use time::{Calendar, date::GregorianDate};
    /// 
    /// let my_birthday = GregorianDate::from_parts(2008, 04, 22);
    /// assert!(my_birthday.is_ok());
    /// let my_birthday = my_birthday.unwrap();
    /// assert_eq!(my_birthday.year(), 2008);
    /// assert_eq!(my_birthday.month(), 04);
    /// assert_eq!(my_birthday.day(), 22);
    /// 
    /// // Leap year :)
    /// assert!(GregorianDate::from_parts(2020, 02, 29).is_ok());
    /// // Error: Not a leap year
    /// assert!(GregorianDate::from_parts(1900, 02, 29).is_err());
    /// // Leap year, because it's divisible by 400 ;)
    /// assert!(GregorianDate::from_parts(2000, 02, 29).is_ok());
    /// ```
    pub fn from_parts(
        year: i128,
        month: <Self as Calendar>::Month,
        day: <Self as Calendar>::Day,
    ) -> Result<Self, errors::DateCreationError> {
        if !(1..=12).contains(&month) {
            return Err(errors::DateCreationError::InvalidMonth(month));
        }
        dbg!(Self::is_leap_year(year));
        // convert to the appropriate list indices
        let month = month - 1;
        let days_in_month = if Self::is_leap_year(year) {
            Self::LEAP_DAYS_IN_MONTH
        } else {Self::REG_DAYS_IN_MONTH};

        if !(0..days_in_month[month as usize])
            .contains(&(day-1))
        {
            return Err(errors::DateCreationError::InvalidDay(day));
        }

        Ok(Self { year, day, month })
    }
}

impl From<&GregorianDate> for StandardCalendar {
    fn from(date: &GregorianDate) -> Self {
        // TODO: fix
        StandardCalendar {
            days: date.year * 365 + date.day as i128,
        }
    }
}

impl From<StandardCalendar> for GregorianDate {
    fn from(standard: StandardCalendar) -> Self {
        // TODO: fix
        Self {
            year: standard.days / 365,
            month: ((standard.days % 365) % 12) as u8,
            day: (standard.days % 365) as u8,
        }
    }
}

impl Calendar for GregorianDate {
    type Day = u8;
    type Month = u8;

    fn day(&self) -> Self::Day {
        self.day
    }

    fn year(&self) -> crate::Year {
        self.year
    }

    fn month(&self) -> Self::Month {
        self.month + 1
    }

    fn reference_date() -> Self {
        Self { year: 1, month: 0, day: 0 }
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
    /// # use time::{Calendar, date::GregorianDate};
    /// assert!(GregorianDate::is_leap_year(2020));
    /// assert!(GregorianDate::is_leap_year(2000));
    /// 
    /// assert!(!GregorianDate::is_leap_year(1900));
    /// assert!(!GregorianDate::is_leap_year(2017));
    /// assert!(!GregorianDate::is_leap_year(2018));
    /// ```
    fn is_leap_year(year: crate::Year) -> bool {
        year % 4 == 0 && ((year % 400 == 0) || year % 100 != 0)
    }
}

mod errors {
    use crate::calendar::Calendar;

    use super::GregorianDate;

    #[derive(Debug, Clone, Copy)]
    pub enum DateCreationError {
        InvalidMonth(<GregorianDate as Calendar>::Month),
        InvalidDay(<GregorianDate as Calendar>::Day),
    }
}
