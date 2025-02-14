import { createRouter, createWebHistory } from "vue-router";
import Photos from "@/views/Photos.vue";
import Model from "@/views/Model.vue";
import Videos from "@/views/Videos.vue";
import About from "@/views/AboutMe.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
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
  ],
});

export default router;
