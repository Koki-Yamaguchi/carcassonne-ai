<script setup lang="ts">
import { onMounted, ref, computed } from "vue";
import { API } from "../api";
import { Problem, Player } from "../types";
import { store } from "../store";
import DraftProblemItems from "../components/DraftProblemItems.vue";
import { translate } from "../locales/translate";

const problems = ref<Problem[]>([]);
const player = ref<Player | null>(null);

onMounted(async () => {
  const api = new API();

  player.value = await api.getPlayerByUserID(store.userID);

  if (!isAdmin.value) {
    return;
  }

  const res = await api.getDraftProblems();
  problems.value = res.problems;
});

const isAdmin = computed(() => {
  return player.value && player.value.id === 2;
});
</script>

<template>
  <div class="p-4">
    <div v-if="!isAdmin">{{ translate("not_authorized") }}</div>
    <div v-else>
      <div>
        <p class="text-gray-600 text-lg mb-2">レビュー待ちの問題</p>
        <DraftProblemItems :problems="problems" />
      </div>
    </div>
  </div>
</template>
