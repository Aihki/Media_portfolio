<template>
  <div class="bg-gray-800 shadow-lg rounded-lg p-6">
    <!-- Upload Form -->
   <UploadForm 
    type="Video"
      @upload="uploadVideo" 
      acceptTypes="video/*" 
      :maxSize="100 * 1024 * 1024"
    />

    <!-- Loading and Error States -->
    <div v-if="loading" class="text-center py-4">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto"></div>
    </div>
    <div v-if="error" class="text-red-400 text-center py-4">
      {{ error }}
    </div>

    <div v-if="!loading && videos.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div v-for="video in videos" :key="video" class="border border-gray-700 rounded-lg overflow-hidden bg-gray-900">
        <video 
          controls 
          class="w-full aspect-video"
          @error="handleVideoError"
          preload="metadata"
          crossorigin="anonymous"
        >
          <source :src="video" type="video/mp4">
          <source :src="video" type="video/webm">
          Your browser does not support the video tag.
        </video>
      </div>
    </div>
    <div v-if="!loading && videos.length === 0" class="text-gray-400 text-center py-4">
      No videos found
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { uploadVideo as uploadVideoAPI, listVideos } from '@/api';
import UploadForm from "@/components/UploadForm.vue";

const videoFile = ref<File | null>(null);
const videos = ref<string[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);

const handleVideoUpload = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;
  
  if (!file.type.startsWith('video/')) {
    error.value = 'Please select a valid video file';
    return;
  }
    
  if (file.size > 100 * 1024 * 1024) { // 100MB limit
    error.value = 'Video file is too large. Maximum size is 100MB';
    return;
  }
  
  videoFile.value = file;
  error.value = null;
};

const fetchVideos = async () => {
  try {
    loading.value = true;
    error.value = null;
    videos.value = await listVideos();
  } catch (err) {
    error.value = 'Failed to load videos';
    console.error('Error loading videos:', err);
  } finally {
    loading.value = false;
  }
};

const handleVideoError = (event: Event) => {
  const videoElement = event.target as HTMLVideoElement;
  console.error('Video loading error:', videoElement.error);
  

  const currentSrc = videoElement.currentSrc;
  if (currentSrc.endsWith('mp4')) {
    const webmSrc = currentSrc.replace('.mp4', '.webm');
    videoElement.src = webmSrc;
  } else if (currentSrc.endsWith('webm')) {
    const mp4Src = currentSrc.replace('.webm', '.mp4');
    videoElement.src = mp4Src;
  }
  
  error.value = 'Error loading video. Trying alternative format...';
  
  // Attempt to load the video again
  videoElement.load();
};

const uploadVideo = async () => {
  if (!videoFile.value) return;
  
  try {
    loading.value = true;
    error.value = null;
    
    const formData = new FormData();
    formData.append("video", videoFile.value, videoFile.value.name);
    formData.append("type", videoFile.value.type);
    
    await uploadVideoAPI(formData);
    
  
    await new Promise(resolve => setTimeout(resolve, 1000));
    await fetchVideos();
    

    videoFile.value = null;
    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
    if (input) input.value = '';
    
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to upload video';
    console.error('Upload error:', err);
  } finally {
    loading.value = false;
  }
};

onMounted(fetchVideos);
</script>

<style scoped>
video {
  max-height: 500px;
  object-fit: contain;
  background-color: #1a1a1a;
}
</style>