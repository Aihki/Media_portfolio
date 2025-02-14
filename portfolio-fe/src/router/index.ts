import { createRouter, createWebHistory } from 'vue-router'
import Photos from '@/views/Photos.vue'
import Model from '@/views/Model.vue'
import Videos from '@/views/Videos.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Photos
    },
    {
      path: '/models',
      name: 'models',
      component: Model
    },
    {
      path: '/videos',
      name: 'videos',
      component: Videos
    }
  ]
})

export default router
