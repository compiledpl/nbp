use reqwest::Client;
use url::Url;

#[derive(Clone)]
pub struct ServiceClient {
    http_client: Client,
    base_url: Url,
}

impl ServiceClient {
    pub fn new(base_url: Url) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let http_client = Client::builder().default_headers(headers).build().unwrap();

        Self {
            http_client,
            base_url,
        }
    }

    pub fn get_http_client(&self) -> &Client {
        &self.http_client
    }

    pub fn get_base_url(&self) -> &Url {
        &self.base_url
    }
}
