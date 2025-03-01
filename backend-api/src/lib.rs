//! Portfolio Backend API
//! 
//! This is the main documentation for the Portfolio Backend API.
//! The API provides endpoints for managing photos, 3D models, videos,
//! and their categories.
//! This library provides the core functionality for the portfolio backend:
//! - Data models for content and users
//! - Request handlers for all API endpoints
//! - Routing configuration
//! - Database connection management

pub mod models;
pub mod handlers;
pub mod routes;
pub mod db;
