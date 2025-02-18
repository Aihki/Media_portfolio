use axum::{extract::State, Json};
use axum::http::StatusCode;
use mongodb::Database;
use std::sync::Arc;
use serde_json::{json, Value};
use futures_util::TryStreamExt;
use crate::models::Category;

pub async fn create_category(
    State(db): State<Arc<Database>>,
    Json(category): Json<Category>,
) -> Result<Json<Value>, StatusCode> {
    let result = db.collection("category")
        .insert_one(category, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "_id": result.inserted_id,
        "status": "success"
    })))
}

pub async fn list_categories(
    State(db): State<Arc<Database>>,
) -> Result<Json<Vec<Category>>, StatusCode> {
    let categories = db.collection::<Category>("category")
        .find(None, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let categories: Vec<Category> = categories
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(categories))
}
