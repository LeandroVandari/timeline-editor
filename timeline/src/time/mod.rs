pub struct StandardCalendar {
    days_from_reference_date: i128,
}

pub trait Calendar
where
    Self: for<'any> From<&'any StandardCalendar>,
    for<'any> &'any Self: Into<StandardCalendar>,
{
    fn convert_to<T: Calendar>(&self) -> T
    where
        T: for<'any> From<&'any StandardCalendar>,
        StandardCalendar: for<'any> From<&'any T>,
    {
        (&<&Self as Into<StandardCalendar>>::into(self)).into()
    }
}
