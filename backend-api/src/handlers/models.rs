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
    http::StatusCode,
    response::{IntoResponse, Response},
    body::StreamBody,
    http::header,
};
use std::{fs, path::{PathBuf, Path}, sync::Arc};  // Added Path import
use mongodb::Database;
use crate::models::{Model, ModelResponse, Category};  
use serde_json::json;
use uuid::Uuid;
use futures_util::StreamExt;
use mongodb::bson::{oid::ObjectId, doc};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

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
    let mut filename = String::new();
    let mut name = String::new();
    let mut category = String::new();

    while let Some(mut field) = multipart.next_field().await.map_err(|e| {
        eprintln!("Error processing multipart form: {}", e);
        StatusCode::BAD_REQUEST
    })? {
        let field_name = field.name().unwrap_or("").to_string();
        println!("Processing field: {:?}", field_name);

        match field_name.as_str() {
            "file" => {
                // Get original filename
                let orig_name = field.file_name().unwrap_or("").to_string();
                if !orig_name.ends_with(".splat") {
                    eprintln!("Invalid file type: {}", orig_name);
                    return Err(StatusCode::BAD_REQUEST);
                }

                // Collect file data
                let mut data = Vec::new();
                while let Some(chunk) = field.chunk().await.map_err(|e| {
                    eprintln!("Error reading chunk: {}", e);
                    StatusCode::BAD_REQUEST
                })? {
                    data.extend_from_slice(&chunk);
                }

                // Ensure proper 4-byte alignment for float32 values
                let float_size = 4;  // size of float32
                let total_floats = data.len() / float_size;
                let aligned_length = total_floats * float_size;
                
                // Create aligned data buffer
                let mut aligned_data = Vec::with_capacity(aligned_length);
                aligned_data.extend_from_slice(&data[..aligned_length]);

                // Save aligned data
                let filepath = format!("{}/{}", MODEL_FOLDER, filename);
                fs::write(&filepath, &aligned_data).map_err(|e| {
                    eprintln!("Failed to save file: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            },
            "name" => {
                name = field.text().await.map_err(|e| {
                    eprintln!("Error reading name: {}", e);
                    StatusCode::BAD_REQUEST
                })?;
            },
            "category" => {
                category = field.text().await.map_err(|e| {
                    eprintln!("Error reading category: {}", e);
                    StatusCode::BAD_REQUEST
                })?;
            },
            _ => {}
        }
    }

    // Save model metadata to database
    if name.is_empty() || category.is_empty() || filename.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let category_object_id = mongodb::bson::oid::ObjectId::parse_str(&category)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let model = Model::new(name, filename.clone(), category_object_id);
    
    match db.collection::<Model>("models")
        .insert_one(model, None)
        .await {
        Ok(_) => {
            let response = json!({
                "url": format!("/static/models/{}", filename),
                "filename": filename,
                "success": true
            });
            Ok(Json(response))
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
                        // Only include .splat files
                        if e.path().is_file() && 
                           e.path().extension().map_or(false, |ext| ext == "splat") {
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

pub async fn get_file(AxumPath(filename): AxumPath<String>) -> impl IntoResponse {
    let path = PathBuf::from(MODEL_FOLDER).join(&filename);
    
    match File::open(&path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = StreamBody::new(stream);

            Response::builder()
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header(header::ACCEPT_RANGES, "bytes")
                .header(header::CACHE_CONTROL, "no-cache")
                .header(header::PRAGMA, "no-cache")
                .header("Content-Transfer-Encoding", "binary")
                .header("Cross-Origin-Resource-Policy", "cross-origin")
                .body(body)
                .unwrap()
                .into_response()
        }
        Err(_) => StatusCode::NOT_FOUND.into_response()
    }
}