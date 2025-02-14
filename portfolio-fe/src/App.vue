<template>
  <div class="min-h-screen bg-gray-900 text-gray-200">
    <div class="container mx-auto px-4 py-8">
      <header class="max-w-4xl mx-auto text-center mb-4">
        <h1 class="text-4xl font-bold text-gray-100 mb-4">My Media Portfolio</h1>
        <p class="text-xl text-gray-400 leading-relaxed max-w-2xl mx-auto">
          Welcome to my media portfolio. Here you'll find my collection of videos, 
          3D splats, ComfyUI-enhanced images, and personal photography work.
        </p>
      </header>

      <nav class="sticky top-0 bg-gray-800/90 backdrop-blur-sm rounded-lg shadow-lg mb-8 max-w-2xl mx-auto">
        <div class="overflow-x-auto flex space-x-6 py-2 px-4 scrollbar-hide justify-center">
          <router-link to="/" 
             class="text-sm text-gray-300 hover:text-white whitespace-nowrap transition-colors duration-200 flex-shrink-0">
            Photos
          </router-link>
          <router-link to="/models" 
             class="text-sm text-gray-300 hover:text-white whitespace-nowrap transition-colors duration-200 flex-shrink-0">
            3D Models
          </router-link>
          <router-link to="/videos" 
             class="text-sm text-gray-300 hover:text-white whitespace-nowrap transition-colors duration-200 flex-shrink-0">
            Videos
          </router-link>
        </div>
      </nav>

      <router-view></router-view>
    </div>
  </div>
</template>

<style>
.scrollbar-hide::-webkit-scrollbar {
  display: none;
}
.scrollbar-hide {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
