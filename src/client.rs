pub mod paths;
pub(crate) mod service_client;

use crate::client::paths::Paths;
use std::sync::Arc;
use url::Url;

const NBP_DEFAULT_BASE_URL: &str = "https://api.nbp.pl";

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
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub fn paths(&self) -> Paths {
        self.paths.as_ref().clone()
    }
}
