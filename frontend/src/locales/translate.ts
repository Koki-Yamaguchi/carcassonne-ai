import * as ja from "../locales/ja.json";
import * as en from "../locales/en.json";
import { store } from "../store";

export const translate = (s: keyof typeof ja) => {
  const lang =
    store.language !== "" ? store.language : window.navigator.language;
  if (lang === "ja") {
    return ja[s];
  } else {
    return en[s];
  }
};
