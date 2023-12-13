<script setup lang="ts">
import { translate } from "../locales/translate";
import { ref, onMounted, computed } from "vue";
import { newTile, idToTileKind } from "../tiles";
import ChevronIcon from "../components/ChevronIcon.vue";
import ProposalItems from "../components/ProposalItems.vue";
import DraftProblemItems from "../components/DraftProblemItems.vue";
import { API } from "../api";
import { Player, ProblemProposal, Problem } from "../types";

const tableID = ref<string>("");
const remainingTileCount = ref<string>("");
const tileID = ref<number>(0);
const creatorID = ref<string>("");
const allTileIDs = ref<number[]>(Array.from(Array(24).keys()));
const selectOpen = ref<boolean>(false);
const player = ref<Player | null>(null);
const proposals = ref<ProblemProposal[]>([]);
const draftProblems = ref<Problem[]>([]);
import { store } from "../store";

const parseNumber = (value: string): number => {
  const parsed = parseInt(value, 10);
  if (Number.isNaN(parsed)) {
    return -1;
  }
  return parsed;
};

const propose = async () => {
  if (!player.value || !remainingTileCount.value) {
    return;
  }
  const api = new API();

  const rem = parseNumber(remainingTileCount.value);
  if (rem === -1) {
    alert("remaining tile count is not valid");
    return;
  }

  const cid = parseNumber(creatorID.value);
  if (creatorID.value !== "" && cid === -1) {
    alert("creator id is not valid");
    return;
  }

  await api.createProblemProposal(
    tableID.value,
    rem,
    creatorID.value !== "" ? cid : player.value.id,
    tileID.value
  );
};

onMounted(async () => {
  const api = new API();

  player.value = await api.getPlayerByUserID(store.userID);

  proposals.value = await api.getProblemProposals(player.value.id);

  const res = await api.getDraftProblems(player.value.id);

  draftProblems.value = res.problems;
});

const isAdmin = computed(() => {
  return player.value && player.value.id === 2;
});
</script>

<template>
  <div class="m-6">
    <p class="text-gray-600 text-lg mb-2">問題を提案する</p>
    <p class="text-sm text-gray-600 mb-2">
      BGA のテーブル ID
      とタイル残り枚数と配置するタイルを指定して問題の提案をすることができます。
    </p>
    <form class="w-full max-w-sm">
      <div class="md:flex md:items-center mb-6">
        <div class="md:w-1/3">
          <label
            class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4"
            for="inline-table-id"
          >
            {{ translate("table_id") }}
          </label>
        </div>
        <div class="md:w-2/3">
          <input
            class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-green-300"
            id="inline-table-id"
            type="text"
            v-model="tableID"
          />
        </div>
      </div>
      <div class="md:flex md:items-center mb-6">
        <div class="md:w-1/3">
          <label
            class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4"
            for="inline-remaining-tile-count"
          >
            {{ translate("remaining_tile_count") }}
          </label>
        </div>
        <div class="md:w-2/3">
          <input
            class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-green-300"
            id="inline-remaining-tile-count"
            v-model="remainingTileCount"
          />
        </div>
      </div>
      <div v-if="isAdmin" class="md:flex md:items-center mb-6">
        <div class="md:w-1/3">
          <label
            class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4"
            for="creator-id"
          >
            creator id
          </label>
        </div>
        <div class="md:w-2/3">
          <input
            class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-green-300"
            id="creator-id"
            v-model="creatorID"
          />
        </div>
      </div>
      <div class="md:flex md:items-center mb-6">
        <div class="md:w-1/3">
          <label class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4">
            {{ translate("placing_tile") }}
          </label>
        </div>
        <div class="md:w-2/3 relative">
          <div class="flex">
            <img
              class="w-10"
              :src="newTile(0, idToTileKind(tileID), null, -1, -1).src"
            />
            <div
              class="hover:cursor-pointer"
              @click="
                () => {
                  selectOpen = !selectOpen;
                }
              "
            >
              <ChevronIcon :direction="selectOpen ? 'right' : 'bottom'" />
            </div>
          </div>
          <div class="absolute flex flex-wrap" v-if="selectOpen">
            <div
              class="w-10 hover:cursor-pointer"
              v-for="id in allTileIDs"
              :value="id"
              :key="id"
            >
              <img
                :src="newTile(0, idToTileKind(id), null, -1, -1).src"
                @click="
                  () => {
                    tileID = id;
                    selectOpen = false;
                  }
                "
              />
            </div>
          </div>
        </div>
      </div>
      <div class="md:flex md:items-center">
        <div class="md:w-1/3"></div>
        <div class="md:w-2/3">
          <button
            class="shadow bg-green-500 hover:bg-green-400 focus:shadow-outline focus:outline-none text-white py-2 px-4 rounded"
            type="button"
            @click="propose"
          >
            {{ translate("propose") }}
          </button>
        </div>
      </div>
    </form>
    <hr class="my-4" />
    <div>
      <p class="text-gray-600 text-lg mb-2">提案した問題</p>
      <ProposalItems :proposals="proposals" />
    </div>
    <hr class="my-4" />
    <div>
      <p class="text-gray-600 text-lg mb-2">レビュー待ちの問題</p>
      <DraftProblemItems :problems="draftProblems" />
    </div>
  </div>
</template>
