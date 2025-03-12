//! Photo handling module
//! 
//! Provides functionality for:
//! - Photo upload
//! - Photo retrieval
//! - Photo listing
//! - Photo deletion
//! - Individual photo file serving

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
use crate::models::{Photo, PhotoResponse, Category}; 
use futures_util::StreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;

/// Directory where photos are stored
pub const PHOTO_FOLDER: &str = "static/photos";

/// Handles photo upload requests
/// 
/// # Arguments
/// * `db` - MongoDB database connection
/// * `multipart` - Multipart form data containing photo file and metadata
/// 
/// # Returns
/// Returns the URL and filename of the uploaded photo, or an error status
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


    if name.is_empty() || category_id.is_empty() || saved_filename.is_empty() {
        eprintln!("Missing required fields: name={}, category={}, filename={}", 
            !name.is_empty(), 
            !category_id.is_empty(), 
            !saved_filename.is_empty()
        );
        return Err(StatusCode::BAD_REQUEST);
    }


    let category_object_id = match mongodb::bson::oid::ObjectId::parse_str(&category_id) {
        Ok(oid) => oid,
        Err(e) => {
            eprintln!("Invalid category ID format: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };


    let photo = Photo::new(name, saved_filename.clone(), category_object_id);


    match db.collection::<Photo>("photos")
        .insert_one(photo, None)
        .await {
        Ok(_) => {
            let response = json!({
                "url": format!("/static/photos/{}", saved_filename),
                "filename": saved_filename,
                "success": true
            });
            Ok(Json(response))
        },
        Err(e) => {
            eprintln!("Failed to save photo to database: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Retrieves a specific photo file
/// 
/// # Arguments
/// * `filename` - Name of the photo file to retrieve
/// 
/// # Returns
/// Returns the photo file as a stream response or a 404 error
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

/// Lists all available photos
/// 
/// # Returns
/// Returns a list of photo URLs, or an error status
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

/// Retrieves detailed information about all photos
/// 
/// # Arguments
/// * `db` - MongoDB database connection
/// 
/// # Returns
/// Returns a list of photo details with category information
pub async fn get_photos(
    State(db): State<Arc<Database>>
) -> Result<Json<Vec<PhotoResponse>>, StatusCode> {
    println!("📸 Fetching photos from MongoDB");
    
    let photos_collection = db.collection::<Photo>("photos");
    let categories_collection = db.collection::<Category>("category"); 
    
    println!("🔍 Checking category collection");
    let categories = categories_collection.find(None, None).await.map_err(|e| {
        eprintln!("Failed to query category collection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut categories_vec = Vec::new();
    let mut cursor = categories;
    
    while let Some(category_result) = cursor.next().await {
        match category_result {
            Ok(category) => {
                println!("📁 Found category: id={}, name={}", 
                    category.id.as_ref().map(|id| id.to_hex()).unwrap_or_default(),
                    category.name
                );
                categories_vec.push(category);
            },
            Err(e) => {
                eprintln!("Error reading category: {}", e);
            }
        }
    }
    println!("📊 Total categories found: {}", categories_vec.len());

    match photos_collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut photos = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(photo) => {
                        println!("\n🔍 Processing photo: {}", photo.name);
                        println!("   Looking for category ID: {}", photo.category_id.to_hex());
                        
       
                        println!("   Comparing against categories:");
                        for cat in &categories_vec {
                            if let Some(id) = cat.id {
                                println!("   - {} ({})", id.to_hex(), cat.name);
                                if id.to_hex() == photo.category_id.to_hex() {
                                    println!("     ✅ Match found!");
                                }
                            }
                        }

                        let category_name = categories_vec.iter()
                            .find(|c| c.id.unwrap_or_default().to_hex() == photo.category_id.to_hex())
                            .map(|c| {
                                println!("   ✅ Found matching category: {}", c.name);
                                c.name.clone()
                            })
                            .unwrap_or_else(|| {
                                println!("   ⚠️ No category found for ID");
                                "Unknown Category".to_string()
                            });

                        let mut photo_response = photo.to_response();
                        photo_response.category_name = category_name;
                        photos.push(photo_response);
                    },
                    Err(e) => {
                        eprintln!("Error processing photo: {}", e);
                        continue;
                    }
                }
            }
            println!("📸 Found {} photos in database", photos.len());
            Ok(Json(photos))
        },
        Err(e) => {
            eprintln!("❌ Error fetching photos from MongoDB: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Deletes a specific photo
/// 
/// # Arguments
/// * `db` - MongoDB database connection
/// * `id` - ID of the photo to delete
/// 
/// # Returns
/// * `Ok(StatusCode::NO_CONTENT)` - Photo was successfully deleted
/// * `Err(StatusCode::NOT_FOUND)` - Photo with given ID was not found
/// * `Err(StatusCode::BAD_REQUEST)` - Invalid ID format
/// * `Err(StatusCode::INTERNAL_SERVER_ERROR)` - Database error
pub async fn delete_photo(
    State(db): State<Arc<Database>>,
    AxumPath(id): AxumPath<String>,
) -> Result<StatusCode, StatusCode> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    match db.collection::<Photo>("photos")
        .delete_one(doc! { "_id": object_id }, None)
        .await {
        Ok(result) if result.deleted_count == 1 => Ok(StatusCode::NO_CONTENT),
        Ok(_) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
