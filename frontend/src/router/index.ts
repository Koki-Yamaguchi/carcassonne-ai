import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";
import GameView from "../views/GameView.vue";
import GamesView from "../views/GamesView.vue";
import ProblemsView from "../views/ProblemsView.vue";
import ProblemView from "../views/ProblemView.vue";
import SignupView from "../views/SignupView.vue";
import SigninView from "../views/SigninView.vue";
import SettingsView from "../views/SettingsView.vue";
import CompetitiveView from "../views/CompetitiveView.vue";
import LobbyView from "../views/LobbyView.vue";
import ReplayView from "../views/ReplayView.vue";
import ResultView from "../views/ResultView.vue";
import ProposeProblemView from "../views/ProposeProblemView.vue";
import EditProblemView from "../views/EditProblemView.vue";
import DraftProblemsView from "../views/DraftProblemsView.vue";
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
      path: "/games",
      name: "games",
      component: GamesView,
    },
    {
      path: "/games/:id",
      name: "game",
      component: GameView,
    },
    {
      path: "/games/:id/result",
      name: "result",
      component: ResultView,
    },
    {
      path: "/games/:id/replay",
      name: "replay",
      component: ReplayView,
    },
    {
      path: "/problems/propose",
      name: "propose problem",
      component: ProposeProblemView,
    },
    {
      path: "/problems/:id",
      name: "problem",
      component: ProblemView,
    },
    {
      path: "/problems/:id/edit",
      name: "edit problem",
      component: EditProblemView,
    },
    {
      path: "/problems",
      name: "problems",
      component: ProblemsView,
    },
    {
      path: "/draft-problems",
      name: "draft problems",
      component: DraftProblemsView,
    },
    {
      path: "/settings",
      name: "settings",
      component: SettingsView,
    },
    {
      path: "/competitive",
      name: "competitive mode",
      component: CompetitiveView,
    },
    {
      path: "/lobby",
      name: "lobby",
      component: LobbyView,
    },
  ],
  scrollBehavior(_, __, savedPosition) {
    if (savedPosition) {
      return new Promise((resolve) => {
        setTimeout(() => {
          resolve(savedPosition);
        }, 200); // scroll after data is fetched and rendered
      });
    } else {
      return { top: 0 };
    }
  },
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
  const lang = localStorage.getItem("language");
  if (lang) {
    store.setLanguage(lang);
  }
  store.setAuthenticating(true);
  const currentUser = (await getCurrentUser()) as any;
  store.setAuthenticating(false);
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
