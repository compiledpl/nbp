pub mod paths;
pub(crate) mod service_client;

use crate::client::paths::Paths;
use std::sync::Arc;
use url::Url;

const NBP_DEFAULT_BASE_URL: &str = "https://api.nbp.pl";

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
    paths: Arc<Paths>,
}

impl Default for NbpClient {
    fn default() -> Self {
        let base_url =
            Url::parse(NBP_DEFAULT_BASE_URL).expect("Failed to parse NBP default base URL");
        NbpClient {
            base_url,
            paths: Arc::new(Paths::default()),
        }
    }
}

impl NbpClient {
    /// Returns the base URL for the NBP API.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Returns the configured API paths.
    pub fn paths(&self) -> Paths {
        self.paths.as_ref().clone()
    }
}
