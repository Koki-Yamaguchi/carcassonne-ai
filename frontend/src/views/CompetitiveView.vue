<script setup lang="ts">
import { translate } from "../locales/translate";

const createGame = () => {};
</script>
<template>
  <div class="p-6">
    <p>{{ translate("competitive_mode") }}</p>
    <p class="text-sm text-gray-700">
      {{ translate("competitive_mode_description") }}
    </p>
    <button
      class="mt-4 bg-gray-500 hover:bg-gray-400 text-[#eeeeee] rounded px-4 py-2"
      @click="createGame"
    >
      {{ translate("play_rated_match") }}
    </button>
  </div>
</template>
