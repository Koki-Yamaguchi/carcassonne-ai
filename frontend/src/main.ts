import { createApp } from "vue";
import "./style.css";
import router from "./router";
import App from "./App.vue";

import { initializeApp } from "firebase/app";

const app = createApp(App);

app.use(router);

const firebaseConfig = {
  apiKey: "",
  authDomain: "",
  projectId: "",
};
initializeApp(firebaseConfig);

app.mount("#app");
