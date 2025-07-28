use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub type NbpResult<T> = Result<T, NbpError>;

/// Error type for NBP API operations.
///
/// Represents various types of errors that can occur when interacting
/// with the NBP API, including network errors, parsing errors, and
/// API-specific errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NbpError {
    r#type: ErrorType,
    context: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorType {
    CannotDeserializeBody,
    RequestFailed,
    InvalidArgument,
    NotFound,
    BadRequest,
    InternalError,
}

impl Display for NbpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {:?} - {}", self.r#type, self.context)
    }
}

impl std::error::Error for NbpError {}

impl NbpError {
    /// Creates a new error indicating that a request failed.
    pub fn request_failed(context: String) -> Self {
        NbpError {
            r#type: ErrorType::RequestFailed,
            context,
        }
    }

    /// Creates a new error indicating that the response body could not be deserialized.
    pub fn cannot_deserialize_body(context: String) -> Self {
        NbpError {
            r#type: ErrorType::CannotDeserializeBody,
            context,
        }
    }

    /// Creates a new error indicating that an invalid argument was provided.
    pub fn invalid_argument(context: String) -> Self {
        NbpError {
            r#type: ErrorType::InvalidArgument,
            context,
        }
    }

    /// Creates a new error indicating that a resource was not found.
    pub fn not_found(context: String) -> Self {
        NbpError {
            r#type: ErrorType::NotFound,
            context,
        }
    }

    /// Creates a new error indicating that a bad request was made.
    pub fn bad_request(context: String) -> Self {
        NbpError {
            r#type: ErrorType::BadRequest,
            context,
        }
    }

    /// Creates a new error indicating an internal server error.
    pub fn internal_error(context: String) -> Self {
        NbpError {
            r#type: ErrorType::InternalError,
            context,
        }
    }
}
