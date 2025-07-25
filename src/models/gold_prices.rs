use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub struct GoldPrice {
    pub date: NaiveDate,
    pub price: f64,
}

impl<'de> Deserialize<'de> for GoldPrice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;

        let date_str = map
            .remove("data")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .ok_or_else(|| serde::de::Error::missing_field("data"))?;

        let date =
            NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(serde::de::Error::custom)?;

        let price = map
            .remove("cena")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| serde::de::Error::missing_field("cena"))?;

        Ok(GoldPrice { date, price })
    }
}

impl Serialize for GoldPrice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serde_json::Map::new();

        map.insert(
            "data".to_string(),
            serde_json::Value::String(self.date.to_string()),
        );
        map.insert(
            "cena".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(self.price).unwrap()),
        );

        map.serialize(serializer)
    }
}

impl GoldPrice {
    pub fn new(date: NaiveDate, price: f64) -> Self {
        Self { date, price }
    }
}
