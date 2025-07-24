mod get_gold_prices;

use crate::api::gold_prices::get_gold_prices::GetGoldPricesBuilder;
use crate::client::NbpClient;
use crate::client::service_client::ServiceClient;
use crate::models::date_parameters::NoDateParameter;

pub struct GoldPricesHandler {
    client: ServiceClient,
}

impl NbpClient {
    /// Creates a new [`GoldPricesHandler`] that allows you to access
    /// NBP Gold Prices API.
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
    pub fn prices(&self) -> GetGoldPricesBuilder<NoDateParameter> {
        GetGoldPricesBuilder::new(self.client.clone())
    }
}
