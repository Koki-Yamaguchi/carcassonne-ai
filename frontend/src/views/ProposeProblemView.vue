<script setup lang="ts">
import { translate } from "../locales/translate";
import { ref, onMounted } from "vue";
import { newTile, idToTileKind } from "../tiles";
import ChevronIcon from "../components/ChevronIcon.vue";
import { API } from "../api";
import { Player } from "../types";

const tableID = ref<string>("");
const remainingTileCount = ref<number | null>(null);
const tileID = ref<number>(0);
const allTileIDs = ref<number[]>(Array.from(Array(24).keys()));
const selectOpen = ref<boolean>(false);
const player = ref<Player | null>(null);
import { store } from "../store";

const propose = async () => {
  if (!player.value || !remainingTileCount.value) {
    return;
  }
  console.log({ tableID, tileID, remainingTileCount });
  const api = new API();

  await api.createProblemProposal(
    tableID.value,
    remainingTileCount.value,
    player.value.id,
    tileID.value
  );
};

onMounted(async () => {
  const api = new API();

  player.value = await api.getPlayerByUserID(store.userID);
});
</script>

<template>
  <div class="m-10">
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
  </div>
</template>
