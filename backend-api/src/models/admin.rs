//! Admin authentication models
//! 
//! Defines structures for admin authentication and login responses

use serde::{Deserialize, Serialize};

/// Represents an admin user in the database
#[derive(Debug, Serialize, Deserialize)]
pub struct Admin {
    /// MongoDB ObjectId, optional for new admins
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    /// Admin username
    #[serde(rename = "Username")]  
    pub username: String,
    /// Admin password (should be hashed in production)
    #[serde(rename = "Password")] 
    pub password: String,
}

/// Login request credentials structure
#[derive(Debug, Deserialize)]
pub struct LoginCredentials {
    /// Username for login
    pub username: String,
    /// Password for login
    pub password: String,
}

/// Login response containing JWT token
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    /// JWT token for authenticated requests
    pub token: String,
}
