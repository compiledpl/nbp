use crate::models::currency_code::CurrencyCode;
use crate::models::table_type::TableType;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct CurrencyExchangeTable {
    pub table: TableType,
    pub no: String,
    pub trading_date: Option<NaiveDate>,
    pub effective_date: Option<NaiveDate>,
    pub rates: Vec<ExchangeRate>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub code: CurrencyCode,
    pub bid: Option<Decimal>,
    pub ask: Option<Decimal>,
    pub mid: Option<Decimal>,
}

impl<'de> Deserialize<'de> for CurrencyExchangeTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;

        let table_str = map
            .remove("table")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .ok_or_else(|| serde::de::Error::missing_field("table"))?;
        let table = TableType::from_str(&table_str).map_err(serde::de::Error::custom)?;

        let no = map
            .remove("no")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .ok_or_else(|| serde::de::Error::missing_field("no"))?;

        let trading_date = map
            .remove("tradingDate")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

        let effective_date = map
            .remove("effectiveDate")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

        let rates_array = map
            .remove("rates")
            .and_then(|v| v.as_array().map(|v| v.to_vec()))
            .ok_or_else(|| serde::de::Error::missing_field("rates"))?;

        let rates = rates_array
            .iter()
            .map(|v| ExchangeRate::deserialize(v))
            .collect::<Result<Vec<_>, _>>()
            .map_err(serde::de::Error::custom)?;

        Ok(CurrencyExchangeTable {
            table,
            no,
            trading_date,
            effective_date,
            rates,
        })
    }
}

impl Serialize for CurrencyExchangeTable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serde_json::Map::new();

        map.insert(
            "table".to_string(),
            serde_json::Value::String(self.table.to_string()),
        );
        map.insert("no".to_string(), serde_json::Value::String(self.no.clone()));

        if let Some(trading_date) = &self.trading_date {
            map.insert(
                "tradingDate".to_string(),
                serde_json::Value::String(trading_date.to_string()),
            );
        }

        if let Some(effective_date) = &self.effective_date {
            map.insert(
                "effectiveDate".to_string(),
                serde_json::Value::String(effective_date.to_string()),
            );
        }

        let rates_value = serde_json::to_value(&self.rates).map_err(serde::ser::Error::custom)?;
        map.insert("rates".to_string(), rates_value);

        map.serialize(serializer)
    }
}
