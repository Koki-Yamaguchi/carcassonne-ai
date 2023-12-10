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

export const translate_with_arg = (s: string, arg: any) => {
  if (isJapaneseSetting()) {
    switch (s) {
      case "solved_problem_description":
        return `これは最適手が計算された問題です。同点の場合は後手の勝ちとします。この問題には ${arg} つの最適手があります。`;
    }
  } else {
    switch (s) {
      case "solved_problem_description":
        return `This is a solved problem. Consider a tie as a win for the second player. In this problem, there are ${arg} optimal moves.`;
    }
  }
};
