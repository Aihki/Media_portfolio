//! 3D Model type and its response type
//! 
//! Defines the structure for 3D model storage and API responses

use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

/// Represents a 3D model in the database
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    /// MongoDB ObjectId, optional for new models
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// Name of the 3D model
    pub name: String,
    /// Filename of the stored model
    pub filename: String,
    /// Category ID the model belongs to
    pub category_id: ObjectId,
    /// Timestamp when the model was created
    pub created_at: DateTime,
}

/// API response structure for 3D models
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelResponse {
    pub id: String,
    pub name: String,
    pub filename: String,
    pub url: String,
    pub category_id: String,
    pub category_name: String,
    pub created_at: DateTime,
}

impl Model {
    /// Creates a new Model instance
    /// 
    /// # Arguments
    /// * `name` - Name of the 3D model
    /// * `filename` - Name of the stored file
    /// * `category_id` - ID of the category this model belongs to
    pub fn new(name: String, filename: String, category_id: ObjectId) -> Self {
        Self {
            id: None,
            name,
            filename,
            category_id,
            created_at: DateTime::now(),
        }
    }

    /// Converts the Model into a ModelResponse
    pub fn to_response(&self) -> ModelResponse {
        ModelResponse {
            id: self.id.unwrap_or_default().to_string(),
            name: self.name.clone(),
            filename: self.filename.clone(),
            url: format!("/static/models/{}", self.filename),
            category_id: self.category_id.to_string(),
            category_name: String::new(),  
            created_at: self.created_at,
        }
    }
}
