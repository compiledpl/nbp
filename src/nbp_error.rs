use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub type NbpResult<T> = Result<T, NbpError>;
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
    pub fn request_failed(context: String) -> Self {
        NbpError {
            r#type: ErrorType::RequestFailed,
            context,
        }
    }

    pub fn cannot_deserialize_body(context: String) -> Self {
        NbpError {
            r#type: ErrorType::CannotDeserializeBody,
            context,
        }
    }

    pub fn not_found(context: String) -> Self {
        NbpError {
            r#type: ErrorType::NotFound,
            context,
        }
    }

    pub fn bad_request(context: String) -> Self {
        NbpError {
            r#type: ErrorType::BadRequest,
            context,
        }
    }

    pub fn internal_error(context: String) -> Self {
        NbpError {
            r#type: ErrorType::InternalError,
            context,
        }
    }
}
