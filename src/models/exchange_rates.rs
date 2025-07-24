use crate::models::currency_code::CurrencyCode;
use crate::models::table_type::TableType;
use chrono::NaiveDate;
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
#[serde(rename_all = "camelCase")]
pub struct ExchangeRate {
    pub code: CurrencyCode,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub mid: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyExchangeRates {
    pub table: TableType,
    pub code: CurrencyCode,
    pub rates: Vec<ExchangeRateRecord>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExchangeRateRecord {
    pub no: String,
    pub effective_date: NaiveDate,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub mid: Option<f64>,
}

impl<'de> Deserialize<'de> for ExchangeRateRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;

        let no = map
            .remove("no")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .ok_or_else(|| serde::de::Error::missing_field("no"))?;

        let effective_date_str = map
            .remove("effectiveDate")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .ok_or_else(|| serde::de::Error::missing_field("effectiveDate"))?;
        let effective_date = NaiveDate::parse_from_str(&effective_date_str, "%Y-%m-%d")
            .map_err(serde::de::Error::custom)?;

        let bid = map.remove("bid").and_then(|v| v.as_f64());

        let ask = map.remove("ask").and_then(|v| v.as_f64());

        let mid = map.remove("mid").and_then(|v| v.as_f64());

        Ok(ExchangeRateRecord {
            no,
            effective_date,
            bid,
            ask,
            mid,
        })
    }
}

impl Serialize for ExchangeRateRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serde_json::Map::new();

        map.insert("no".to_string(), serde_json::Value::String(self.no.clone()));
        map.insert(
            "effectiveDate".to_string(),
            serde_json::Value::String(self.effective_date.to_string()),
        );

        if let Some(bid) = self.bid {
            map.insert(
                "bid".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(bid).unwrap()),
            );
        }

        if let Some(ask) = self.ask {
            map.insert(
                "ask".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(ask).unwrap()),
            );
        }

        if let Some(mid) = self.mid {
            map.insert(
                "mid".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(mid).unwrap()),
            );
        }

        map.serialize(serializer)
    }
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
            .map(ExchangeRate::deserialize)
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
