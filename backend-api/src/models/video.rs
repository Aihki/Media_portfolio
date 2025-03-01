//! Video model and its response type
//! 
//! Defines the structure for video storage and API responses

use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

/// Represents a video in the database
#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    /// MongoDB ObjectId, optional for new videos
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// Name of the video
    pub name: String,
    /// Filename of the stored video
    pub filename: String,
    /// Category ID the video belongs to
    pub category_id: ObjectId,
    /// Timestamp when the video was created
    pub created_at: DateTime,
}

/// API response structure for videos
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoResponse {
    pub id: String,
    pub name: String,
    pub filename: String,
    pub url: String,
    pub category_id: String,
    pub category_name: String,
    pub created_at: DateTime,
}

impl Video {
    /// Creates a new Video instance
    /// 
    /// # Arguments
    /// * `name` - Name of the video
    /// * `filename` - Name of the stored file
    /// * `category_id` - ID of the category this video belongs to
    pub fn new(name: String, filename: String, category_id: ObjectId) -> Self {
        Self {
            id: None,
            name,
            filename,
            category_id,
            created_at: DateTime::now(),
        }
    }

    /// Converts the Video into a VideoResponse
    pub fn to_response(&self) -> VideoResponse {
        VideoResponse {
            id: self.id.unwrap_or_default().to_string(),
            name: self.name.clone(),
            filename: self.filename.clone(),
            url: format!("/static/videos/{}", self.filename),
            category_id: self.category_id.to_string(),
            category_name: String::new(), 
            created_at: self.created_at,
        }
    }
}
