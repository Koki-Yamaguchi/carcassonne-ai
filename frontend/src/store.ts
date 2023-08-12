import { reactive } from "vue";

export const store = reactive({
  userID: "",
  setUserID(userID: string) {
    this.userID = userID;
  },

  authenticating: false,
  setAuthenticating(authenticating: boolean) {
    this.authenticating = authenticating;
  },

  authenticated: false,
  setAuthenticated(authenticated: boolean) {
    this.authenticated = authenticated;
  },

  language: "",
  setLanguage(language: string) {
    this.language = language;
  },
});
