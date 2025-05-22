pub struct StandardCalendar {
    days_from_reference_date: i128,
}

pub trait Calendar: for<'any> From<&'any StandardCalendar>
where
    StandardCalendar: for<'any> From<&'any Self>,
{
    fn convert_to<T: Calendar>(&self) -> T
    where
        StandardCalendar: for<'any> From<&'any T>,
    {
        (&<&Self as Into<StandardCalendar>>::into(self)).into()
    }
}
