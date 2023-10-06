<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { API } from "../api";
import GameItems from "../components/GameItems.vue";
import PlayerRanking from "../components/PlayerRanking.vue";
import { translate } from "../locales/translate";
import { store } from "../store";
import { Game, Player } from "../types";

const router = useRouter();
const player = ref<Player | null>(null);
const players = ref<Player[]>([]);
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
    true
  );
  router.push(`/games/${game.id}`);
};

const seeMore = async () => {
  if (!player.value) {
    return;
  }
  router.push({
    path: "/games",
    query: { player: player.value.id, is_rated: "true" },
  });
};

onMounted(async () => {
  const api = new API();
  player.value = await api.getPlayerByUserID(store.userID);
  players.value = await api.getPlayers();
  games.value = await api.getGames(player.value.id, true, 5);
});
</script>
<template>
  <div class="p-6">
    <p>{{ translate("competitive_mode") }}</p>
    <p class="text-sm text-gray-700">
      {{ translate("competitive_mode_description") }}
    </p>
    <div class="flex flex-col items-center">
      <button
        class="mt-4 bg-gray-500 hover:bg-gray-400 text-[#eeeeee] rounded px-4 py-2"
        @click="createGame"
      >
        {{ translate("play_rated_match") }}
      </button>
    </div>
    <PlayerRanking :players="players" />
    <GameItems
      class="mt-4"
      :games="games"
      :pointOfViewPlayerID="player ? player.id : null"
    />
    <div
      @click="seeMore"
      class="text-gray-500 hover:cursor-pointer underline text-xs text-right mt-2 mr-2"
    >
      {{ translate("see_more") }}
    </div>
  </div>
</template>
