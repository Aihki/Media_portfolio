//! Data models for the portfolio application
//! 
//! This module contains all the data structures used in the application:
//! - `admin`: Authentication and user management
//! - `category`: Content categorization
//! - `photo`: Photo storage and responses
//! - `model`: 3D model storage and responses
//! - `video`: Video storage and responses

pub mod admin;
pub mod category;
pub mod photo;
pub mod model;
pub mod video;

pub use category::Category;
pub use photo::{Photo, PhotoResponse};
pub use model::{Model, ModelResponse};
pub use video::{Video, VideoResponse};
