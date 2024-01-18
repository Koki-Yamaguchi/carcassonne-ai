<script setup lang="ts">
import { API } from "./../api";
import { store } from "../store";
import { Player } from "../types";
import { onMounted, ref } from "vue";
import { translate } from "../locales/translate";

const player = ref<Player | null>(null);

onMounted(async () => {
  const api = new API();
  player.value = await api.getPlayerByUserID(store.userID);
});
</script>

<template>
  <div class="text-gray-500">
    <p class="p-6">{{ translate("description") }}</p>
    <img class="w-full max-w-md" src="../assets/img/board_example.png" />
  </div>
  <div class="p-6 text-gray-700">
    <div class="mt-6 p-4 rounded-md border border-gray-100 shadow-lg">
      <p>{{ translate("normal_mode") }}</p>
      <p class="mt-2 text-sm text-gray-500">
        {{ translate("normal_mode_description") }}
      </p>
    </div>
    <div class="mt-6 p-4 rounded-md border border-gray-100 shadow-lg">
      <p>{{ translate("competitive_mode") }}</p>
      <p class="mt-2 text-sm text-gray-500">
        {{ translate("competitive_mode_description") }}
      </p>
    </div>
    <div class="mt-6 p-4 rounded-md border border-gray-100 shadow-lg">
      <p>{{ translate("random_match") }}</p>
      <p class="mt-2 text-sm text-gray-500">
        {{ translate("random_match_description") }}
      </p>
    </div>
    <div class="mt-6 p-4 rounded-md border border-gray-100 shadow-lg">
      <p>{{ translate("problems") }}</p>
      <p class="mt-2 text-sm text-gray-500">
        {{ translate("problems_description") }}
      </p>
    </div>
    <div class="mt-6 p-4 rounded-md border border-gray-100 shadow-lg">
      <p>{{ translate("propose_problem") }}</p>
      <p class="mt-2 text-sm text-gray-500">
        {{ translate("propose_problem_short_description") }}
      </p>
    </div>
  </div>
</template>
