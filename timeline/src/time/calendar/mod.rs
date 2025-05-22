pub mod gregorian;

pub struct StandardCalendar {
    days_from_reference_date: i128,
}

impl StandardCalendar {
    fn new(days_from: i128) -> Self {
        Self { days_from_reference_date: days_from }
    }
}

type Year = i128;

pub trait Calendar: for<'any> From<&'any StandardCalendar>
where
    StandardCalendar: for<'any> From<&'any Self>,
{
    type Month;
    type Day;
    fn convert_to<T: Calendar>(&self) -> T
    where
        StandardCalendar: for<'any> From<&'any T>,
    {
        (&<&Self as Into<StandardCalendar>>::into(self)).into()
    }

    fn day(&self) -> Self::Day;
    fn month(&self) -> Self::Month;
    fn year(&self) -> Year;
    fn start_date() -> Self;
    fn add_days(&mut self, days: i128);
    fn as_days(&mut self) -> i128;

}


