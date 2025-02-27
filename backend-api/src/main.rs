use std::{fs, time::Duration};
use tower_http::cors::{CorsLayer, Any};
use axum::http::{Method, header};
use std::sync::Arc;

mod models;
mod handlers;
mod routes;
mod db;

use handlers::{
    photos::PHOTO_FOLDER,
    models::MODEL_FOLDER,
    videos::VIDEO_FOLDER
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let database = db::connect()
        .await
        .expect("Failed to connect to MongoDB");
   
    let app_state = Arc::new(database);

    for dir in [
        PHOTO_FOLDER,
        MODEL_FOLDER,
        VIDEO_FOLDER,
    ] {
        if let Err(e) = fs::create_dir_all(dir) {
            eprintln!("❌ Failed to create directory {}: {}", dir, e);
            return;
        }
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS  
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ORIGIN,
            header::AUTHORIZATION,
            header::CONTENT_LENGTH,
            header::CACHE_CONTROL,
            header::PRAGMA,
            header::ACCESS_CONTROL_REQUEST_METHOD,  
            header::ACCESS_CONTROL_REQUEST_HEADERS, 
        ])
        .max_age(Duration::from_secs(3600));

    let app = routes::create_routes(app_state).layer(cors);

    let addr = "127.0.0.1:3000";
    println!("🚀 Server running at http://{}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
