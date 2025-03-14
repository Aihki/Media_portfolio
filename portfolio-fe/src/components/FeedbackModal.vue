<template>
  <div v-if="isOpen" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-10">
    <div class="bg-gray-800 rounded-lg p-6 w-full max-w-md">
      <h2 class="text-xl font-bold text-gray-200 mb-4">Give Feedback</h2>
      
      <div class="mb-4">
        <label class="block text-gray-300 mb-2">Rating</label>
        <div class="flex gap-2">
          <button
            v-for="star in 5"
            :key="star"
            @click="rating = star"
            class="text-2xl"
            :class="star <= rating ? 'text-yellow-500' : 'text-gray-500'"
          >
            ★
          </button>
        </div>
      </div>

      <div class="mb-4">
        <label class="block text-gray-300 mb-2">Comment</label>
        <textarea
          v-model="comment"
          class="w-full px-3 py-2 bg-gray-700 text-white rounded-md"
          rows="4"
          placeholder="Your feedback..."
        ></textarea>
      </div>

      <div class="flex justify-end gap-2">
        <button
          @click="$emit('close')"
          class="px-4 py-2 bg-gray-700 text-white rounded-md hover:bg-gray-600"
        >
          Cancel
        </button>
        <button
          @click="submitFeedback"
          class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
          :disabled="!rating"
        >
          Submit
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { addFeedback } from '../services/feedbackService';

const props = defineProps<{
  isOpen: boolean;
  contentId: string;
  contentType: 'model' | 'photo' | 'video';
}>();

const emit = defineEmits(['close', 'submitted']);

const rating = ref(0);
const comment = ref('');

async function submitFeedback() {
  try {
    await addFeedback({
      contentId: props.contentId,
      contentType: props.contentType,
      rating: rating.value,
      comment: comment.value
    });
    
    emit('submitted');
    emit('close');

    rating.value = 0;
    comment.value = '';
  } catch (error) {
    console.error('Error submitting feedback:', error);
  }
}
</script>
