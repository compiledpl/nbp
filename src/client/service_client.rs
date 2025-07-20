use crate::models::date_parameters::DateParameter;
use crate::nbp_error::{NbpError, NbpResult};
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

    pub async fn get<T>(&self, route: String, date_parameter: Option<DateParameter>) -> NbpResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let route = self.base_url.join(&route).unwrap();

        if let Some(date_parameter) = date_parameter {
            match date_parameter {
                DateParameter::Today => route.join("today").unwrap(),
                DateParameter::LastDay => route.join("last").unwrap(),
                DateParameter::LastDays(days_number) => {
                    route.join("last").unwrap();
                    route.join(&days_number.to_string()).unwrap()
                }
                DateParameter::Date(date) => route.join(&date.to_string()).unwrap(),
                DateParameter::DateRange(start_date, end_date) => {
                    route.join(&start_date.to_string()).unwrap();
                    route.join(&end_date.to_string()).unwrap()
                }
            };
        }

        let response = self
            .http_client
            .get(route.clone())
            .send()
            .await
            .map_err(|e| {
                NbpError::request_failed(format!("Request failed for route {}: {}", route, e))
            })?;

        if !response.status().is_success() {
            return Err(NbpError::request_failed(format!(
                "Request failed for route {}: {}",
                route,
                response.status()
            )));
        }

        response
            .text()
            .await
            .map_err(|e| NbpError::request_failed(format!("Failed to read response text: {}", e)))
            .and_then(|text| {
                println!("Response text: {}", text);
                serde_json::from_str(&text).map_err(|e| {
                    NbpError::cannot_deserialize_body(format!(
                        "Failed to deserialize response for route {}: {}",
                        route, e
                    ))
                })
            })
    }
}
