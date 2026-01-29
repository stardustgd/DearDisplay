use axum::{
    Json,
    extract::multipart::MultipartError,
    http::{
        StatusCode,
        header::{self, InvalidHeaderValue},
    },
    response::IntoResponse,
};
use image::ImageError;
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

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<ImageError> for ApiError {
    fn from(err: ImageError) -> Self {
        ApiError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<MultipartError> for ApiError {
    fn from(err: MultipartError) -> Self {
        ApiError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<InvalidHeaderValue> for ApiError {
    fn from(err: InvalidHeaderValue) -> Self {
        ApiError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<axum::http::Error> for ApiError {
    fn from(err: axum::http::Error) -> Self {
        ApiError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
