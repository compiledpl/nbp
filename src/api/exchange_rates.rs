mod get_rates;
mod get_tables;

use crate::api::exchange_rates::get_rates::GetRatesBuilder;
use crate::api::exchange_rates::get_tables::GetTablesBuilder;
use crate::client::NbpClient;
use crate::client::service_client::ServiceClient;
use crate::models::currency_code::CurrencyCode;
use crate::models::date_parameters::NoDateParameter;
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

    pub fn rates(
        &self,
        table: TableType,
        currency: CurrencyCode,
    ) -> GetRatesBuilder<NoDateParameter> {
        GetRatesBuilder::new(self.client.clone(), table, currency)
        //TODO: consider removing table and match table-currency automatically or add some builder method like trading() or just new handler method trading_rates()
    }
}
