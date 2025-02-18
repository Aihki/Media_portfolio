use axum::{extract::State, Json};
use axum::http::StatusCode;
use mongodb::Database;
use std::sync::Arc;
use futures_util::TryStreamExt;
use crate::models::Category;

pub async fn create_category(
    State(db): State<Arc<Database>>,
    Json(category): Json<Category>,
) -> Result<Json<Category>, StatusCode> {
    let collection = db.collection::<Category>("category");
    
    let result = collection
        .insert_one(category.clone(), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let created_category = Category {
        id: Some(result.inserted_id.as_object_id().unwrap()),
        name: category.name,
    };

    Ok(Json(created_category))
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
