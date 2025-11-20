use axum::{Json, Router, routing::get};
use serde::Serialize;
use tracing::info;

#[derive(Serialize)]
struct DisplayMessage {
    status: u8,
    message: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/api/display", get(get_message));

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
        message: "Hello world".to_string(),
    })
}
