use std::num::NonZeroI128;
use std::ops::{Range, Sub};

use crate::StandardCalendar;
use crate::calendar::Calendar;

/// A date in the [Gregorian Calendar](https://en.wikipedia.org/wiki/Gregorian_calendar).
#[derive(Debug, PartialEq, Eq)]
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
    /// use time::{Calendar, date::gregorian::{Date, Month, year}};
    ///
    /// let my_birthday = Date::from_parts(year!(2008), Month::April, 22);
    /// assert!(my_birthday.is_ok());
    /// let my_birthday = my_birthday.unwrap();
    /// assert_eq!(my_birthday.year(), year!(2008));
    /// assert_eq!(my_birthday.month(), Month::April);
    /// assert_eq!(my_birthday.day(), 22);
    ///
    /// // Leap year :)
    /// assert!(Date::from_parts(year!(2020), Month::February, 29).is_ok());
    /// // Error: Not a leap year
    /// assert!(Date::from_parts(year!(1900), Month::February, 29).is_err());
    /// // Leap year, because it's divisible by 400 ;)
    /// assert!(Date::from_parts(year!(2000), Month::February, 29).is_ok());
    /// # Ok::<(),std::num::IntErrorKind>(())
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

    fn leap_days_between(first: &Self, second: &Self) -> usize {
        let (first, second) = if first > second {
            (second, first)
        } else {
            (first, second)
        };

        // Doesn't include either `first` or `second`
        let leap_years = ((first.year.0.get() + 1)..second.year.0.get())
            .filter(|year| Self::is_leap_year(Year::try_from(*year).unwrap()))
            .count();

        let mut leap_days = leap_years;
        if first.month <= Month::February && Self::is_leap_year(first.year) {
            leap_days += 1;
        }
        if second.month > Month::February && Self::is_leap_year(second.year) {
            leap_days += 1;
        }

        leap_days
    }
}

impl From<&Date> for StandardCalendar {
    fn from(date: &Date) -> Self {
        // TODO: fix
        StandardCalendar { days: todo!() }
    }
}

impl From<StandardCalendar> for Date {
    fn from(standard: StandardCalendar) -> Self {
        // TODO: fix
        Self {
            year: todo!(),
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
            year: year!(1),
            month: Month::January,
            day: 0,
        }
    }
    fn add_days(&mut self, days: i128) {
        // TODO: fix
        todo!()
        /* self.year += days / 365;
        self.day += (days % 365) as u8; */
    }

    fn as_days(&self) -> i128 {
        // TODO: fix
        todo!();
        //self.year * 365 + self.day as i128
    }

    fn days_between(first: &Self, second: &Self) -> i128 {
        let (first, second) = if first > second {
            (second, first)
        } else {
            (first, second)
        };

        let days_in_month = if Self::is_leap_year(second.year) {
            Self::LEAP_DAYS_IN_MONTH
        } else {
            Self::REG_DAYS_IN_MONTH
        };
        let days_last_year: u8 = days_in_month
            .iter()
            .take(second.month as usize - 1)
            .sum::<u8>()
            + second.day;

        // remember: includes leap days in first and second (FUCK)
        let leap_days = Self::leap_days_between(first, second);

        let days_other_years = if first.year - second.year > 1 {
            (first.year - second.year - 1) * 365 + leap_days as i128
        } else {
            0
        };

        todo!()
    }

    /// Returns whether the date is a leap year.
    ///
    /// Leap years represent added days to the year, in order to mantain sync with Earth's rotation.
    /// For more information on how leap years are checked, read [`Year::is_leap_year`].
    fn is_leap_year(year: Self::Year) -> bool {
        year.is_leap_year()
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.year.cmp(&other.year) {
            std::cmp::Ordering::Equal => {}
            order => return order,
        }
        match self.month.cmp(&other.month) {
            std::cmp::Ordering::Equal => {}
            order => return order,
        }
        self.day.cmp(&other.day)
    }
}

/// Representation of a year for the [Gregorian Calendar](https://en.wikipedia.org/wiki/Gregorian_calendar).
///
/// For creation with variables, use [`Year::new`]. For creation with literals, use the [`year`] macro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year(std::num::NonZeroI128);

