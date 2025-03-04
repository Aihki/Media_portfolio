<template>
  <div class="h-screen overflow-auto">
    <div class="flex items-center justify-center h-[20%] mb-0">
      <h1 class="text-2xl font-semibold text-gray-200 mb-4">
        <TypewriterText :text="'Welcome back admin'" :delay="100" />
      </h1>
    </div>
    <div class="flex items-center justify-center bg-gray-900 mt-0 px-4">
      <div class="w-full md:w-2/3 lg:w-1/3 bg-gray-800 rounded-lg shadow-lg p-4 md:p-8">
        <h1 class="text-lg md:text-xl text-gray-200 mb-4">Portfolio Stats</h1>
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 md:gap-4">
          <div class="text-center p-3 md:p-4 bg-gray-700 rounded-lg">
            <p class="text-xl md:text-2xl font-bold text-blue-400">{{ stats.photos_count }}</p>
            <p class="text-sm md:text-base text-gray-300">Photos</p>
          </div>
          <div class="text-center p-3 md:p-4 bg-gray-700 rounded-lg">
            <p class="text-xl md:text-2xl font-bold text-green-400">{{ stats.models_count }}</p>
            <p class="text-sm md:text-base text-gray-300">Models</p>
          </div>
          <div class="text-center p-3 md:p-4 bg-gray-700 rounded-lg">
            <p class="text-xl md:text-2xl font-bold text-purple-400">{{ stats.videos_count }}</p>
            <p class="text-sm md:text-base text-gray-300">Videos</p>
          </div>
        </div>
      </div>
    </div>

    <div class="flex justify-center mt-8 px-4">
      <div class="w-full md:w-2/3 lg:w-1/3 bg-gray-800 rounded-lg shadow-lg p-4 md:p-8">
        <h2 class="text-lg md:text-xl text-gray-200 mb-4">Recent Feedback</h2>
        <div class="space-y-4">
          <div v-if="loading" class="text-center py-4">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto"></div>
          </div>
          
          <div v-else-if="recentFeedback.length === 0" class="text-gray-400 text-center">
            No feedback yet
          </div>

          <div
            v-for="feedback in recentFeedback"
            :key="feedback.id"
            class="bg-gray-700 rounded-lg p-4"
          >
            <div class="flex items-center justify-between mb-2">
              <span class="text-gray-300">
                {{ capitalizeFirstLetter(feedback.contentType) }}
              </span>
              <span class="text-yellow-400">
                {{ "★".repeat(feedback.rating) }}{{ "☆".repeat(5 - feedback.rating) }}
              </span>
            </div>
            <p class="text-gray-300">{{ feedback.comment }}</p>
            <p class="text-sm text-gray-400 mt-2">
              {{ formatDate(feedback.createdAt) }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import TypewriterText from '../components/TypewriterText.vue';
import { getStats, type Stats } from '../api';
import { getRecentFeedback } from '../services/feedbackService';
import type { Feedback } from '../services/feedbackService';

const stats = ref<Stats>({
  photos_count: 0,
  models_count: 0,
  videos_count: 0,
});

const recentFeedback = ref<Feedback[]>([]);
const loading = ref(true);

function capitalizeFirstLetter(string: string): string {
  return string.charAt(0).toUpperCase() + string.slice(1);
}

function formatDate(date: Date): string {
  if (!date) return '';
  return new Intl.DateTimeFormat('en-US', {
    dateStyle: 'medium',
    timeStyle: 'short'
  }).format(date);
}

onMounted(async () => {
  try {
    loading.value = true;
    const [statsData, feedbackData] = await Promise.all([
      getStats(),
      getRecentFeedback()
    ]);
    
    stats.value = statsData;
    recentFeedback.value = feedbackData;
  } catch (error) {
    console.error('Failed to fetch data:', error);
  } finally {
    loading.value = false;
  }
});
</script>
