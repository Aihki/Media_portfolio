//! Database connection management
//! 
//! Handles MongoDB connection initialization and configuration.
//! Requires the MONGODB_URI environment variable to be set.

use mongodb::{Client, Database};
use std::env;

/// Establishes a connection to MongoDB
/// 
/// # Environment Variables Required
/// 
/// * `MONGODB_URI` - MongoDB connection string
/// 
/// # Returns
/// 
/// * `Ok(Database)` - Connected MongoDB database instance
/// * `Err(mongodb::error::Error)` - Connection error
/// 
/// # Panics
/// 
/// Panics if MONGODB_URI environment variable is not set
pub async fn connect() -> Result<Database, mongodb::error::Error> {
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    
    println!("📡 Attempting to connect to MongoDB...");   
    let client = Client::with_uri_str(&uri).await?;
    

    match client.list_database_names(None, None).await {
        Ok(_names) => {
            println!("✅ Successfully connected to MongoDB!");
        },
        Err(e) => {
            eprintln!("❌ Failed to verify MongoDB connection");
            eprintln!("Error details: {:?}", e);
            return Err(e);
        }
    }


    let database = client.database("Portfolio");
    println!("📊 Using database: Portfolio");
    
    Ok(database)
}
