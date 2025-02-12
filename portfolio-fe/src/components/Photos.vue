<template>
  <div class="bg-gray-800 shadow-lg rounded-lg p-6">
    <!-- Upload Form -->
    <div class="mb-8 border-b border-gray-700 pb-6">
      <h3 class="text-lg font-semibold mb-4 text-gray-200">Upload New Photo</h3>
      <div class="flex gap-4">
        <div class="flex-grow">
          <input 
            type="file" 
            @change="handlePhotoUpload" 
            accept="image/*" 
            class="w-full text-gray-300 file:bg-gray-700 file:border-0 file:text-gray-300 file:px-4 file:py-2 file:rounded-md file:hover:bg-gray-600"
          />
        </div>
        <button 
          @click="uploadPhoto" 
          :disabled="!photoFile"
          class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-gray-600 disabled:text-gray-400 transition-colors"
        >
          Upload Photo
        </button>
      </div>
    </div>

    <!-- Loading and Error States -->
    <div v-if="loading" class="text-center py-4">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto"></div>
    </div>
    <div v-if="error" class="text-red-400 text-center py-4">
      {{ error }}
    </div>

    <!-- Photos Grid -->
    <div v-if="!loading && photos.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div v-for="photo in photos" :key="photo" class="border border-gray-700 rounded-lg p-4 shadow-md bg-gray-900">
        <img :src="photo" alt="Uploaded photo" class="w-full h-auto aspect-video object-cover rounded"/>
      </div>
    </div>
    <div v-if="!loading && photos.length === 0" class="text-gray-400 text-center py-4">
      No photos found
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { uploadPhoto as uploadPhotoAPI, listPhotos } from '@/api';

const photoFile = ref<File | null>(null);
const photos = ref<string[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);

const handlePhotoUpload = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (file) photoFile.value = file;
};

const fetchPhotos = async () => {
  try {
    loading.value = true;
    error.value = null;
    photos.value = await listPhotos();
  } catch (err) {
    error.value = 'Failed to load photos';
    console.error('Error loading photos:', err);
  } finally {
    loading.value = false;
  }
};

const uploadPhoto = async () => {
  if (!photoFile.value) return;
  
  try {
    loading.value = true;
    const formData = new FormData();
    formData.append('photo', photoFile.value);
    
    await uploadPhotoAPI(formData);
    await fetchPhotos();
    
    // Reset form
    photoFile.value = null;
    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
    if (input) input.value = '';
  } catch (err) {
    error.value = 'Failed to upload photo';
    console.error('Upload error:', err);
  } finally {
    loading.value = false;
  }
};

onMounted(fetchPhotos);
</script>
