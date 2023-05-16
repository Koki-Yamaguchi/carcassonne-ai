import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";
import GameView from "../views/GameView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/games/:id",
      name: "game",
      component: GameView,
    },
    /*
    {
      path: "/simulator/settings",
      name: "settings",
      component: SettingsView,
    },
    */
  ],
});

export default router;
