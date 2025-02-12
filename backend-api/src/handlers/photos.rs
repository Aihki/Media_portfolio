use axum::{
    extract::{Multipart, Path},
    Json,
    response::{IntoResponse, Response},
    http::{StatusCode, header},
    body::StreamBody,
};
use std::{fs, path::PathBuf};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;
use uuid::Uuid;
use serde_json::json;

pub const PHOTO_FOLDER: &str = "static/photos";

pub async fn upload_photo(mut multipart: Multipart) -> Result<Json<serde_json::Value>, StatusCode> {
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        eprintln!("Error getting next field: {}", e);
        StatusCode::BAD_REQUEST
    })? {
        let filename = field.file_name()
            .map(|f| f.to_string())
            .unwrap_or_else(|| format!("photo_{}.jpg", Uuid::new_v4()));
        
        let filepath = format!("{}/{}", PHOTO_FOLDER, filename);
        println!("💾 Attempting to save photo to: {}", filepath);
        
        if !PathBuf::from(PHOTO_FOLDER).exists() {
            fs::create_dir_all(PHOTO_FOLDER).map_err(|e| {
                eprintln!("Failed to create directory: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        }
        
        let data = match field.bytes().await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to read upload data: {}", e);
                return Err(StatusCode::BAD_REQUEST);
            }
        };

        if data.is_empty() {
            eprintln!("Received empty file data");
            return Err(StatusCode::BAD_REQUEST);
        }


        match File::create(&filepath).await {
            Ok(mut file) => {
                if let Err(e) = file.write_all(&data).await {
                    eprintln!("Failed to write file: {}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
            Err(e) => {
                eprintln!("Failed to create file: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }

        let url = format!("/static/photos/{}", filename);
        println!("✅ Photo saved successfully: {}", url);
        
        return Ok(Json(json!({
            "url": url,
            "filename": filename
        })));
    }
    
    eprintln!("No file field found in request");
    Err(StatusCode::BAD_REQUEST)
}

pub async fn get_file(Path(filename): Path<String>) -> impl IntoResponse {
    let path = PathBuf::from(PHOTO_FOLDER).join(filename);
    
    match File::open(&path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = StreamBody::new(stream);
            
            Response::builder()
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .body(body)
                .unwrap()
                .into_response()
        }
        Err(_) => StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn list_photos() -> Result<Json<Vec<String>>, StatusCode> {
    println!("📸 Listing photos from: {}", PHOTO_FOLDER);
    
    if !PathBuf::from(PHOTO_FOLDER).exists() {
        fs::create_dir_all(PHOTO_FOLDER)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    match fs::read_dir(PHOTO_FOLDER) {
        Ok(entries) => {
            let photos: Vec<String> = entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        if e.path().is_file() {
                            Some(format!("/static/photos/{}", 
                                e.file_name().to_string_lossy()))
                        } else {
                            None
                        }
                    })
                })
                .collect();
            
            println!("📸 Found {} photos", photos.len());
            Ok(Json(photos))
        },
        Err(e) => {
            eprintln!("❌ Error reading photos: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
