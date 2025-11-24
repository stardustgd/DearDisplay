use axum::{Json, Router, extract::Multipart, routing::get};
use serde::Serialize;

pub fn routes() -> Router {
    Router::new().route("/api/display", get(get_display).post(post_display))
}

#[derive(Serialize)]
struct DisplayMessage {
    status: u16,
    image_bmp: String,
}

async fn get_display() -> Json<DisplayMessage> {
    Json(DisplayMessage {
        status: 200,
        image_bmp: "".to_string(),
    })
}

async fn post_display(mut multipart: Multipart) -> Json<DisplayMessage> {
    let mut image_bytes: Option<Vec<u8>> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let bytes = field.bytes().await.unwrap();

        tokio::fs::write("uploaded.png", &bytes).await.unwrap();
        image_bytes = Some(bytes.to_vec());
    }

    let Some(image_bytes) = image_bytes else {
        return Json(DisplayMessage {
            status: 400,
            image_bmp: "".to_string(),
        });
    };

    let image = image::load_from_memory(&image_bytes).expect("Invalid image");

    let mut bmp_buf = Vec::new();

    {
        let mut cursor = std::io::Cursor::new(&mut bmp_buf);
        image
            .write_to(&mut cursor, image::ImageFormat::Bmp)
            .expect("Failed to encode BMP")
    }

    tokio::fs::write("output.bmp", &bmp_buf)
        .await
        .expect("failed to write bmp");

    let hex = bmp_buf
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>();

    Json(DisplayMessage {
        status: 200,
        image_bmp: hex,
    })
}
