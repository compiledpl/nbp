mod get_tables;

use crate::api::exchange_rates::get_tables::GetTablesBuilder;
use crate::client::NbpClient;
use crate::client::service_client::ServiceClient;
use crate::models::date_parameters::NoDateParameter;
// use crate::models::currency_code::CurrencyCode;
use crate::models::table_type::TableType;

pub struct ExchangeRatesHandler {
    client: ServiceClient,
}

impl NbpClient {
    /// Creates a new [`ExchangeRatesHandler`] that allows you to access
    /// NBP Exchange Rates API.
    pub fn exchange_rates(&self) -> ExchangeRatesHandler {
        ExchangeRatesHandler {
            client: ServiceClient::new(
                self.base_url()
                    .join(self.paths().exchange_rates.as_str())
                    .unwrap(),
            ),
        }
    }
}

impl ExchangeRatesHandler {
    pub fn tables(&self, table: TableType) -> GetTablesBuilder<NoDateParameter> {
        GetTablesBuilder::new(self.client.clone(), table)
    }

    // pub async fn rates(&self, table_type: TableType, currency: CurrencyCode) -> NbpResult<GetRatesBuilder> {
    //     GetRatesBuilder::new(self, currency)
    // }
}
