use axum::{
    extract::{Multipart, State},
    Json,
    http::StatusCode
};
use std::{fs, path::PathBuf, sync::Arc};
use mongodb::Database;
use crate::models::Model;  
use serde_json::json;
use std::io::Write;
use uuid::Uuid;

pub const MODEL_FOLDER: &str = "static/models";

pub async fn upload_model(
    State(db): State<Arc<Database>>,
    mut multipart: Multipart
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut name = String::new();
    let mut category_id = String::new();
    let mut saved_filename = String::new();

    while let Some(mut field) = multipart.next_field().await.map_err(|e| {  // Added mut here
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

                // Read chunks and write directly to file
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

    // Validate fields
    if name.is_empty() || category_id.is_empty() || saved_filename.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Convert category_id to ObjectId
    let category_object_id = mongodb::bson::oid::ObjectId::parse_str(&category_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Create and save model document
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