//! Photo model and its response type
//! 
//! Defines the structure for photo storage and API responses

use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

/// Represents a photo in the database
#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    /// MongoDB ObjectId, optional for new photos
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// Name of the photo
    pub name: String,
    /// Filename of the stored photo
    pub filename: String,
    /// Category ID the photo belongs to
    pub category_id: ObjectId,
    /// Timestamp when the photo was created
    pub created_at: DateTime,
}

/// API response structure for photos
#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoResponse {
    pub id: String,
    pub name: String,
    pub filename: String,
    pub url: String,
    pub category_id: String,
    pub category_name: String, 
    pub created_at: DateTime,
}

impl Photo {
    /// Creates a new Photo instance
    /// 
    /// # Arguments
    /// * `name` - Name of the photo
    /// * `filename` - Name of the stored file
    /// * `category_id` - ID of the category this photo belongs to
    pub fn new(name: String, filename: String, category_id: ObjectId) -> Self {
        Self {
            id: None,
            name,
            filename,
            category_id,
            created_at: DateTime::now(),
        }
    }

    /// Converts the Photo into a PhotoResponse
    pub fn to_response(&self) -> PhotoResponse {
        PhotoResponse {
            id: self.id.unwrap_or_default().to_string(),
            name: self.name.clone(),
            filename: self.filename.clone(),
            url: format!("/static/photos/{}", self.filename),
            category_id: self.category_id.to_string(),
            category_name: String::new(), 
            created_at: self.created_at,
        }
    }
}
