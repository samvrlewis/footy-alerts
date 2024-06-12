use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Store: {0}")]
    Store(#[from] store::Error),
    #[error("Error converting games from store: {0}")]
    GameConversion(serde_json::error::Error),
    #[error("Error decoding query string for subscription: {0}")]
    SubscriptionUrlDecoding(std::string::FromUtf8Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
