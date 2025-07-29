//! # NBP API Client
//!
//! A Rust library for accessing the National Bank of Poland's API.
//!
//! This library provides a type-safe and ergonomic interface for accessing
//! various financial data such as exchange rates and gold prices.
//!
//! ## Example
//!
//! ```rust
//! use nbp::client::NbpClient;
//! use nbp::models::{TableType, CurrencyCode};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = NbpClient::default();
//!     
//!     // Get USD exchange rates for the last 5 days
//!     let rates = client
//!         .exchange_rates()
//!         .rates(TableType::A, CurrencyCode::USD)
//!         .last_days(5)
//!         .send()
//!         .await?;
//!     
//!     println!("Found {} USD rates", rates.rates.len());
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod client;
pub mod models;
pub mod nbp_error;
