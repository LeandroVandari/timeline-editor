use super::StandardCalendar;

pub struct GregorianDate {
    year: i128,
    day: u16
}

impl<'any> From<&'any GregorianDate> for StandardCalendar {
    fn from(date: &'any GregorianDate) -> Self {
        // Wrong: need to consider 1528 thing + handle negative years properly
        StandardCalendar::new(date.year * 365 + date.day as i128)
    }
}

impl<'any> From<&'any StandardCalendar> for GregorianDate {
    fn from(value: &'any StandardCalendar) -> Self {
        Self { year: value.days_from_reference_date / 365, day: (value.days_from_reference_date % 365) as u16 }
    }
}

impl super::Calendar for GregorianDate {
    type Day = u16;
    type Month = u8;

    fn day(&self) -> Self::Day {
        self.day
    }

    fn year(&self) -> super::Year {
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