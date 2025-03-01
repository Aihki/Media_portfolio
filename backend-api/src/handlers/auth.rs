//! Authentication handler module
//! 
//! Provides functionality for user authentication and JWT token generation.

use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use chrono::{self, Utc, Duration};

use crate::models::admin::{Admin, LoginCredentials, LoginResponse};

/// JWT claims structure for token generation
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

/// Handles admin user login requests
/// 
/// # Arguments
/// 
/// * `db` - MongoDB database connection
/// * `credentials` - Login credentials containing username and password
/// 
/// # Returns
/// 
/// Returns a JWT token on successful authentication, or appropriate error status
pub async fn login_handler(
    State(db): State<Arc<Database>>,
    Json(credentials): Json<LoginCredentials>,
) -> Result<Json<LoginResponse>, StatusCode> { 
    let collection = db.collection::<Admin>("admin");


    let query = mongodb::bson::doc! { 
        "Username": &credentials.username  
    };    
    match collection.find_one(query, None).await {
        Ok(Some(admin)) => {
            if admin.password == credentials.password {
                let claims = Claims {
                    sub: admin.username,
                    exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
                };

                match encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret("your-secret-key".as_bytes()),
                ) {
                    Ok(token) => Ok(Json(LoginResponse { token })),
                    Err(e) => {
                        eprintln!("❌ Token creation failed: {:?}", e);
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            } else {
                println!("❌ Password mismatch.");
                Err(StatusCode::UNAUTHORIZED)
            }
        },
        Ok(None) => {
            println!("❌ No user found with username: {}", credentials.username);
            Err(StatusCode::UNAUTHORIZED)
        },
        Err(e) => {
            eprintln!("❌ Database error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
