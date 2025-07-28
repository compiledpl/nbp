pub(crate) mod service_client;

use url::Url;

const NBP_DEFAULT_BASE_URL: &str = "https://api.nbp.pl";
pub const NBP_API_EXCHANGE_RATES_ENDPOINT: &str = "api/exchangerates/";
pub const NBP_API_GOLD_PRICES_ENDPOINT: &str = "api/cenyzlota/";

/// Main client for accessing the National Bank of Poland API.
///
/// This client provides access to various NBP API endpoints such as
/// exchange rates and gold prices.
///
/// ## Example
///
/// ```rust
/// use nbp::client::NbpClient;
///
/// let client = NbpClient::default();
/// let exchange_rates = client.exchange_rates();
/// let gold_prices = client.gold_prices();
/// ```
#[derive(Clone)]
pub struct NbpClient {
    base_url: Url,
}

impl Default for NbpClient {
    fn default() -> Self {
        let base_url =
            Url::parse(NBP_DEFAULT_BASE_URL).expect("Failed to parse NBP default base URL");
        NbpClient { base_url }
    }
}

impl NbpClient {
    /// Returns the base URL for the NBP API.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
}
