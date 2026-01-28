use axum::{
    Json,
    http::{StatusCode, header},
    response::IntoResponse,
};
use serde_json::json;

#[derive(Debug)]
pub struct ApiError {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;

        (
            status_code,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "message": self.message, "status": self.status_code.as_u16() })),
        )
            .into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: std::error::Error,
{
    fn from(err: E) -> Self {
        ApiError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
