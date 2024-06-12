use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct ApiResponse<J> {
    json_body: J,
    status_code: StatusCode,
}

impl<J> ApiResponse<J> {
    pub fn new(json_body: J, status_code: StatusCode) -> Self {
        Self {
            json_body,
            status_code,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        let mut resp = axum::Json(self.json_body).into_response();
        *resp.status_mut() = self.status_code;
        resp
    }
}
