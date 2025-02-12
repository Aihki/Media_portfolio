<template>
  <div class="min-h-screen bg-gray-900 text-gray-200">
    <div class="container mx-auto px-4 py-8">

      <header class="max-w-4xl mx-auto text-center mb-12">
        <h1 class="text-4xl font-bold text-gray-100 mb-4">My Media Portfolio</h1>
        <p class="text-xl text-gray-400 leading-relaxed max-w-2xl mx-auto">
          Welcome to my media portfolio. Here you'll find my collection of videos, 
          3D splats, ComfyUI-enhanced images, and personal photography work.
        </p>
      </header>

      <div class="flex flex-col items-center gap-8">
        <section class="w-full max-w-6xl">
            <h2 class="text-2xl font-bold text-gray-100">
              Photo Gallery
            </h2>
            <Photos />
        </section>

        <section class="w-full max-w-6xl">
            <h2 class="text-2xl font-bold text-gray-100">
              3D Models
            </h2>
            <Model />
        </section>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import Photos from "@/components/Photos.vue";
import Model from "@/components/Model.vue";

</script>
