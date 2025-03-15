# Media Portfolio Project

A full-stack web application for showcasing media content including photos, videos, and 3D models with categorization, admin authentication, and user feedback features.



## Project Overview

This portfolio project consists of two main components:

- **Backend API**: A Rust-based server using Axum framework with MongoDB for data storage
- **Frontend**: A Vue 3 application with TypeScript, TailwindCSS, and various media handling capabilities

### Key Features

- **Media Management**: Upload, view, and manage photos, videos, and 3D models
- **3D Model Viewer**: Interactive 3D model viewing using BabylonJS
- **Authentication**: Secure admin authentication for content management
- **Feedback System**: Allow visitors to provide feedback on content using Firebase
- **AI Integration**: Image classification using TensorFlow.js for automated tagging
- **Responsive Design**: Mobile-friendly interface with TailwindCSS
- **Pixel Art Canvas**: Interactive pixel art creation tool

## Tech Stack

### Backend
- **Language**: Rust
- **Framework**: Axum
- **Database**: MongoDB
- **Features**: Multipart file uploads, CORS support

### Frontend
- **Framework**: Vue 3 with Composition API
- **State Management**: Pinia
- **Styling**: TailwindCSS
- **Router**: Vue Router
- **3D Rendering**: BabylonJS
- **Image Classification**: TensorFlow.js with MobileNet
- **Feedback Storage**: Firebase Firestore
- **HTTP Client**: Axios

## Getting Started

### Prerequisites

- Rust (latest stable)
- Node.js (v16+)
- MongoDB (running instance)
- Firebase account (for feedback features)

### Backend Setup

1. Navigate to the backend directory:
   ```
   cd backend-api
   ```

2. Create a `.env` file with the following variables:
   ```
   MONGODB_URI=mongodb://localhost:27017
   DATABASE_NAME=portfolio
   ```

3. Run the development server:
   ```
   cargo run
   ```
   The server will be available at `http://localhost:3000`.

### Frontend Setup

1. Navigate to the frontend directory:
   ```
   cd portfolio-fe
   ```

2. Create a `.env` file with the following variables:
   ```
   VITE_API_URL=http://localhost:3000
   VITE_FIREBASE_API_KEY=your-firebase-api-key
   VITE_FIREBASE_AUTH_DOMAIN=your-project.firebaseapp.com
   VITE_FIREBASE_PROJECT_ID=your-project-id
   VITE_FIREBASE_STORAGE_BUCKET=your-storage-bucket
   VITE_FIREBASE_MESSAGING_SENDER_ID=your-sender-id
   VITE_FIREBASE_APP_ID=your-app-id
   ```

3. Install dependencies:
   ```
   npm install
   ```

4. Run the development server:
   ```
   npm run dev
   ```
   The app will be available at `http://localhost:5173`.

## Project Structure

### Backend Structure
```
backend-api/
├── src/
│   ├── handlers/         # Request handlers for different resources
│   ├── models/           # Database models and schemas
│   ├── routes/           # API route definitions
│   ├── db.rs             # Database connection management
│   ├── lib.rs            # Library exports
│   └── main.rs           # Application entrypoint
├── static/               # Static file storage
│   ├── models/           # Stored 3D models
│   ├── photos/           # Stored photos
│   └── videos/           # Stored videos
└── Cargo.toml            # Rust dependencies
```

### Frontend Structure
```
portfolio-fe/
├── src/
│   ├── components/       # Reusable Vue components
│   ├── views/            # Page components
│   ├── utils/            # Utility functions and services
│   ├── services/         # API service layers
│   ├── types/            # TypeScript type definitions
│   ├── router/           # Vue Router configuration
│   ├── api.ts            # API client functions
│   └── main.ts           # Application entrypoint
├── public/               # Public static assets
└── package.json          # Node.js dependencies
```

## API Endpoints

### Authentication
- `POST /api/login` - Admin authentication

### Photos
- `GET /api/photos` - List all photos
- `GET /api/photos/details` - Get photos with details
- `POST /api/upload-photo` - Upload a new photo
- `DELETE /api/photos/:id` - Delete a photo

### Videos
- `GET /api/videos` - List all videos
- `GET /api/videos/details` - Get videos with details
- `POST /api/upload-video` - Upload a new video
- `DELETE /api/videos/:id` - Delete a video

### Models (3D)
- `GET /api/models` - List all 3D models
- `GET /api/models/details` - Get 3D models with details
- `POST /api/upload-model` - Upload a new 3D model
- `DELETE /api/models/:id` - Delete a 3D model

### Categories
- `GET /api/categories` - List all categories
- `POST /api/categories` - Create a new category

### Statistics
- `GET /api/stats` - Get content statistics



