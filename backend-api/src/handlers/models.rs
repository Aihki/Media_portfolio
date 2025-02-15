use axum::{extract::Multipart, Json, http::StatusCode};
use std::{fs, path::PathBuf};
use uuid::Uuid;
use serde_json::json;
use std::io::Write;

pub const MODEL_FOLDER: &str = "static/models";

pub async fn upload_model(mut multipart: Multipart) -> Result<Json<serde_json::Value>, StatusCode> {
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        eprintln!("Error getting next field: {}", e);
        StatusCode::BAD_REQUEST
    })? {
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

        let url = format!("/static/models/{}", filename);
        println!("✅ Model saved successfully: {}", url);
        
        return Ok(Json(json!({
            "url": url,
            "filename": filename
        })));
    }

    eprintln!("No file field found in request");
    Err(StatusCode::BAD_REQUEST)
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