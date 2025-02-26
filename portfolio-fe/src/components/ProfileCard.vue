<template>
  <div class="max-w-2xl mx-auto bg-gray-800 rounded-lg shadow-xl overflow-hidden">
    <div class="flex flex-col md:flex-row">
      <div class="md:w-1/3">
        <svg viewBox="0 0 200 200" class="w-48 h-48 animate-spin-slow">
          <polygon 
            points="100,20 170,60 170,140 100,180 30,140 30,60"
            class="stroke-gray-400 fill-none stroke-2"
          />
          <line x1="100" y1="20" x2="100" y2="180" class="stroke-gray-400 stroke-1 opacity-100" />
          <line x1="30" y1="60" x2="170" y2="140" class="stroke-gray-400 stroke-1 opacity-100" />
          <line x1="30" y1="140" x2="170" y2="60" class="stroke-gray-400 stroke-1 opacity-100" />
        </svg>
      </div>
      <div class="p-6 md:w-2/3">
        <h2 class="text-2xl font-bold text-gray-100 mb-2">{{ name }}</h2>
        <p class="text-gray-400 text-sm mb-4">{{ title }}</p>
        <div class="mb-4">
          <TypewriterText :text="bio" />
        </div>

        <!-- Add Contact Links Section -->
        <div class="mt-6 border-t border-gray-700 pt-4">
          <h3 class="text-lg font-semibold text-gray-100 mb-3">Contact Me</h3>
          <div class="grid grid-cols-2 gap-3">
            <a
              v-for="contact in contactLinks"
              :key="contact.label"
              :href="contact.url"
              target="_blank"
              class="flex items-center space-x-2 text-gray-400 hover:text-gray-100 transition-colors"
            >
              <i :class="contact.icon" class="text-lg"></i>
              <span>{{ contact.label }}</span>
            </a>
          </div>
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
  contactLinks: Array<{
    label: string;
    url: string;
    icon: string;
  }>;
}>();
</script>
