use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub type NbpResult<T> = Result<T, NbpError>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NbpError {
    typ: ErrorType,
    context: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorType {
    CannotDeserializeBody,
    RequestFailed,
    InvalidArgument,
    NotFound,
}

impl Display for NbpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {:?} - {}", self.typ, self.context)
    }
}

impl std::error::Error for NbpError {}

impl NbpError {
    pub fn request_failed(context: String) -> Self {
        NbpError {
            typ: ErrorType::RequestFailed,
            context,
        }
    }

    pub fn cannot_deserialize_body(context: String) -> Self {
        NbpError {
            typ: ErrorType::CannotDeserializeBody,
            context,
        }
    }
}
