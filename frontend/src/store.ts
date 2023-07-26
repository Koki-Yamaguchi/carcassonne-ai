import { reactive } from "vue";

export const store = reactive({
  userID: "",
  authenticated: false,
  setUserID(userID: string) {
    this.userID = userID;
  },
  setAuthenticated(authenticated: boolean) {
    this.authenticated = authenticated;
  },
});
