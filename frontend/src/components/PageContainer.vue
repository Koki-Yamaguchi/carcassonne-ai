<script setup lang="ts">
import { translate } from "../locales/translate";
import NavigationBar from "./NavigationBar.vue";
import { ref } from "vue";

const year = ref<number>(new Date().getFullYear());
</script>

<template>
  <div class="min-h-screen relative">
    <NavigationBar />
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
      <div class="text-center text-xs">
        {{ `© 2023-${year} Koki Yamaguchi` }}
      </div>
    </footer>
  </div>
</template>
