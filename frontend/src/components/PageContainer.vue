<script setup lang="ts">
import { getAuth, signOut } from "firebase/auth";
import { useRouter } from "vue-router";
import { store } from "../store";

const router = useRouter();
const signout = () => {
  const auth = getAuth();
  signOut(auth);
  store.setAuthenticated(false);
  router.push("/signin");
};
</script>

<template>
  <div class="bg-gray-200 w-full h-14 flex justify-between px-4">
    <div class="flex flex-col justify-center text-lg" @click="router.push('/')">
      <div>Carcassonne AI</div>
    </div>
    <div
      v-if="!store.authenticated"
      class="flex flex-col justify-center text-lg"
      @click="router.push('/signin')"
    >
      <div>Sign In</div>
    </div>
    <div v-else class="flex flex-col justify-center text-lg" @click="signout">
      Sign Out
    </div>
  </div>
  <slot />
</template>
