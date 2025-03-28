<template>
  <div class="flex flex-col gap-6">
    <div v-if="authStore.isAuthenticated" class="bg-gray-800 shadow-lg rounded-lg p-6">
      <h2 class="text-xl font-bold text-gray-200 mb-4 text-center">Upload New Video</h2>
      <div class="border-2 border-gray-700 rounded-lg p-4">
        <UploadForm
          type="Video"
          @upload="uploadVideo"
          acceptTypes="video/*"
          :maxSize="100 * 1024 * 1024"
        />
      </div>
    </div>

    <div class="bg-gray-800 shadow-lg rounded-lg p-6">
      <h2 class="text-xl font-bold text-gray-200 mb-4 text-center">Video Gallery</h2>
      
      <CategoryFilter @filter="filterVideos" />

      <div v-if="loading" class="text-center py-4">
        <div
          class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto"
        ></div>
      </div>
      <div v-if="error" class="text-red-400 text-center py-4">
        {{ error }}
      </div>

      <div
        v-if="!loading && filteredVideos.length > 0"
        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
      >
        <div
          v-for="video in filteredVideos"
          :key="video.id"
          class="border border-gray-700 rounded-lg overflow-hidden bg-gray-900 relative"
        >
          <div class="absolute bottom-2 right-2 flex gap-2">
            <button
              @click.stop="openFeedback(video)"
              class="bg-blue-600 hover:bg-blue-700 text-white p-2 rounded transition-colors"
              aria-label="Give feedback"
            >
              <i class="pi pi-star"></i>
            </button>
            <button
              v-if="authStore.isAuthenticated"
              @click="deleteVideo(video.id)"
              class="bg-red-600 hover:bg-red-700 text-white p-2 rounded transition-colors"
              aria-label="Delete video"
            >
              <i class="pi pi-trash"></i>
            </button>
          </div>
          <video
            controls
            class="w-full aspect-video"
            @error="handleVideoError"
            preload="metadata"
            crossorigin="anonymous"
          >
            <source :src="video.url" type="video/mp4" />
            <source :src="video.url" type="video/webm" />
            Your browser does not support the video tag.
          </video>
          <div class="p-4 text-gray-300">
            <h3 class="font-semibold">{{ video.name }}</h3>
            <p class="text-sm text-gray-400">Category: {{ video.category_name }}</p>
          </div>
        </div>
      </div>
      <div
        v-if="!loading && filteredVideos.length === 0"
        class="text-gray-400 text-center py-4"
      >
        No videos found
      </div>
    </div>
  </div>

  <FeedbackModal
    v-if="feedbackItem"
    :is-open="!!feedbackItem"
    :content-id="feedbackItem?.id"
    content-type="video"
    @close="feedbackItem = null"
    @submitted="handleFeedbackSubmitted"
  />
</template>

<script setup lang="ts">
  import { ref, onMounted, computed } from 'vue';
  import { getVideosWithDetails, type Video, uploadVideo as uploadVideoAPI, deleteVideo as deleteVideoAPI } from '../api';
  import UploadForm from '../components/UploadForm.vue';
  import { useAuthStore } from '../utils/AuthStore';
  import CategoryFilter from '../components/CategoryFilter.vue';
  import FeedbackModal from '../components/FeedbackModal.vue';

  interface UploadData {
    file: File;
    name: string;
    categoryId: { $oid: string } | string;
  }

  const videos = ref<Video[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const authStore = useAuthStore();

  const selectedCategories = ref<string[] | null>(null);

  const filteredVideos = computed(() => {
    if (!selectedCategories.value || selectedCategories.value.length === 0) return videos.value;
    return videos.value.filter(video => selectedCategories.value?.includes(video.category_id));
  });

  const filterVideos = (categories: string[] | null) => {
    selectedCategories.value = categories;
  };

  const fetchVideos = async () => {
    try {
      loading.value = true;
      error.value = null;
      videos.value = await getVideosWithDetails();
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

    videoElement.load();
  };

  const uploadVideo = async (uploadData: UploadData) => {
    try {
      loading.value = true;
      error.value = null;

      console.log('Uploading video:', {
        file: uploadData.file.name,
        name: uploadData.name,
        categoryId: uploadData.categoryId
      });

      await uploadVideoAPI({
        file: uploadData.file,
        name: uploadData.name,
        categoryId: typeof uploadData.categoryId === 'object' && '$oid' in uploadData.categoryId 
          ? uploadData.categoryId.$oid 
          : uploadData.categoryId
      });

      await fetchVideos();
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to upload video';
      console.error('Upload error:', err);
    } finally {
      loading.value = false;
    }
  };

  const deleteVideo = async (id: string) => {
    if (!confirm('Are you sure you want to delete this video?')) return;
    
    try {
      loading.value = true;
      await deleteVideoAPI(id);
      await fetchVideos();
    } catch (err) {
      error.value = 'Failed to delete video';
      console.error('Delete error:', err);
    } finally {
      loading.value = false;
    }
  };

  const feedbackItem = ref<Video | null>(null);

  function openFeedback(video: Video) {
    feedbackItem.value = video;
  }

  function handleFeedbackSubmitted() {
    console.log('Feedback submitted successfully');
  }

  onMounted(fetchVideos);
</script>

<style scoped>
  video {
    max-height: 500px;
    object-fit: contain;
    background-color: #1a1a1a;
  }
</style>
