use axum::{extract::{Multipart, State}, Json, http::StatusCode};
use std::{fs, path::PathBuf, sync::Arc};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use serde_json::json;
use mongodb::Database;
use crate::models::Video;

pub const VIDEO_FOLDER: &str = "static/videos";

pub async fn upload_video(
    State(db): State<Arc<Database>>,
    mut multipart: Multipart
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut name = String::new();
    let mut category_id = String::new();
    let mut saved_filename = String::new();

    println!("Starting video upload...");

    while let Some(mut field) = multipart.next_field().await.map_err(|e| {
        eprintln!("Error getting next field: {}", e);
        StatusCode::BAD_REQUEST
    })? {
        println!("Processing field: {:?}", field.name());

        match field.name() {
            Some("name") => {
                name = field.text().await.map_err(|e| {
                    eprintln!("Error reading name: {}", e);
                    StatusCode::BAD_REQUEST
                })?;
                println!("Got name: {}", name);
            },
            Some("category") => {
                category_id = field.text().await.map_err(|e| {
                    eprintln!("Error reading category: {}", e);
                    StatusCode::BAD_REQUEST
                })?;
                println!("Got category_id: {}", category_id);
            },
            Some("file") => {
                let filename = format!("video_{}_{}.mp4", 
                    Uuid::new_v4(),
                    chrono::Local::now().format("%Y%m%d")
                );
                
                let filepath = format!("{}/{}", VIDEO_FOLDER, filename);
                println!("📹 Saving video to: {}", filepath);
                
                if !PathBuf::from(VIDEO_FOLDER).exists() {
                    fs::create_dir_all(VIDEO_FOLDER).map_err(|e| {
                        eprintln!("Failed to create directory: {}", e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;
                }

                let mut file = File::create(&filepath).await.map_err(|e| {
                    eprintln!("Failed to create file: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

                // Process chunks
                while let Some(chunk) = field.chunk().await.map_err(|e| {
                    eprintln!("Error reading chunk: {}", e);
                    StatusCode::BAD_REQUEST
                })? {
                    file.write_all(&chunk).await.map_err(|e| {
                        eprintln!("Error writing chunk: {}", e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;
                }

                saved_filename = filename;
                println!("✅ Video saved successfully: {}", saved_filename);
            },
            _ => {
                println!("Skipping unknown field: {:?}", field.name());
            }
        }
    }

    // Validate fields
    if name.is_empty() || category_id.is_empty() || saved_filename.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Convert category_id to ObjectId
    let category_object_id = mongodb::bson::oid::ObjectId::parse_str(&category_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Create and save video document
    let video = Video::new(name, saved_filename.clone(), category_object_id);
    
    match db.collection::<Video>("videos")
        .insert_one(video, None)
        .await {
        Ok(_) => {
            let url = format!("/static/videos/{}", saved_filename);
            Ok(Json(json!({
                "url": url,
                "filename": saved_filename
            })))
        },
        Err(e) => {
            eprintln!("Failed to save video to database: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn list_videos() -> Result<Json<Vec<String>>, StatusCode> {
    if !PathBuf::from(VIDEO_FOLDER).exists() {
        fs::create_dir_all(VIDEO_FOLDER)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    match fs::read_dir(VIDEO_FOLDER) {
        Ok(entries) => {
            let videos = entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        if e.path().is_file() {
                            Some(format!("/static/videos/{}", 
                                e.file_name().to_string_lossy()))
                        } else {
                            None	
                        }
                    })
                })
                .collect();
            
            Ok(Json(videos))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
