import { createRouter, createWebHistory } from "vue-router";
import Photos from "../views/Photos.vue";
import Model from "../views/Model.vue";
import Videos from "../views/Videos.vue";
import About from "../views/AboutMe.vue";
import Login from "../components/Login.vue";
import Sandbox from "../views/Sandbox.vue";
import Welcome from "../views/Welcome.vue";
import PixelCanvas from "../views/PixelCanvas.vue";
import { useAuthStore } from '../utils/AuthStore';


const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: Photos,
    },
    {
      path: "/models",
      name: "models",
      component: Model,
    },
    {
      path: "/videos",
      name: "videos",
      component: Videos,
    },
    {
        path: "/about",
        name: "about",
        component: About,
    },
    {
      path:"/login",
      name:"login",
      component: Login,
    },
    {
      path:"/welcome",
      name:"welcome",
      component: Welcome,
      meta: {
        requiresAuth: true
      }
    },
    {
      path: "/sandbox",
      name: "sandbox",
      component: Sandbox,
    },
    {
      path: "/pixel-art",
      name: "PixelArt",
      component: PixelCanvas,
    }
  ],
});
router.beforeEach((to, _from, next) => {
  const authStore = useAuthStore();

  // Check if the route is requesting a static model file first
  if (to.path.startsWith('/models/') && 
      (to.path.endsWith('.glb') || to.path.endsWith('.gltf') || 
       to.path.endsWith('.usdz') || to.path.endsWith('.splat'))) {
    window.location.href = to.fullPath;
    return;
  }

  if (to.meta.requiresAuth) {
    if (!authStore.isAuthenticated) {
      next({
        path: '/login',
        query: { redirect: to.fullPath }
      });
    } else {
      next(); 
    }
  } else {
    next(); 
  }
});

export default router;
