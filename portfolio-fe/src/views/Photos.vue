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
      <div v-if="loading" class="text-center py-4">
        <div
          class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto"
        ></div>
      </div>
      <div v-if="error" class="text-red-400 text-center py-4">
        {{ error }}
      </div>

      <div
        v-if="!loading && photos.length > 0"
        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
      >
        <div
          v-for="photo in photos"
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
            <button
              v-if="authStore.isAuthenticated"
              @click="deletePhoto(photo.id)"
              class="absolute top-2 right-2 bg-red-600 hover:bg-red-700 text-white rounded-full p-2 transition-colors"
              aria-label="Delete photo"
            >
              <i class="pi pi-trash"></i>
            </button>
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
        v-if="!loading && photos.length === 0"
        class="text-gray-400 text-center py-4"
      >
        No photos found
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import {
    uploadPhoto as uploadPhotoAPI,
    getPhotosWithDetails,
    deletePhoto as deletePhotoAPI,
    type Photo,
  } from '@/api';
  import PhotoView from '@/components/PhotoView.vue';
  import UploadForm from '@/components/UploadForm.vue';
  import { useAuthStore } from '@/utils/AuthStore';

  const photos = ref<Photo[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const photoView = ref<{ url: string; name: string } | null>(null);
  const authStore = useAuthStore();

  const fetchPhotos = async () => {
    try {
      loading.value = true;
      error.value = null;
      photos.value = await getPhotosWithDetails();
      console.log('Loaded photos with details:', photos.value);
    } catch (err) {
      error.value = 'Failed to load photos';
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
    } catch (error) {
      console.error('Upload error:', error);
      error.value =
        error instanceof Error ? error.message : 'Failed to upload photo';
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
      await fetchPhotos(); // Refresh the list
    } catch (err) {
      error.value = 'Failed to delete photo';
      console.error('Delete error:', err);
    } finally {
      loading.value = false;
    }
  };

  onMounted(fetchPhotos);
</script>
