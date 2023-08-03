import { createApp } from "vue";
import "./style.css";
import router from "./router";
import App from "./App.vue";

import { initializeApp } from "firebase/app";

const app = createApp(App);

app.use(router);

const firebaseConfig = {
  apiKey: import.meta.env.VITE_FIREBASE_API_KEY,
  projectId: import.meta.env.VITE_FIREBASE_PROJECT_ID,
};
initializeApp(firebaseConfig);

app.mount("#app");
