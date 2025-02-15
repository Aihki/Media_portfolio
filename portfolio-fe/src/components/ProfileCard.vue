<template>
  <div class="max-w-2xl mx-auto bg-gray-800 rounded-lg shadow-xl overflow-hidden">
    <div class="flex flex-col md:flex-row">
      <div class="md:w-1/3">
      </div>
      <div class="p-6 md:w-2/3">
        <h2 class="text-2xl font-bold text-gray-100 mb-2">{{ name }}</h2>
        <p class="text-gray-400 text-sm mb-4">{{ title }}</p>
        <p class="text-gray-300 mb-4">{{ bio }}</p>
        
        <div class="flex space-x-4">
          <a v-for="link in socialLinks" 
             :key="link.url" 
             :href="link.url" 
             target="_blank"
             class="text-gray-400 hover:text-gray-100 transition-colors">
            <i :class="link.icon" class="text-xl"></i>
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  profileImage: string;
  name: string;
  title: string;
  bio: string;
  socialLinks: Array<{
    url: string;
    icon: string;
  }>;
}>();
</script>
