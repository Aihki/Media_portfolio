<template>
  <div class="min-h-screen bg-gray-900 text-gray-200">
    <div class="background">
      <div v-for="n in 4" :key="n" 
           class="box"
           :style="{
             top: `${Math.random() * 100}%`,
             left: `${Math.random() * 100}%`,
             width: `${Math.random() * 100 + 50}px`,
             height: `${Math.random() * 100 + 50}px`,
             animationDuration: `${Math.random() * 20 + 10}s`,
             animationDirection: n % 2 ? 'normal' : 'reverse'
           }">
      </div>
    </div>
    
    <div class="container mx-auto px-4 py-8 relative">
      <header class="max-w-4xl mx-auto text-center mb-4">
        <h1 class="text-4xl font-bold text-gray-100 mb-4">
          My Media Portfolio
        </h1>
        <p class="text-xl text-gray-400 leading-relaxed max-w-2xl mx-auto">
          Welcome to my media portfolio. Here you'll find my collection of
          videos, 3D splats, and personal photography
          work.
        </p>
      </header>

      <nav class="sticky top-0 bg-gray-800/90 backdrop-blur-sm rounded-lg shadow-lg mb-8 max-w-2xl mx-auto">
        <div class="overflow-x-auto flex space-x-6 py-2 px-4 scrollbar-hide whitespace-nowrap">
          <router-link
            v-for="link in navigationLinks"
            :key="link.path"
            :to="link.path"
            class="text-sm text-gray-300 hover:text-white transition-colors duration-200 flex-shrink-0 px-3 py-1.5"
            :class="{ 'text-white font-medium': $route.path === link.path }"
          >
            {{ link.name }}
          </router-link>
        </div>
      </nav>

      <router-view></router-view>
    </div>
    <HandGestureNavigation />
  </div>
</template>

<script setup lang="ts">
import HandGestureNavigation from './components/HandGestureNavigation.vue';

const navigationLinks = [
  { path: '/', name: 'Photos' },
  { path: '/models', name: '3D Models' },
  { path: '/videos', name: 'Videos' },
  { path: '/sandbox', name: 'Sandbox' },
  { path: '/pixel-art', name: 'Pixel Art' },
  { path: '/about', name: 'About Me' }
];
</script>

<style>
.scrollbar-hide::-webkit-scrollbar {
  display: none;
}
.scrollbar-hide {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

  .background {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    z-index: 0;
  }

  .box {
    position: absolute;
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow: 0 0 15px rgba(255, 255, 255, 0.05);
    backdrop-filter: blur(1px);
    animation: rotate linear infinite;
    border-radius: 2px;
    background: linear-gradient(
      45deg,
      rgba(255, 255, 255, 0.02),
      rgba(255, 255, 255, 0.01)
    );
  }

  @keyframes rotate {
    from {
      transform: rotate(0deg) translate(30px) rotate(0deg);
    }
    to {
      transform: rotate(360deg) translate(30px) rotate(-360deg);
    }
  }

  @media (max-width: 640px) {
    nav {
      position: sticky;
      top: 0;
      z-index: 50;
    }
    
    nav .router-link-active {
      background-color: rgb(55 65 81 / var(--tw-bg-opacity));
    }
  }
</style>
