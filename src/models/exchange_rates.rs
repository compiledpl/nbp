use crate::models::currency_code::CurrencyCode;
use crate::models::table_type::TableType;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRateRecord {
    pub no: String,
    pub effective_date: NaiveDate,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub mid: Option<f64>,
}
