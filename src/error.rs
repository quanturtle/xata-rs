#[derive(Debug)]
pub enum XataClientError {
    Generic,
    NoContent,
    BadRequest,
    InvalidAPIKey,
    Forbidden,
    NotFound,
    InvalidDatabaseURL,
    RateLimitExceeded,
    ServerError,
    ReqwestError(reqwest::Error),
    Deserialization(serde_json::Error)
}

impl From<reqwest::Error> for XataClientError {
    fn from(error: reqwest::Error) -> Self {
        XataClientError::ReqwestError(error)
    }
}

impl From<serde_json::Error> for XataClientError {
    fn from(error: serde_json::Error) -> Self {
        XataClientError::Deserialization(error)
    }
}