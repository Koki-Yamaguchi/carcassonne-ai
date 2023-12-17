<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import ProblemItem from "../components/ProblemItem.vue";
import { API } from "../api";
import { Problem, Player, Vote } from "../types";
import { translate, isJapaneseSetting } from "../locales/translate";
import { store } from "../store";
import SpinnerIcon from "../components/SpinnerIcon.vue";
import { useRoute, useRouter } from "vue-router";
import PaginationBar from "../components/PaginationBar.vue";
import RecentVoteItems from "../components/RecentVoteItems.vue";

const route = useRoute();
const router = useRouter();

const problems = ref<Problem[]>([]);
const voted = ref<boolean[]>([]);
const player = ref<Player | null>(null);
const votedProblemIDs = ref<number[]>([]);
const loading = ref<boolean>(false);

const orderBy = ref<string>("-start_at");
const page = ref<number>(0);
const totalCount = ref<number>(0);
const LIMIT = 10;

const recentVotes = ref<Vote[]>([]);

onMounted(async () => {
  loading.value = true;

  handleQueryString();
  const api = new API();

  player.value = await api.getPlayerByUserID(store.userID);

  const myVotes = await api.getVotes(null, player.value.id);
  votedProblemIDs.value = myVotes.map((v) => v.problemID);

  await updateProblems();

  recentVotes.value = await api.getVotes(null, null);

  loading.value = false;
});

const handleQueryString = () => {
  if (route.query.order_by) {
    orderBy.value = route.query.order_by as string;
  }
  if (route.query.page) {
    page.value = parseInt(route.query.page as string, 10);
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

const handleClickProblemName = (problemID: number) => {
  router.push(`/problems/${problemID}`);
};
</script>

<template>
  <div class="p-6">
    <p class="text-lg">{{ translate("problems") }}</p>
    <p class="my-2 text-sm text-gray-700">
      {{ translate("problems_description") }}
    </p>
    <p class="mt-4">{{ translate("propose_problem") }}</p>
    <div v-if="player" class="my-2">
      <p v-if="isJapaneseSetting()" class="text-sm text-gray-700">
        問題案の提供は<a
          :href="`https://docs.google.com/forms/d/e/1FAIpQLSe85DfoZf7m1bOT8Gs_e3j6OqelPm04RAO1wkU8j-pKPeK4pw/viewform?usp=pp_url&entry.1931067216=${player.name}`"
          target="_blank"
          rel="noopener noreferrer"
          class="underline"
          >こちら</a
        >から行うことができます。
      </p>
      <p v-else class="text-sm text-gray-700">
        You can propose problem ideas from
        <a
          :href="`https://docs.google.com/forms/d/e/1FAIpQLSeFIPJ5KVZoZEJOF6PB-qS6lWONhp2oO_kPQRrb_VoCUT9tBA/viewform?usp=pp_url&entry.1905734667=${player.name}`"
          target="_blank"
          rel="noopener noreferrer"
          class="underline"
          >here</a
        >.
      </p>
    </div>
    <p class="mt-4">{{ translate("recent_votes") }}</p>
    <div v-if="recentVotes.length > 0" class="mt-2">
      <RecentVoteItems
        :votes="recentVotes"
        :handleClickProblemName="handleClickProblemName"
      />
    </div>
    <p class="mt-4">{{ translate("problem_list") }}</p>
    <select
      class="text-xs border-2 rounded py-1 px-2 mt-2 text-gray-700 focus:outline-none focus:bg-white focus:border-green-300"
      v-model="orderBy"
    >
      <option value="-start_at">{{ translate("newest") }}</option>
      <option value="start_at">{{ translate("oldest") }}</option>
      <option value="-vote_count">{{ translate("most_voted") }}</option>
      <option value="vote_count">{{ translate("least_voted") }}</option>
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
