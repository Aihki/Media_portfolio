<template>
  <div class="container mx-auto px-4 py-8">
    <header class="max-w-4xl mx-auto text-center mb-8">
      <h1 class="text-4xl font-bold text-gray-100 mb-4">About Me</h1>
    </header>

    <ProfileCard
      name="Nestori Laine"
      title="photography enthusiast"
      bio="I'm a photography enthusiast who loves to capture the beauty of the world through my lens."
      :contactLinks="[
        {
          label: 'Email',
          url: 'mailto:your.email@example.com',
          icon: 'pi pi-envelope'
        },
        {
          label: 'LinkedIn',
          url: 'https://linkedin.com/in/yourprofile',
          icon: 'pi pi-linkedin'
        },
        {
          label: 'GitHub',
          url: 'https://github.com/yourusername',
          icon: 'pi pi-github'
        },
        {
          label: 'Phone',
          url: 'tel:+1234567890',
          icon: 'pi pi-phone'
        }
      ]"
    />
  </div>
</template>

<script setup lang="ts">
  import ProfileCard from '@/components/ProfileCard.vue';
  import 'primeicons/primeicons.css'
</script>
