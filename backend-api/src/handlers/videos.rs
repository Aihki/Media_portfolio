use axum::{extract::Multipart, Json, http::StatusCode};
use std::{fs, path::PathBuf};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use serde_json::json;

pub const VIDEO_FOLDER: &str = "static/videos";

pub async fn upload_video(mut multipart: Multipart) -> Result<Json<serde_json::Value>, StatusCode> {
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        eprintln!("Error getting next field: {}", e);
        StatusCode::BAD_REQUEST
    })? {
        let filename = field.file_name()
            .map(|f| f.to_string())
            .unwrap_or_else(|| format!("{}.mp4", Uuid::new_v4()));
        
        let filepath = format!("{}/{}", VIDEO_FOLDER, filename);
        println!("🎥 Attempting to save video to: {}", filepath);
        
        if !PathBuf::from(VIDEO_FOLDER).exists() {
            fs::create_dir_all(VIDEO_FOLDER).map_err(|e| {
                eprintln!("Failed to create directory: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        }

        let data = match field.bytes().await {
            Ok(data) => {
                if data.is_empty() {
                    eprintln!("Received empty file data");
                    return Err(StatusCode::BAD_REQUEST);
                }
                data
            },
            Err(e) => {
                eprintln!("Failed to read upload data: {}", e);
                return Err(StatusCode::BAD_REQUEST);
            }
        };
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

        let url = format!("/static/videos/{}", filename);
        println!("✅ Video saved successfully: {}", url);
        
        return Ok(Json(json!({
            "url": url,
            "filename": filename
        })));
    }

    eprintln!("No file field found in request");
    Err(StatusCode::BAD_REQUEST)
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
