use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paths {
    pub exchange_rates: String,
    pub gold_prices: String,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            exchange_rates: "/api/exchangerates/".to_string(),
            gold_prices: "/api/cenyzlota/".to_string(),
        }
    }
}
