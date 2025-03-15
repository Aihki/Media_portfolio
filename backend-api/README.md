    # Portfolio Backend API

A Rust-based backend API for the media portfolio application, built with Axum framework and MongoDB.

## Features



- MongoDB integration for data persistence
- File storage for photos, videos, and 3D models
- CORS configuration for frontend integration

## Tech Stack

- **Language**: Rust
- **Web Framework**: Axum
- **Database**: MongoDB (via mongodb crate)
- **File Handling**: Multipart form data processing
- **CORS**: tower-http middleware

## Project Structure

```
src/
├── handlers/         # Request handlers for API endpoints
│   ├── auth.rs       # Authentication handlers
│   ├── categories.rs # Category management
│   ├── models.rs     # 3D model handling
│   ├── photos.rs     # Photo handling
│   ├── stats.rs      # Statistics endpoints
│   ├── videos.rs     # Video handling
│   └── mod.rs        # Module exports
├── models/           # Data models
│   ├── admin.rs      # Admin user model
│   ├── category.rs   # Category model
│   ├── model.rs      # 3D model data structure
│   ├── photo.rs      # Photo data structure
│   ├── video.rs      # Video data structure
│   └── mod.rs        # Module exports
├── routes.rs         # API route definitions
├── db.rs             # Database connection management
├── lib.rs            # Library exports
└── main.rs           # Application entry point
```

## Getting Started

### Prerequisites

- Rust (latest stable)
- MongoDB (running instance)
- Cargo (Rust package manager)

### Environment Variables

Create a `.env` file in the project root with:

```
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=portfolio
```

### Running the API

1. Build and run the project:
   ```
   cargo run
   ```

2. The API will be available at `http://localhost:3000`

## API Endpoints

### Authentication
- `POST /api/login` - Admin login

### Photos
- `GET /api/photos` - List all photo files
- `GET /api/photos/details` - Get detailed information about photos
- `POST /api/upload-photo` - Upload a new photo (multipart/form-data)
- `DELETE /api/photos/:id` - Delete a photo by ID

### Videos
- `GET /api/videos` - List all video files
- `GET /api/videos/details` - Get detailed information about videos
- `POST /api/upload-video` - Upload a new video (multipart/form-data)
- `DELETE /api/videos/:id` - Delete a video by ID

### 3D Models
- `GET /api/models` - List all model files
- `GET /api/models/details` - Get detailed information about models
- `POST /api/upload-model` - Upload a new 3D model (multipart/form-data)
- `DELETE /api/models/:id` - Delete a model by ID

### Categories
- `GET /api/categories` - List all categories
- `POST /api/categories` - Create a new category

### Statistics
- `GET /api/stats` - Get content statistics (counts of photos, videos, models)

## Static File Access

Static files are served from:
- `GET /static/photos/{filename}` - Access uploaded photos
- `GET /static/videos/{filename}` - Access uploaded videos
- `GET /static/models/{filename}` - Access uploaded 3D models
