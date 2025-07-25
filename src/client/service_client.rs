use crate::nbp_error::{NbpError, NbpResult};
use reqwest::Client;
use url::Url;

#[derive(Clone, Debug)]
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

    pub fn join_path(&mut self, path: &str) {
        self.base_url = self.base_url.join(path).unwrap();
    }

    pub async fn get<T>(&self) -> NbpResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self
            .http_client
            .get(self.base_url.clone())
            .send()
            .await
            .map_err(|e| {
                NbpError::request_failed(format!(
                    "Request failed for route {}: {}",
                    self.base_url, e
                ))
            })?;

        if !response.status().is_success() {
            return Err(NbpError::request_failed(format!(
                "Request failed for route {}: {}",
                self.base_url,
                response.status()
            )));
        }

        response
            .text()
            .await
            .map_err(|e| NbpError::request_failed(format!("Failed to read response text: {}", e)))
            .and_then(|text| {
                serde_json::from_str(&text).map_err(|e| {
                    NbpError::cannot_deserialize_body(format!(
                        "Failed to deserialize response for route {}: {}",
                        self.base_url, e
                    ))
                })
            })
    }
}
