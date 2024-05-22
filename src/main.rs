use std::env;

use auth_service::{http, model::ModelManager};
use axum::{http::Method, Router};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!("cwd: {}", env::current_dir().unwrap().display());
    dotenv::from_filename(".env").unwrap();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let mm = ModelManager::new().await.unwrap();
    let app = new_router(mm);

    axum::serve(listener, app).await.unwrap();
}

fn new_router(mm: ModelManager) -> Router {
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::HEAD])
        .allow_headers(Any)
        .allow_origin(Any);
    Router::new().merge(http::new_router(mm)).layer(cors_layer)
}
