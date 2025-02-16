<template>
  <div class="max-w-2xl mx-auto bg-gray-800 rounded-lg shadow-xl overflow-hidden">
    <div class="flex flex-col md:flex-row">
      <div class="md:w-1/3">
   <svg viewBox="0 0 200 200" class="w-48 h-48">
          <!-- Main hexagon -->
          <polygon 
            points="100,20 170,60 170,140 100,180 30,140 30,60"
            class="stroke-gray-400 fill-none stroke-2"
          />
          <!-- Inner connecting lines -->
          <line x1="100" y1="20" x2="100" y2="180" class="stroke-gray-400 stroke-1 opacity-30" />
          <line x1="30" y1="60" x2="170" y2="140" class="stroke-gray-400 stroke-1 opacity-30" />
          <line x1="30" y1="140" x2="170" y2="60" class="stroke-gray-400 stroke-1 opacity-30" />
        </svg>
      </div>
      <div class="p-6 md:w-2/3">
        <h2 class="text-2xl font-bold text-gray-100 mb-2">{{ name }}</h2>
        <p class="text-gray-400 text-sm mb-4">{{ title }}</p>
        <div class="mb-4">
          <TypewriterText :text="bio" />
        </div>

        <div v-if="socialLinks" class="flex space-x-4 mt-4">
          <a
            v-for="link in socialLinks"
            :key="link.url"
            :href="link.url"
            target="_blank"
            class="text-gray-400 hover:text-gray-100 transition-colors"
          >
            <i :class="link.icon" class="text-xl"></i>
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import TypewriterText from './TypewriterText.vue';

defineProps<{
  name: string;
  title: string;
  bio: string;
  socialLinks?: Array<{
    url: string;
    icon: string;
  }>;
}>();
</script>
