use axum::{
    extract::{Multipart, Path as AxumPath, State},
    Json,
    response::{IntoResponse, Response},
    http::{StatusCode, header},
    body::StreamBody,
};
use std::{fs, path::{PathBuf, Path}, sync::Arc};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;
use uuid::Uuid;
use serde_json::json;
use mongodb::Database;
use crate::models::Photo;

pub const PHOTO_FOLDER: &str = "static/photos";

pub async fn upload_photo(
    State(db): State<Arc<Database>>,
    mut multipart: Multipart
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut name = String::new();
    let mut category_id = String::new();
    let mut saved_filename = String::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        eprintln!("Error getting next field: {}", e);
        StatusCode::BAD_REQUEST
    })? {
        println!("Processing field name: {:?}", field.name());
        
        match field.name() {
            Some("name") => {
                name = field.text().await.map_err(|e| {
                    eprintln!("Error reading name field: {}", e);
                    StatusCode::BAD_REQUEST
                })?;
            },
            Some("category") => {
                category_id = field.text().await.map_err(|e| {
                    eprintln!("Error reading category field: {}", e);
                    StatusCode::BAD_REQUEST
                })?;
            },
            Some("file") => {
                let original_filename = field.file_name()
                    .map(|f| f.to_string())
                    .unwrap_or_else(|| "unknown".to_string());

                let extension = Path::new(&original_filename)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("jpg");

                let filename = format!("photo_{}_{}.{}", 
                    Uuid::new_v4(), 
                    chrono::Local::now().format("%Y%m%d"),
                    extension
                );
                
                saved_filename = filename.clone();
                let filepath = format!("{}/{}", PHOTO_FOLDER, filename);

                println!("💾 Attempting to save photo to: {} (original: {})", filepath, original_filename);
        
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
            },
            _ => {
                println!("Received unknown field: {:?}", field.name());
            }
        }
    }

    // Validate required fields
    if name.is_empty() || category_id.is_empty() || saved_filename.is_empty() {
        eprintln!("Missing required fields: name={}, category={}, filename={}", 
            !name.is_empty(), 
            !category_id.is_empty(), 
            !saved_filename.is_empty()
        );
        return Err(StatusCode::BAD_REQUEST);
    }

    // Convert category_id string to ObjectId
    let category_object_id = match mongodb::bson::oid::ObjectId::parse_str(&category_id) {
        Ok(oid) => oid,
        Err(e) => {
            eprintln!("Invalid category ID format: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Create photo document
    let photo = Photo::new(name, saved_filename.clone(), category_object_id);

    // Save to MongoDB
    match db.collection::<Photo>("photos")
        .insert_one(photo, None)
        .await {
        Ok(_) => {
            let url = format!("/static/photos/{}", saved_filename);
            Ok(Json(json!({
                "url": url,
                "filename": saved_filename
            })))
        },
        Err(e) => {
            eprintln!("Failed to save photo to database: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_file(AxumPath(filename): AxumPath<String>) -> impl IntoResponse {
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
