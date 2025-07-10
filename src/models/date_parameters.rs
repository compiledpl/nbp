use chrono::NaiveDate;

pub enum DateParameter {
    Today,
    LastDay,
    Date(NaiveDate),
    DateRange(NaiveDate, NaiveDate),
}

pub struct NoDateParameter;
pub struct WithDateParameter;
