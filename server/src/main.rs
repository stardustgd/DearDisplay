use axum::{Json, Router, extract::Multipart, routing::get};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[derive(Serialize)]
struct DisplayMessage {
    status: u16,
    image_bmp: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/api/display", get(get_message).post(post_message))
        .layer(cors);

    info!("Running on localhost:4000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello world"
}

async fn get_message() -> Json<DisplayMessage> {
    Json(DisplayMessage {
        status: 200,
        image_bmp: "".to_string(),
    })
}

async fn post_message(mut multipart: Multipart) -> Json<DisplayMessage> {
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
