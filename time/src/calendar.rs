/// A standard reference calendar that counts the days from a reference date.
///
/// Its purpose is as a "Rosetta Stone" to act as a man-in-the-middle between other [`Calendar`] conversions.
///
/// The date chosen as _day 0_ was January 1st of year 1 in the [Gregorian Calendar](https://en.wikipedia.org/wiki/Gregorian_calendar).
/// This choice was made because the Gregorian Calendar is the most widely used worldwide,
/// and thus making conversions to and from that as cheap as possible makes sense.
pub struct StandardCalendar {
    /// How many days have passed since 01/01/01 (in the [Gregorian Calendar](https://en.wikipedia.org/wiki/Gregorian_calendar)).
    pub days: i128,
}

impl StandardCalendar {
    /// Creates a new [`StandardCalendar`] from the given difference since _day 0_.
    pub fn new(days_from: i128) -> Self {
        Self { days: days_from }
    }
}

/// Trait that provides tools for general calendar management.
///
/// This should be implemented by `date` types that adhere to some calendar's rules.
/// The main aspect provided by this trait is conversion between arbitrary [`Calendar`]s.
pub trait Calendar {
    /// How this calendar expresses its months.
    ///
    /// Usually, a [`u8`] is enough to represent all months, but one might also want an enum to represent that.
    type Month;
    /// How this calendar expresses its days (inside a month).
    ///
    /// Usually, a [`u8`] is enough to represent the amount of dates in a month.
    type Day;

    /// Converts a date from one calendar to another.
    fn convert_to<T: Calendar>(&self) -> T {
        T::from_standard(&self.to_standard())
    }

    /// Convert from the date in the current calendar to the [`StandardCalendar`] (days passed since _day 0_).
    fn to_standard(&self) -> StandardCalendar;
    /// Convert from a [`StandardCalendar`] to `Self`.
    fn from_standard(standard: &StandardCalendar) -> Self;

    /// Which day it is in the month in the date contained by this calendar.
    fn day(&self) -> Self::Day;
    /// Which month it is in the year in the date contained by this calendar.
    fn month(&self) -> Self::Month;
    /// Which year it is in the date contained by this calendar.
    fn year(&self) -> super::Year;

    /// Which date is this [`Calendar`]'s reference date.
    ///
    /// The reference date is _day 0_, when the calendar places the "beginning of time", and upon which it
    /// bases its dates.
    fn reference_date() -> Self;

    /// Add a given amount of days to the current date.
    fn add_days(&mut self, days: i128);
    /// Return this date as an amount of days passed since the [`reference_date`](Calendar::reference_date).
    fn as_days(&self) -> i128;
}
