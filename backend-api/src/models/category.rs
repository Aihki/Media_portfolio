//! Category model definition
//! 
//! Defines the structure for content categories in the portfolio

use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

/// Represents a content category in the database
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    /// MongoDB ObjectId, optional for new categories
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// Name of the category
    pub name: String,
}
