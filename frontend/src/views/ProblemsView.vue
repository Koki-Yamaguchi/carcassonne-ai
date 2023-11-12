<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import ProblemItem from "../components/ProblemItem.vue";
import { API } from "../api";
import { Problem, Player } from "../types";
import { translate } from "../locales/translate";
import { store } from "../store";
import SpinnerIcon from "../components/SpinnerIcon.vue";
import { useRoute, useRouter } from "vue-router";
import PaginationBar from "../components/PaginationBar.vue";

const route = useRoute();
const router = useRouter();

const problems = ref<Problem[]>([]);
const voted = ref<boolean[]>([]);
const player = ref<Player | null>(null);
const votedProblemIDs = ref<number[]>([]);
const loading = ref<boolean>(false);

const orderBy = ref<string>("-id");
const page = ref<number>(0);
const totalCount = ref<number>(0);
const LIMIT = 10;

onMounted(async () => {
  loading.value = true;

  handleQueryString();
  const api = new API();

  player.value = await api.getPlayerByUserID(store.userID);

  const myVotes = await api.getVotes(null, player.value.id);
  votedProblemIDs.value = myVotes.map((v) => v.problemID);

  await updateProblems();

  loading.value = false;
});

const handleQueryString = () => {
  if (route.query.order_by) {
    orderBy.value = route.query.order_by;
  }
  if (route.query.page) {
    page.value = parseInt(route.query.page, 10);
  }
};

const setQueryString = () => {
  router.push({
    query: {
      order_by: orderBy.value,
      page: page.value,
    },
  });
};

watch(orderBy, async () => {
  loading.value = true;

  page.value = 0;
  await updateProblems();

  loading.value = false;
});

const handlePageClicked = async (pg: number) => {
  if (pg === page.value) {
    return;
  }
  loading.value = true;

  page.value = pg;
  await updateProblems();

  loading.value = false;
};

const updateProblems = async () => {
  const api = new API();

  const res = await api.getProblems(page.value, orderBy.value, LIMIT);
  problems.value = res.problems;
  voted.value = problems.value.map((p) => votedProblemIDs.value.includes(p.id));
  totalCount.value = res.totalCount;
  setQueryString();
};
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
    <div v-else-if="voted.length > 0">
      <div class="mt-4">
        <ProblemItem
          v-for="(problem, idx) in problems"
          :problem="problem"
          :voted="voted[idx]"
          :key="problem.id"
        />
      </div>
      <PaginationBar
        :page="page"
        :limit="LIMIT"
        :totalCount="totalCount"
        @pageClicked="handlePageClicked"
      />
    </div>
  </div>
</template>
