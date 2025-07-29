use crate::nbp_error::{NbpError, NbpResult};
use reqwest::{Client, StatusCode};
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

    pub fn join_path(&mut self, path: &str) -> NbpResult<()> {
        self.base_url = self.base_url.join(path).map_err(|e| {
            NbpError::InvalidArgument(format!("Failed to join path '{}': {}", path, e))
        })?;
        Ok(())
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
            .map_err(|e| NbpError::RequestFailed(e.to_string()))?;

        match response.status() {
            StatusCode::NOT_FOUND => {
                return Err(NbpError::NotFound(format!(
                    "Resource not found for route {}",
                    self.base_url
                )));
            }
            StatusCode::BAD_REQUEST => {
                return Err(NbpError::BadRequest(format!(
                    "Bad request for route {}",
                    self.base_url
                )));
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                return Err(NbpError::InternalError(format!(
                    "Internal server error for route {}",
                    self.base_url
                )));
            }
            status if status.is_success() => {}
            status => {
                return Err(NbpError::HttpError(status.as_u16()));
            }
        }

        response.json::<T>().await.map_err(|e| {
            NbpError::CannotDeserializeBody(format!(
                "Failed to deserialize response for route {}: {}",
                self.base_url, e
            ))
        })
    }
}
