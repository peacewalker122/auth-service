use std::env;

use auth_service::{http, model::ModelManager};
use axum::Router;

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
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::serve(listener, app).await.unwrap();
}

fn new_router(mm: ModelManager) -> Router {
    Router::new().merge(http::new_router(mm))
}
