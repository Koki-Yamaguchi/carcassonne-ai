<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import ProblemItem from "../components/ProblemItem.vue";
import { API } from "../api";
import { Problem, Player } from "../types";
import { translate } from "../locales/translate";
import { store } from "../store";
import SpinnerIcon from "../components/SpinnerIcon.vue";
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const router = useRouter();

const problems = ref<Problem[]>([]);
const voted = ref<boolean[]>([]);
const player = ref<Player | null>(null);
const loading = ref<boolean>(false);
const orderBy = ref<string>("-id");
const page = ref<number>(0);

onMounted(async () => {
  handleQueryString();
  setQueryString();
  loading.value = true;
  const api = new API();

  player.value = await api.getPlayerByUserID(store.userID);
  problems.value = await api.getProblems(page.value, orderBy.value, 10);

  const myVotes = await api.getVotes(null, player.value.id);
  const votedProblemIDs = myVotes.map((v) => v.problemID);
  voted.value = problems.value.map((p) => votedProblemIDs.includes(p.id));

  loading.value = false;
});

const handleQueryString = () => {
  if (route.query.order_by) {
    orderBy.value = route.query.order_by;
  }
  if (route.query.page) {
    page.value = route.query.page;
  }
};

const setQueryString = () => {
  router.push({ query: { order_by: orderBy.value, page: page.value } });
};

watch(orderBy, async (newOrderBy) => {
  loading.value = true;
  const api = new API();

  problems.value = await api.getProblems(0, newOrderBy, 10);
  setQueryString();

  loading.value = false;
});
</script>

<template>
  <div class="p-6">
    <p class="">{{ translate("problems") }}</p>
    <p class="my-2 text-sm text-gray-700">
      {{ translate("problems_description") }}
    </p>
    <select
      class="text-xs border-2 rounded py-1 px-2 text-gray-700 focus:outline-none focus:bg-white focus:border-green-300"
      v-model="orderBy"
    >
      <option value="-id">新しい順</option>
      <option value="id">古い順</option>
      <option value="-vote_count">投票の多い順</option>
      <option value="vote_count">投票の少ない順</option>
    </select>
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
