use axum::{
    Router,
    routing::{get, post},
    extract::DefaultBodyLimit,
};
use tower_http::services::ServeDir;
use crate::handlers::{photos, models, videos};

pub fn create_routes() -> Router {
    Router::new()
        .route("/upload/model", post(models::upload_model))
        .route("/api/models", get(models::list_models))
        .route("/files/:filename", get(photos::get_file))
        .route("/api/photos", get(photos::list_photos))
        .route("/api/videos", get(videos::list_videos))
        .route("/api/upload-video", post(videos::upload_video)) 
        
        .nest_service("/static/photos", ServeDir::new(photos::PHOTO_FOLDER))
        .nest_service("/static/models", ServeDir::new(models::MODEL_FOLDER))
        .nest_service("/static/videos", ServeDir::new(videos::VIDEO_FOLDER))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 500))
}
