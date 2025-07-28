mod get_rates;
mod get_tables;

use crate::api::exchange_rates::get_rates::GetRatesBuilder;
use crate::api::exchange_rates::get_tables::GetTablesBuilder;
use crate::client::service_client::ServiceClient;
use crate::client::{NBP_API_EXCHANGE_RATES_ENDPOINT, NbpClient};
use crate::models::currency_code::CurrencyCode;
use crate::models::date_parameters::NoDateParameter;
use crate::models::table_type::TableType;

/// Handler for exchange rates API operations.
///
/// Provides methods to access exchange rates data from the NBP API.
/// Supports different table types (A, B, C) and various date ranges.
pub struct ExchangeRatesHandler {
    client: ServiceClient,
}

impl NbpClient {
    /// Creates a new [`ExchangeRatesHandler`] that allows you to access
    /// NBP Exchange Rates API.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use nbp::client::NbpClient;
    /// use nbp::models::TableType;
    ///
    /// let client = NbpClient::default();
    /// let exchange_rates = client.exchange_rates();
    ///
    /// // Get table A for today
    /// let tables = exchange_rates.tables(TableType::A).today();
    /// ```
    pub fn exchange_rates(&self) -> ExchangeRatesHandler {
        ExchangeRatesHandler {
            client: ServiceClient::new(
                self.base_url()
                    .join(NBP_API_EXCHANGE_RATES_ENDPOINT)
                    .unwrap(),
            ),
        }
    }
}

impl ExchangeRatesHandler {
    /// Creates a builder for getting exchange rate tables.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use nbp::client::NbpClient;
    /// use nbp::models::TableType;
    ///
    /// let client = NbpClient::default();
    /// let tables = client
    ///     .exchange_rates()
    ///     .tables(TableType::A)
    ///     .last_days(5);
    /// ```
    pub fn tables(&self, table: TableType) -> GetTablesBuilder<NoDateParameter> {
        GetTablesBuilder::new(self.client.clone(), table)
    }

    /// Creates a builder for getting exchange rates for a specific currency.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use nbp::client::NbpClient;
    /// use nbp::models::{TableType, CurrencyCode};
    ///
    /// let client = NbpClient::default();
    /// let rates = client
    ///     .exchange_rates()
    ///     .rates(TableType::A, CurrencyCode::USD)
    ///     .last_days(5);
    /// ```
    pub fn rates(
        &self,
        table: TableType,
        currency: CurrencyCode,
    ) -> GetRatesBuilder<NoDateParameter> {
        GetRatesBuilder::new(self.client.clone(), table, currency)
    }
}
