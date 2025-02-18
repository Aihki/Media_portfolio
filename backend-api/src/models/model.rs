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
}
