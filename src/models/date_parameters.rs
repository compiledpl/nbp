use chrono::NaiveDate;

pub enum DateParameter {
    Today,
    LastDay,
    LastDays(u8),
    Date(NaiveDate),
    DateRange(NaiveDate, NaiveDate),
}

pub struct NoDateParameter;
pub struct WithDateParameter;
