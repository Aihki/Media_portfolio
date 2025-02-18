<template>
  <div class="bg-gray-800 shadow-lg rounded-lg p-6">
    <UploadForm
      type="Photo"
      acceptTypes="image/*"
      :maxSize="10 * 1024 * 1024"
      @upload="handleUpload"
    />

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
        :key="photo"
        class="border border-gray-700 rounded-lg p-4 shadow-md bg-gray-900"
      >
        <img
          :src="photo"
          :alt="`Photo ${photo.split('/').pop()}`"
          class="w-full h-auto aspect-video object-cover rounded cursor-pointer"
          @error="handleImageError"
          @click="photoView = photo"
        />
      </div>
    </div>

    <PhotoView
      v-if="photoView"
      :photoUrl="photoView"
      @close="photoView = null"
    />
    <div
      v-if="!loading && photos.length === 0"
      class="text-gray-400 text-center py-4"
    >
      No photos found
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import { uploadPhoto as uploadPhotoAPI, listPhotos } from '@/api';
  import PhotoView from '@/components/PhotoView.vue';
  import UploadForm from '@/components/UploadForm.vue';

  const photos = ref<string[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const photoView = ref<string | null>(null);

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

  const handleUpload = async (uploadData: { file: File; name: string; categoryId: { $oid: string } | string }) => {
    try {
      console.log('Photos component received:', {
        hasFile: !!uploadData.file,
        fileName: uploadData.file?.name,
        name: uploadData.name,
        categoryId: uploadData.categoryId
      });

      const data = {
        file: uploadData.file,
        name: uploadData.name,
        categoryId: typeof uploadData.categoryId === 'object' && '$oid' in uploadData.categoryId 
          ? uploadData.categoryId.$oid 
          : uploadData.categoryId
      };

      await uploadPhotoAPI(data);
      await fetchPhotos();
    } catch (error) {
      console.error('Upload error:', error);
      error.value = error instanceof Error ? error.message : 'Failed to upload photo';
    }
  };

  const handleImageError = (event: Event) => {
    const img = event.target as HTMLImageElement;
    console.error('Failed to load image:', img.src);
    error.value = `Failed to load image: ${img.src.split('/').pop()}`;
  };
  onMounted(fetchPhotos);
</script>
