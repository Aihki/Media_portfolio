use axum::{
    Router,
    routing::{get, post},
    extract::DefaultBodyLimit,
};
use tower_http::services::ServeDir;
use crate::handlers::{photos};

pub fn create_routes() -> Router {
    Router::new()
        .route("/api/models", get(models::list_models))
        .route("/files/:filename", get(images::get_file))
        .route("/api/photos", get(photos::list_photos))
        .nest_service("/static/photos", ServeDir::new(photos::PHOTO_FOLDER))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 500))
}
    