impl Year {
    /// Constructor for [`Year`].
    ///
    /// Simply wraps the given `year` and returns a new [`Year`].
    ///
    /// # Examples
    /// ```
    /// # use std::num::NonZeroI128;
    /// use time::date::gregorian;
    /// let some_year = gregorian::Year::new(NonZeroI128::new(1528).unwrap());
    /// ```
    pub fn new(year: std::num::NonZeroI128) -> Self {
        Self(year)
    }

    /// Returns whether this is a leap year.
    ///
    /// In the [Gregorian Calendar](https://en.wikipedia.org/wiki/Gregorian_calendar), a leap year happens in all years that
    /// are divisible by 4, except by those that are divisible by 100, except in turn those that are divisible by 400.
    ///
    /// # Examples
    /// ```
    /// # use time::{Calendar, date::gregorian};
    /// assert!(gregorian::year!(2020).is_leap_year());
    /// assert!(gregorian::year!(2000).is_leap_year());
    ///
    /// assert!(!gregorian::year!(1900).is_leap_year());
    /// assert!(!gregorian::year!(2017).is_leap_year());
    /// assert!(!gregorian::year!(2018).is_leap_year());
    /// # Ok::<(),std::num::IntErrorKind>(())
    /// ```
    pub fn is_leap_year(&self) -> bool {
        let inner = self.0.get();
        inner % 4 == 0 && ((inner % 400 == 0) || inner % 100 != 0)
    }

    /// Returns the difference between the years.
    ///
    /// That is, how many [Year]s 'apart' they are. Is equivalent to `self - other`, but more idiomatic.
    ///
    /// Since there is no _year 0_, this is **not** equivalent to `i128 - i128`.
    ///
    /// # Examples
    /// ```
    /// # use time::date::gregorian::year;
    /// assert_eq!(year!(1528).difference(year!(1528)), 0);
    /// assert_eq!(year!(1528).difference(year!(1527)), 1);
    ///
    /// // Edge cases: no year 0!
    /// assert_eq!(year!(1).difference(year!(-1)), 1);
    /// assert_eq!(year!(-1).difference(year!(1)), -1);
    /// ```
    pub fn difference(self, other: Year) -> i128 {
        self - other
    }
}

impl TryFrom<i128> for Year {
    type Error = std::num::IntErrorKind;
    fn try_from(year: i128) -> Result<Self, Self::Error> {
        let year = NonZeroI128::new(year).ok_or(std::num::IntErrorKind::Zero)?;
        Ok(Year::new(year))
    }
}

impl Sub<Year> for Year {
    type Output = i128;
    /// A subtraction between years is handled as the difference between them.
    ///
    /// As the intention of a subtraction isn't immediately clear, prefer to use the
    /// [Year::difference] method.
    fn sub(self, rhs: Year) -> Self::Output {
        let (this, other) = (self.0.get(), rhs.0.get());
        let diff = this - other;
        // This is needed because an year 0 doesn't exist, so we need to correct the subtraction.
        if self.0.is_positive() && rhs.0.is_negative() {
            return diff - 1;
        } else if self.0.is_negative() && rhs.0.is_positive() {
            return diff + 1;
        }
        diff
    }
}

/// Macro for creating a [`Year`] from a literal.
///
/// Saves from having to create a [`NonZeroI128`] or using the fallible [`TryFrom`] implementation.
///
/// # Examples
/// ```
/// // Works!
/// use time::date::gregorian;
///
/// assert_eq!(gregorian::year!(1), gregorian::Year::try_from(1).unwrap());
/// assert_eq!(gregorian::year!(2020), gregorian::Year::try_from(2020).unwrap());
/// assert_eq!(gregorian::year!(1528), gregorian::Year::try_from(1528).unwrap());
/// assert_eq!(gregorian::year!(-1), gregorian::Year::try_from(-1).unwrap()); // Works with negative years too!
/// ```
/// Will not compile if the given year is 0.
/// ```compile_fail
/// # use time::date::gregorian;
/// gregorian::year!(0);
/// ```
/// Will not compile if the given argument is not a number.
/// ```compile_fail
/// # use time::date::gregorian;
/// gregorian::year!("hi");
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! year {
    (0) => {
        compile_error!("Year provided to this macro must not be zero.").
    };
    ($year:literal) => {
        {let year: i128 = $year;
            unsafe {$crate::date::gregorian::Year::new(std::num::NonZeroI128::new_unchecked(year))}
        }
    };
}
#[doc(inline)]
pub use year;

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
