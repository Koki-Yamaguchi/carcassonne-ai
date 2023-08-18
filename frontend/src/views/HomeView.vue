<script setup lang="ts">
import { API } from "./../api";
import { useRouter } from "vue-router";
import { store } from "../store";
import { Game, Player } from "../types";
import { onMounted, ref } from "vue";
import GameItems from "../components/GameItems.vue";
import { translate } from "../locales/translate";

const router = useRouter();
const player = ref<Player | null>(null);
const games = ref<Game[]>([]);

const createGame = async () => {
  if (!player.value) {
    return;
  }
  const api = new API();
  const player0ID = player.value.id;
  const player1ID = 1;
  const player0Color = player.value.meepleColor;
  const player1Color = player0Color === "red" ? "yellow" : "red";
  const game = await api.createGame(
    player0ID,
    player1ID,
    player0Color,
    player1Color,
    false
  );
  router.push(`/games/${game.id}`);
};

const seeMore = async () => {
  if (!player.value) {
    return;
  }
  router.push({
    path: "/games",
    query: { player: player.value.id, is_rated: "false" },
  });
};
onMounted(async () => {
  const api = new API();
  player.value = await api.getPlayer(store.userID);
  games.value = await api.getGames(player.value.id, false, 5);
});
</script>

<template>
  <div class="p-6">
    <p class="text-sm text-gray-700">{{ translate("description") }}</p>
    <p class="mt-6">{{ translate("normal_mode") }}</p>
    <p class="text-sm text-gray-700">
      {{ translate("normal_mode_description") }}
    </p>
    <div class="flex flex-col items-center">
      <button
        class="bg-gray-500 hover:bg-gray-400 text-[#eeeeee] rounded px-4 py-2 mt-2"
        @click="createGame"
      >
        {{ translate("play_now") }}
      </button>
    </div>
    <GameItems class="mt-4" :games="games" />
    <div
      @click="seeMore"
      class="text-gray-500 underline text-xs text-right mt-2 mr-2"
    >
      {{ translate("see_more") }}
    </div>
  </div>
</template>
