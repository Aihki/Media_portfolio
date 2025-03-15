# Portfolio Frontend Application

A Vue 3-based frontend application for the media portfolio, featuring photo, video, and 3D model display with interactive features.

## Features

- Interactive media gallery for photos, videos, and 3D models
- 3D model visualization using BabylonJS
- Image classification with TensorFlow.js and MobileNet
- User feedback collection with Firebase Firestore
- Pixel art creation canvas
- Admin authentication for content management
- Responsive design with TailwindCSS

## Tech Stack

- **Framework**: Vue 3 with Composition API
- **Build Tool**: Vite
- **Language**: TypeScript
- **Styling**: TailwindCSS
- **State Management**: Pinia
- **Routing**: Vue Router
- **HTTP Client**: Axios
- **3D Graphics**: BabylonJS
- **AI Integration**: TensorFlow.js with MobileNet
- **Database**: Firebase Firestore (for user feedback)
- **File Upload**: Multipart/form-data with progress tracking

## Project Structure

```
src/
├── components/         # Reusable Vue components
│   ├── CategoryFilter.vue  # Category filtering component
│   ├── FeedbackModal.vue   # User feedback modal
│   ├── ModelView.vue       # 3D model viewer
│   ├── PhotoView.vue       # Photo viewer
│   └── UploadForm.vue      # Media upload form
├── views/              # Page components
│   ├── Login.vue       # Admin login page
│   ├── Model.vue       # 3D model gallery page
│   ├── Photos.vue      # Photo gallery page
│   ├── PixelCanvas.vue # Pixel art creation page
│   ├── Sandbox.vue     # 3D model playground
│   ├── Videos.vue      # Video gallery page
│   └── Welcome.vue     # Admin dashboard
├── utils/              # Utility functions
│   ├── AuthStore.ts    # Authentication store
│   ├── firebase.ts     # Firebase configuration
│   └── imageClassifier.ts # TensorFlow image classification
├── services/
│   └── feedbackService.ts # Feedback data handling
├── types/              # TypeScript type definitions
├── router/             # Vue Router configuration
├── api.ts              # API client functions
├── App.vue             # Root component
└── main.ts             # Application entry point
```

## Getting Started

### Prerequisites

- Node.js (v16+)
- npm or yarn
- Backend API running (portfolio backend)
- Firebase project (for feedback features)

### Environment Variables

Create a `.env` file in the project root with:

```
VITE_API_URL=http://localhost:3000
VITE_FIREBASE_API_KEY=your-firebase-api-key
VITE_FIREBASE_AUTH_DOMAIN=your-project.firebaseapp.com
VITE_FIREBASE_PROJECT_ID=your-project-id
VITE_FIREBASE_STORAGE_BUCKET=your-storage-bucket
VITE_FIREBASE_MESSAGING_SENDER_ID=your-sender-id
VITE_FIREBASE_APP_ID=your-app-id
```

### Setup and Development

1. Install dependencies:
   ```
   npm install
   ```

2. Start the development server:
   ```
   npm run dev
   ```

3. The application will be available at `http://localhost:5173`

### Building for Production

```
npm run build
```

The built files will be in the `dist` directory.

## Key Components

### Media Galleries

- **Photos.vue**: Photo gallery with upload, filtering, and feedback
- **Videos.vue**: Video gallery with upload, filtering, and playback
- **Model.vue**: 3D model gallery with upload and interactive viewing

### Interactive Features

- **ModelView.vue**: 3D model viewer using BabylonJS
- **Sandbox.vue**: Interactive 3D model exploration space
- **PixelCanvas.vue**: Pixel art creation tool

### Admin Features

- **Login.vue**: Admin authentication
- **Welcome.vue**: Dashboard with portfolio statistics
- Content management (upload/delete) in each gallery

### AI Features

- **imageClassifier.ts**: TensorFlow.js integration for automatic image classification
- Automatic tag suggestions for uploaded photos

## Feedback System

The application uses Firebase Firestore to collect and store user feedback:

- **FeedbackModal.vue**: Modal for collecting star ratings and comments
- **feedbackService.ts**: Firebase integration for storing feedback

## API Integration

All backend communication is handled through the `api.ts` file, which provides typed functions for:

- Authentication
- Media uploads
- Retrieving content with details
- Content deletion
- Categories management
