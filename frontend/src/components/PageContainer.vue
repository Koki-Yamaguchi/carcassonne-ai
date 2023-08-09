<script setup lang="ts">
import { useRouter } from "vue-router";
import { store } from "../store";
import CogIcon from "./CogIcon.vue";
import { translate } from "../locales/translate";

const router = useRouter();
</script>

<template>
  <div class="bg-gray-200 w-full h-14 flex justify-between px-4">
    <div class="flex flex-col justify-center text-lg" @click="router.push('/')">
      <div>Top Carcassonner</div>
    </div>
    <div
      v-if="!store.authenticated"
      class="flex flex-col justify-center text-lg"
      @click="router.push('/signin')"
    >
      <div>{{ translate("sign_in") }}</div>
    </div>
    <div
      v-else
      class="flex flex-col justify-center"
      @click="router.push('/settings')"
    >
      <CogIcon />
    </div>
  </div>
  <slot />
</template>
