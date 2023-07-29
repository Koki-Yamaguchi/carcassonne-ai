import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";
import GameView from "../views/GameView.vue";
import SignupView from "../views/SignupView.vue";
import SigninView from "../views/SigninView.vue";
import SettingsView from "../views/SettingsView.vue";
import { getAuth, onAuthStateChanged } from "firebase/auth";
import { store } from "../store";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/signup",
      name: "signup",
      component: SignupView,
    },
    {
      path: "/signin",
      name: "signin",
      component: SigninView,
    },
    {
      path: "/games/:id",
      name: "game",
      component: GameView,
    },
    {
      path: "/settings",
      name: "settings",
      component: SettingsView,
    },
  ],
});

const getCurrentUser = () => {
  const auth = getAuth();
  return new Promise((resolve, reject) => {
    const unsubscribe = onAuthStateChanged(
      auth,
      (user) => {
        unsubscribe();
        resolve(user);
      },
      reject
    );
  });
};

router.beforeEach(async (to) => {
  const currentUser = (await getCurrentUser()) as any;
  if (to.path !== "/signin" && to.path !== "/signup") {
    if (currentUser) {
      store.setAuthenticated(true);
      store.setUserID(currentUser.uid);
    } else {
      store.setAuthenticated(false);
      return "/signin";
    }
  }
});

export default router;
