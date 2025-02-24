<template>
  <div class="h-screen">
    <div class="flex items-center justify-center h-[20%] mb-0">
      <h1 class="text-2xl font-semibold text-gray-200 mb-4">
        <TypewriterText :text="'Welcome back admin'" :delay="100" />
      </h1>
    </div>
    <div class="flex items-center justify-center bg-gray-900 mt-0">
      <div class="w-[33%] bg-gray-800 rounded-lg shadow-lg p-8">
        <h1 class="text-xl text-gray-200 mb-4">Portfolio Stats</h1>
        <div class="grid grid-cols-3 gap-4">
          <div class="text-center p-4 bg-gray-700 rounded-lg">
            <p class="text-2xl font-bold text-blue-400">{{ stats.photos_count }}</p>
            <p class="text-gray-300">Photos</p>
          </div>
          <div class="text-center p-4 bg-gray-700 rounded-lg">
            <p class="text-2xl font-bold text-green-400">{{ stats.models_count }}</p>
            <p class="text-gray-300">Models</p>
          </div>
          <div class="text-center p-4 bg-gray-700 rounded-lg">
            <p class="text-2xl font-bold text-purple-400">{{ stats.videos_count }}</p>
            <p class="text-gray-300">Videos</p>
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

const stats = ref<Stats>({
  photos_count: 0,
  models_count: 0,
  videos_count: 0,
});

onMounted(async () => {
  try {
    stats.value = await getStats();
  } catch (error) {
    console.error('Failed to fetch stats:', error);
  }
});
</script>
