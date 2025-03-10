//! 3D model handling module
//! 
//! Provides functionality for:
//! - Model upload
//! - Model retrieval
//! - Model listing
//! - Model deletion

use axum::{
    extract::{Multipart, State, Path as AxumPath},
    Json,
    http::StatusCode
};
use std::{fs, path::PathBuf, sync::Arc};
use mongodb::Database;
use crate::models::{Model, ModelResponse, Category};  
use serde_json::json;
use std::io::Write;
use uuid::Uuid;
use futures_util::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::doc;

/// Directory where 3D models are stored
pub const MODEL_FOLDER: &str = "static/models";

/// Handles 3D model upload requests
/// 
/// # Arguments
/// * `db` - MongoDB database connection
/// * `multipart` - Multipart form data containing model file and metadata
/// 
/// # Returns
/// Returns the URL and filename of the uploaded model
pub async fn upload_model(
    State(db): State<Arc<Database>>,
    mut multipart: Multipart
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut name = String::new();
    let mut category_id = String::new();
    let mut saved_filename = String::new();

    while let Some(mut field) = multipart.next_field().await.map_err(|e| { 
        eprintln!("Error getting next field: {}", e);
        StatusCode::BAD_REQUEST
    })? {
        match field.name() {
            Some("name") => {
                name = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            },
            Some("category") => {
                category_id = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            },
            Some("file") => {
                let filename = field.file_name()
                    .map(|f| f.to_string())
                    .unwrap_or_else(|| format!("{}.splat", Uuid::new_v4()));
                
                println!("📦 Uploading model: {}", filename);
                
                let filepath = format!("{}/{}", MODEL_FOLDER, filename);
                
                if !PathBuf::from(MODEL_FOLDER).exists() {
                    fs::create_dir_all(MODEL_FOLDER).map_err(|e| {
                        eprintln!("Failed to create directory: {}", e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;
                }


                let mut file = std::fs::File::create(&filepath).map_err(|e| {
                    eprintln!("Failed to create file: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

                while let Some(chunk) = field.chunk().await.map_err(|e| {
                    eprintln!("Error reading chunk: {}", e);
                    StatusCode::BAD_REQUEST
                })? {
                    file.write_all(&chunk).map_err(|e| {
                        eprintln!("Failed to write chunk: {}", e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;
                }

                saved_filename = filename.clone();
            },
            _ => {}
        }
    }


    if name.is_empty() || category_id.is_empty() || saved_filename.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let category_object_id = mongodb::bson::oid::ObjectId::parse_str(&category_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;


    let model = Model::new(name, saved_filename.clone(), category_object_id);
    
    match db.collection::<Model>("models")
        .insert_one(model, None)
        .await {
        Ok(_) => {
            let url = format!("/static/models/{}", saved_filename);
            Ok(Json(json!({
                "url": url,
                "filename": saved_filename
            })))
        },
        Err(e) => {
            eprintln!("Failed to save model to database: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Lists all available 3D models
/// 
/// # Returns
/// Returns a list of model URLs
pub async fn list_models() -> Result<Json<Vec<String>>, StatusCode> {
    if !PathBuf::from(MODEL_FOLDER).exists() {
        fs::create_dir_all(MODEL_FOLDER)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    match fs::read_dir(MODEL_FOLDER) {
        Ok(paths) => {
            let models = paths
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        if e.path().is_file() {
                            Some(format!("/static/models/{}", 
                                e.file_name().to_string_lossy()))
                        } else {
                            None
                        }
                    })
                })
                .collect();
            
            Ok(Json(models))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

/// Retrieves detailed information about all models
/// 
/// # Arguments
/// * `db` - MongoDB database connection
/// 
/// # Returns
/// Returns a list of model details with category information
pub async fn get_models(
    State(db): State<Arc<Database>>
) -> Result<Json<Vec<ModelResponse>>, StatusCode> {
    println!("📦 Fetching models from MongoDB");
    
    let models_collection = db.collection::<Model>("models");
    let categories_collection = db.collection::<Category>("category");
    

    let mut categories_vec = Vec::new();
    let mut cursor = categories_collection.find(None, None).await.map_err(|e| {
        eprintln!("Failed to query categories: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    while let Some(result) = cursor.next().await {
        match result {
            Ok(category) => categories_vec.push(category),
            Err(e) => eprintln!("Error reading category: {}", e),
        }
    }

    let mut cursor = models_collection.find(None, None).await.map_err(|e| {
        eprintln!("Failed to query models: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut models = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(model) => {
                let mut model_response = model.to_response();
                model_response.category_name = categories_vec.iter()
                    .find(|c| c.id.unwrap_or_default() == model.category_id)
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| "Unknown Category".to_string());
                models.push(model_response);
            },
            Err(e) => eprintln!("Error reading model: {}", e),
        }
    }

    Ok(Json(models))
}

/// Deletes a specific model
/// 
/// # Arguments
/// * `db` - MongoDB database connection
/// * `id` - ID of the model to delete
pub async fn delete_model(
    State(db): State<Arc<Database>>,
    AxumPath(id): AxumPath<String>
) -> Result<StatusCode, StatusCode> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    match db.collection::<Model>("models")
        .delete_one(doc! { "_id": object_id }, None)
        .await {
        Ok(result) if result.deleted_count == 1 => Ok(StatusCode::NO_CONTENT),
        Ok(_) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}