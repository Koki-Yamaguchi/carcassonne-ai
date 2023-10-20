<script setup lang="ts">
import { onMounted, ref } from "vue";
import ProblemItem from "../components/ProblemItem.vue";
import { API } from "../api";
import { Problem, Player } from "../types";
import { translate } from "../locales/translate";
import { store } from "../store";
import SpinnerIcon from "../components/SpinnerIcon.vue";

const problems = ref<Problem[]>([]);
const voted = ref<boolean[]>([]);
const player = ref<Player | null>(null);
const loading = ref<boolean>(false);

onMounted(async () => {
  loading.value = true;
  const api = new API();

  player.value = await api.getPlayerByUserID(store.userID);
  problems.value = await api.getProblems();

  const myVotes = await api.getVotes(null, player.value.id);
  const votedProblemIDs = myVotes.map((v) => v.problemID);
  voted.value = problems.value.map((p) => votedProblemIDs.includes(p.id));

  loading.value = false;
});
</script>

<template>
  <div class="p-6">
    <p class="">{{ translate("problems") }}</p>
    <p class="my-2 text-sm text-gray-700">
      {{ translate("problems_description") }}
    </p>
    <div v-if="loading"><SpinnerIcon /></div>
    <div v-else class="mt-4">
      <ProblemItem
        v-for="(problem, idx) in problems"
        :problem="problem"
        :voted="voted[idx]"
        :key="problem.id"
      />
    </div>
  </div>
</template>
