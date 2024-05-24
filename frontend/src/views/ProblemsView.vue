<script setup lang="ts">
import { onMounted, ref } from "vue";
import ProblemItem from "../components/ProblemItem.vue";
import { API } from "../api";
import { Problem, Player, Vote, Creator } from "../types";
import { translate } from "../locales/translate";
import { store } from "../store";
import SpinnerIcon from "../components/SpinnerIcon.vue";
import { useRoute, useRouter } from "vue-router";
import PaginationBar from "../components/PaginationBar.vue";
import RecentVoteItems from "../components/RecentVoteItems.vue";

const route = useRoute();
const router = useRouter();

const problems = ref<Problem[]>([]);
const player = ref<Player | null>(null);
const loading = ref<boolean>(false);

const orderBy = ref<string>("-start_at");
const page = ref<number>(0);
const totalCount = ref<number>(0);
const creator = ref<number>(-1);
const LIMIT = 10;

const recentVotes = ref<Vote[]>([]);

const creators = ref<Creator[]>([]);

onMounted(async () => {
  loading.value = true;

  handleQueryString();
  const api = new API();

  player.value = await api.getPlayerByUserID(store.userID);

  await updateProblems();

  recentVotes.value = await api.getVotes(null, null);

  creators.value = await api.getCreators();

  loading.value = false;
});

const handleQueryString = () => {
  if (route.query.order_by) {
    orderBy.value = route.query.order_by as string;
  }
  if (route.query.page) {
    page.value = parseInt(route.query.page as string, 10);
  }
  if (route.query.creator) {
    creator.value = parseInt(route.query.creator as string, 10);
  }
};

const setQueryString = () => {
  router.push({
    query: {
      order_by: orderBy.value,
      page: page.value,
      creator: creator.value,
    },
  });
};

const handleChange = async () => {
  loading.value = true;

  page.value = 0;
  await updateProblems();

  loading.value = false;
};

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
  if (!player.value) {
    return;
  }

  const api = new API();

  const res = await api.getProblems(
    page.value,
    orderBy.value,
    LIMIT,
    player.value.id,
    creator.value !== -1 ? creator.value : undefined
  );
  problems.value = res.problems;
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
    <p class="mt-4">{{ translate("recent_votes") }}</p>
    <div v-if="recentVotes.length > 0" class="mt-2">
      <RecentVoteItems
        :votes="recentVotes"
        :handleClickProblemName="handleClickProblemName"
      />
    </div>
    <p class="mt-4">{{ translate("problem_list") }}</p>
    <div class="flex gap-2">
      <select
        class="text-xs border-2 rounded py-1 px-2 mt-2 text-gray-700 focus:outline-none focus:bg-white focus:border-green-300"
        @change="handleChange"
        v-model="orderBy"
      >
        <option value="-start_at">{{ translate("newest") }}</option>
        <option value="start_at">{{ translate("oldest") }}</option>
        <option value="-vote_count">{{ translate("most_voted") }}</option>
        <option value="vote_count">{{ translate("least_voted") }}</option>
        <option value="-favorite_count">
          {{ translate("most_favorited") }}
        </option>
        <option value="favorite_count">
          {{ translate("least_favorited") }}
        </option>
      </select>
      <select
        class="text-xs border-2 rounded py-1 px-2 mt-2 text-gray-700 focus:outline-none focus:bg-white focus:border-green-300"
        @change="handleChange"
        v-model="creator"
      >
        <option :value="-1">{{ translate("created_by") }}</option>
        <option
          v-for="creator in creators"
          :key="creator.id"
          :value="creator.id"
        >
          {{ creator.name }}
        </option>
      </select>
    </div>
    <div v-if="loading"><SpinnerIcon /></div>
    <div>
      <div class="mt-4">
        <ProblemItem
          v-for="problem in problems"
          :problem="problem"
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
