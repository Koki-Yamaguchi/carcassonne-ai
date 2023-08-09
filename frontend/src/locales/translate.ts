import * as ja from "../locales/ja.json";
import * as en from "../locales/en.json";

export const translate = (s: keyof typeof ja) => {
  // FIXME: check local storage setting, if the user sets a language manually, which should not be changed just because of browser setting
  if (window.navigator.language !== "ja") {
    return ja[s];
  } else {
    return en[s];
  }
};
