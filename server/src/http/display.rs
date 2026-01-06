use crate::image_processing::image_to_bin;
use axum::{
    Json, Router,
    body::Body,
    extract::Multipart,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use serde::Serialize;
use tokio_util::io::ReaderStream;

pub fn routes() -> Router {
    Router::new().route("/api/display", get(get_display).post(post_display))
}

#[derive(Serialize)]
struct DisplayMessage {
    status: u16,
}

async fn get_display() -> impl IntoResponse {
    let file = match tokio::fs::File::open("output.bin").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    let file_metadata = file.metadata().await.unwrap();
    let file_length = file_metadata.len().to_string();

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=UTF-8"),
    );
    headers.insert(
        header::CONTENT_LENGTH,
        HeaderValue::from_str(&file_length).unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_static("inline; filename=\"output.bin\""),
    );

    Ok((headers, body))
}

async fn post_display(mut multipart: Multipart) -> impl IntoResponse {
    let mut image_bytes: Option<Vec<u8>> = None;

    // Get image bytes
    while let Some(field) = multipart.next_field().await.unwrap() {
        let bytes = field.bytes().await.unwrap();
        tokio::fs::write("uploaded.png", &bytes).await.unwrap();
        image_bytes = Some(bytes.to_vec());
    }

    let Some(image_bytes) = image_bytes else {
        return Json(DisplayMessage { status: 400 });
    };

    // Convert image to a format compatible with the e-ink display
    let bin_bytes = image_to_bin(&image_bytes);

    tokio::fs::write("output.bin", &bin_bytes).await.unwrap();

    Json(DisplayMessage { status: 200 })
}
