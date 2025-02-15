<template>
  <div 
    class="fixed inset-0 z-50 bg-black bg-opacity-75 flex items-center justify-center"
    @click="handleBackdropClick"
  >
    <div class="relative max-w-4xl max-h-[90vh] p-4">
      <!-- Close button -->
      <button 
        @click="$emit('close')" 
        class="absolute -top-4 -right-4 bg-gray-800 text-white w-8 h-8 rounded-full hover:bg-gray-700"
      >
        ×
      </button>

      <!-- Image -->
      <img 
        :src="photoUrl" 
        :alt="photoUrl.split('/').pop()" 
        class="max-w-full max-h-[80vh] object-contain rounded-lg"
        @click.stop
      />

      <!-- Optional: filename display -->
      <div class="mt-2 text-center text-white">
        {{ photoUrl.split('/').pop() }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  photoUrl: string;
}>();
const emit = defineEmits<{
  (e: 'close'): void;
}>();

const handleBackdropClick = (e: MouseEvent) => {
  if (e.target === e.currentTarget) {
    emit('close');
  }
};
</script>