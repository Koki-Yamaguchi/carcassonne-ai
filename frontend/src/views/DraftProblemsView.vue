<script setup lang="ts">
import { onMounted, ref, computed } from "vue";
import { API } from "../api";
import { Problem, Player } from "../types";
import { store } from "../store";
import DraftProblemItems from "../components/DraftProblemItems.vue";
import { translate } from "../locales/translate";

const draftProblems = ref<Problem[]>([]);
const privateProblems = ref<Problem[]>([]);
const player = ref<Player | null>(null);

onMounted(async () => {
  const api = new API();

  player.value = await api.getPlayerByUserID(store.userID);

  if (!isAdmin.value) {
    return;
  }

  const res0 = await api.getPrivateProblems(true);
  draftProblems.value = res0.problems;

  const res1 = await api.getPrivateProblems(false);
  privateProblems.value = res1.problems;
});

const isAdmin = computed(() => {
  return player.value && player.value.id === 2;
});
</script>

<template>
  <div v-if="!isAdmin">{{ translate("not_authorized") }}</div>
  <div v-else>
    <div class="p-4">
      <p class="text-gray-600 text-lg mb-2">レビュー待ちの問題</p>
      <DraftProblemItems :problems="draftProblems" />
    </div>
    <div class="p-2">
      <p class="text-gray-600 text-lg mb-2">出題予定の問題</p>
      <DraftProblemItems :problems="privateProblems" />
    </div>
  </div>
</template>
