use mongodb::{Client, Database};
use std::env;

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
