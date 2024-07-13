use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sentry::Hub;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Store: {0}")]
    Store(#[from] crate::store::Error),
    #[error("Error converting games from store: {0}")]
    GameConversion(serde_json::error::Error),
    #[error("Error decoding query string for subscription: {0}")]
    SubscriptionUrlDecoding(std::string::FromUtf8Error),
    #[error("Notifier: {0}")]
    Notifier(#[from] crate::notifier::Error),
    #[error("Not authorized")]
    Unauthorized,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        if matches!(self, ApiError::Unauthorized) {
            return StatusCode::UNAUTHORIZED.into_response();
        }
        Hub::current().capture_error(&self);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
