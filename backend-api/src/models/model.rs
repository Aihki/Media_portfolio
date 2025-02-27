use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub filename: String,
    pub category_id: ObjectId,
    pub created_at: DateTime,
}

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
    pub fn new(name: String, filename: String, category_id: ObjectId) -> Self {
        Self {
            id: None,
            name,
            filename,
            category_id,
            created_at: DateTime::now(),
        }
    }

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
