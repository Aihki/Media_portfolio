//! Statistics handling module
//! 
//! Provides functionality for retrieving statistics about stored content

use axum::{
    extract::State,
    Json,
};
use mongodb::{Database, bson::Document};
use serde::Serialize;
use std::sync::Arc;

/// Statistics about stored content
#[derive(Serialize)]
pub struct Stats {
    /// Number of photos stored
    photos_count: u64,
    /// Number of 3D models stored
    models_count: u64,
    /// Number of videos stored
    videos_count: u64,
}

/// Retrieves statistics about stored content
/// 
/// # Arguments
/// * `db` - MongoDB database connection
/// 
/// # Returns
/// Returns counts of photos, models, and videos
pub async fn get_stats(
    State(db): State<Arc<Database>>,
) -> Json<Stats> {
    let photos_count = db.collection::<Document>("photos").count_documents(None, None)
        .await.unwrap_or(0);
    let models_count = db.collection::<Document>("models").count_documents(None, None)
        .await.unwrap_or(0);
    let videos_count = db.collection::<Document>("videos").count_documents(None, None)
        .await.unwrap_or(0);

    Json(Stats {
        photos_count,
        models_count,
        videos_count,
    })
}
