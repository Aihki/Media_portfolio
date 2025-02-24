use axum::{
    extract::State,
    Json,
};
use mongodb::{Database, bson::Document};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct Stats {
    photos_count: u64,
    models_count: u64,
    videos_count: u64,
}

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
