use serde::{Deserialize, Serialize};
use std::{error, fmt};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorObject {
    status: u16,
    code: ErrorCode,
    message: String,
}
impl fmt::Display for ErrorObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Notion API Error:\n  Status: {}\n  Code: {}\n  Message: {}\n",
            self.status, self.code, self.message
        )
    }
}
impl error::Error for ErrorObject {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InvalidJson,
    InvalidRequestUrl,
    InvalidRequest,
    ValidationError,
    Unauthorized,
    RestrictedResource,
    ObjectNotFound,
    ConflictError,
    RateLimited,
    InternalServerError,
    ServiceUnavailable,
}
impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson => write!(f, "Invalid JSON"),
            Self::InvalidRequestUrl => write!(f, "Invalid Request Url"),
            Self::InvalidRequest => write!(f, "Invalid Request"),
            Self::ValidationError => write!(f, "Validation Error"),
            Self::Unauthorized => write!(f, "Unauthorized"),
            Self::RestrictedResource => write!(f, "Restricted Resource"),
            Self::ObjectNotFound => write!(f, "Object Not Found"),
            Self::ConflictError => write!(f, "Conflict Error"),
            Self::RateLimited => write!(f, "Rate Limited"),
            Self::InternalServerError => write!(f, "Internal Server Error"),
            Self::ServiceUnavailable => write!(f, "Service Unavailable"),
        }
    }
}
