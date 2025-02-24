<template>
  <div class="h-screen">
    <div class="flex items-center justify-center h-[50%]">
      <h1 class="text-2xl font-semibold text-gray-200 mb-4">
        <TypewriterText 
          :text="'Welcome to my Portfolio'" 
          :delay="100"
        />
      </h1>
    </div>
  </div>
</template>

<script setup lang="ts">
import TypewriterText from '../components/TypewriterText.vue';
</script>