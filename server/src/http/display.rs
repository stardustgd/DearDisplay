use crate::error::ApiError;
use crate::http::screenshot::screenshot_handler;
use crate::image_processing::image_to_bin;
use axum::{
    Json, Router,
    body::Body,
    extract::Multipart,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
};
use serde::Serialize;

pub fn routes() -> Router {
    Router::new().route("/api/display", get(get_display).post(post_display))
}

#[derive(Serialize)]
struct DisplayMessage {
    status: u16,
}

async fn get_display() -> Result<impl IntoResponse, ApiError> {
    screenshot_handler().await?;

    let file = tokio::fs::read("uploaded.png").await?;
    let bin_bytes = image_to_bin(&file)?;

    tokio::fs::write("output.bin", &bin_bytes).await?;

    let file_length = bin_bytes.len().to_string();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain; charset=UTF-8")
        .header("content-length", file_length)
        .body(Body::from(bin_bytes))?)
}

async fn post_display(mut multipart: Multipart) -> Result<Json<DisplayMessage>, ApiError> {
    let mut image_bytes: Option<Vec<u8>> = None;

    // Get image bytes
    while let Some(field) = multipart.next_field().await? {
        let bytes = field.bytes().await?;
        tokio::fs::write("uploaded.png", &bytes).await?;
        image_bytes = Some(bytes.to_vec());
    }

    let image_bytes = image_bytes.ok_or_else(|| ApiError {
        message: "No image to process".to_string(),
        status_code: StatusCode::BAD_REQUEST,
    })?;

    // Convert image to a format compatible with the e-ink display
    let bin_bytes = image_to_bin(&image_bytes)?;

    tokio::fs::write("output.bin", &bin_bytes).await?;

    Ok(Json(DisplayMessage { status: 200 }))
}
