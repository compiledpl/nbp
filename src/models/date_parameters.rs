use chrono::NaiveDate;

pub enum DateParameter {
    Today,
    LastDay,
    LastDays(u8),
    Date(NaiveDate),
    DateRange(NaiveDate, NaiveDate),
}

impl DateParameter {
    pub fn to_path_segment(self) -> String {
        match self {
            DateParameter::Today => "today".to_string(),
            DateParameter::LastDay => "last".to_string(),
            DateParameter::LastDays(days_number) => format!("last/{}", days_number),
            DateParameter::Date(date) => date.to_string(),
            DateParameter::DateRange(start_date, end_date) => {
                format!("{}/{}", start_date, end_date)
            }
        }
    }
}

pub struct NoDateParameter;
pub struct WithDateParameter;
