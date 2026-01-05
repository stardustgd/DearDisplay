use crate::image_processing::image_to_bin;
use axum::{
    Json, Router,
    body::Body,
    extract::Multipart,
    http::{StatusCode, header},
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
    image_bmp: Vec<u8>,
}

async fn get_display() -> impl IntoResponse {
    let file = match tokio::fs::File::open("output.bmp").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let headers = [
        (header::CONTENT_TYPE, "text/plain; charset=UTF-8"),
        (
            header::CONTENT_DISPOSITION,
            "inline; filename=\"output.bmp\"",
        ),
    ];

    Ok((headers, body))
}

async fn post_display(mut multipart: Multipart) -> Json<DisplayMessage> {
    let mut image_bytes: Option<Vec<u8>> = None;

    // Get image bytes
    while let Some(field) = multipart.next_field().await.unwrap() {
        let bytes = field.bytes().await.unwrap();
        tokio::fs::write("uploaded.png", &bytes).await.unwrap();
        image_bytes = Some(bytes.to_vec());
    }

    let Some(image_bytes) = image_bytes else {
        return Json(DisplayMessage {
            status: 400,
            image_bmp: vec![],
        });
    };

    // Convert image to a format compatible with the e-ink display
    let bin_bytes = image_to_bin(&image_bytes);

    tokio::fs::write("output.bin", &bin_bytes).await.unwrap();

    Json(DisplayMessage {
        status: 200,
        image_bmp: bin_bytes,
    })
}
