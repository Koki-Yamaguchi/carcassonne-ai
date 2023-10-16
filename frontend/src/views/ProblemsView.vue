<script setup lang="ts">
import { onMounted, ref } from "vue";
import ProblemItem from "../components/ProblemItem.vue";
import { API } from "../api";
import { Problem, Player } from "../types";
import { translate } from "../locales/translate";
import { store } from "../store";

const problems = ref<Problem[]>([]);
const voted = ref<boolean[]>([]);
const player = ref<Player | null>(null);

onMounted(async () => {
  const api = new API();

  player.value = await api.getPlayerByUserID(store.userID);
  problems.value = await api.getProblems();

  const votes = await api.getVotes();
  const votedProblemIDs = votes
    .filter((v) => v.playerID == player.value?.id)
    .map((v) => v.problemID);
  problems.value.forEach((problem) => {
    if (votedProblemIDs.includes(problem.id)) {
      voted.value.push(true);
    } else {
      voted.value.push(false);
    }
  });
});
</script>

<template>
  <div class="p-6">
    <p class="">{{ translate("problems") }}</p>
    <p class="my-2 text-sm text-gray-700">
      {{ translate("problems_description") }}
    </p>
    <div v-if="voted.length > 0" class="mt-4">
      <ProblemItem
        v-for="(problem, idx) in problems"
        :problem="problem"
        :voted="voted[idx]"
        :key="problem.id"
      />
    </div>
  </div>
</template>
