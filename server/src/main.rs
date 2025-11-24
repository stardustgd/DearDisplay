use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

mod http;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let routes = Router::new().merge(http::display::routes()).layer(cors);

    info!("Running on localhost:4000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    axum::serve(listener, routes).await.unwrap();
}
