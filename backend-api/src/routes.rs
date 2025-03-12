//! API route configuration
//! 
//! Defines all API routes and their handlers.
//! Includes:
//! - File upload endpoints
//! - Content management endpoints
//! - Authentication endpoints
//! - Static file serving

use axum::{
    Router,
    routing::{get, post, delete},
    extract::DefaultBodyLimit,
};
use tower_http::services::ServeDir;
use crate::handlers::{photos, models, videos, categories, stats};
use std::sync::Arc;
use mongodb::Database;
use crate::handlers::auth::login_handler;

/// Creates the router with all API routes
/// 
/// # Arguments
/// 
/// * `db` - MongoDB database connection wrapped in Arc
/// 
/// # Returns
/// 
/// Router instance configured with all endpoints and middleware
pub fn create_routes(db: Arc<Database>) -> Router {
    Router::new()
        .route("/api/upload-photo", post(photos::upload_photo))
        .route("/api/upload-model", post(models::upload_model))
        .route("/api/upload-video", post(videos::upload_video))
        .route("/api/models", get(models::list_models))
        .route("/files/:filename", get(photos::get_file))
        .route("/api/photos", get(photos::list_photos))
        .route("/api/videos", get(videos::list_videos))
        .route("/api/login", post(login_handler))
        .route("/api/categories", post(categories::create_category))
        .route("/api/categories", get(categories::list_categories))
        .route("/api/photos/details", get(photos::get_photos))
        .route("/api/models/details", get(models::get_models))
        .route("/api/videos/details", get(videos::get_videos))
        .route("/api/stats", get(stats::get_stats))
        .route("/api/models/:id", delete(models::delete_model))
        .route("/api/photos/:id", delete(photos::delete_photo))
        .route("/api/videos/:id", delete(videos::delete_video))
        .nest_service("/static/photos", ServeDir::new(photos::PHOTO_FOLDER))
        .nest_service("/static/models", ServeDir::new(models::MODEL_FOLDER))
        .nest_service("/static/videos", ServeDir::new(videos::VIDEO_FOLDER))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 500))
        .with_state(db)
}
