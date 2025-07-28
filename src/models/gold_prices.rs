use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GoldPrice {
    #[serde(rename = "data")]
    pub date: NaiveDate,
    #[serde(rename = "cena")]
    pub price: f64,
}

impl GoldPrice {
    pub fn new(date: NaiveDate, price: f64) -> Self {
        Self { date, price }
    }
}
