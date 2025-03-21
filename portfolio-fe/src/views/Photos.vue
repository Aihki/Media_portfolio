<template>
  <div class="flex flex-col gap-6">
    <div
      v-if="authStore.isAuthenticated"
      class="bg-gray-800 shadow-lg rounded-lg p-6"
    >
      <h2 class="text-xl font-bold text-gray-200 mb-4">Upload New Photo</h2>
      <div class="border-2 border-gray-700 rounded-lg p-4">
        <UploadForm
          type="Photo"
          acceptTypes="image/*"
          :maxSize="10 * 1024 * 1024"
          @upload="handleUpload"
        />
      </div>
    </div>

    <div class="bg-gray-800 shadow-lg rounded-lg p-6">
      <h2 class="text-xl font-bold text-gray-200 mb-4 text-center">
        Photo Gallery
      </h2>
      
      <CategoryFilter @filter="filterPhotos" />

      <div v-if="loading" class="text-center py-4">
        <div
          class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto"
        ></div>
      </div>
      <div v-if="error" class="text-red-400 text-center py-4">
        {{ error }}
      </div>

      <div
        v-if="!loading && filteredPhotos.length > 0"
        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
      >
        <div
          v-for="photo in filteredPhotos"
          :key="photo.id"
          class="border border-gray-700 rounded-lg p-4 shadow-md bg-gray-900"
        >
          <div class="relative">
            <img
              :src="photo.url"
              :alt="photo.name"
              class="w-full h-auto aspect-video object-cover rounded cursor-pointer"
              @error="handleImageError"
              @click="photoView = { url: photo.url, name: photo.name }"
            />
            <div class="absolute bottom-2 right-2 flex gap-2">
              <button
                @click.stop="openFeedback(photo)"
                class="bg-blue-600 hover:bg-blue-700 text-white p-2 rounded transition-colors"
                aria-label="Give feedback"
              >
                <i class="pi pi-star"></i>
              </button>
              <button
                v-if="authStore.isAuthenticated"
                @click="deletePhoto(photo.id)"
                class="bg-red-600 hover:bg-red-700 text-white p-2 transition-colors"
                aria-label="Delete photo"
              >
                <i class="pi pi-trash"></i>
              </button>
            </div>
          </div>
          <div class="mt-2 text-gray-300">
            <h3 class="font-semibold">{{ photo.name }}</h3>
            <p class="text-sm text-gray-400">
              Category: {{ photo.category_name }}
            </p>
          </div>
        </div>
      </div>

      <PhotoView
        v-if="photoView"
        :photoUrl="photoView.url"
        :photoName="photoView.name"
        @close="photoView = null"
      />
      <div
        v-if="!loading && filteredPhotos.length === 0"
        class="text-gray-400 text-center py-4"
      >
        No photos found
      </div>
    </div>
  </div>

  <FeedbackModal
    v-if="feedbackItem"
    :is-open="!!feedbackItem"
    :content-id="feedbackItem?.id"
    content-type="photo"
    @close="feedbackItem = null"
    @submitted="handleFeedbackSubmitted"
  />
</template>

<script setup lang="ts">
  import { ref, onMounted, computed } from 'vue';
  import {
    uploadPhoto as uploadPhotoAPI,
    getPhotosWithDetails,
    deletePhoto as deletePhotoAPI,
    type Photo,
  } from '../api';
  import PhotoView from '../components/PhotoView.vue'; 
  import UploadForm from '../components/UploadForm.vue';
  import { useAuthStore } from '../utils/AuthStore'; 
  import CategoryFilter from '../components/CategoryFilter.vue';
  import FeedbackModal from '../components/FeedbackModal.vue';

  const photos = ref<Photo[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const photoView = ref<{ url: string; name: string } | null>(null);
  const authStore = useAuthStore();
  const selectedCategories = ref<string[] | null>(null);
  const feedbackItem = ref<Photo | null>(null);

  const filteredPhotos = computed(() => {
    if (!selectedCategories.value || selectedCategories.value.length === 0) return photos.value;
    return photos.value.filter(photo => selectedCategories.value?.includes(photo.category_id));
  });

  const fetchPhotos = async () => {
    try {
      loading.value = true;
      error.value = null;
      photos.value = await getPhotosWithDetails();
      console.log('Loaded photos with details:', photos.value);
      
      // Handle case when API is unreachable
      if (photos.value.length === 0) {
        error.value = 'Could not connect to server. Please make sure the backend is running.';
      }
    } catch (err) {
      error.value = 'Failed to load photos. Is the server running?';
      console.error('Error loading photos:', err);
    } finally {
      loading.value = false;
    }
  };

  const handleUpload = async (uploadData: {
    file: File;
    name: string;
    categoryId: { $oid: string } | string;
  }) => {
    try {
      console.log('Photos component received:', {
        hasFile: !!uploadData.file,
        fileName: uploadData.file?.name,
        name: uploadData.name,
        categoryId: uploadData.categoryId,
      });

      const data = {
        file: uploadData.file,
        name: uploadData.name,
        categoryId:
          typeof uploadData.categoryId === 'object' &&
          '$oid' in uploadData.categoryId
            ? uploadData.categoryId.$oid
            : uploadData.categoryId,
      };

      await uploadPhotoAPI(data);
      await fetchPhotos();
    } catch (err) {
      console.error('Upload error:', err);
      if (err instanceof Error) {
        error.value = err.message;
      } else {
        error.value = 'Failed to upload photo';
      }
    }
  };

  const handleImageError = (event: Event) => {
    const img = event.target as HTMLImageElement;
    console.error('Failed to load image:', img.src);
    error.value = `Failed to load image: ${img.src.split('/').pop()}`;
  };

  const deletePhoto = async (id: string) => {
    if (!confirm('Are you sure you want to delete this photo?')) return;
    
    try {
      loading.value = true;
      await deletePhotoAPI(id);
      await fetchPhotos();
    } catch (err) {
      error.value = 'Failed to delete photo';
      console.error('Delete error:', err);
    } finally {
      loading.value = false;
    }
  };

  const filterPhotos = (categories: string[] | null) => {
    selectedCategories.value = categories;
  };

  function openFeedback(photo: Photo) {
    feedbackItem.value = photo;
  }

  function handleFeedbackSubmitted() {
    console.log('Feedback submitted successfully');
  }

  onMounted(fetchPhotos);
</script>
