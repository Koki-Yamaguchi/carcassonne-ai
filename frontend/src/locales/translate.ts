import * as ja from "../locales/ja.json";
import * as en from "../locales/en.json";
import { store } from "../store";

export const isJapaneseSetting = () => {
  const lang =
    store.language !== "" ? store.language : window.navigator.language;
  return lang === "ja";
};

export const translate = (s: keyof typeof ja) => {
  if (isJapaneseSetting()) {
    return ja[s];
  } else {
    return en[s];
  }
};
