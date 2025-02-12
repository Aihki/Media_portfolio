use std::{fs, time::Duration};
use tower_http::cors::{CorsLayer, Any};
use axum::http::{Method, header};

mod handlers;
mod routes;

use handlers::{models, photos};

#[tokio::main]
async fn main() {
    for dir in [
        photos::PHOTO_FOLDER,
        models::MODEL_FOLDER,
    ] {
        if let Err(e) = fs::create_dir_all(dir) {
            eprintln!("❌ Failed to create directory {}: {}", dir, e);
            return;
        }
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ORIGIN,
            header::AUTHORIZATION,
            header::CONTENT_LENGTH,
            header::CACHE_CONTROL,
            header::PRAGMA,
        ])
        .max_age(Duration::from_secs(3600));

    let app = routes::create_routes().layer(cors);

    let addr = "127.0.0.1:3000";
    println!("🚀 Server running at http://{}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
