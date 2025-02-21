<template>
    <div class="container mx-auto px-4 py-8">
      <header class="max-w-4xl mx-auto text-center mb-8">
        <h1 class="text-4xl font-bold text-gray-100 mb-4">About Me</h1>
      </header>

      <ProfileCard
        name="Your Name"
        title="photography enthusiast"
        bio="I'm a photography enthusiast who loves to capture the beauty of the world through my lens."
      />
    </div>
</template>

<script setup lang="ts">
  import ProfileCard from '@/components/ProfileCard.vue';
</script>
