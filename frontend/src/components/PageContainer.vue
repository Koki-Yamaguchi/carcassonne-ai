<script setup lang="ts">
import { useRouter } from "vue-router";
import { store } from "../store";
import CogIcon from "./CogIcon.vue";
import { translate } from "../locales/translate";
import SpinnerIcon from "./SpinnerIcon.vue";

const router = useRouter();
</script>

<template>
  <div class="min-h-screen relative">
    <div class="bg-gray-200 w-full h-14 flex justify-between px-4">
      <div
        class="flex flex-col justify-center text-lg"
        @click="router.push('/')"
      >
        <div>Top Carcassonner</div>
      </div>
      <div class="flex flex-col justify-center text-lg">
        <div v-if="store.authenticating"><SpinnerIcon /></div>
        <div v-else-if="!store.authenticated" @click="router.push('/signin')">
          {{ translate("sign_in") }}
        </div>
        <div v-else @click="router.push('/settings')">
          <CogIcon />
        </div>
      </div>
    </div>
    <div class="pb-24">
      <slot />
    </div>
    <footer class="bg-gray-200 absolute bottom-0 h-24 w-full">
      <div class="p-4 flex gap-6 text-sm">
        <a
          href="https://docs.google.com/forms/d/e/1FAIpQLSeRZwEVwLVy8T2KnoI6JHIoRt6k3JrSqDM6u1l98u-5_zuAAw/viewform"
          target="_blank"
          rel="noopener noreferrer"
          >{{ translate("contacts_or_bug_reports") }}</a
        >
      </div>
      <div class="text-center text-xs">© 2023 Koki Yamaguchi</div>
    </footer>
  </div>
</template>
