//! Handler modules for the portfolio backend API
//! 
//! This module contains all the request handlers for different resources:
//! - `photos`: Handles photo upload, retrieval and management
//! - `models`: Handles 3D model upload, retrieval and management
//! - `videos`: Handles video upload, retrieval and management
//! - `auth`: Handles authentication and authorization
//! - `categories`: Handles category management
//! - `stats`: Handles statistics retrieval

pub mod photos;
pub mod models;
pub mod videos;
pub mod auth;
pub mod categories;
pub mod stats;

