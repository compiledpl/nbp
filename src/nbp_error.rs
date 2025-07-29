use thiserror::Error;

pub type NbpResult<T> = Result<T, NbpError>;

/// Error type for NBP API operations.
///
/// Represents various types of errors that can occur when interacting
/// with the NBP API, including network errors, parsing errors, and
/// API-specific errors.
#[derive(Error, Debug)]
pub enum NbpError {
    #[error("Failed to deserialize response body: {0}")]
    CannotDeserializeBody(String),

    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("HTTP error: {0}")]
    HttpError(u16),

    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
