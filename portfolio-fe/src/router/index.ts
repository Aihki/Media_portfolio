import { createRouter, createWebHistory } from "vue-router";
import Photos from "../views/Photos.vue";
import Model from "../views/Model.vue";
import Videos from "../views/Videos.vue";
import About from "../views/AboutMe.vue";
import Login from "../components/Login.vue";
import Sandbox from "../views/Sandbox.vue";
import Welcome from "../views/Welcome.vue";

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
    },
    {
      path: "/sandbox",
      name: "sandbox",
      component: Sandbox,
    }
  ],
});

export default router;
