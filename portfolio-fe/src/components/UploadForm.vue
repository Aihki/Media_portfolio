<template>
  <div class="mb-8 border-b border-gray-700 pb-6">
    <h3 class="text-lg font-semibold mb-4 text-gray-200">
      Upload New {{ type }}
    </h3>
    <div class="flex gap-4">
      <div class="flex-grow">
        <input
          type="file"
          @change="handleFileUpload"
          :accept="acceptTypes"
          class="w-full text-gray-300 file:bg-gray-700 file:border-0 file:text-gray-300 file:px-4 file:py-2 file:rounded-md file:hover:bg-gray-600"
        />
      </div>
      <button
        @click="handleUpload"
        :disabled="!file"
        class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-gray-600 disabled:text-gray-400 transition-colors"
      >
        Upload {{ type }}
      </button>
    </div>
    <div v-if="error" class="text-red-400 mt-2">
      {{ error }}
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue';

  const props = defineProps<{
    type: 'Photo' | 'Video' | 'Model';
    acceptTypes: string;
    maxSize?: number;
  }>();

  const emit = defineEmits<{
    (e: 'upload', file: File): void;
  }>();

  const file = ref<File | null>(null);
  const error = ref<string | null>(null);

  const handleFileUpload = (event: Event) => {
    const selectedFile = (event.target as HTMLInputElement).files?.[0];
    if (!selectedFile) return;

    if (props.maxSize && selectedFile.size > props.maxSize) {
      error.value = `File is too large. Maximum size is ${props.maxSize / (1024 * 1024)}MB`;
      return;
    }

    file.value = selectedFile;
    error.value = null;
  };

  const handleUpload = async () => {
    if (!file.value) return;
    emit('upload', file.value);
    file.value = null;
    const input = document.querySelector(
      'input[type="file"]'
    ) as HTMLInputElement;
    if (input) input.value = '';
  };
</script>
