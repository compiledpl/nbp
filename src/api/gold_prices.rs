mod get_gold_prices;

use crate::api::gold_prices::get_gold_prices::GetGoldPricesBuilder;
use crate::client::NbpClient;
use crate::client::service_client::ServiceClient;
use crate::models::date_parameters::NoDateParameter;

/// Handler for gold prices API operations.
///
/// Provides methods to access gold prices data from the NBP API.
/// Supports various date ranges and periods.
pub struct GoldPricesHandler {
    client: ServiceClient,
}

impl NbpClient {
    /// Creates a new [`GoldPricesHandler`] that allows you to access
    /// NBP Gold Prices API.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use nbp::client::NbpClient;
    ///
    /// let client = NbpClient::default();
    /// let gold_prices = client.gold_prices();
    ///
    /// // Get gold prices for the last 5 days
    /// let prices = gold_prices.prices().last_days(5);
    /// ```
    pub fn gold_prices(&self) -> GoldPricesHandler {
        GoldPricesHandler {
            client: ServiceClient::new(
                self.base_url()
                    .join(self.paths().gold_prices.as_str())
                    .unwrap(),
            ),
        }
    }
}

impl GoldPricesHandler {
    /// Creates a builder for getting gold prices.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use nbp::client::NbpClient;
    ///
    /// let client = NbpClient::default();
    /// let prices = client
    ///     .gold_prices()
    ///     .prices()
    ///     .last_days(5);
    /// ```
    pub fn prices(&self) -> GetGoldPricesBuilder<NoDateParameter> {
        GetGoldPricesBuilder::new(self.client.clone())
    }
}
